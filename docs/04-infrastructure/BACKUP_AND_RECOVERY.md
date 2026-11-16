# OpenDesk — Backup, Disaster Recovery & Data Migration

## 1. Disaster Recovery Objectives

- **Recovery Point Objective (RPO)**: $\le 1\text{ hour}$ (Maximum allowable data loss window).
- **Recovery Time Objective (RTO)**: $\le 15\text{ minutes}$ (Maximum time to restore full service after failure).

---

## 2. Automated PostgreSQL Backup Procedures

### A. Daily Logical Backup (`pg_dump`)
Automated via a scheduled host cron script executing inside the PostgreSQL container:

```bash
#!/bin/bash
# /opt/opendesk/scripts/backup-db.sh
set -eo pipefail

BACKUP_DIR="/var/backups/opendesk"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
FILENAME="${BACKUP_DIR}/opendesk_db_${TIMESTAMP}.sql.gz"

mkdir -p "${BACKUP_DIR}"

# Execute pg_dump with gzip compression
docker exec -t opendesk-postgres pg_dump -U opendesk opendesk | gzip > "${FILENAME}"

# Retain backups for 30 days locally
find "${BACKUP_DIR}" -type f -name "*.sql.gz" -mtime +30 -delete

echo "[INFO] Database backup completed: ${FILENAME}"
```

### B. Point-in-Time Recovery (PITR) for Enterprise Cloud
- PostgreSQL Write-Ahead Logs (WAL) are continuously archived to an S3/MinIO bucket using `WAL-G` or `pgBackRest`.
- Allows rolling back database state to any specific second within the last 14 days.

---

## 3. Restoration Runbook (Disaster Recovery)

If a host server suffers catastrophic hardware failure:

### Step 1: Deploy Fresh Docker Compose Environment
```bash
git clone https://github.com/opendesk-io/opendesk.git /opt/opendesk
cd /opt/opendesk/deploy/docker-compose
# Restore original .env file with identical cryptographic secrets
cp /secure-vault/opendesk.env .env
docker compose up -d postgres
```

### Step 2: Restore Database Dump
```bash
# Decompress and stream SQL dump into fresh database
gunzip -c /var/backups/opendesk/opendesk_db_latest.sql.gz | \
  docker exec -i opendesk-postgres psql -U opendesk -d opendesk
```

### Step 3: Launch Remaining Services
```bash
docker compose up -d
docker compose ps
```
- Devices running the OpenDesk background service will automatically reconnect to the newly restored server within their next heartbeat interval (maximum 20 seconds).
