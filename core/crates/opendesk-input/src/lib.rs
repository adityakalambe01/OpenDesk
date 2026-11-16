//! Platform-independent input injection contract and event rate limiting.
//!
//! Native backends implement [`InputBackend`] and are wrapped in
//! [`RateLimitedInputInjector`], which makes it impossible for callers using
//! the public [`InputInjector`] interface to bypass rate limiting.
#![forbid(unsafe_code)]

use std::{collections::VecDeque, sync::Mutex, time::Instant};

use opendesk_proto::InputMessage;
use thiserror::Error;

/// Result of applying the input rate limit to one event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjectionOutcome {
    /// The event was passed to the platform backend.
    Injected,
    /// The event was dropped because the one-second limit was reached.
    /// Callers should surface or aggregate this as an overflow alert.
    RateLimited,
}

/// Errors reported by platform input injection backends.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InputError {
    #[error("input injection permission denied")]
    PermissionDenied,
    #[error("unsupported input event: {0}")]
    Unsupported(String),
    #[error("input backend failed: {0}")]
    Backend(String),
    #[error("input rate limiter is unavailable because its lock was poisoned")]
    RateLimiterUnavailable,
}

/// Native platform backend. Applications should expose it through
/// [`RateLimitedInputInjector`] rather than calling it directly for remote input.
pub trait InputBackend: Send + Sync {
    fn inject(&self, event: &InputMessage) -> Result<(), InputError>;
}

/// Public injection interface. Its sealed implementation ensures events pass
/// through [`RateLimitedInputInjector`].
#[allow(private_bounds)]
pub trait InputInjector: Send + Sync + private::Sealed {
    /// Inject one event. A rate-limited event is dropped and reported as
    /// [`InjectionOutcome::RateLimited`]; the configured alert handler is
    /// invoked automatically for that outcome.
    fn inject(&self, event: &InputMessage) -> Result<InjectionOutcome, InputError>;
}

mod private {
    pub trait Sealed {}
}

impl<B: InputBackend> private::Sealed for RateLimitedInputInjector<B> {}

/// Thread-safe adapter that enforces a rolling-window event limit before
/// calling its native backend.
pub struct RateLimitedInputInjector<B> {
    backend: B,
    limiter: Mutex<RollingWindowLimiter>,
    on_rate_limited: Box<dyn Fn() + Send + Sync>,
}

impl<B> RateLimitedInputInjector<B> {
    /// Create a default 1,000 events/second injector. `on_rate_limited` is
    /// called at most once per limiter interval while events are being dropped.
    /// Each dropped event still returns [`InjectionOutcome::RateLimited`].
    pub fn new(backend: B, on_rate_limited: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            backend,
            limiter: Mutex::new(RollingWindowLimiter::default()),
            on_rate_limited: Box::new(on_rate_limited),
        }
    }

    /// Construct an injector with a custom rolling-window limit.
    pub fn with_limit(
        backend: B,
        capacity: usize,
        interval: std::time::Duration,
        on_rate_limited: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        Self {
            backend,
            limiter: Mutex::new(RollingWindowLimiter::new(capacity, interval)),
            on_rate_limited: Box::new(on_rate_limited),
        }
    }
}

impl<B: InputBackend> InputInjector for RateLimitedInputInjector<B> {
    fn inject(&self, event: &InputMessage) -> Result<InjectionOutcome, InputError> {
        let (allowed, should_alert) = self
            .limiter
            .lock()
            .map_err(|_| InputError::RateLimiterUnavailable)?
            .acquire_with_alert();
        if !allowed {
            if should_alert {
                (self.on_rate_limited)();
            }
            return Ok(InjectionOutcome::RateLimited);
        }

        self.backend.inject(event)?;
        Ok(InjectionOutcome::Injected)
    }
}

/// Rolling-window limiter that enforces a strict maximum over the preceding
/// interval, without allowing an initial burst.
#[derive(Debug)]
pub struct RollingWindowLimiter {
    capacity: usize,
    interval: std::time::Duration,
    accepted: VecDeque<Instant>,
    last_alert: Option<Instant>,
}

