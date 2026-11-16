# OpenDesk — Database Schema & Data Architecture (PostgreSQL 17 & Redis)

## 1. Relational Database Schema (PostgreSQL 17)

The relational schema is managed using Entity Framework Core 10 code-first migrations with strict foreign key constraints, UUID v7 primary keys, and comprehensive indexing.

```sql
-- OpenDesk Production PostgreSQL Schema Baseline

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Organizations (Multi-tenancy)
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(150) NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,
    plan_tier VARCHAR(32) NOT NULL DEFAULT 'community',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(120) NOT NULL,
    role VARCHAR(32) NOT NULL DEFAULT 'Member', -- SuperAdmin, Admin, Technician, Member
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    two_factor_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    two_factor_secret VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_org_email ON users(organization_id, email);

-- 3. Refresh Tokens
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(128) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_refresh_tokens_user ON refresh_tokens(user_id, token_hash);

-- 4. Devices (Registered Hosts)
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    hardware_fingerprint VARCHAR(128) NOT NULL,
    ed25519_public_key VARCHAR(64) NOT NULL,
    hostname VARCHAR(128) NOT NULL,
    alias VARCHAR(128),
    operating_system VARCHAR(32) NOT NULL, -- Windows, Linux, macOS, Android, iOS
    os_version VARCHAR(64) NOT NULL,
    architecture VARCHAR(16) NOT NULL,     -- x86_64, aarch64, armv7
    agent_version VARCHAR(32) NOT NULL,
    is_unattended_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    last_known_ip INET,
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_device_org_fingerprint UNIQUE(organization_id, hardware_fingerprint)
);

CREATE INDEX idx_devices_org_status ON devices(organization_id, last_seen_at);

-- 5. Device Assignments (User-to-Device Permissions)
CREATE TABLE device_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_level VARCHAR(32) NOT NULL DEFAULT 'FullControl', -- FullControl, ViewOnly, FileTransferOnly
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_device_user_assignment UNIQUE(device_id, user_id)
);

-- 6. Remote Sessions (Active & Historical)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_code VARCHAR(16),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    access_mode VARCHAR(24) NOT NULL,      -- Unattended, RemoteSupport
    transport_mode VARCHAR(16) NOT NULL,   -- DirectP2P, TurnRelay
    client_ip INET NOT NULL,
    host_ip INET NOT NULL,
    bytes_transmitted BIGINT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMPTZ,
    termination_reason VARCHAR(64)        -- UserEnded, HostEnded, Timeout, Error
);

CREATE INDEX idx_sessions_device_started ON sessions(device_id, started_at DESC);
CREATE INDEX idx_sessions_user_started ON sessions(user_id, started_at DESC);

-- 7. Audit Logs (Append-Only Immutable Ledger)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(64) NOT NULL,          -- DeviceClaimed, SessionStarted, SessionEnded, FileTransferred, PermissionGranted
    resource_type VARCHAR(32) NOT NULL,
    resource_id VARCHAR(64) NOT NULL,
    ip_address INET,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audit_org_timestamp ON audit_logs(organization_id, created_at DESC);
CREATE INDEX idx_audit_action ON audit_logs(action, created_at DESC);
```

---

## 2. Redis Key Architecture & Ephemeral State

Redis 7.4 manages volatile runtime state, caching, and pub/sub distribution:

| Key Pattern | Data Type | TTL | Purpose |
|---|---|---|---|
| `presence:device:{deviceId}` | Hash | 45 seconds | Active host heartbeat, connection ID, IP address, and system load. |
| `presence:user:{userId}` | Set | 120 seconds | Set of active WebSocket connection IDs for a user across devices. |
| `support:code:{sessionCode}` | Hash | 10 minutes | Attended session token, host device ID, and ephemeral PIN hash. |
| `ratelimit:{ip}:{endpoint}` | String (Counter)| 60 seconds | Sliding-window request counter for brute-force prevention. |
| `signalr:backplane:*` | Pub/Sub Channels| N/A | SignalR inter-node Redis message bus for scaling out hubs. |

---

## 3. Data Retention & Pruning Policies

1. **Audit Logs**:
   - Retained for a minimum of 365 days in production compliance environments.
   - Older records are compressed and archived to cold object storage (S3 / Glacier / MinIO) using automated partitioning.
2. **Session Records**:
   - Closed sessions (`ended_at IS NOT NULL`) are indexed and retained for 90 days.
3. **Volatile Support Codes**:
   - Automatically expired and pruned via Redis TTLs upon completion or after 10 minutes of inactivity.
