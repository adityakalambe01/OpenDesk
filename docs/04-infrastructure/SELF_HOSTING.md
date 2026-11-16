# OpenDesk — Self-Hosting Administrator Guide

## 1. System Requirements & Prerequisites

OpenDesk provides a turn-key self-hosted deployment running via Docker Compose.

### Hardware Prerequisites:
- **Minimum**: 2 vCPU, 4 GB RAM, 20 GB SSD storage (supports ~100 concurrent devices).
- **Recommended**: 4 vCPU, 8 GB RAM, 50 GB NVMe storage (supports ~1,000 concurrent devices).
- **Network**: Dedicated Public IPv4 address; minimum 50 Mbps symmetrical upload/download bandwidth.

### DNS Records Setup:
Configure two DNS `A` records pointing to your server's public IP:
- `desk.yourdomain.com` $\rightarrow$ OpenDesk Web Dashboard & API
- `relay.yourdomain.com` $\rightarrow$ Coturn STUN/TURN Relay

---

## 2. Firewall Port Configuration

Open the following ports on your cloud provider (AWS Security Groups, DigitalOcean Firewall, Hetzner, or router):

| Port / Range | Protocol | Inbound / Outbound | Service | Purpose |
|---|:---:|:---:|---|---|
| **80** | TCP | Inbound | Nginx | HTTP (Let's Encrypt ACME verification) |
| **443** | TCP | Inbound | Nginx | HTTPS Web Dashboard & SignalR Hub |
| **3478** | UDP & TCP | Inbound | Coturn | Standard STUN / TURN port |
| **5349** | UDP & TCP | Inbound | Coturn | Secure TURNS (TLS) |
| **49152 - 65535** | UDP | Inbound | Coturn | Ephemeral media relay socket range |

---

## 3. Step-by-Step Installation

### Step 1: Clone Repository & Create Environment Configuration
```bash
git clone https://github.com/opendesk-io/opendesk.git /opt/opendesk
cd /opt/opendesk/deploy/docker-compose
cp .env.example .env
```

### Step 2: Configure Environment Variables (`.env`)
Generate strong cryptographic secrets:
```bash
# Generate secrets using openssl
POSTGRES_PASSWORD=$(openssl rand -hex 24)
REDIS_PASSWORD=$(openssl rand -hex 24)
JWT_SECRET_KEY=$(openssl rand -hex 32)
COTURN_SHARED_SECRET=$(openssl rand -hex 32)

cat <<EOF > .env
SERVER_DOMAIN=desk.yourdomain.com
RELAY_DOMAIN=relay.yourdomain.com
EXTERNAL_IP=$(curl -s ifconfig.me)

POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
REDIS_PASSWORD=${REDIS_PASSWORD}
JWT_SECRET_KEY=${JWT_SECRET_KEY}
COTURN_SHARED_SECRET=${COTURN_SHARED_SECRET}
EOF
```

### Step 3: Launch Services
```bash
docker compose up -d
```

### Step 4: Provision Initial SuperAdmin Account
```bash
docker compose exec server dotnet OpenDesk.Server.dll --seed-admin \
  --email "admin@yourdomain.com" \
  --password "TemporarySecurePassword123!"
```

### Step 5: Configure Desktop and Mobile Clients to Use Self-Hosted Server
In the OpenDesk Desktop or Mobile client:
1. Open **Settings $\rightarrow$ Network & Server**.
2. Select **Self-Hosted Instance**.
3. Enter Server URL: `https://desk.yourdomain.com`.
4. Click **Test & Apply Connection**.
