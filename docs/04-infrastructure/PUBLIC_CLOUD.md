# OpenDesk — Public Cloud Multi-Tenant Architecture

## 1. Cloud Infrastructure Overview

The OpenDesk Public Cloud utilizes an identical application codebase to the self-hosted edition, deployed on Kubernetes (EKS / GKE) with managed database clusters and globally distributed relay nodes:

```mermaid
graph TD
    subgraph Edge["Global Edge Tier (Cloudflare / Route53)"]
        AnycastDNS["Geo-DNS / Latency-Based Routing (Route53)"]
        WAF["Cloudflare Enterprise WAF & DDoS Shield"]
    end

    subgraph K8sCluster["Kubernetes Control Plane (EKS Multi-AZ)"]
        Ingress["Envoy Gateway / ALB Ingress Controller"]
        APIPods["OpenDesk API Pods (Horizontal Pod Autoscaler: 3-50 Pods)"]
        SignalingPods["OpenDesk SignalR Pods (HPA: 5-100 Pods)"]
    end

    subgraph ManagedData["Managed Multi-AZ Data Tier"]
        RDS[(Amazon Aurora PostgreSQL 17 Multi-AZ)]
        ElastiCache[(Amazon ElastiCache Redis 7.4 Cluster)]
    end

    subgraph GlobalRelays["Multi-Region Coturn Relay Fleet (EC2 Auto Scaling Groups)"]
        RelayUS["US-East Coturn (Virginia)"]
        RelayEU["EU-Central Coturn (Frankfurt)"]
        RelayAPAC["AP-South Coturn (Mumbai)"]
    end

    AnycastDNS --> WAF
    WAF --> Ingress
    Ingress --> APIPods
    Ingress --> SignalingPods
    APIPods --> RDS
    APIPods --> ElastiCache
    SignalingPods --> ElastiCache

    AnycastDNS -.->|Route to nearest TURN IP| RelayUS
    AnycastDNS -.->|Route to nearest TURN IP| RelayEU
    AnycastDNS -.->|Route to nearest TURN IP| RelayAPAC
```

---

## 2. Horizontal Scaling & High Availability

1. **SignalR WebSocket Clustering**:
   - Backend signaling nodes scale horizontally using `Microsoft.AspNetCore.SignalR.StackExchangeRedis`.
   - Pods are stateless. If a signaling pod crashes, clients immediately reconnect to another pod; state is maintained in Redis.
2. **Geo-Distributed Coturn Fleet**:
   - Media relay nodes run on bare-metal or compute-optimized EC2 instances (`c6in.2xlarge` with 25 Gbps network interfaces) located in 8 AWS regions globally.
   - The backend `CoturnTokenService` queries the connecting client's GeoIP and issues TURN credentials targeting the closest geographic relay node.
