# Supabase Docker Compose and API Gateway Architecture Analysis

## Executive Summary

This document analyzes Supabase's Docker Compose architecture, focusing on service composition, API gateway routing (Kong), environment variable management, startup dependencies, and operational design. While Supabase uses **Kong** as its API gateway, the analysis includes guidance on adapting this architecture to use **Traefik** for smaller systems.

---

## 1. Overall Service Architecture

### Service Overview

Supabase's Docker Compose stack consists of multiple microservices that work together to provide a complete backend-as-a-service platform:

```
┌─────────────────────────────────────────────────────────────────┐
│                    Supabase Service Architecture                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  API Gateway Layer (Kong)                                │  │
│  │  - Single entry point for all API requests               │  │
│  │  - Routes to: /rest/v1, /auth/v1, /storage/v1, etc.     │  │
│  │  - Handles CORS, authentication, rate limiting           │  │
│  └──────────────────────┬───────────────────────────────────┘  │
│                         │ Routes requests                       │
│         ┌───────────────┼───────────────┬───────────────┐      │
│         ▼               ▼               ▼               ▼      │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   │
│  │ PostgREST│   │  GoTrue  │   │  Storage │   │ Realtime │   │
│  │   (REST) │   │   (Auth) │   │   (S3)   │   │ (WS/SSE) │   │
│  └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘   │
│       │              │               │               │          │
│       └──────────────┼───────────────┼───────────────┘          │
│                      ▼               ▼                          │
│              ┌──────────────────────────────┐                   │
│              │   PostgreSQL Database (db)   │                   │
│              │   - Main database            │                   │
│              │   - Auth schema              │                   │
│              │   - Storage metadata         │                   │
│              └──────────────────────────────┘                   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Supporting Services                                      │  │
│  │  - Studio (Admin UI)                                      │  │
│  │  - Analytics (Logflare)                                   │  │
│  │  - Meta (Postgres metadata)                               │  │
│  │  - Functions (Edge functions runtime)                     │  │
│  │  - Vector (Log aggregation)                               │  │
│  │  - Supavisor (Connection pooler)                          │  │
│  │  - Imgproxy (Image transformations)                       │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Core Services

#### 1. **Kong API Gateway** (`kong`)
- **Purpose**: Single entry point for all API requests
- **Ports**: 8000 (HTTP), 8443 (HTTPS)
- **Configuration**: Declarative YAML config (`kong.yml`)
- **Plugins**: CORS, request-transformer, key-auth, ACL, basic-auth
- **Routing**: Routes requests to backend services based on path prefixes

#### 2. **PostgREST** (`rest`)
- **Purpose**: Auto-generated REST API from PostgreSQL schema
- **Port**: 3000 (internal)
- **Exposed via**: `/rest/v1/*` through Kong
- **Database**: Connects as `authenticator` role
- **JWT**: Validates JWT tokens for authorization

#### 3. **GoTrue** (`auth`)
- **Purpose**: Authentication and user management
- **Port**: 9999 (internal)
- **Exposed via**: `/auth/v1/*` through Kong
- **Database**: Uses `supabase_auth_admin` role
- **Features**: Email/password, OAuth, MFA, JWT issuance

#### 4. **Storage API** (`storage`)
- **Purpose**: File storage service (S3-compatible)
- **Port**: 5000 (internal)
- **Exposed via**: `/storage/v1/*` through Kong
- **Backend**: File system or S3 (configurable)
- **Dependencies**: PostgREST (for metadata), Imgproxy (for transformations)

#### 5. **Realtime** (`realtime`)
- **Purpose**: WebSocket/SSE server for real-time subscriptions
- **Port**: 4000 (internal)
- **Exposed via**: `/realtime/v1/*` through Kong
- **Database**: Listens to PostgreSQL WAL for changes

#### 6. **PostgreSQL** (`db`)
- **Purpose**: Primary database
- **Port**: 5432 (internal, exposed via Supavisor)
- **Image**: `supabase/postgres:15.8.1.060` (includes PostGIS, extensions)
- **Schemas**: `public`, `auth`, `storage`, `_realtime`, `_analytics`, `_supabase`

### Supporting Services

- **Studio**: Admin UI for managing Supabase projects
- **Analytics (Logflare)**: Log aggregation and analytics
- **Meta**: PostgreSQL metadata API
- **Functions**: Deno-based edge functions runtime
- **Vector**: Log aggregation pipeline
- **Supavisor**: PostgreSQL connection pooler
- **Imgproxy**: Image transformation service

---

## 2. How Kong Routes Requests to Internal Services

### Kong Configuration Pattern

Supabase uses **declarative configuration** for Kong (DB-less mode):

```yaml
# kong.yml (conceptual structure)
_format_version: "3.0"

services:
  # PostgREST API
  - name: rest
    url: http://rest:3000
    routes:
      - name: rest-v1
        paths:
          - /rest/v1
        strip_path: false

  # GoTrue Auth API
  - name: auth
    url: http://auth:9999
    routes:
      - name: auth-v1
        paths:
          - /auth/v1
        strip_path: false

  # Storage API
  - name: storage
    url: http://storage:5000
    routes:
      - name: storage-v1
        paths:
          - /storage/v1
        strip_path: false

  # Realtime API
  - name: realtime
    url: http://realtime:4000
    routes:
      - name: realtime-v1
        paths:
          - /realtime/v1
        strip_path: false
```

### Request Flow Example

```
1. Client Request
   │
   │  GET https://api.example.com/rest/v1/users
   │  Authorization: Bearer <jwt-token>
   │
   ▼

2. Kong Gateway (Port 8000)
   │
   │  2.1. Parse request
   │       - Extract path: /rest/v1/users
   │       - Extract headers: Authorization
   │
   │  2.2. Match route
   │       - Path matches: /rest/v1/*
   │       - Route: rest-v1
   │       - Service: rest (http://rest:3000)
   │
   │  2.3. Apply plugins
   │       - CORS: Add CORS headers
   │       - Request transformer: Add/remove headers
   │       - Key auth: Validate API key (if needed)
   │
   │  2.4. Forward request
   │       - URL: http://rest:3000/rest/v1/users
   │       - Headers: Forwarded from client + Kong-added
   │
   ▼

3. PostgREST Service (Port 3000)
   │
   │  3.1. Receive request
   │       - Path: /rest/v1/users
   │       - Authorization: Bearer <jwt-token>
   │
   │  3.2. Validate JWT
   │       - Extract JWT from Authorization header
   │       - Verify signature using JWT_SECRET
   │       - Extract user role (anon, authenticated, service_role)
   │
   │  3.3. Query database
   │       - Role: anon (from JWT)
   │       - SQL: SELECT * FROM users WHERE RLS allows access
   │       - Execute as: authenticator role (with user role from JWT)
   │
   │  3.4. Return response
   │       - Status: 200 OK
   │       - Body: JSON array of users
   │
   ▼

4. Kong Gateway
   │
   │  4.1. Receive response from PostgREST
   │  4.2. Apply response plugins (if any)
   │  4.3. Forward to client
   │
   ▼

5. Client receives response
```

### Kong Environment Configuration

```yaml
# docker-compose.yml
kong:
  image: kong:2.8.1
  environment:
    KONG_DATABASE: "off"  # DB-less mode (declarative config)
    KONG_DECLARATIVE_CONFIG: /home/kong/kong.yml
    KONG_DNS_ORDER: LAST,A,CNAME
    KONG_PLUGINS: request-transformer,cors,key-auth,acl,basic-auth
    KONG_NGINX_PROXY_PROXY_BUFFER_SIZE: 160k
    KONG_NGINX_PROXY_PROXY_BUFFERS: 64 160k
    SUPABASE_ANON_KEY: ${ANON_KEY}
    SUPABASE_SERVICE_KEY: ${SERVICE_ROLE_KEY}
  volumes:
    - ../files/volumes/api/kong.yml:/home/kong/temp.yml:ro
  entrypoint: bash -c 'eval "echo \"$$(cat ~/temp.yml)\"" > ~/kong.yml && /docker-entrypoint.sh kong docker-start'
```

**Key Points:**
- **DB-less mode**: Kong uses declarative YAML config (no database needed)
- **Template substitution**: Environment variables are substituted in `kong.yml` at startup
- **Plugins**: CORS, request transformation, authentication plugins enabled
- **Buffer sizes**: Tuned for large responses (160k buffers)

---

## 3. How Storage (MinIO/S3) is Exposed Securely

### Storage Service Architecture

Supabase's storage service can use either:
1. **File system backend** (default for local development)
2. **S3 backend** (for production, including MinIO)

### Storage Service Configuration

```yaml
# docker-compose.yml
storage:
  image: supabase/storage-api:v1.22.7
  environment:
    ANON_KEY: ${ANON_KEY}
    SERVICE_KEY: ${SERVICE_ROLE_KEY}
    POSTGREST_URL: http://rest:3000
    PGRST_JWT_SECRET: ${JWT_SECRET}
    DATABASE_URL: postgres://supabase_storage_admin:${POSTGRES_PASSWORD}@db:5432/${POSTGRES_DB}
    
    # File system backend (default)
    STORAGE_BACKEND: file
    FILE_STORAGE_BACKEND_PATH: /var/lib/storage
    
    # OR S3 backend (for production)
    # STORAGE_BACKEND: s3
    # GLOBAL_S3_BUCKET: your-bucket-name
    # REGION: us-east-1
    # AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
    # AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
    # AWS_ENDPOINT: http://minio:9000  # For MinIO
    
    FILE_SIZE_LIMIT: 52428800  # 50MB
    ENABLE_IMAGE_TRANSFORMATION: "true"
    IMGPROXY_URL: http://imgproxy:5001
  volumes:
    - ../files/volumes/storage:/var/lib/storage:z
```

### Storage Request Flow

```
1. Client Request (Upload File)
   │
   │  POST https://api.example.com/storage/v1/object/bucket-name/file.jpg
   │  Authorization: Bearer <jwt-token>
   │  Content-Type: image/jpeg
   │  <file data>
   │
   ▼

2. Kong Gateway
   │
   │  2.1. Route to storage service
   │       - Path: /storage/v1/*
   │       - Forward to: http://storage:5000
   │
   ▼

3. Storage API Service
   │
   │  3.1. Validate JWT token
   │       - Extract user from JWT
   │       - Check permissions (via PostgREST)
   │
   │  3.2. Check bucket permissions
   │       - Query PostgREST: GET /rest/v1/storage.buckets
   │       - Verify user has INSERT permission
   │
   │  3.3. Store file
   │       - If STORAGE_BACKEND=file:
   │         → Write to /var/lib/storage/bucket-name/file.jpg
   │       - If STORAGE_BACKEND=s3:
   │         → Upload to S3/MinIO bucket
   │
   │  3.4. Store metadata
   │       - INSERT into storage.objects table
   │       - Store: name, bucket_id, owner, metadata, etc.
   │
   │  3.5. Return response
   │       - Status: 200 OK
   │       - Body: { "Key": "file.jpg", "Path": "/bucket-name/file.jpg" }
   │
   ▼

4. Client receives confirmation
```

### MinIO Integration (S3 Backend)

For production deployments with MinIO:

```yaml
# docker-compose.yml (with MinIO)
services:
  minio:
    image: minio/minio:latest
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: ${MINIO_ROOT_USER}
      MINIO_ROOT_PASSWORD: ${MINIO_ROOT_PASSWORD}
    volumes:
      - minio-data:/data
    expose:
      - 9000  # S3 API
      - 9001  # Console

  storage:
    environment:
      STORAGE_BACKEND: s3
      GLOBAL_S3_BUCKET: supabase-storage
      REGION: us-east-1
      AWS_ACCESS_KEY_ID: ${MINIO_ROOT_USER}
      AWS_SECRET_ACCESS_KEY: ${MINIO_ROOT_PASSWORD}
      AWS_ENDPOINT: http://minio:9000  # Internal MinIO endpoint
      AWS_FORCE_PATH_STYLE: "true"  # Required for MinIO
```

### Security Considerations

1. **Authentication**: All storage requests require valid JWT tokens
2. **Authorization**: Row-level security (RLS) policies control access
3. **Internal communication**: Storage service communicates with PostgREST internally (not exposed)
4. **MinIO access**: MinIO is only accessible from within Docker network (not exposed to host)
5. **Presigned URLs**: Storage API can generate presigned URLs for direct S3/MinIO access (bypassing API)

---

## 4. Environment Variables and Secrets Management

### Environment Variable Structure

Supabase uses a `.env` file for configuration, with variables referenced in `docker-compose.yml`:

```bash
# .env file (example)
# Database
POSTGRES_PASSWORD=your-secure-password
POSTGRES_DB=postgres
POSTGRES_USER=postgres
POSTGRES_HOST=db
POSTGRES_PORT=5432

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-token-with-at-least-32-characters-long
JWT_EXPIRY=3600

# API Keys (generated, not user-provided)
ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
SERVICE_ROLE_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

# URLs
API_EXTERNAL_URL=http://localhost:8000
SUPABASE_PUBLIC_URL=http://localhost:8000
SITE_URL=http://localhost:3000

# Auth Configuration
ENABLE_EMAIL_SIGNUP=true
ENABLE_EMAIL_AUTOCONFIRM=false
DISABLE_SIGNUP=false

# SMTP (for email)
SMTP_ADMIN_EMAIL=noreply@example.com
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=your-smtp-user
SMTP_PASS=your-smtp-password
SMTP_SENDER_NAME=Supabase

# Storage
STORAGE_BACKEND=file
FILE_STORAGE_BACKEND_PATH=/var/lib/storage

# Analytics
LOGFLARE_API_KEY=your-logflare-api-key
SECRET_KEY_BASE=your-secret-key-base
VAULT_ENC_KEY=your-vault-encryption-key
```

### Variable Substitution in Docker Compose

```yaml
# docker-compose.yml
services:
  auth:
    environment:
      # Direct substitution
      GOTRUE_JWT_SECRET: ${JWT_SECRET}
      
      # With default value
      OPENAI_API_KEY: ${OPENAI_API_KEY:-}
      
      # In connection strings
      GOTRUE_DB_DATABASE_URL: postgres://supabase_auth_admin:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
```

### Secrets Management Patterns

#### 1. **Environment File** (Local Development)
```bash
# .env file (never commit to git)
POSTGRES_PASSWORD=dev-password
JWT_SECRET=dev-secret
```

#### 2. **Docker Secrets** (Production)
```yaml
# docker-compose.yml
services:
  db:
    secrets:
      - postgres_password
    environment:
      POSTGRES_PASSWORD_FILE: /run/secrets/postgres_password

secrets:
  postgres_password:
    file: ./secrets/postgres_password.txt
```

#### 3. **External Secret Management** (Production)
- Use HashiCorp Vault, AWS Secrets Manager, or similar
- Inject secrets at runtime via environment variables
- Never store secrets in version control

### Key Generation

Supabase generates `ANON_KEY` and `SERVICE_ROLE_KEY` from `JWT_SECRET`:

```bash
# Generate JWT secret
JWT_SECRET=$(openssl rand -base64 32)

# ANON_KEY and SERVICE_ROLE_KEY are JWTs signed with JWT_SECRET
# They contain different claims (role: anon vs service_role)
```

---

## 5. Startup and Dependency Order

### Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                    Startup Dependency Order                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Level 0 (No dependencies):                                 │
│  ┌──────────┐                                               │
│  │  vector  │  (Log aggregation, independent)               │
│  └──────────┘                                               │
│                                                              │
│  Level 1 (Depends on Level 0):                              │
│  ┌──────────┐                                               │
│  │    db    │  (PostgreSQL, depends on vector)              │
│  └────┬─────┘                                               │
│       │                                                      │
│  Level 2 (Depends on Level 1):                              │
│  ┌──────────┐  ┌──────────┐                                │
│  │analytics │  │  meta    │  (Both depend on db)            │
│  └────┬─────┘  └──────────┘                                │
│       │                                                      │
│  Level 3 (Depends on Level 2):                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │   kong   │  │  studio  │  │functions │  (All depend on  │
│  └────┬─────┘  └──────────┘  └──────────┘   analytics)     │
│       │                                                      │
│  Level 4 (Depends on Level 3 + db):                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │   rest   │  │   auth   │  │ realtime │  (All depend on  │
│  └────┬─────┘  └──────────┘  └──────────┘   db + analytics)│
│       │                                                      │
│  Level 5 (Depends on Level 4):                              │
│  ┌──────────┐                                               │
│  │ storage  │  (Depends on rest + db)                       │
│  └──────────┘                                               │
│                                                              │
│  Level 6 (Depends on Level 5):                              │
│  ┌──────────┐                                               │
│  │supavisor │  (Depends on db + analytics)                  │
│  └──────────┘                                               │
└─────────────────────────────────────────────────────────────┘
```

### Docker Compose Dependency Configuration

```yaml
# Example: auth service dependencies
auth:
  depends_on:
    db:
      condition: service_healthy  # Wait for db to be healthy
    analytics:
      condition: service_healthy  # Wait for analytics to be healthy

# Example: storage service dependencies
storage:
  depends_on:
    db:
      condition: service_healthy
    rest:
      condition: service_started  # Only wait for start, not health
    imgproxy:
      condition: service_started
```

### Health Check Strategy

Services use health checks to ensure they're ready before dependent services start:

```yaml
# Example: db health check
db:
  healthcheck:
    test: ["CMD", "pg_isready", "-U", "postgres", "-h", "localhost"]
    interval: 5s
    timeout: 5s
    retries: 10

# Example: auth health check
auth:
  healthcheck:
    test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:9999/health"]
    timeout: 5s
    interval: 5s
    retries: 3
```

### Startup Sequence Example

```
1. vector starts (no dependencies)
   │
   ▼

2. db starts (waits for vector to be healthy)
   │  - Initializes PostgreSQL
   │  - Runs migration scripts
   │  - Health check: pg_isready
   │
   ▼

3. analytics and meta start (wait for db to be healthy)
   │  - Connect to database
   │  - Initialize schemas
   │  - Health check: HTTP /health
   │
   ▼

4. kong, studio, functions start (wait for analytics to be healthy)
   │  - Kong loads declarative config
   │  - Studio connects to meta service
   │  - Functions runtime initializes
   │
   ▼

5. rest, auth, realtime start (wait for db + analytics)
   │  - PostgREST connects to database
   │  - GoTrue initializes auth tables
   │  - Realtime connects to database WAL
   │
   ▼

6. storage starts (waits for rest + db)
   │  - Connects to PostgREST for metadata
   │  - Initializes storage buckets
   │
   ▼

7. supavisor starts (waits for db + analytics)
   │  - Initializes connection pooler
   │  - Exposes PostgreSQL on port 5432
```

---

## 6. Local Development vs Production

### Local Development Configuration

```yaml
# docker-compose.yml (local)
services:
  kong:
    # Expose ports for local access
    ports:
      - "8000:8000"   # HTTP
      - "8443:8443"   # HTTPS
    environment:
      KONG_DATABASE: "off"
      # No SSL/TLS termination (handled by Traefik/Nginx in production)

  db:
    # Expose port for direct database access
    ports:
      - "5432:5432"

  storage:
    environment:
      STORAGE_BACKEND: file  # Local file system
      FILE_STORAGE_BACKEND_PATH: /var/lib/storage

  # No external reverse proxy needed
  # Access directly via localhost:8000
```

### Production Configuration

```yaml
# docker-compose.yml (production)
services:
  kong:
    # Don't expose ports (behind Traefik/Nginx)
    expose:
      - 8000
      - 8443
    # No ports mapping

  db:
    # Don't expose port (use Supavisor pooler)
    # No ports mapping

  storage:
    environment:
      STORAGE_BACKEND: s3  # S3/MinIO backend
      GLOBAL_S3_BUCKET: production-bucket
      AWS_ENDPOINT: https://s3.amazonaws.com
      # Or for MinIO:
      # AWS_ENDPOINT: http://minio:9000

  # Add Traefik/Nginx reverse proxy
  traefik:
    image: traefik:v2.10
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik/traefik.yml:/traefik.yml:ro
      - ./traefik/letsencrypt:/letsencrypt
    command:
      - --api.insecure=true
      - --providers.docker=true
      - --entrypoints.web.address=:80
      - --entrypoints.websecure.address=:443
      - --certificatesresolvers.letsencrypt.acme.tlschallenge=true
      - --certificatesresolvers.letsencrypt.acme.email=admin@example.com
      - --certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json
```

### Traefik Configuration for Supabase

```yaml
# traefik.yml
services:
  traefik:
    image: traefik:v2.10
    command:
      - --api.insecure=true
      - --providers.docker=true
      - --providers.docker.exposedbydefault=false
      - --entrypoints.web.address=:80
      - --entrypoints.websecure.address=:443
      - --certificatesresolvers.letsencrypt.acme.tlschallenge=true
      - --certificatesresolvers.letsencrypt.acme.email=admin@example.com
      - --certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./letsencrypt:/letsencrypt

  kong:
    image: kong:2.8.1
    expose:
      - 8000
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.kong.rule=Host(`api.example.com`)"
      - "traefik.http.routers.kong.entrypoints=websecure"
      - "traefik.http.routers.kong.tls.certresolver=letsencrypt"
      - "traefik.http.services.kong.loadbalancer.server.port=8000"
```

### Key Differences

| Aspect | Local Development | Production |
|--------|------------------|------------|
| **API Gateway** | Kong exposed on `localhost:8000` | Kong behind Traefik/Nginx |
| **SSL/TLS** | None (HTTP only) | Traefik handles SSL termination |
| **Database Access** | Direct port exposure (5432) | Via Supavisor pooler only |
| **Storage Backend** | File system | S3/MinIO |
| **Secrets** | `.env` file | External secret management |
| **Monitoring** | Basic logging | Full observability stack |
| **Scaling** | Single instance | Multiple instances with load balancing |

---

## 7. Essential vs Optional Components

### Essential Services (Minimum Viable Stack)

For a smaller system, these services are **essential**:

```yaml
# Minimal Supabase stack
services:
  # 1. Database (REQUIRED)
  db:
    image: supabase/postgres:15.8.1.060
    # All other services depend on this

  # 2. API Gateway (REQUIRED)
  kong:
    image: kong:2.8.1
    # Single entry point for all APIs

  # 3. REST API (REQUIRED)
  rest:
    image: postgrest/postgrest:v12.2.11
    # Auto-generated REST API from database

  # 4. Auth Service (REQUIRED if using auth)
  auth:
    image: supabase/gotrue:v2.171.0
    # User authentication and JWT issuance

  # 5. Storage Service (REQUIRED if using storage)
  storage:
    image: supabase/storage-api:v1.22.7
    # File storage (can use file backend for simplicity)
```

### Optional Services (Can Be Removed)

These services can be **removed** for smaller systems:

```yaml
# Optional services (comment out if not needed)

# 1. Studio (Admin UI) - OPTIONAL
# studio:
#   image: supabase/studio:latest
#   # Only needed for admin interface

# 2. Realtime - OPTIONAL
# realtime:
#   image: supabase/realtime:v2.34.47
#   # Only needed for WebSocket/SSE subscriptions

# 3. Analytics - OPTIONAL
# analytics:
#   image: supabase/logflare:1.12.0
#   # Only needed for log aggregation

# 4. Meta - OPTIONAL
# meta:
#   image: supabase/postgres-meta:v0.88.9
#   # Only needed for database metadata API

# 5. Functions - OPTIONAL
# functions:
#   image: supabase/edge-runtime:v1.67.4
#   # Only needed for edge functions

# 6. Vector - OPTIONAL
# vector:
#   image: timberio/vector:0.28.1-alpine
#   # Only needed for log aggregation pipeline

# 7. Supavisor - OPTIONAL
# supavisor:
#   image: supabase/supavisor:2.5.1
#   # Only needed for connection pooling

# 8. Imgproxy - OPTIONAL
# imgproxy:
#   image: darthsim/imgproxy:v3.8.0
#   # Only needed for image transformations
```

### Minimal Stack for Small System

```yaml
# Minimal docker-compose.yml for small system
version: '3.8'

services:
  # Database
  db:
    image: supabase/postgres:15.8.1.060
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    volumes:
      - db-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD", "pg_isready", "-U", "postgres"]
      interval: 5s
      timeout: 5s
      retries: 10

  # API Gateway (Kong or Traefik)
  traefik:
    image: traefik:v2.10
    command:
      - --api.insecure=true
      - --providers.docker=true
      - --entrypoints.web.address=:80
    ports:
      - "80:80"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro

  # REST API
  rest:
    image: postgrest/postgrest:v12.2.11
    environment:
      PGRST_DB_URI: postgres://authenticator:${POSTGRES_PASSWORD}@db:5432/${POSTGRES_DB}
      PGRST_DB_SCHEMAS: public
      PGRST_DB_ANON_ROLE: anon
      PGRST_JWT_SECRET: ${JWT_SECRET}
    depends_on:
      db:
        condition: service_healthy
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.rest.rule=PathPrefix(`/rest/v1`)"
      - "traefik.http.services.rest.loadbalancer.server.port=3000"

  # Auth Service
  auth:
    image: supabase/gotrue:v2.171.0
    environment:
      GOTRUE_DB_DATABASE_URL: postgres://supabase_auth_admin:${POSTGRES_PASSWORD}@db:5432/${POSTGRES_DB}
      GOTRUE_JWT_SECRET: ${JWT_SECRET}
      GOTRUE_SITE_URL: ${SITE_URL}
    depends_on:
      db:
        condition: service_healthy
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.auth.rule=PathPrefix(`/auth/v1`)"
      - "traefik.http.services.auth.loadbalancer.server.port=9999"

volumes:
  db-data:
```

### Service Removal Impact

| Service | Impact if Removed | Workaround |
|---------|------------------|------------|
| **Studio** | No admin UI | Use `psql` or database client directly |
| **Realtime** | No WebSocket/SSE | Use polling or remove real-time features |
| **Analytics** | No log aggregation | Use Docker logs or external logging |
| **Meta** | No metadata API | Query PostgreSQL system tables directly |
| **Functions** | No edge functions | Move logic to application layer |
| **Vector** | No log pipeline | Use Docker logs directly |
| **Supavisor** | No connection pooling | Direct database connections (less efficient) |
| **Imgproxy** | No image transformations | Handle transformations in application layer |

---

## 8. Adapting to Traefik (Instead of Kong)

### Why Use Traefik Instead of Kong?

**Advantages of Traefik:**
- **Simpler configuration**: Docker labels vs YAML files
- **Automatic SSL**: Built-in Let's Encrypt support
- **Docker-native**: Better integration with Docker Compose
- **Smaller footprint**: Less resource-intensive
- **Easier for small systems**: Less complexity

**When to Use Kong:**
- **Advanced routing**: Complex routing rules, plugins
- **API management**: Rate limiting, API versioning
- **Multi-protocol**: gRPC, WebSocket, HTTP/2
- **Enterprise features**: OAuth, API keys, ACLs

### Traefik Configuration for Supabase Services

```yaml
# docker-compose.yml with Traefik
version: '3.8'

services:
  traefik:
    image: traefik:v2.10
    command:
      - --api.insecure=true
      - --providers.docker=true
      - --providers.docker.exposedbydefault=false
      - --entrypoints.web.address=:80
      - --entrypoints.websecure.address=:443
      - --certificatesresolvers.letsencrypt.acme.tlschallenge=true
      - --certificatesresolvers.letsencrypt.acme.email=admin@example.com
      - --certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./letsencrypt:/letsencrypt

  # PostgREST
  rest:
    image: postgrest/postgrest:v12.2.11
    expose:
      - 3000
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.rest.rule=PathPrefix(`/rest/v1`)"
      - "traefik.http.routers.rest.entrypoints=websecure"
      - "traefik.http.routers.rest.tls.certresolver=letsencrypt"
      - "traefik.http.services.rest.loadbalancer.server.port=3000"
      - "traefik.http.middlewares.rest-stripprefix.stripprefix.prefixes=/rest/v1"
      - "traefik.http.routers.rest.middlewares=rest-stripprefix"

  # GoTrue Auth
  auth:
    image: supabase/gotrue:v2.171.0
    expose:
      - 9999
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.auth.rule=PathPrefix(`/auth/v1`)"
      - "traefik.http.routers.auth.entrypoints=websecure"
      - "traefik.http.routers.auth.tls.certresolver=letsencrypt"
      - "traefik.http.services.auth.loadbalancer.server.port=9999"
      - "traefik.http.middlewares.auth-stripprefix.stripprefix.prefixes=/auth/v1"
      - "traefik.http.routers.auth.middlewares=auth-stripprefix"

  # Storage API
  storage:
    image: supabase/storage-api:v1.22.7
    expose:
      - 5000
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.storage.rule=PathPrefix(`/storage/v1`)"
      - "traefik.http.routers.storage.entrypoints=websecure"
      - "traefik.http.routers.storage.tls.certresolver=letsencrypt"
      - "traefik.http.services.storage.loadbalancer.server.port=5000"
      - "traefik.http.middlewares.storage-stripprefix.stripprefix.prefixes=/storage/v1"
      - "traefik.http.routers.storage.middlewares=storage-stripprefix"

  # MinIO (if using S3 backend)
  minio:
    image: minio/minio:latest
    command: server /data --console-address ":9001"
    expose:
      - 9000
      - 9001
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.minio-api.rule=Host(`storage.example.com`)"
      - "traefik.http.routers.minio-api.entrypoints=websecure"
      - "traefik.http.routers.minio-api.tls.certresolver=letsencrypt"
      - "traefik.http.services.minio-api.loadbalancer.server.port=9000"
      - "traefik.http.routers.minio-console.rule=Host(`storage-console.example.com`)"
      - "traefik.http.routers.minio-console.entrypoints=websecure"
      - "traefik.http.routers.minio-console.tls.certresolver=letsencrypt"
      - "traefik.http.services.minio-console.loadbalancer.server.port=9001"
    environment:
      MINIO_ROOT_USER: ${MINIO_ROOT_USER}
      MINIO_ROOT_PASSWORD: ${MINIO_ROOT_PASSWORD}
    volumes:
      - minio-data:/data
```

### Key Differences: Kong vs Traefik

| Feature | Kong | Traefik |
|---------|------|---------|
| **Configuration** | YAML file (declarative) | Docker labels (declarative) |
| **SSL/TLS** | Manual setup | Automatic Let's Encrypt |
| **Routing** | Path-based, complex rules | Path-based, host-based |
| **Plugins** | Extensive plugin ecosystem | Middleware-based |
| **Complexity** | Higher (more features) | Lower (simpler) |
| **Resource Usage** | Higher | Lower |
| **Best For** | Enterprise, complex APIs | Small-medium systems |

---

## 9. Request Flow Summary

### Complete Request Flow (Kong → PostgREST)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Complete Request Flow                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Client Request                                               │
│     GET https://api.example.com/rest/v1/users                    │
│     Authorization: Bearer <jwt-token>                            │
│                                                                  │
│  2. Traefik/Nginx (Port 80/443)                                 │
│     - SSL termination                                            │
│     - Route to Kong (or directly to services)                    │
│                                                                  │
│  3. Kong API Gateway (Port 8000)                                │
│     - Parse request path: /rest/v1/users                         │
│     - Match route: rest-v1                                       │
│     - Apply plugins: CORS, request-transformer                   │
│     - Forward to: http://rest:3000/rest/v1/users                 │
│                                                                  │
│  4. PostgREST Service (Port 3000)                               │
│     - Extract JWT from Authorization header                      │
│     - Validate JWT signature (using JWT_SECRET)                  │
│     - Extract user role: anon                                    │
│     - Connect to database as: authenticator role                 │
│     - Execute SQL: SELECT * FROM users                           │
│     - Apply RLS policies (based on user role)                    │
│     - Return JSON response                                       │
│                                                                  │
│  5. Kong API Gateway                                            │
│     - Receive response from PostgREST                            │
│     - Apply response plugins (if any)                            │
│     - Forward to client                                          │
│                                                                  │
│  6. Traefik/Nginx                                               │
│     - Forward response to client                                 │
│                                                                  │
│  7. Client receives response                                     │
│     Status: 200 OK                                               │
│     Body: [{ "id": 1, "email": "user@example.com" }, ...]       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 10. Key Takeaways and Recommendations

### Architectural Patterns

1. **API Gateway Pattern**: Single entry point (Kong/Traefik) routes to multiple backend services
2. **Service Mesh**: Services communicate internally via Docker network (not exposed to host)
3. **Health Checks**: Dependencies wait for services to be healthy before starting
4. **Environment Variables**: Centralized configuration via `.env` file
5. **Secrets Management**: Sensitive data in environment variables (or external secret manager)

### Best Practices

1. **Don't Expose Internal Services**: Use `expose` instead of `ports` for internal services
2. **Use Health Checks**: Ensure services are ready before dependent services start
3. **Separate Concerns**: Each service has a single responsibility
4. **Secure by Default**: Services require authentication (JWT tokens)
5. **Scalable Design**: Services can be scaled independently

### For Smaller Systems

1. **Start Minimal**: Begin with db, rest, auth, and API gateway
2. **Add Services Incrementally**: Add storage, realtime, etc. as needed
3. **Use Traefik**: Simpler than Kong for small systems
4. **File Backend for Storage**: Use file system backend instead of S3/MinIO initially
5. **Skip Optional Services**: Remove analytics, meta, functions if not needed

### Production Considerations

1. **Use External Secret Management**: Don't store secrets in `.env` files
2. **Enable SSL/TLS**: Use Traefik with Let's Encrypt for automatic certificates
3. **Use S3/MinIO Backend**: File system backend doesn't scale
4. **Monitor Services**: Add Prometheus/Grafana for observability
5. **Backup Database**: Regular backups of PostgreSQL data

---

## References

- Supabase Docker Compose: https://github.com/supabase/supabase/tree/master/docker
- Kong Documentation: https://docs.konghq.com/
- Traefik Documentation: https://doc.traefik.io/traefik/
- PostgREST Documentation: https://postgrest.org/
- GoTrue Documentation: https://github.com/supabase/gotrue

