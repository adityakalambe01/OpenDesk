# OpenDesk — REST API Endpoints Specification

## 1. Authentication Endpoints (`/api/v1/auth`)

### `POST /api/v1/auth/login`
Authenticates a user and returns short-lived JWT access and refresh tokens.

**Request Body:**
```json
{
  "email": "technician@company.com",
  "password": "CorrectHorseBatteryStaple123!",
  "totpCode": "481920"
}
```

**Response (200 OK):**
```json
{
  "accessToken": "eyJhbGciOiJFUzI1NiIs...",
  "tokenType": "Bearer",
  "expiresInSeconds": 900,
  "refreshToken": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "user": {
    "id": "119e6679-7425-40de-944b-e07fc1f90ae1",
    "email": "technician@company.com",
    "fullName": "Aditya Sharma",
    "role": "Technician"
  }
}
```

---

## 2. Device Management Endpoints (`/api/v1/devices`)

### `POST /api/v1/devices/register`
Invoked by the OpenDesk host background daemon during initial installation.

**Request Body:**
```json
{
  "hardwareFingerprint": "9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d",
  "ed25519PublicKey": "e4d909c290d0fb1ca068ffaddf22cbd0a08e1a6d953a9f07bb22d3e1",
  "hostname": "WORKSTATION-WIN11",
  "operatingSystem": "Windows",
  "osVersion": "10.0.26100",
  "architecture": "x86_64",
  "agentVersion": "0.1.0"
}
```

**Response (201 Created):**
```json
{
  "deviceId": "482-195-302",
  "registeredAt": "2026-11-15T21:45:00Z",
  "deviceToken": "dev_sec_91823abf1092e"
}
```

### `GET /api/v1/devices`
Returns the list of devices registered or assigned to the authenticated user.

**Response (200 OK):**
```json
[
  {
    "id": "119e6679-7425-40de-944b-e07fc1f90ae1",
    "deviceId": "482-195-302",
    "hostname": "WORKSTATION-WIN11",
    "alias": "Office CAD Rig",
    "operatingSystem": "Windows",
    "isOnline": true,
    "lastSeenAt": "2026-11-15T21:47:00Z",
    "unattendedEnabled": true,
    "permissionLevel": "FullControl"
  }
]
```

---

## 3. Remote Support Session Endpoints (`/api/v1/sessions`)

### `POST /api/v1/sessions/support/create`
Generates a temporary attended session code and PIN for remote assistance.

**Response (201 Created):**
```json
{
  "sessionCode": "482-195-302",
  "pin": "K7M2W9",
  "expiresAt": "2026-11-15T21:57:00Z",
  "ttlSeconds": 600
}
```

### `POST /api/v1/sessions/{sessionId}/revoke`
Immediately terminates an active session.

**Response (204 No Content)**
