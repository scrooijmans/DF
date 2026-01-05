#!/bin/bash

# MudRock Supabase Setup Script
# This script helps set up a standalone Supabase instance for MudRock

set -e

echo "🚀 MudRock Supabase Setup Script"
echo "================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is installed
check_docker() {
    print_status "Checking Docker installation..."
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    
    print_success "Docker and Docker Compose are installed"
}

# Generate secure secrets
generate_secrets() {
    print_status "Generating secure secrets..."
    
    # Generate JWT secret (64 characters)
    JWT_SECRET=$(openssl rand -base64 48 | tr -d "=+/" | cut -c1-64)
    
    # Generate PostgreSQL password (32 characters)
    POSTGRES_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
    
    # Generate dashboard password (16 characters)
    DASHBOARD_PASSWORD=$(openssl rand -base64 12 | tr -d "=+/" | cut -c1-16)
    
    print_success "Secrets generated successfully"
}

# Create environment file
create_env_file() {
    print_status "Creating environment file..."
    
    if [ -f ".env" ]; then
        print_warning "Environment file already exists. Creating backup..."
        cp .env .env.backup.$(date +%Y%m%d_%H%M%S)
    fi
    
    cat > .env << EOF
# MudRock Supabase Environment Configuration
# Generated on $(date)

# =============================================================================
# DOMAIN CONFIGURATION
# =============================================================================
SUPABASE_DOMAIN=supabase.yourdomain.com
SITE_URL=https://yourdomain.com

# =============================================================================
# DATABASE CONFIGURATION
# =============================================================================
POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
POSTGRES_DB=postgres

# =============================================================================
# JWT CONFIGURATION
# =============================================================================
JWT_SECRET=${JWT_SECRET}

# =============================================================================
# DASHBOARD AUTHENTICATION
# =============================================================================
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=${DASHBOARD_PASSWORD}

# =============================================================================
# SMTP CONFIGURATION (for email authentication)
# =============================================================================
SMTP_HOST=
SMTP_PORT=587
SMTP_USER=
SMTP_PASS=
SMTP_ADMIN_EMAIL=admin@yourdomain.com
SMTP_SENDER_NAME=MudRock

# =============================================================================
# OAUTH PROVIDERS (disabled - using Supabase Auth only)
# =============================================================================
# GITHUB_CLIENT_ID=
# GITHUB_CLIENT_SECRET=
# GOOGLE_CLIENT_ID=
# GOOGLE_CLIENT_SECRET=

# =============================================================================
# STORAGE CONFIGURATION
# =============================================================================
STORAGE_BACKEND=file
GLOBAL_S3_BUCKET=
REGION=us-east-1

# =============================================================================
# ADVANCED CONFIGURATION
# =============================================================================
POOLER_TENANT_ID=mudrock-tenant
POOLER_PROXY_PORT=6543
POOLER_PROXY_PORT_TRANSACTION=6544
REALTIME_JWT_SECRET=${JWT_SECRET}
REALTIME_DB_ENC_KEY=supabaserealtime
FILE_SIZE_LIMIT=52428800
STORAGE_FILE_SIZE_LIMIT=52428800
EOF

    print_success "Environment file created"
}

