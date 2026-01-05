# Dokploy Setup Guide for MudRock Enterprise

## 🎯 **Overview**

This guide provides a comprehensive, reliable method for installing Dokploy on your Hetzner VPS to prevent the common installation issues we encountered. The guide includes automated scripts and manual fallback procedures.

## 🚨 **Common Issues We Encountered**

1. **Docker Swarm Network Conflicts**: Subnet conflicts preventing service startup
2. **Missing Dependencies**: PostgreSQL and Redis services not properly configured
3. **Service Startup Order**: Services starting before dependencies were ready
4. **Network Configuration**: Incorrect advertise address causing connectivity issues

## ✅ **Solution: Automated Installation Script**

We've created a comprehensive installation script that handles all these issues automatically.

### **Script Location**
```
scripts/infrastructure/install-dokploy.sh
```

### **What the Script Does**

1. **Pre-flight Checks**: Verifies system requirements and environment
2. **Docker Installation**: Installs Docker if not present
3. **Cleanup**: Removes any existing broken installations
4. **Swarm Initialization**: Sets up Docker Swarm with proper IP detection
5. **Network Creation**: Creates the required overlay network
6. **Dependency Installation**: Installs PostgreSQL and Redis services first
7. **Service Startup**: Waits for dependencies before starting Dokploy
8. **Health Checks**: Verifies all services are running correctly
9. **Traefik Setup**: Installs the reverse proxy for Dokploy

## 🚀 **Quick Installation**

### **Method 1: Automated Script (Recommended)**

```bash
# Download and run the installation script
curl -sSL https://raw.githubusercontent.com/scrooijmans/MudRock/main/scripts/infrastructure/install-dokploy.sh | bash
```

### **Method 2: Manual Installation**

```bash
# Clone the repository
git clone https://github.com/scrooijmans/MudRock.git
cd MudRock

# Make the script executable
chmod +x scripts/infrastructure/install-dokploy.sh

# Run the installation
sudo ./scripts/infrastructure/install-dokploy.sh
```

## 🔧 **Manual Installation Steps (Fallback)**

If the automated script fails, follow these manual steps:

### **Step 1: Prerequisites**

```bash
# Update system
apt update && apt upgrade -y

# Install required packages
apt install -y curl wget git nano htop ufw postgresql-client redis-tools

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
rm get-docker.sh
systemctl enable docker
systemctl start docker
```

### **Step 2: Clean Up Existing Installation**

```bash
# Remove any existing Dokploy services
docker service rm dokploy dokploy-postgres dokploy-redis 2>/dev/null || true
docker network rm dokploy-network 2>/dev/null || true
docker stop dokploy 2>/dev/null || true
docker rm dokploy 2>/dev/null || true
```

### **Step 3: Initialize Docker Swarm**

```bash
# Get server IP address
ADVERTISE_ADDR=$(curl -4s --connect-timeout 5 https://ifconfig.io)

# Initialize Docker Swarm
docker swarm init --advertise-addr $ADVERTISE_ADDR
```

### **Step 4: Create Network**

```bash
# Create Dokploy network
docker network create --driver overlay --attachable dokploy-network
```

### **Step 5: Install Dependencies**

```bash
# Create directories
mkdir -p /etc/dokploy
chmod -R 777 /etc/dokploy

# Install PostgreSQL
docker service create \
    --name dokploy-postgres \
    --constraint 'node.role==manager' \
    --network dokploy-network \
    --env POSTGRES_USER=dokploy \
    --env POSTGRES_DB=dokploy \
    --env POSTGRES_PASSWORD=amukds4wi9001583845717ad2 \
    --mount type=volume,source=dokploy-postgres-database,target=/var/lib/postgresql/data \
    postgres:16

# Install Redis
docker service create \
    --name dokploy-redis \
    --constraint 'node.role==manager' \
    --network dokploy-network \
    --mount type=volume,source=redis-data-volume,target=/data \
    redis:7
```

### **Step 6: Wait for Dependencies**

```bash
# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL..."
while ! docker service ls | grep -q "dokploy-postgres.*1/1"; do
    sleep 2
done
echo "PostgreSQL is ready"

# Wait for Redis to be ready
echo "Waiting for Redis..."
while ! docker service ls | grep -q "dokploy-redis.*1/1"; do
    sleep 2
done
echo "Redis is ready"
```

### **Step 7: Install Dokploy**

