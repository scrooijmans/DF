# Self-Hosting Supabase on Hetzner Cloud: Complete Guide

## 🎯 **Overview**

This guide shows you how to self-host Supabase on a Hetzner Cloud virtual private server (VPS). This approach is cost-effective compared to Supabase's hosted plans ($20-30/month) and gives you full control over your infrastructure.

## 🏗️ **Architecture Overview**

```
Hetzner Cloud VPS
├── Operating System: Ubuntu with Docker pre-installed
├── Supabase Stack:
│   ├── PostgreSQL Database
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

## 🚀 **Step 1: Hetzner Cloud Setup**

### **1.1 Create Hetzner Cloud Account**
```bash
# Sign up at https://hetzner.com/cloud
# Verify your account and add payment method
# Recommended: Enable 2FA for security
```

### **1.2 Create New Project**
```bash
# In Hetzner Cloud Console:
1. Click "Add Project"
2. Name: "supabase"
3. Click "Add Project"
```

### **1.3 Generate SSH Key (Local Machine)**
```bash
# Open command prompt/terminal on your local machine
# Generate SSH key pair
ssh-keygen

# You'll see output like:
# Generating public/private rsa key pair.
# Enter file in which to save the key (/home/user/.ssh/id_rsa): [Press Enter]
# Enter passphrase (empty for no passphrase): [Enter your passphrase, e.g., 123]
# Enter same passphrase again: [Confirm your passphrase]

# The key will be saved in:
# - Private key: ~/.ssh/id_rsa
# - Public key: ~/.ssh/id_rsa.pub
```

### **1.4 Add SSH Key to Hetzner**
```bash
# Copy your public key
cat ~/.ssh/id_rsa.pub

# In Hetzner Cloud Console:
1. Go to "Security" → "SSH Keys"
2. Click "Add SSH Key"
3. Paste your public key
4. Name: "your-name-desktop"
5. Click "Add Key"
```

### **1.5 Create Server**
```bash
# In Hetzner Cloud Console:
1. Go to your "supabase" project
2. Click "Add Server"

# Server Configuration:
├── Location: Germany (or your preferred location)
├── Image: Apps → Docker (pre-installed)
├── Type: CX11 (cheapest tier, can scale up later)
├── SSH Key: Select your added key
├── Name: "supabase"
└── Click "Create and Buy Server"

# Note: First month is usage-based billing
# If you don't use the server, cost is much lower
```

## 🔧 **Step 2: Connect to Server**

### **2.1 SSH into Your Server**
```bash
# Get your server's public IP from Hetzner console
# SSH into your server
ssh root@YOUR_SERVER_IP

# First time connection will show:
# The authenticity of host 'YOUR_SERVER_IP' can't be established.
# Are you sure you want to continue connecting (yes/no)? yes

# Enter your SSH key passphrase when prompted
# You should see: root@supabase:~#
```

### **2.2 Verify Server Setup**
```bash
# Check what's on the server
ls -la

# You should see basic system files
# The server is ready for Supabase installation
```

## 🐳 **Step 3: Install Supabase**

### **3.1 Clone Supabase Repository**
```bash
# Clone the Supabase repository
git clone --depth 1 https://github.com/supabase/supabase

# Change to the supabase directory
cd supabase

