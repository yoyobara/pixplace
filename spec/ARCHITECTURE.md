# System Architecture

```mermaid
graph TD
    subgraph Clients
        Browser[Web Browser]
        Mobile[Mobile App]
    end

    subgraph Frontend_Layer [Frontend Layer - Horizontally Scalable]
        FE1[Frontend Server 1]
        FE2[Frontend Server 2]
        FEN[Frontend Server N]
    end

    subgraph Backend_Infrastructure
        LB[Load Balancer / Ingress]
        
        subgraph Backend_Layer [Backend Layer - Horizontally Scalable]
            BE1[Backend Server 1]
            BE2[Backend Server 2]
            BEN[Backend Server N]
        end
    end

    subgraph Data_Layer [Data Layer]
        PG[(PostgreSQL)]
        RD[(Redis)]
        KF[[Kafka]]
    end

    %% Flow
    Browser --> FE1
    Mobile --> FE2

    FE1 --> LB
    FE2 --> LB
    FEN --> LB

    LB --> BE1
    LB --> BE2
    LB --> BEN

    BE1 --> PG
    BE1 --> RD
    BE1 --> KF
    BE2 --> PG
    BE2 --> RD
    BE2 --> KF
    BEN --> PG
    BEN --> RD
    BEN --> KF
```

### Components Description

1.  **Frontend Layer**: These servers host the static assets (React/Vite) or handle Server-Side Rendering (SSR). They can be scaled horizontally behind a CDN or another global load balancer.
2.  **Load Balancer**: Distributes incoming API traffic from the frontend/clients across the healthy backend instances.
3.  **Backend Layer (Rust)**: The application logic, scaled horizontally. Since they are stateless (state is in DB/Cache), we can spin up as many instances as needed.
4.  **PostgreSQL**: The primary relational database for persistent storage.
5.  **Redis**: Used for caching, session management, or as a message broker to reduce DB load and improve performance.
6.  **Kafka**: Distributed event streaming platform used for asynchronous communication, event-driven architecture, or log aggregation.