# Create docker-compose file
create_docker_compose() {
    print_status "Creating Docker Compose file..."
    
    cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  supabase-db:
    image: supabase/postgres:15.1.0.117
    restart: unless-stopped
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
      POSTGRES_INITDB_ARGS: --auth-host=scram-sha-256
      POSTGRES_HOST_AUTH_METHOD: scram-sha-256
    volumes:
      - supabase_db_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

  supabase-auth:
    image: supabase/gotrue:v2.158.1
    restart: unless-stopped
    depends_on:
      supabase-db:
        condition: service_healthy
    environment:
      GOTRUE_API_HOST: 0.0.0.0
      GOTRUE_API_PORT: 9999
      GOTRUE_DB_DRIVER: postgres
      GOTRUE_DB_DATABASE_URL: postgres://supabase_auth_admin:${POSTGRES_PASSWORD}@supabase-db:5432/${POSTGRES_DB}
      GOTRUE_SITE_URL: ${SITE_URL}
      GOTRUE_URI_ALLOW_LIST: ${SITE_URL}
      GOTRUE_JWT_SECRET: ${JWT_SECRET}
      GOTRUE_JWT_EXP: 3600
      GOTRUE_JWT_AUD: authenticated
      GOTRUE_JWT_DEFAULT_GROUP_NAME: authenticated
      GOTRUE_JWT_ADMIN_ROLES: supabase_admin,service_role
      GOTRUE_DISABLE_SIGNUP: false
      GOTRUE_EXTERNAL_EMAIL_ENABLED: true
      GOTRUE_EXTERNAL_PHONE_ENABLED: true
      GOTRUE_MAILER_AUTOCONFIRM: true
      GOTRUE_MAILER_SECURE_EMAIL_CHANGE_ENABLED: true
      GOTRUE_SMTP_HOST: ${SMTP_HOST}
      GOTRUE_SMTP_PORT: ${SMTP_PORT}
      GOTRUE_SMTP_USER: ${SMTP_USER}
      GOTRUE_SMTP_PASS: ${SMTP_PASS}
      GOTRUE_SMTP_ADMIN_EMAIL: ${SMTP_ADMIN_EMAIL}
      GOTRUE_SMTP_SENDER_NAME: ${SMTP_SENDER_NAME}
      GOTRUE_EXTERNAL_GITHUB_ENABLED: false
      GOTRUE_EXTERNAL_GOOGLE_ENABLED: false
    ports:
      - "9999:9999"

  supabase-rest:
    image: postgrest/postgrest:v12.2.0
    restart: unless-stopped
    depends_on:
      supabase-db:
        condition: service_healthy
    environment:
      PGRST_DB_URI: postgres://authenticator:${POSTGRES_PASSWORD}@supabase-db:5432/${POSTGRES_DB}
      PGRST_DB_SCHEMAS: public,storage,graphql_public
      PGRST_DB_ANON_ROLE: anon
      PGRST_JWT_SECRET: ${JWT_SECRET}
      PGRST_DB_USE_LEGACY_GUCS: false
    ports:
      - "3000:3000"

  supabase-realtime:
    image: supabase/realtime:v2.30.15
    restart: unless-stopped
    depends_on:
      supabase-db:
        condition: service_healthy
    environment:
      PORT: 4000
      DB_HOST: supabase-db
      DB_PORT: 5432
      DB_USER: supabase_admin
      DB_PASSWORD: ${POSTGRES_PASSWORD}
      DB_NAME: ${POSTGRES_DB}
      DB_AFTER_CONNECT_QUERY: SET search_path TO _realtime
      DB_ENC_KEY: supabaserealtime
      API_JWT_SECRET: ${JWT_SECRET}
      FLY_ALLOC_ID: fly123
      FLY_APP_NAME: realtime
      SECRET_KEY_BASE: UpNVntn3cDxHJpq99YMc1T1AQgQpc8kfYTuRgBiYa15BLrx8etQoXz3gZv1/u2oq
      ERL_AFLAGS: -proto_dist inet_tcp
      ENABLE_TAILSCALE: false
      DNS_NODES: ""
    ports:
      - "4000:4000"

  supabase-storage:
    image: supabase/storage-api:v1.10.5
    restart: unless-stopped
    depends_on:
      supabase-db:
        condition: service_healthy
    environment:
      ANON_KEY: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0
      SERVICE_KEY: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImV4cCI6MTk4MzgxMjk5Nn0.EGIM96RAZx35lJzdJsyH-qQwv8Hdp7fsn3W0YpN81IU
      POSTGREST_URL: http://supabase-rest:3000
      PGRST_JWT_SECRET: ${JWT_SECRET}
      DATABASE_URL: postgres://supabase_storage_admin:${POSTGRES_PASSWORD}@supabase-db:5432/${POSTGRES_DB}
      FILE_SIZE_LIMIT: 52428800
      STORAGE_BACKEND: ${STORAGE_BACKEND}
      GLOBAL_S3_BUCKET: ${GLOBAL_S3_BUCKET}
      REGION: ${REGION}
      FILE_STORAGE_BACKEND_PATH: /var/lib/storage
    volumes:
      - supabase_storage_data:/var/lib/storage
    ports:
      - "5000:5000"

  supabase-kong:
    image: kong:2.8.1
    restart: unless-stopped
    depends_on:
      - supabase-auth
      - supabase-rest
      - supabase-realtime
      - supabase-storage
    environment:
      KONG_DATABASE: off
      KONG_DECLARATIVE_CONFIG: /var/lib/kong/kong.yml
      KONG_DNS_ORDER: LAST,A,CNAME
      KONG_PLUGINS: request-transformer,cors,key-auth,acl,basic-auth
    volumes:
      - supabase_kong_data:/var/lib/kong
      - ./kong.yml:/var/lib/kong/kong.yml:ro
    ports:
      - "8000:8000"

  supabase-studio:
    image: supabase/studio:20241215-0b8c8b4
    restart: unless-stopped
    depends_on:
      - supabase-kong
    environment:
      STUDIO_PG_META_URL: http://supabase-meta:8080
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      DEFAULT_ORGANIZATION_NAME: MudRock
      DEFAULT_PROJECT_NAME: Default Project
      SUPABASE_URL: https://${SUPABASE_DOMAIN}
      SUPABASE_PUBLIC_URL: https://${SUPABASE_DOMAIN}
      SUPABASE_ANON_KEY: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0
      SUPABASE_SERVICE_KEY: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImV4cCI6MTk4MzgxMjk5Nn0.EGIM96RAZx35lJzdJsyH-qQwv8Hdp7fsn3W0YpN81IU
    ports:
      - "3001:3000"

  supabase-meta:
    image: supabase/postgres-meta:v0.84.2
    restart: unless-stopped
    depends_on:
      supabase-db:
        condition: service_healthy
    environment:
      PG_META_PORT: 8080
      PG_META_DB_HOST: supabase-db
      PG_META_DB_PORT: 5432
      PG_META_DB_NAME: ${POSTGRES_DB}
      PG_META_DB_USER: supabase_admin
      PG_META_DB_PASSWORD: ${POSTGRES_PASSWORD}
    ports:
      - "8080:8080"

volumes:
  supabase_db_data:
  supabase_storage_data:
  supabase_kong_data:

networks:
  default:
    driver: bridge
EOF

    print_success "Docker Compose file created"
}

