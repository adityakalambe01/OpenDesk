# OpenDesk — API & Protocol Versioning Strategy

## 1. REST API Versioning Policy

OpenDesk standardizes on **URI Path Versioning** for all public HTTP endpoints:
- Format: `/api/v{major}/...` (e.g. `/api/v1/devices`).
- **Breaking Changes**: Increment the major version (e.g., `/api/v2`). Breaking changes include removing endpoints, renaming fields, or modifying HTTP verb semantics.
- **Non-Breaking Changes**: Adding optional query parameters, adding new non-mandatory JSON response fields, or creating new endpoints does not increment the API version.

---

## 2. API Deprecation Schedule & Headers (RFC 8594)

When an API version or endpoint is scheduled for sunset, responses will include standard HTTP deprecation headers:
```http
Deprecation: @1795046400
Sunset: Wed, 18 Nov 2026 00:00:00 GMT
Link: <https://opendesk.io/docs/migration/v1-to-v2>; rel="sunset"
```
- **Deprecation Window**: Deprecated endpoints remain fully operational for a minimum of **6 months** following public announcement before final decommissioning.

---

## 3. WebRTC Wire Protocol Version Negotiation

Binary data channels (`opendesk-control`, `opendesk-input`, `opendesk-file`) transmit a 1-byte version field in the packet header:
1. `ClientHello` advertises `supported_versions: [1, 2]`.
2. The host inspects the set and selects the highest mutually supported version in `HostHello`: `negotiated_version = 1`.
3. If no common protocol version exists, the connection is aborted with error code `0x000F: IncompatibleProtocolVersion`.
