# Self-Hosting Supabase on DigitalOcean: Complete Guide

## 🎯 **Overview**

This guide provides a complete walkthrough for deploying Supabase on DigitalOcean using Docker. This approach gives you full control over your geoscience data platform while maintaining enterprise-grade security and performance.

## 🏗️ **Architecture Overview**

```
DigitalOcean Droplet
├── Operating System: Ubuntu 22.04 LTS
├── Docker Engine
├── Supabase Stack:
│   ├── PostgreSQL 15 + PostGIS
│   ├── Kong API Gateway
│   ├── GoTrue Authentication
│   ├── PostgREST API
│   ├── Realtime WebSocket
│   └── Storage Service
├── MudRock Custom Services:
│   ├── Qdrant Vector Database
│   ├── Parquet Processing Engine
│   ├── Geoscience Analysis
│   └── Enterprise Security Layer
└── Monitoring & Backup:
    ├── Prometheus
    ├── Grafana
    └── Automated Backups
```

## 🚀 **Step 1: DigitalOcean Setup**

### **1.1 Create DigitalOcean Account**
```bash
# Sign up at https://digitalocean.com
# Verify your account and add payment method
# Recommended: Enable 2FA for security
```

### **1.2 Create Droplet**
```bash
# Recommended specifications for MudRock:
Droplet Configuration:
├── Choose an image: Ubuntu 22.04 LTS
├── Choose a plan: Basic
├── Choose a size: 
│   ├── Regular CPU: 4 vCPUs
│   ├── Memory: 8GB RAM
│   ├── SSD: 160GB
│   └── Transfer: 4TB
├── Choose a datacenter region:
│   ├── NYC1 (US East)
│   ├── SFO2 (US West)
│   ├── LON1 (Europe)
│   └── SGP1 (Asia)
├── Choose authentication:
│   ├── SSH key (recommended)
│   └── Password (less secure)
└── Finalize and create
```

### **1.3 Initial Server Setup**
```bash
# SSH into your droplet
ssh root@your-droplet-ip

# Create a new user (security best practice)
adduser mudrock
usermod -aG sudo mudrock

# Switch to new user
su - mudrock

# Update system
sudo apt update && sudo apt upgrade -y

# Install essential packages
sudo apt install -y curl wget git unzip
```

## 🐳 **Step 2: Docker Installation**

### **2.1 Install Docker Engine**
```bash
# Remove old versions
sudo apt remove docker docker-engine docker.io containerd runc

# Install prerequisites
sudo apt install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

# Add Docker's official GPG key
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# Add Docker repository
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io

# Add user to docker group
sudo usermod -aG docker $USER

# Start and enable Docker
sudo systemctl start docker
sudo systemctl enable docker

# Verify installation
docker --version
```

### **2.2 Install Docker Compose**
```bash
# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.20.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# Verify installation
docker-compose --version
```

## 🔧 **Step 3: Supabase Deployment**

### **3.1 Clone Supabase Repository**
```bash
# Clone Supabase
git clone --depth 1 https://github.com/supabase/supabase
cd supabase

# Create project directory
mkdir mudrock-project
cd mudrock-project

# Copy Docker files
cp -rf ../docker/* .
cp ../docker/.env.example .env
```

### **3.2 Configure Environment**
```bash
# Edit environment file
nano .env

# Essential configurations for MudRock:
POSTGRES_PASSWORD=your-super-secure-password
JWT_SECRET=your-super-secure-jwt-secret
ANON_KEY=your-anon-key
SERVICE_ROLE_KEY=your-service-role-key

# Database configuration
POSTGRES_DB=mudrock
POSTGRES_USER=postgres

# API configuration
API_EXTERNAL_URL=https://your-domain.com
SITE_URL=https://your-domain.com

# Email configuration (for auth)
SMTP_ADMIN_EMAIL=noreply@your-domain.com
SMTP_HOST=your-smtp-host
SMTP_PORT=587
SMTP_USER=your-smtp-user
SMTP_PASS=your-smtp-password
SMTP_SENDER_NAME=MudRock

# Storage configuration
STORAGE_BACKEND=file
GLOBAL_S3_BUCKET=your-bucket-name
REGION=your-region
```

### **3.3 Deploy Supabase**
```bash
# Pull latest images
docker-compose pull

# Start services
docker-compose up -d

# Check service status
docker-compose ps

# View logs
docker-compose logs -f
```

## 🏢 **Step 4: Enterprise Customization**

