# OpenDesk — Networking, WebRTC & NAT Traversal Specification

## 1. WebRTC Protocol Stack in OpenDesk

OpenDesk utilizes WebRTC as its standard transport framework, establishing low-latency, encrypted peer-to-peer tunnels across varying network topologies:

```
+-----------------------------------------------------------------------------------+
| APPLICATION LAYER                                                                |
|  [ Video Frames ]    [ Audio Chunks ]    [ Input Events ]    [ File Blocks ]      |
+-------------------+-------------------+-------------------+-----------------------+
| PROTOCOL LAYER                                                                    |
|  [ RTP / RTCP ]   |  [ RTP / RTCP ]   | [ SCTP Channels ] |  [ SCTP Channels ]    |
+-------------------+-------------------+-------------------+-----------------------+
| SECURITY LAYER                                                                    |
|           [ SRTP (AES-128-GCM) ]      |          [ DTLS 1.3 (RFC 9147) ]          |
+---------------------------------------+-------------------------------------------+
| TRANSPORT LAYER                                                                   |
|                         [ ICE (RFC 8445) / STUN / TURN ]                          |
+-----------------------------------------------------------------------------------+
| NETWORK LAYER                                                                     |
|                                [ UDP / IPv4 & IPv6 ]                              |
|                          (TCP fallback via TURN port 443)                         |
+-----------------------------------------------------------------------------------+
```

---

## 2. ICE Candidate Gathering & NAT Traversal Topology

```mermaid
graph TD
    subgraph HostNAT["Host Local Network (NAT A)"]
        HostPeer["OpenDesk Host Engine"]
    end

    subgraph ClientNAT["Client Local Network (NAT B)"]
        ClientPeer["OpenDesk Remote Client"]
    end

    subgraph Infrastructure["Signaling & Traversal Infrastructure"]
        STUNServer["OpenDesk STUN (RFC 8489)\nPort 3478 UDP"]
        TURNRelay["Coturn TURN Relay (RFC 8656)\nPorts 3478 / 5349 / 443"]
        SignalR["ASP.NET Core Signaling Hub"]
    end

    %% Gathering
    HostPeer -->|1. STUN Binding Request| STUNServer
    ClientPeer -->|1. STUN Binding Request| STUNServer
    STUNServer -.->|Host Reflexive Candidates| HostPeer
    STUNServer -.->|Client Reflexive Candidates| ClientPeer

    %% Signaling
    HostPeer <-->|2. Exchange Trickle ICE Candidates| SignalR
    ClientPeer <-->|2. Exchange Trickle ICE Candidates| SignalR

    %% Direct Connection
    HostPeer <===>|Path A: Direct P2P Hole-Punching (UDP)| ClientPeer

    %% Relay Fallback
    HostPeer -.->|Path B: Ephemeral Allocation| TURNRelay
    ClientPeer -.->|Path B: Relayed Media Channel| TURNRelay
```

### Candidate Priority Hierarchy:
1. **Host Candidates (`typ host`)**: Direct private LAN addresses (e.g., `192.168.1.150:52410`). Preferred when peers are within the same office or local network.
2. **Server Reflexive Candidates (`typ srflx`)**: Discovered via STUN binding requests to `stun:relay.opendesk.io:3478`. Successful for Full-Cone, Restricted Cone, and Port-Restricted NAT environments (> 85% of consumer residential connections).
3. **Relay Candidates (`typ relay`)**: Allocated on coturn when both peers reside behind Symmetric NATs or strict corporate firewalls. Traffic is encrypted end-to-end; coturn only acts as an opaque packet forwarder.

---

## 3. Dynamic Congestion Control & Bitrate Adaptation

Video streaming adapts dynamically to network conditions via **Google Congestion Control (GCC)** and **Transport-Wide Congestion Control (TWCC)**:

```mermaid
flowchart TD
    VideoSource["Capture Engine (60 FPS)"] --> Encoder["Hardware Encoder"]
    Encoder --> Sender["WebRTC RTP Sender"]
    Sender -->|RTP Packets with TWCC Header Extensions| Network["Variable WAN Link"]
    Network --> Receiver["Client WebRTC Receiver"]
    Receiver -->|RTCP TWCC Feedback Packets (Every 50ms)| Sender
    Sender --> Estimator["GCC Bandwidth Estimator\n(Loss-Based & Delay-Based)"]
    Estimator -->|Adjust Target Bitrate: 500 Kbps - 25 Mbps| Encoder
    Estimator -->|On Severe Packet Loss: Request Keyframe| VideoSource
```

### Adaptation Parameters:
- **Delay-Based Controller**: Analyzes packet arrival inter-arrival time offsets ($\Delta d$). If $\Delta d > \text{threshold}$, network queuing is detected, triggering bitrate reductions *before* packet loss occurs.
- **Loss-Based Controller**:
  - Packet Loss $< 2\%$: Increase bitrate by $8\%$ per second.
  - Packet Loss $2\% - 10\%$: Maintain current bitrate.
  - Packet Loss $> 10\%$: Multiplicative decrease: $\text{Bitrate}_{\text{new}} = \text{Bitrate} \times (1 - 0.5 \times \text{LossRate})$.
- **Framerate Throttling**: If available bandwidth falls below 1.2 Mbps, the encoder throttles framerate from 60 FPS down to 30 FPS, and down to 15 FPS below 500 Kbps to preserve image sharpness.

---

## 4. Coturn Relay Configuration Standard

The self-hosted Docker deployment provisions coturn with the following baseline `/etc/turnserver.conf`:

```ini
# /etc/turnserver.conf - OpenDesk Hardened Coturn Configuration
listening-port=3478
tls-listening-port=5349
alt-listening-port=443
alt-tls-listening-port=443

# Relay IP assignments
listening-ip=0.0.0.0
min-port=49152
max-port=65535

# Zero-knowledge Ephemeral Token Authentication (RFC 5766)
use-auth-secret
static-auth-secret=ENV_OPENDESK_COTURN_SECRET
realm=relay.opendesk.io

# Security hardening
no-stdout-log
syslog
no-multicast-peers
no-cli
no-loopback-peers
denied-peer-ip=10.0.0.0-10.255.255.255
denied-peer-ip=172.16.0.0-172.31.255.255
denied-peer-ip=192.168.0.0-192.168.255.255
denied-peer-ip=127.0.0.0-127.255.255.255
```
