# OpenDesk — API Design Principles & Architecture

## 1. REST & Real-Time API Principles

OpenDesk exposes two primary communication interfaces:
1. **RESTful Management API**: Operates over HTTPS, handling authentication, device management, session history, and administrative operations.
2. **SignalR WebSocket Signaling Hub**: Operates over WSS (WebSockets), facilitating real-time bidirectional presence, session invitations, and WebRTC SDP/ICE exchange.

---

## 2. Standard Request & Response Conventions

- **Content Type**: `application/json; charset=utf-8`.
- **Date & Time**: ISO 8601 UTC format (`YYYY-MM-DDTHH:mm:ss.sssZ`).
- **Identifiers**: 128-bit RFC 4122 UUIDs (e.g., `4a7b3c21-8f90-4e12-b918-7264a938c110`).
- **Authentication**: `Authorization: Bearer <JWT_ACCESS_TOKEN>`.

---

## 3. Standardized Error Handling (RFC 7807 Problem Details)

All API errors return RFC 7807 compliant JSON payloads:

```json
{
  "type": "https://opendesk.io/errors/session-code-expired",
  "title": "Support Session Code Expired",
  "status": 410,
  "detail": "The session code 482-195-302 expired at 2026-11-15T21:40:00Z after 10 minutes of inactivity.",
  "instance": "/api/v1/sessions/support/join",
  "traceId": "00-9f8231ab421c08e5-e837194820a-01"
}
```

---

## 4. Rate Limiting Headers

To prevent API abuse and brute-force credential stuffing, responses include standard rate-limiting headers:
- `X-RateLimit-Limit`: Maximum requests permitted within the window (e.g. `60`).
- `X-RateLimit-Remaining`: Number of requests remaining in the current window.
- `X-RateLimit-Reset`: Unix timestamp when the rate limit window resets.