# Create Kong configuration
create_kong_config() {
    print_status "Creating Kong configuration..."
    
    cat > kong.yml << 'EOF'
_format_version: "1.1"

consumers:
  - username: ${DASHBOARD_USERNAME}
    custom_id: ${DASHBOARD_USERNAME}
    basicauth_credentials:
      - username: ${DASHBOARD_USERNAME}
        password: ${DASHBOARD_PASSWORD}

services:
  - name: supabase-auth-v1-open
    url: http://supabase-auth:9999/verify
    routes:
      - name: supabase-auth-v1-open
        strip_path: true
        paths:
          - /auth/v1/verify
    plugins:
      - name: cors
  - name: supabase-auth-v1-open-callback
    url: http://supabase-auth:9999/callback
    routes:
      - name: supabase-auth-v1-open-callback
        strip_path: true
        paths:
          - /auth/v1/callback
    plugins:
      - name: cors
  - name: supabase-auth-v1-open-authorize
    url: http://supabase-auth:9999/authorize
    routes:
      - name: supabase-auth-v1-open-authorize
        strip_path: true
        paths:
          - /auth/v1/authorize
    plugins:
      - name: cors
  - name: supabase-auth-v1
    _comment: "GoTrue: /auth/v1/* -> http://supabase-auth:9999/*"
    url: http://supabase-auth:9999/
    routes:
      - name: supabase-auth-v1-all
        strip_path: true
        paths:
          - /auth/v1/
  - name: supabase-rest
    _comment: "PostgREST: /rest/v1/* -> http://supabase-rest:3000/*"
    url: http://supabase-rest:3000/
    routes:
      - name: supabase-rest-all
        strip_path: true
        paths:
          - /rest/v1/
  - name: supabase-realtime
    _comment: "Realtime: /realtime/v1/* -> ws://supabase-realtime:4000/socket/*"
    url: http://supabase-realtime:4000/socket/
    routes:
      - name: supabase-realtime-all
        strip_path: true
        paths:
          - /realtime/v1/
  - name: supabase-storage
    _comment: "Storage: /storage/v1/* -> http://supabase-storage:5000/*"
    url: http://supabase-storage:5000/
    routes:
      - name: supabase-storage-all
        strip_path: true
        paths:
          - /storage/v1/
  - name: supabase-meta
    _comment: "pg-meta: /pg/* -> http://supabase-meta:8080/*"
    url: http://supabase-meta:8080/
    routes:
      - name: supabase-meta-all
        strip_path: true
        paths:
          - /pg/
  - name: supabase-studio
    _comment: "Studio: /* -> http://supabase-studio:3000/*"
    url: http://supabase-studio:3000/
    routes:
      - name: supabase-studio-all
        paths:
          - /
    plugins:
      - name: basic-auth
        config:
          hide_credentials: false
EOF

    print_success "Kong configuration created"
}

# Start services
start_services() {
    print_status "Starting Supabase services..."
    
    # Pull images first
    print_status "Pulling Docker images..."
    docker-compose pull
    
    # Start services
    print_status "Starting services..."
    docker-compose up -d
    
    print_success "Services started successfully"
}

# Display connection information
show_connection_info() {
    print_success "Setup completed successfully!"
    echo ""
    echo "🔗 Connection Information:"
    echo "========================="
    echo "Supabase Studio: http://localhost:3001"
    echo "API Gateway: http://localhost:8000"
    echo "Database: localhost:5432"
    echo ""
    echo "📋 Credentials:"
    echo "==============="
    echo "Studio Username: admin"
    echo "Studio Password: ${DASHBOARD_PASSWORD}"
    echo "Database Password: ${POSTGRES_PASSWORD}"
    echo ""
    echo "🔑 API Keys:"
    echo "============"
    echo "Anon Key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0"
    echo "Service Key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImV4cCI6MTk4MzgxMjk5Nn0.EGIM96RAZx35lJzdJsyH-qQwv8Hdp7fsn3W0YpN81IU"
    echo ""
    echo "📝 Next Steps:"
    echo "=============="
    echo "1. Update the SUPABASE_DOMAIN and SITE_URL in .env file"
    echo "2. Configure SMTP settings for email authentication"
    echo "3. Set up OAuth providers (GitHub, Google, etc.)"
    echo "4. Configure your reverse proxy (Nginx) to point to this instance"
    echo "5. Update your application to use the new Supabase instance"
    echo ""
    echo "🛠️  Management Commands:"
    echo "========================"
    echo "Stop services: docker-compose down"
    echo "Start services: docker-compose up -d"
    echo "View logs: docker-compose logs -f"
    echo "Restart services: docker-compose restart"
}

# Main execution
main() {
    check_docker
    generate_secrets
    create_env_file
    create_docker_compose
    create_kong_config
    start_services
    show_connection_info
}

# Run main function
main "$@"
