# Database Schema Design

This document outlines the entity-relationship structure for the PixPlace application.

## Entity Relationship Diagram

```mermaid
erDiagram
    EVENTS {
        int id PK
        string name
        timestamp start_date
        timestamp end_date
        timestamp created_at
        timestamp updated_at
        boolean is_deleted
    }

    USERS {
        int id PK
        string email
        string displayName
        int userIdentity_id FK
        timestamp created_at
        timestamp updated_at
        boolean is_deleted
    }

    USER_IDENTITY {
        int id PK
        string provider
        string provider_user_id
        timestamp created_at
        timestamp updated_at
        boolean is_deleted
    }

    PIXEL_HISTORY {
        int id PK
        smallint x
        smallint y
        smallint color
        int user_id FK
        int event_id FK
        timestamp created_at
        timestamp updated_at
        boolean is_deleted
    }

    USER_IDENTITY ||--o{ USERS : "identifies"
    EVENTS ||--o{ PIXEL_HISTORY : "contains"
    USERS ||--o{ PIXEL_HISTORY : "placed"
```

## Table Definitions

### 1. Events
- `id`: Primary Key
- `name`: Event name
- `start_date`: Start timestamp
- `end_date`: End timestamp
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp
- `is_deleted`: Boolean flag for soft deletion

### 2. Users
- `id`: Primary Key
- `email`: User email
- `displayName`: Public display name
- `userIdentity_id`: Foreign Key to `USER_IDENTITY`
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp
- `is_deleted`: Boolean flag for soft deletion

### 3. User Identity
- `id`: Primary Key
- `provider`: Auth provider name (e.g., google, github)
- `provider_user_id`: Unique ID from the provider
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp
- `is_deleted`: Boolean flag for soft deletion

### 4. Pixel History
- `id`: Primary Key
- `x`: X-coordinate (SMALLINT)
- `y`: Y-coordinate (SMALLINT)
- `color`: Color value (SMALLINT)
- `user_id`: Foreign Key to `USERS`
- `event_id`: Foreign Key to `EVENTS`
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp
- `is_deleted`: Boolean flag for soft deletion