### **4.1 Create Custom Docker Compose**
```yaml
# docker-compose.enterprise.yml
version: '3.8'
services:
  # Supabase services
  db:
    image: supabase/postgres:15.1.0.117
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
      POSTGRES_USER: ${POSTGRES_USER}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init:/docker-entrypoint-initdb.d
    ports:
      - "5432:5432"
    networks:
      - mudrock-network

  # MudRock custom services
  mudrock-analytics:
    image: mudrock/analytics:latest
    environment:
      ENCRYPTION_KEY: ${CUSTOMER_ENCRYPTION_KEY}
      POSTGRES_URL: postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@db:5432/${POSTGRES_DB}
      QDRANT_URL: http://qdrant:6333
    volumes:
      - parquet_data:/data/parquet
      - analysis_results:/data/analysis
    depends_on:
      - db
      - qdrant
    networks:
      - mudrock-network

  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
    volumes:
      - qdrant_data:/qdrant/storage
    networks:
      - mudrock-network

  # Enterprise monitoring
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    ports:
      - "9090:9090"
    networks:
      - mudrock-network

  grafana:
    image: grafana/grafana:latest
    environment:
      GF_SECURITY_ADMIN_PASSWORD: ${GRAFANA_PASSWORD}
    volumes:
      - grafana_data:/var/lib/grafana
    ports:
      - "3000:3000"
    networks:
      - mudrock-network

  # Enterprise security
  nginx:
    image: nginx:alpine
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    ports:
      - "80:80"
      - "443:443"
    depends_on:
      - kong
    networks:
      - mudrock-network

volumes:
  postgres_data:
  parquet_data:
  analysis_results:
  qdrant_data:
  prometheus_data:
  grafana_data:

networks:
  mudrock-network:
    driver: bridge
```

### **4.2 Configure Nginx Reverse Proxy**
```nginx
# nginx/nginx.conf
events {
    worker_connections 1024;
}

http {
    upstream supabase {
        server kong:8000;
    }

    upstream analytics {
        server mudrock-analytics:8080;
    }

    server {
        listen 80;
        server_name your-domain.com;
        return 301 https://$server_name$request_uri;
    }

    server {
        listen 443 ssl http2;
        server_name your-domain.com;

        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;

        # Security headers
        add_header X-Frame-Options DENY;
        add_header X-Content-Type-Options nosniff;
        add_header X-XSS-Protection "1; mode=block";

        location / {
            proxy_pass http://supabase;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        location /analytics/ {
            proxy_pass http://analytics/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
```

### **4.3 Setup SSL Certificates**
```bash
# Install Certbot
sudo apt install -y certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

## 🔐 **Step 5: Security Configuration**

### **5.1 Firewall Setup**
```bash
# Install UFW
sudo apt install -y ufw

# Configure firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH
sudo ufw allow ssh

# Allow HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Allow internal services (optional)
sudo ufw allow from 10.0.0.0/8 to any port 5432  # PostgreSQL
sudo ufw allow from 10.0.0.0/8 to any port 6333  # Qdrant

# Enable firewall
sudo ufw enable
```

### **5.2 Database Security**
```sql
-- Connect to PostgreSQL
docker exec -it mudrock-project-db-1 psql -U postgres

-- Create secure roles
CREATE ROLE mudrock_user WITH LOGIN PASSWORD 'secure-password';
CREATE ROLE mudrock_readonly WITH LOGIN PASSWORD 'readonly-password';

-- Grant permissions
GRANT CONNECT ON DATABASE mudrock TO mudrock_user;
GRANT USAGE ON SCHEMA public TO mudrock_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO mudrock_user;

-- Create read-only role
GRANT CONNECT ON DATABASE mudrock TO mudrock_readonly;
GRANT USAGE ON SCHEMA public TO mudrock_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO mudrock_readonly;
```

### **5.3 Environment Security**
```bash
# Create secure environment file
cat > .env.secure << EOF
# Database
POSTGRES_PASSWORD=$(openssl rand -base64 32)
JWT_SECRET=$(openssl rand -base64 32)
ANON_KEY=$(openssl rand -base64 32)
SERVICE_ROLE_KEY=$(openssl rand -base64 32)

# Encryption
CUSTOMER_ENCRYPTION_KEY=$(openssl rand -base64 32)

# Monitoring
GRAFANA_PASSWORD=$(openssl rand -base64 16)

# API
API_EXTERNAL_URL=https://your-domain.com
SITE_URL=https://your-domain.com
EOF

# Secure the file
chmod 600 .env.secure
```

## 📊 **Step 6: Monitoring & Backup**

### **6.1 Prometheus Configuration**
```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'supabase'
    static_configs:
      - targets: ['kong:8001']

  - job_name: 'postgres'
    static_configs:
      - targets: ['db:5432']

  - job_name: 'mudrock-analytics'
    static_configs:
      - targets: ['mudrock-analytics:8080']

  - job_name: 'qdrant'
    static_configs:
      - targets: ['qdrant:6333']
```

### **6.2 Backup Script**
```bash
#!/bin/bash
# backup.sh

# Database backup
docker exec mudrock-project-db-1 pg_dump -U postgres mudrock > backup_$(date +%Y%m%d_%H%M%S).sql

# File backup
tar -czf files_backup_$(date +%Y%m%d_%H%M%S).tar.gz /data/parquet /data/analysis

