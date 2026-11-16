# OpenDesk — Local Development Environment Setup Guide

## 1. Toolchain & Runtime Prerequisites

Ensure the following runtimes and compilers are installed prior to repository bootstrapping:

| Component | Minimum Version | Installation Command / Link |
|---|---|---|
| **Rust** | 1.82.0+ (Stable) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| **.NET SDK** | 10.0+ | `winget install Microsoft.DotNet.SDK.10` or `https://dotnet.microsoft.com` |
| **Node.js** | 22.x LTS | `winget install OpenJS.NodeJS.LTS` or `fnm use 22` |
| **pnpm** | 9.x+ | `corepack enable && corepack prepare pnpm@latest --activate` |
| **Flutter** | 3.24+ | `https://docs.flutter.dev/get-started/install` |
| **Docker** | Docker Desktop 26+ | `https://www.docker.com/products/docker-desktop/` |

---

## 2. Platform-Specific Native Build Dependencies

### Windows:
- Visual Studio 2022 (Community or higher) with:
  - **Desktop development with C++**
  - **Windows 10/11 SDK (10.0.22621.0 or newer)**
- WebView2 Runtime (pre-installed on Windows 10/11).

### Linux (Ubuntu / Debian):
```bash
sudo apt update && sudo apt install -y \
  build-essential pkg-config libssl-dev \
  libpipewire-0.3-dev libspa-0.2-dev \
  libva-dev libdrm-dev \
  libx11-dev libxtst-dev libxfixes-dev libxrandr-dev \
  libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev
```

### macOS:
- Xcode 15+ with Command Line Tools: `xcode-select --install`.

---

## 3. Step-by-Step Local Workspace Setup

### Step 1: Clone Repository
```bash
git clone https://github.com/opendesk-io/opendesk.git
cd opendesk
```

### Step 2: Launch Local Database & Redis Dependencies
```bash
# Start PostgreSQL 17 and Redis 7.4 in background
docker compose -f deploy/docker-compose/docker-compose.dev.yml up -d
```

### Step 3: Initialize & Migrate Backend Database
```bash
cd server/src/OpenDesk.Api
dotnet ef database update --project ../OpenDesk.Infrastructure
dotnet run
# API will launch at https://localhost:7142; OpenAPI at https://localhost:7142/swagger
```

### Step 4: Launch Desktop Client in Development Mode
In a separate terminal:
```bash
cd clients/desktop
pnpm install
pnpm tauri dev
```
- Spawns the React 19 frontend with Hot-Module Replacement (HMR) and connects to the native Rust `opendesk-engine`.
