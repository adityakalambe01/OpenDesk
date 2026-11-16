# OpenDesk Phase 8 — Backend Control Plane & Web Management Dashboard

## 1. Phase Objective
Complete the production ASP.NET Core 10 backend features and develop the React 19 Web Management Dashboard, providing complete multi-tenant device fleet management, real-time presence monitoring, session authorization policies, audit logging, and a zero-install WebRTC browser viewer.

## 2. Why This Phase Is Necessary
Enterprise and MSP users need a centralized web-based management portal to organize fleets of machines into groups, assign granular permissions to technicians, inspect audit histories for compliance, and initiate remote sessions directly from web browsers without installing native software.

## 3. Scope
- Implementing ASP.NET Core Identity with JWT and refresh token rotation.
- Implementing Organization-level Role-Based Access Control (RBAC: Admin, Manager, Technician, Viewer).
- Building the React 19 Web Management Dashboard (fleet grid, tags, presence, session logs).
- Implementing the browser-based WebRTC remote desktop viewer using WebCodecs and canvas.
- Building the audit log query engine with CSV export and SIEM webhook integration.
- Implementing OpenAPI / Swagger documentation and developer API key management.

## 4. Prerequisites
Completion of Phase 5 (Authentication & Security).

## 5. Technical Implementation Details
- React 19 dashboard built with Vite, TailwindCSS, and TanStack Table for high-performance fleet filtering.
- Browser remote viewer uses WebRTC `RTCPeerConnection` and HTML5 `OffscreenCanvas` with WebCodecs hardware decoding where available in modern browsers.
- Real-time updates delivered via SignalR WebSocket subscriptions.

## 6. Exact Folder and File Changes Expected
```
server/src/
├── OpenDesk.Api/Controllers/
│   ├── OrganizationsController.cs
│   ├── UsersController.cs
│   ├── DevicesController.cs
│   └── AuditController.cs
└── OpenDesk.Application/
    ├── Organizations/
    └── Audit/
clients/web/
├── src/
│   ├── components/
│   │   ├── FleetTable.tsx
│   │   ├── DeviceDetailModal.tsx
│   │   └── WebRemoteViewer.tsx
│   ├── pages/
│   │   ├── DashboardPage.tsx
│   │   ├── DevicesPage.tsx
│   │   └── AuditLogsPage.tsx
│   └── App.tsx
```

## 7. Individual Development Tasks
- **P8-001**: Implement multi-tenant Organization and User RBAC models in ASP.NET Core.
- **P8-002**: Implement Device fleet querying, grouping, and tag management APIs.
- **P8-003**: Implement real-time Redis presence subscriber pushing device status to web dashboards.
- **P8-004**: Scaffold React 19 Web Dashboard using Vite, TailwindCSS, and Lucide icons.
- **P8-005**: Implement interactive device fleet table with instant filtering and search.
- **P8-006**: Implement zero-install WebRTC browser viewer component using WebCodecs/Canvas.
- **P8-007**: Implement audit log viewer with filtering by date, technician, device, and action.
- **P8-008**: Implement automated OpenAPI (Swagger) documentation generation.

## 8. Dependencies
Phase 5 (Authentication & Security).

## 9. Expected Outputs
A complete, responsive Web Management Dashboard and a zero-install browser remote viewer.

## 10. Unit Testing Requirements
- Unit tests verifying RBAC authorization rules in ASP.NET Core controllers.
- Unit tests validating React fleet table filtering and sorting logic.

## 11. Integration Testing Requirements
Integration tests verifying that a device heartbeat update in Redis instantly broadcasts a SignalR event to connected web clients.

## 12. Manual Verification Steps
1. Log into Web Dashboard as Administrator.
2. View registered device fleet $\rightarrow$ Verify real-time online/offline badges.
3. Click "Connect in Browser" on an online machine $\rightarrow$ Verify zero-install Web viewer connects and streams desktop.
4. Perform file transfer $\rightarrow$ Open Audit Logs page $\rightarrow$ Verify transfer event is recorded with SHA-256 hash.

## 13. Acceptance Criteria
- Dashboard renders 1,000+ devices with sub-second filtering and pagination.
- Browser remote viewer functions in Chrome, Edge, Safari, and Firefox.
- Audit logs provide an immutable, exportable trail of all remote activities.

## 14. Definition of Done
Web dashboard and backend APIs deployed, tested, and integrated with the live signaling infrastructure.

## 15. Known Risks
- Browser WebRTC input capture limitations (e.g. inability to capture browser-reserved hotkeys like Ctrl+W or Ctrl+N).
  - *Mitigation*: Provide on-screen action buttons in the web viewer for system combinations.

## 16. Estimated Complexity
Medium (Standard enterprise full-stack development).

## 17. Next-Phase Dependencies
Unlocks **Phase 9 — Self-Hosting Infrastructure & Turnkey Deployment**.