# Upload to DigitalOcean Spaces (optional)
# s3cmd put backup_*.sql s3://your-backup-bucket/
# s3cmd put files_backup_*.tar.gz s3://your-backup-bucket/

# Clean old backups (keep 30 days)
find . -name "backup_*.sql" -mtime +30 -delete
find . -name "files_backup_*.tar.gz" -mtime +30 -delete
```

### **6.3 Setup Automated Backups**
```bash
# Make backup script executable
chmod +x backup.sh

# Add to crontab
crontab -e
# Add: 0 2 * * * /path/to/backup.sh
```

## 🚀 **Step 7: Deployment**

### **7.1 Start All Services**
```bash
# Deploy with enterprise configuration
docker-compose -f docker-compose.enterprise.yml up -d

# Check all services are running
docker-compose ps

# View logs
docker-compose logs -f
```

### **7.2 Verify Deployment**
```bash
# Test API endpoints
curl https://your-domain.com/rest/v1/
curl https://your-domain.com/auth/v1/
curl https://your-domain.com/storage/v1/

# Test custom services
curl https://your-domain.com/analytics/health
curl http://localhost:6333/health
```

### **7.3 Initialize Database**
```sql
-- Connect to database
docker exec -it mudrock-project-db-1 psql -U postgres -d mudrock

-- Create MudRock schema
CREATE SCHEMA IF NOT EXISTS mudrock;

-- Create tables (using your schema.sql)
\i /path/to/schema.sql

-- Create PostGIS extension
CREATE EXTENSION IF NOT EXISTS postgis;
```

## 📈 **Step 8: Performance Optimization**

### **8.1 PostgreSQL Tuning**
```sql
-- Optimize PostgreSQL for geoscience workloads
ALTER SYSTEM SET shared_buffers = '2GB';
ALTER SYSTEM SET effective_cache_size = '6GB';
ALTER SYSTEM SET maintenance_work_mem = '256MB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
ALTER SYSTEM SET default_statistics_target = 100;

-- Reload configuration
SELECT pg_reload_conf();
```

### **8.2 Docker Resource Limits**
```yaml
# Add to docker-compose.enterprise.yml
services:
  db:
    deploy:
      resources:
        limits:
          memory: 4G
        reservations:
          memory: 2G
  
  mudrock-analytics:
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '2.0'
```

## 🔧 **Step 9: Maintenance**

### **9.1 Update Services**
```bash
# Pull latest images
docker-compose pull

# Restart services
docker-compose down
docker-compose up -d

# Check for updates
docker-compose logs --tail=100
```

### **9.2 Monitor Resources**
```bash
# Check disk usage
df -h

# Check memory usage
free -h

# Check Docker resources
docker stats

# Check service logs
docker-compose logs -f [service-name]
```

## 🎯 **Cost Breakdown**

### **DigitalOcean Droplet Costs**
```bash
# Recommended configuration:
Droplet: $40/month (4 vCPU, 8GB RAM, 160GB SSD)
Bandwidth: Included (4TB transfer)
Backup: $8/month (optional)
Monitoring: $5/month (optional)

Total: ~$53/month
```

### **Comparison with Alternatives**
| Provider | Specs | Monthly Cost | Features |
|----------|-------|--------------|----------|
| **DigitalOcean** | 4 vCPU, 8GB RAM | $40/month | ✅ Simple, reliable |
| **AWS EC2** | t3.xlarge | $120/month | ❌ Expensive |
| **Azure VM** | Standard_D4s_v3 | $180/month | ❌ Very expensive |
| **Hetzner** | 4 vCPU, 8GB RAM | $30/month | ⚠️ Limited regions |

## 🚨 **Troubleshooting**

### **Common Issues**

**1. Port Conflicts**
```bash
# Check what's using port 80/443
sudo netstat -tulpn | grep :80
sudo netstat -tulpn | grep :443

# Kill conflicting processes
sudo kill -9 [PID]
```

**2. Docker Permission Issues**
```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Logout and login again
exit
ssh mudrock@your-droplet-ip
```

**3. SSL Certificate Issues**
```bash
# Check certificate status
sudo certbot certificates

# Renew manually
sudo certbot renew --dry-run
```

**4. Database Connection Issues**
```bash
# Check PostgreSQL logs
docker-compose logs db

# Test connection
docker exec -it mudrock-project-db-1 psql -U postgres -d mudrock
```

## 🎯 **Next Steps**

1. **Domain Configuration**: Point your domain to DigitalOcean's nameservers
2. **Email Setup**: Configure SMTP for user authentication
3. **Monitoring**: Set up alerts for disk space, memory, and uptime
4. **Backup Strategy**: Implement automated backups to DigitalOcean Spaces
5. **Security Audit**: Regular security updates and vulnerability scans

This setup provides a robust, scalable foundation for your geoscience data platform with full control over your infrastructure and data. 