```bash
# Install Dokploy service
docker service create \
    --name dokploy \
    --replicas 1 \
    --network dokploy-network \
    --mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
    --mount type=bind,source=/etc/dokploy,target=/etc/dokploy \
    --mount type=volume,source=dokploy-docker-config,target=/root/.docker \
    --publish published=3000,target=3000,mode=host \
    --update-parallelism 1 \
    --update-order stop-first \
    --constraint 'node.role == manager' \
    -e ADVERTISE_ADDR=$ADVERTISE_ADDR \
    dokploy/dokploy:latest
```

### **Step 8: Install Traefik**

```bash
# Install Traefik
docker run -d \
    --name dokploy-traefik \
    --network dokploy-network \
    --restart always \
    -v /etc/dokploy/traefik/traefik.yml:/etc/traefik/traefik.yml \
    -v /etc/dokploy/traefik/dynamic:/etc/dokploy/traefik/dynamic \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -p 80:80/tcp \
    -p 443:443/tcp \
    -p 443:443/udp \
    traefik:v3.1.2
```

## 🔍 **Verification Steps**

### **Check Service Status**

```bash
# Check all Dokploy services
docker service ls | grep dokploy

# Expected output:
# dokploy           1/1        dokploy/dokploy:latest
# dokploy-postgres  1/1        postgres:16
# dokploy-redis     1/1        redis:7
```

### **Test Accessibility**

```bash
# Test Dokploy dashboard
curl -I http://localhost:3000

# Expected response: HTTP/1.1 200 OK
```

### **Check Logs**

```bash
# Check Dokploy logs
docker service logs dokploy

# Check for any errors
docker service logs dokploy | grep -i error
```

## 🛠️ **Troubleshooting**

### **Issue: Services Not Starting**

```bash
# Check service status
docker service ls

# Check service logs
docker service logs <service-name>

# Restart a service
docker service update --force <service-name>
```

### **Issue: Network Problems**

```bash
# Check network status
docker network ls | grep dokploy

# Recreate network if needed
docker network rm dokploy-network
docker network create --driver overlay --attachable dokploy-network
```

### **Issue: Port Conflicts**

```bash
# Check what's using port 3000
netstat -tlnp | grep :3000

# Kill conflicting processes
sudo kill -9 <PID>
```

### **Issue: Docker Swarm Problems**

```bash
# Leave and rejoin swarm
docker swarm leave --force
docker swarm init --advertise-addr <your-ip>
```

## 📋 **Post-Installation Configuration**

### **1. Access Dokploy Dashboard**

Open your browser and go to:
```
http://your-server-ip:3000
```

### **2. Create Admin Account**

1. Fill in the registration form
2. Set a strong password
3. Complete the initial setup

### **3. Configure Git Integration**

1. Go to Git settings in Dokploy
2. Connect your GitHub account
3. Select repositories for deployment

### **4. Deploy Supabase**

1. Create a new Docker Compose project
2. Upload your `docker-compose.yml` file
3. Configure environment variables
4. Deploy the project

## 🔒 **Security Considerations**

### **Firewall Configuration**

```bash
# Allow necessary ports
ufw allow 22    # SSH
ufw allow 80    # HTTP
ufw allow 443   # HTTPS
ufw allow 3000  # Dokploy
ufw enable
```

### **SSL Configuration**

1. In Dokploy dashboard, go to your project
2. Add your domain name
3. Enable SSL (Let's Encrypt)
4. Force HTTPS redirect

## 📊 **Monitoring and Maintenance**

### **Service Health Checks**

```bash
# Create a monitoring script
cat > /usr/local/bin/check-dokploy.sh << 'EOF'
#!/bin/bash
echo "=== Dokploy Service Status ==="
docker service ls | grep dokploy

echo -e "\n=== Dokploy Accessibility ==="
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000

echo -e "\n=== Disk Usage ==="
df -h /etc/dokploy
EOF

chmod +x /usr/local/bin/check-dokploy.sh
```

### **Backup Configuration**

```bash
# Backup Dokploy configuration
tar -czf dokploy-backup-$(date +%Y%m%d).tar.gz /etc/dokploy
```

## 🎯 **Best Practices**

1. **Always use the automated script** for initial installation
2. **Keep backups** of your Dokploy configuration
3. **Monitor service health** regularly
4. **Update Dokploy** periodically for security patches
5. **Use environment variables** for sensitive configuration
6. **Test deployments** in a staging environment first

## 📞 **Support**

If you encounter issues not covered in this guide:

1. Check the [Dokploy Documentation](https://dokploy.com/docs)
2. Review the service logs for error messages
3. Verify all prerequisites are met
4. Ensure proper network connectivity
5. Check Docker and Docker Swarm status

---

**Last Updated**: January 2025  
**Script Version**: 1.0  
**Compatibility**: Ubuntu 20.04+, Debian 10+, CentOS 8+
