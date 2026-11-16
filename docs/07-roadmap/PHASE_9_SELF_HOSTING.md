# OpenDesk Phase 9 — Self-Hosting Infrastructure & Turnkey Deployment

## 1. Phase Objective
Produce a hardened, turnkey Docker Compose deployment package with automated TLS termination, self-hosted PostgreSQL 17, Redis 7.4, Coturn STUN/TURN relay, comprehensive backup/recovery runbooks, and zero-downtime database migration tooling.

## 2. Why This Phase Is Necessary
OpenDesk's core mission is sovereign self-hosting. Privacy-conscious organizations, healthcare providers, financial institutions, and homelab enthusiasts require the ability to run their own independent infrastructure with a single terminal command, without external cloud dependencies.

## 3. Scope
- Authoring production-ready multi-stage Dockerfiles for `opendesk-server`.
- Creating the production `docker-compose.yml` with health checks, restart policies, and resource constraints.
- Configuring Nginx reverse proxy with automated Let's Encrypt SSL/TLS certificate renewal.
- Configuring coturn with host networking and dynamic ephemeral HMAC-SHA1 authentication.
- Implementing database migration automation via `dotnet ef database update` on container startup.
- Documenting backup, restoration, disaster recovery, and server upgrade procedures.

## 4. Prerequisites
Completion of Phase 6 (Cross-Platform) and Phase 8 (Backend Dashboard).

## 5. Technical Implementation Details
- Uses official minimal base images (Chiseled Ubuntu or Alpine) to minimize container attack surface.
- Coturn runs with host networking to eliminate Docker bridge NAT overhead on high-throughput UDP media packets.
- Nginx terminates TLS 1.3 and reverse-proxies HTTP API traffic to port 5000 and WebSocket traffic with long-lived timeouts.

## 6. Exact Folder and File Changes Expected
```
deploy/
├── docker/
│   ├── Dockerfile.server
│   ├── Dockerfile.coturn
│   └── Dockerfile.nginx
├── docker-compose/
│   ├── docker-compose.yml
│   ├── .env.example
│   └── nginx/
│       └── nginx.conf
└── scripts/
    ├── backup-db.sh
    ├── restore-db.sh
    └── generate-secrets.sh
```

## 7. Individual Development Tasks
- **P9-001**: Author optimized multi-stage `Dockerfile.server` for ASP.NET Core 10 backend.
- **P9-002**: Author production `docker-compose.yml` orchestrating Server, Postgres, Redis, Coturn, and Nginx.
- **P9-003**: Configure Nginx reverse proxy with automated Let's Encrypt TLS renewal via Certbot.
- **P9-004**: Configure hardened `turnserver.conf` with IP filtering and ephemeral authentication.
- **P9-005**: Implement automated database migration execution on container startup.
- **P9-006**: Author shell scripts for secret generation, database backup, and disaster restoration.
- **P9-007**: Validate full clean installation on fresh Linux cloud VPS instances (Ubuntu 22.04 / 24.04).

## 8. Dependencies
Phase 6 (Desktop Cross-Platform) and Phase 8 (Backend Dashboard).

## 9. Expected Outputs
A production-grade, single-command self-hosting stack deployable on any Linux server with Docker.

## 10. Unit Testing Requirements
- Unit tests verifying that configuration loaders properly parse environment variables and fail fast if mandatory secrets are missing.

## 11. Integration Testing Requirements
Automated integration test spinning up the complete Docker Compose stack and verifying healthy HTTP and WebSocket status codes on port 443.

## 12. Manual Verification Steps
1. Provision fresh Ubuntu 24.04 VPS with public IP and DNS records.
2. Clone repository and run `generate-secrets.sh`.
3. Execute `docker compose up -d`.
4. Open `https://desk.yourdomain.com` in browser $\rightarrow$ Verify valid Let's Encrypt certificate and login screen.
5. Connect desktop client to self-hosted server $\rightarrow$ Verify P2P and TURN relay connections succeed.

## 13. Acceptance Criteria
- Entire server stack deploys in $< 3\text{ minutes}$ using `docker compose up -d`.
- 100% feature parity between self-hosted deployment and cloud architecture.
- Backup and restore script restores full database state in $< 60\text{ seconds}$.

## 14. Definition of Done
Self-hosted Docker Compose stack tested and verified on clean cloud instances with complete administrator documentation.

## 15. Known Risks
- Users misconfiguring firewall ports for UDP Coturn ranges.
  - *Mitigation*: In-app diagnostic tool checks STUN/TURN connectivity and alerts administrator to blocked ports.

## 16. Estimated Complexity
Medium (Docker networking, Nginx TLS, Linux systems administration).

## 17. Next-Phase Dependencies
Unlocks **Phase 10 — Testing, Hardening & Public Beta Release**.