impl Default for RollingWindowLimiter {
    fn default() -> Self {
        Self::new(1_000, std::time::Duration::from_secs(1))
    }
}

impl RollingWindowLimiter {
    /// Create a strict rolling-window limit. Zero capacity is clamped to one;
    /// a zero interval is clamped to one nanosecond.
    pub fn new(capacity: usize, interval: std::time::Duration) -> Self {
        Self {
            capacity: capacity.max(1),
            interval: interval.max(std::time::Duration::from_nanos(1)),
            accepted: VecDeque::new(),
            last_alert: None,
        }
    }

    /// Consume one available event allowance. Returns false when the event
    /// must be dropped and surfaced as an overflow alert.
    pub fn try_acquire(&mut self) -> bool {
        self.acquire_with_alert().0
    }

    fn acquire_with_alert(&mut self) -> (bool, bool) {
        self.acquire_with_alert_at(Instant::now())
    }

    fn acquire_with_alert_at(&mut self, now: Instant) -> (bool, bool) {
        while self
            .accepted
            .front()
            .is_some_and(|accepted| now.duration_since(*accepted) >= self.interval)
        {
            self.accepted.pop_front();
        }
        if self.accepted.len() >= self.capacity {
            let should_alert = match self.last_alert {
                Some(last_alert) => now.duration_since(last_alert) >= self.interval,
                None => true,
            };
            if should_alert {
                self.last_alert = Some(now);
            }
            return (false, should_alert);
        }
        self.accepted.push_back(now);
        (true, false)
    }

    /// Remaining allowances in the current rolling interval.
    pub fn available_capacity(&self) -> usize {
        let now = Instant::now();
        let active = self
            .accepted
            .iter()
            .filter(|accepted| now.duration_since(**accepted) < self.interval)
            .count();
        self.capacity.saturating_sub(active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingBackend(AtomicUsize);

    impl InputBackend for CountingBackend {
        fn inject(&self, _event: &InputMessage) -> Result<(), InputError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[test]
    fn rolling_window_drops_overflow_until_an_event_expires() {
        let start = Instant::now();
        let mut limiter = RollingWindowLimiter::new(2, std::time::Duration::from_secs(1));

        assert_eq!(limiter.acquire_with_alert_at(start), (true, false));
        assert_eq!(
            limiter.acquire_with_alert_at(start + std::time::Duration::from_millis(1)),
            (true, false)
        );
        assert_eq!(
            limiter.acquire_with_alert_at(start + std::time::Duration::from_millis(2)),
            (false, true)
        );
        assert_eq!(
            limiter.acquire_with_alert_at(start + std::time::Duration::from_millis(3)),
            (false, false)
        );
        assert_eq!(
            limiter.acquire_with_alert_at(start + std::time::Duration::from_secs(1)),
            (true, false)
        );
    }

    #[test]
    fn injector_drops_excess_events_and_throttles_overflow_alerts() {
        let backend = CountingBackend(AtomicUsize::new(0));
        let alert_count = std::sync::Arc::new(AtomicUsize::new(0));
        let alert_counter = std::sync::Arc::clone(&alert_count);
        let injector = RateLimitedInputInjector::with_limit(
            backend,
            1,
            std::time::Duration::from_secs(60),
            move || {
                alert_counter.fetch_add(1, Ordering::Relaxed);
            },
        );
        let event = InputMessage::SystemAction(opendesk_proto::SystemAction::LockWorkstation);

        assert_eq!(injector.inject(&event), Ok(InjectionOutcome::Injected));
        assert_eq!(injector.inject(&event), Ok(InjectionOutcome::RateLimited));
        assert_eq!(injector.inject(&event), Ok(InjectionOutcome::RateLimited));
        assert_eq!(injector.backend.0.load(Ordering::Relaxed), 1);
        assert_eq!(alert_count.load(Ordering::Relaxed), 1);
    }
}
