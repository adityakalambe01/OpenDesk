use crate::ProtocolMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMessage {
    MouseMove {
        display_id: u32,
        x_norm: f64,
        y_norm: f64,
    },
    MouseDown {
        button: MouseButton,
        click_count: u8,
    },
    MouseUp {
        button: MouseButton,
    },
    MouseWheel {
        delta_x: f64,
        delta_y: f64,
    },
    KeyDown {
        scancode: u32,
        modifiers: u16,
        unicode_char: Option<u32>,
    },
    KeyUp {
        scancode: u32,
        modifiers: u16,
    },
    TextComposition {
        text: String,
    },
    TouchGesture {
        gesture: String,
        x_norm: f64,
        y_norm: f64,
    },
    SystemAction(SystemAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemAction {
    SendCtrlAltDel,
    LockWorkstation,
    ShowDesktop,
    OpenTaskSwitcher,
}

impl ProtocolMessage for InputMessage {
    fn message_type(&self) -> u16 {
        match self {
            Self::MouseMove { .. } => 0x0101,
            Self::MouseDown { .. } => 0x0102,
            Self::MouseUp { .. } => 0x0103,
            Self::MouseWheel { .. } => 0x0104,
            Self::KeyDown { .. } => 0x0105,
            Self::KeyUp { .. } => 0x0106,
            Self::TouchGesture { .. } => 0x0107,
            Self::SystemAction(_) => 0x0108,
            Self::TextComposition { .. } => 0x0109,
        }
    }

    fn validate(&self) -> Result<(), &'static str> {
        let normalized = |value: f64| value.is_finite() && (0.0..=1.0).contains(&value);
        match self {
            Self::MouseMove { x_norm, y_norm, .. } | Self::TouchGesture { x_norm, y_norm, .. }
                if !normalized(*x_norm) || !normalized(*y_norm) =>
            {
                Err("normalized coordinates must be finite and between 0 and 1")
            }
            Self::MouseDown { click_count: 0, .. } => Err("click count must be greater than zero"),
            Self::MouseWheel { delta_x, delta_y }
                if !delta_x.is_finite() || !delta_y.is_finite() =>
            {
                Err("mouse wheel deltas must be finite")
            }
            Self::KeyDown {
                unicode_char: Some(value),
                ..
            } if char::from_u32(*value).is_none() => {
                Err("unicode_char must be a valid Unicode scalar value")
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode, encode};
    #[test]
    fn input_variants_roundtrip() {
        let messages = [
            InputMessage::MouseMove {
                display_id: 1,
                x_norm: 0.5,
                y_norm: 1.0,
            },
            InputMessage::MouseDown {
                button: MouseButton::Left,
                click_count: 2,
            },
            InputMessage::MouseUp {
                button: MouseButton::Right,
            },
            InputMessage::MouseWheel {
                delta_x: -1.0,
                delta_y: 2.0,
            },
            InputMessage::KeyDown {
                scancode: 42,
                modifiers: 3,
                unicode_char: Some(65),
            },
            InputMessage::KeyUp {
                scancode: 42,
                modifiers: 3,
            },
            InputMessage::TextComposition {
                text: "hello".into(),
            },
            InputMessage::TouchGesture {
                gesture: "tap".into(),
                x_norm: 0.2,
                y_norm: 0.8,
            },
            InputMessage::SystemAction(SystemAction::LockWorkstation),
        ];
        for message in messages {
            assert_eq!(
                decode::<InputMessage>(&encode(&message).unwrap()).unwrap(),
                message
            );
        }
    }

    #[test]
    fn rejects_invalid_coordinates_and_non_finite_wheel_deltas() {
        assert!(crate::encode(&InputMessage::MouseMove {
            display_id: 0,
            x_norm: f64::NAN,
            y_norm: 0.5,
        })
        .is_err());
        assert!(crate::encode(&InputMessage::TouchGesture {
            gesture: "tap".into(),
            x_norm: 1.01,
            y_norm: 0.5,
        })
        .is_err());
        assert!(crate::encode(&InputMessage::MouseWheel {
            delta_x: 0.0,
            delta_y: f64::INFINITY,
        })
        .is_err());
    }
}
