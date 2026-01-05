# Dokploy Quick Reference

## 🚀 **One-Line Installation**

```bash
curl -sSL https://raw.githubusercontent.com/scrooijmans/MudRock/main/scripts/infrastructure/install-dokploy.sh | bash
```

## 🔧 **Manual Installation (If Script Fails)**

```bash
# 1. Clean up existing installation
docker service rm dokploy dokploy-postgres dokploy-redis 2>/dev/null || true
docker network rm dokploy-network 2>/dev/null || true

# 2. Initialize Docker Swarm
docker swarm init --advertise-addr $(curl -4s https://ifconfig.io)

# 3. Create network
docker network create --driver overlay --attachable dokploy-network

# 4. Install PostgreSQL
docker service create --name dokploy-postgres --constraint 'node.role==manager' --network dokploy-network --env POSTGRES_USER=dokploy --env POSTGRES_DB=dokploy --env POSTGRES_PASSWORD=amukds4wi9001583845717ad2 --mount type=volume,source=dokploy-postgres-database,target=/var/lib/postgresql/data postgres:16

# 5. Install Redis
docker service create --name dokploy-redis --constraint 'node.role==manager' --network dokploy-network --mount type=volume,source=redis-data-volume,target=/data redis:7

# 6. Wait for dependencies
while ! docker service ls | grep -q "dokploy-postgres.*1/1"; do sleep 2; done
while ! docker service ls | grep -q "dokploy-redis.*1/1"; do sleep 2; done

# 7. Install Dokploy
docker service create --name dokploy --replicas 1 --network dokploy-network --mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock --mount type=bind,source=/etc/dokploy,target=/etc/dokploy --mount type=volume,source=dokploy-docker-config,target=/root/.docker --publish published=3000,target=3000,mode=host --update-parallelism 1 --update-order stop-first --constraint 'node.role == manager' -e ADVERTISE_ADDR=$(curl -4s https://ifconfig.io) dokploy/dokploy:latest

# 8. Install Traefik
docker run -d --name dokploy-traefik --network dokploy-network --restart always -v /etc/dokploy/traefik/traefik.yml:/etc/traefik/traefik.yml -v /etc/dokploy/traefik/dynamic:/etc/dokploy/traefik/dynamic -v /var/run/docker.sock:/var/run/docker.sock -p 80:80/tcp -p 443:443/tcp -p 443:443/udp traefik:v3.1.2
```

## 🔍 **Verification Commands**

```bash
# Check service status
docker service ls | grep dokploy

# Test accessibility
curl -I http://localhost:3000

# Check logs
docker service logs dokploy
```

## 🛠️ **Common Troubleshooting**

```bash
# Restart Dokploy service
docker service update --force dokploy

# Check service logs
docker service logs dokploy | tail -50

# Recreate network
docker network rm dokploy-network
docker network create --driver overlay --attachable dokploy-network

# Reset Docker Swarm
docker swarm leave --force
docker swarm init --advertise-addr $(curl -4s https://ifconfig.io)
```

## 📋 **Access Information**

- **Dashboard URL**: `http://your-server-ip:3000`
- **Default Port**: 3000
- **Admin Setup**: First-time access requires account creation
- **Git Integration**: Configure in Dokploy dashboard

## 🔒 **Security Notes**

- Change default PostgreSQL password in production
- Use strong admin passwords
- Configure firewall rules
- Enable SSL certificates for production use
- Regular backups of `/etc/dokploy` directory

---

**Last Updated**: January 2025  
**Script Version**: 1.0
