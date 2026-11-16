# OpenDesk Phase 4 — Networking, Signaling & NAT Traversal

## 1. Phase Objective
Implement real-time signaling over WebSockets (SignalR), automate STUN candidate gathering, integrate coturn TURN relay fallback, and achieve resilient peer-to-peer remote desktop connections across distinct WAN networks and corporate firewalls.

## 2. Why This Phase Is Necessary
Local LAN connections are trivial compared to real-world internet topologies. Most users reside behind residential or enterprise NAT routers, symmetric firewalls, or dynamic cellular connections. A production remote desktop platform must reliably punch holes through NAT or fall back seamlessly to high-throughput authenticated relays.

## 3. Scope
- Implementing the ASP.NET Core 10 `SignalingHub` using SignalR for SDP offer/answer and Trickle ICE candidate exchange.
- Implementing the Rust signaling client in `opendesk-transport`.
- Integrating STUN (RFC 8489) for server-reflexive candidate harvesting.
- Integrating coturn TURN relay (RFC 8656) with dynamic ephemeral HMAC-SHA1 credentials.
- Implementing automatic ICE restart for transient network drops and IP transitions.
- Testing connections across distinct WAN subnets, mobile hotspots, and simulated NAT topologies.

## 4. Prerequisites
Completion of Phase 3 (Local desktop streaming pipeline).

## 5. Technical Implementation Details
- `SignalingHub` handles room multiplexing, peer pairing, and signaling state.
- `CoturnTokenService` issues short-lived credentials targeting configured relay endpoints.
- `opendesk-transport` implements asynchronous Trickle ICE, sending candidates as they are harvested to reduce connection establishment time to $< 1.5\text{s}$.
- Heartbeat loop sends pings every 2 seconds; triggers ICE restart if 2 consecutive heartbeats are unacknowledged.

## 6. Exact Folder and File Changes Expected
```
server/src/OpenDesk.Signaling/
├── Hubs/
│   └── SignalingHub.cs
├── Services/
│   └── CoturnTokenService.cs
└── DTOs/
    ├── SdpMessageDto.cs
    └── IceCandidateDto.cs
core/crates/opendesk-transport/src/
├── lib.rs
├── signaling_client.rs
├── peer_connection.rs
├── ice.rs
└── stats.rs
```

## 7. Individual Development Tasks
- **P4-001**: Implement `SignalingHub` in ASP.NET Core with room routing and presence tracking.
- **P4-002**: Implement `CoturnTokenService` issuing RFC 5766 HMAC-SHA1 ephemeral credentials.
- **P4-003**: Implement Rust WebSocket signaling client communicating with SignalR hub.
- **P4-004**: Configure STUN candidate gathering in `webrtc-rs` transport layer.
- **P4-005**: Configure authenticated TURN relay allocation in `webrtc-rs`.
- **P4-006**: Implement Trickle ICE candidate streaming over the signaling channel.
- **P4-007**: Implement automatic ICE restart and connection recovery logic upon network drops.
- **P4-008**: Validate traversal across diverse NAT types (Full Cone, Port Restricted, Symmetric).

## 8. Dependencies
Phase 3 (Desktop Remote Access).

## 9. Expected Outputs
A robust WebRTC transport engine establishing peer-to-peer or relay connections between devices on different networks with sub-second setup time.

## 10. Unit Testing Requirements
- Unit tests verifying HMAC-SHA1 TURN token calculation against known RFC 5766 test vectors.
- Unit tests verifying Trickle ICE candidate serialization and deserialization.

## 11. Integration Testing Requirements
Integration tests verifying SignalR hub message routing between two mock client connections.

## 12. Manual Verification Steps
1. Host connected to residential broadband Wi-Fi; Client connected to mobile 5G hotspot.
2. Initiate connection via 9-digit Session ID.
3. Verify connection establishes within 2.0 seconds.
4. Inspect WebRTC stats $\rightarrow$ Verify whether connection is `srflx` (P2P) or `relay` (TURN).
5. Disconnect Wi-Fi on client and reconnect $\rightarrow$ Verify stream recovers via ICE restart within 5 seconds without crashing the viewer.

## 13. Acceptance Criteria
- Connection success rate $\ge 92\%$ for direct P2P over standard NAT; $100\%$ fallback to TURN relay.
- Connection establishment time $< 2.0\text{ seconds}$ on typical broadband connections.
- Graceful recovery from transient 5-second network dropouts via ICE restart.

## 14. Definition of Done
Clients on independent WAN networks establish video and input streaming with automated relay fallback and connection recovery.

## 15. Known Risks
- Strict corporate firewalls blocking outbound UDP on non-standard ports.
  - *Mitigation*: Configure Coturn to listen on port 443 TCP with TLS (TURNS) fallback.

## 16. Estimated Complexity
High (WebRTC internals, NAT traversal, distributed signaling).

## 17. Next-Phase Dependencies
Unlocks **Phase 5 — Authentication and Security Hardening**.