# List contents to verify
ls -la
```

### **3.2 Copy Docker Files**
```bash
# Copy the Docker files to your working directory
cp -rf docker/* .

# Copy the environment file
cp docker/.env.example .env

# Verify files are copied
ls -la
```

### **3.3 Pull Docker Images**
```bash
# Pull the latest Supabase Docker images
docker-compose pull

# This will download all required images:
# - PostgreSQL
# - Kong API Gateway
# - GoTrue Authentication
# - PostgREST API
# - Realtime WebSocket
# - Storage Service
```

### **3.4 Start Supabase Services**
```bash
# Start all services in detached mode
docker-compose up -d

# The -d flag means "detached" - services run in background
# You can continue using the command prompt
```

### **3.5 Verify Services Are Running**
```bash
# Check the status of all services
docker-compose ps

# You should see all services with status "Up"
# Example output:
# Name                    Command               State           Ports
# supabase-db-1          docker-entrypoint.sh postgres    Up      0.0.0.0:5432->5432/tcp
# supabase-kong-1        /kong-wrapper.sh               Up      0.0.0.0:8000->8000/tcp
# supabase-auth-1        /bin/sh -c ./start.sh          Up      0.0.0.0:9999->9999/tcp
# supabase-realtime-1    /bin/sh -c ./start.sh          Up      0.0.0.0:4000->4000/tcp
# supabase-rest-1        /bin/sh -c ./start.sh          Up      0.0.0.0:3000->3000/tcp
# supabase-storage-1     /bin/sh -c ./start.sh          Up      0.0.0.0:5000->5000/tcp
# supabase-studio-1      /bin/sh -c ./start.sh          Up      0.0.0.0:3000->3000/tcp
```

## 🌐 **Step 4: Access Supabase Dashboard**

### **4.1 Access the Dashboard**
```bash
# Open your web browser
# Navigate to: http://YOUR_SERVER_IP:8000

# If nothing shows, try adding the port:
# http://YOUR_SERVER_IP:8000
```

### **4.2 Login to Dashboard**
```bash
# Default credentials:
Username: supabase
Password: this_password_is_insecure_and_should_be_updated

# You should now see the Supabase dashboard
# This is your self-hosted version!
```

## 🔐 **Step 5: Security Configuration**

### **5.1 Change Default Password**
```bash
# In the Supabase dashboard:
1. Go to Settings → Users
2. Find the "supabase" user
3. Click "Edit"
4. Change the password to something secure
5. Save changes
```

### **5.2 Configure Environment Variables**
```bash
# Edit the environment file
nano .env

# Essential configurations to update:
POSTGRES_PASSWORD=your-super-secure-password
JWT_SECRET=your-super-secure-jwt-secret
ANON_KEY=your-anon-key
SERVICE_ROLE_KEY=your-service-role-key

# API configuration
API_EXTERNAL_URL=http://YOUR_SERVER_IP:8000
SITE_URL=http://YOUR_SERVER_IP:8000

# Email configuration (for auth)
SMTP_ADMIN_EMAIL=noreply@your-domain.com
SMTP_HOST=your-smtp-host
SMTP_PORT=587
SMTP_USER=your-smtp-user
SMTP_PASS=your-smtp-password
SMTP_SENDER_NAME=MudRock
```

### **5.3 Restart Services with New Configuration**
```bash
# Stop services
docker-compose down

# Start services with new configuration
docker-compose up -d

# Verify services are running
docker-compose ps
```

## 🏢 **Step 6: Enterprise Customization**

### **6.1 Create Custom Docker Compose**
```yaml
# docker-compose.enterprise.yml
version: '3.8'
services:
  # Supabase services (already running)
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

### **6.2 Setup Firewall**
```bash
# Install UFW firewall
apt update
apt install -y ufw

# Configure firewall
ufw default deny incoming
ufw default allow outgoing

# Allow SSH
ufw allow ssh

# Allow HTTP/HTTPS
ufw allow 80/tcp
ufw allow 443/tcp

# Allow Supabase ports
ufw allow 8000/tcp  # Kong API Gateway
ufw allow 5432/tcp  # PostgreSQL
ufw allow 6333/tcp  # Qdrant

# Enable firewall
ufw enable
```

## 📊 **Step 7: Monitoring & Backup**

### **7.1 Setup Monitoring**
```bash
# Create monitoring directory
mkdir -p monitoring

# Create Prometheus configuration
cat > monitoring/prometheus.yml << EOF
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
EOF
```

### **7.2 Create Backup Script**
```bash
# Create backup script
cat > backup.sh << 'EOF'
#!/bin/bash

# Create backup directory
mkdir -p backups

# Database backup
docker exec supabase-db-1 pg_dump -U postgres postgres > backups/db_backup_$(date +%Y%m%d_%H%M%S).sql

# File backup
tar -czf backups/files_backup_$(date +%Y%m%d_%H%M%S).tar.gz /data/parquet /data/analysis

# Clean old backups (keep 30 days)
find backups -name "db_backup_*.sql" -mtime +30 -delete
find backups -name "files_backup_*.tar.gz" -mtime +30 -delete

echo "Backup completed: $(date)"
EOF

# Make script executable
chmod +x backup.sh

# Setup automated backups (daily at 2 AM)
crontab -e
# Add: 0 2 * * * /root/backup.sh
```

## 🚀 **Step 8: Deploy Enterprise Services**

### **8.1 Start Enterprise Stack**
```bash
# Deploy with enterprise configuration
docker-compose -f docker-compose.enterprise.yml up -d

# Check all services are running
docker-compose ps

# View logs
docker-compose logs -f
```

### **8.2 Verify Deployment**
```bash
# Test API endpoints
curl http://YOUR_SERVER_IP:8000/rest/v1/
curl http://YOUR_SERVER_IP:8000/auth/v1/
curl http://YOUR_SERVER_IP:8000/storage/v1/

# Test custom services
curl http://YOUR_SERVER_IP:8080/analytics/health
curl http://YOUR_SERVER_IP:6333/health
```

### **8.3 Initialize Database**
```sql
-- Connect to database
docker exec -it supabase-db-1 psql -U postgres

-- Create MudRock schema
CREATE SCHEMA IF NOT EXISTS mudrock;

-- Create PostGIS extension
CREATE EXTENSION IF NOT EXISTS postgis;

-- Create tables (using your schema.sql)
\i /path/to/schema.sql
```

## 📈 **Step 9: Performance Optimization**

### **9.1 PostgreSQL Tuning**
```sql
-- Connect to PostgreSQL
docker exec -it supabase-db-1 psql -U postgres

-- Optimize for geoscience workloads
ALTER SYSTEM SET shared_buffers = '2GB';
ALTER SYSTEM SET effective_cache_size = '6GB';
ALTER SYSTEM SET maintenance_work_mem = '256MB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
ALTER SYSTEM SET default_statistics_target = 100;

-- Reload configuration
SELECT pg_reload_conf();
```

### **9.2 Docker Resource Limits**
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

## 🔧 **Step 10: Maintenance**

### **10.1 Update Services**
```bash
# Pull latest images
docker-compose pull

# Restart services
docker-compose down
docker-compose up -d

# Check for updates
docker-compose logs --tail=100
```

### **10.2 Monitor Resources**
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

### **Hetzner Cloud Costs**
```bash
# Recommended configuration:
Server: €4.15/month (CX11 - 2 vCPU, 4GB RAM, 40GB SSD)
Bandwidth: Included (20TB transfer)
Backup: €2.50/month (optional)
Monitoring: €1.50/month (optional)

Total: ~€8.15/month (~$9/month)
```

### **Comparison with Alternatives**
| Provider | Specs | Monthly Cost | Features |
|----------|-------|--------------|----------|
| **Hetzner** | 2 vCPU, 4GB RAM | €4.15/month | ✅ Cheapest, reliable |
| **Supabase Hosted** | Managed | $25/month | ❌ Expensive |
| **DigitalOcean** | 4 vCPU, 8GB RAM | $40/month | ⚠️ More expensive |
| **AWS EC2** | t3.medium | $30/month | ❌ Expensive |

## 🚨 **Troubleshooting**

### **Common Issues**

**1. SSH Connection Issues**
```bash
# Check if SSH key is correct
ssh -i ~/.ssh/id_rsa root@YOUR_SERVER_IP

# If permission denied, check key permissions
chmod 600 ~/.ssh/id_rsa
chmod 644 ~/.ssh/id_rsa.pub
```

**2. Docker Permission Issues**
```bash
# Add user to docker group
usermod -aG docker $USER

# Logout and login again
exit
ssh root@YOUR_SERVER_IP
```

**3. Port Already in Use**
```bash
# Check what's using the port
netstat -tulpn | grep :8000

# Kill conflicting process
kill -9 [PID]
```

**4. Database Connection Issues**
```bash
# Check PostgreSQL logs
docker-compose logs db

# Test connection
docker exec -it supabase-db-1 psql -U postgres
```

## 🎯 **Next Steps**

1. **Domain Configuration**: Point your domain to Hetzner's nameservers
2. **SSL Certificate**: Install Let's Encrypt for HTTPS
3. **Email Setup**: Configure SMTP for user authentication
4. **Monitoring**: Set up alerts for disk space, memory, and uptime
5. **Backup Strategy**: Implement automated backups to Hetzner Cloud Storage
6. **Security Audit**: Regular security updates and vulnerability scans

## 🎯 **Benefits of Hetzner Self-Hosting**

**Cost Savings:**
- €4.15/month vs $25/month for Supabase hosted
- 83% cost reduction
- Usage-based billing in first month

**Full Control:**
- Complete control over your infrastructure
- Custom enterprise features
- Data sovereignty
- No vendor lock-in

**Scalability:**
- Easy to upgrade server size
- Can add more servers for load balancing
- Horizontal scaling possible

**Reliability:**
- German data centers (GDPR compliant)
- 99.9% uptime guarantee
- 24/7 support

This setup provides a robust, cost-effective foundation for your geoscience data platform with full control over your infrastructure and data. 