# Cloud Deployment Guide for MudRock Qdrant Orchestrator

## Overview

This guide shows how to migrate your local Qdrant setup to various cloud environments, from simple cloud deployments to secure private cloud solutions.

## Local Docker Setup (Current)

### Quick Start
```bash
# Start Qdrant and orchestrator locally
docker-compose -f docker-compose.qdrant.yml up -d

# Test the setup
curl http://localhost:8080/health
curl -X POST http://localhost:8080/api/query \
  -H "Content-Type: application/json" \
  -d '{"query": "For all wells in Browse Basin find slowness logs and convert to velocity"}'
```

### Local Configuration
```bash
# Environment variables for local development
export QDRANT_URL=http://localhost:6334
export PARQUET_BASE_PATH=./data/parquet
export TEMP_DIR=./temp
export RUST_LOG=info
export PORT=8080
```

## Cloud Deployment Options

### 1. Qdrant Cloud + Orchestrator on VPS

#### Step 1: Set up Qdrant Cloud
1. Create account at [cloud.qdrant.io](https://cloud.qdrant.io)
2. Create a new cluster
3. Get your API key and cluster URL

#### Step 2: Deploy Orchestrator to VPS
```bash
# On your VPS (Ubuntu/Debian)
sudo apt update
sudo apt install docker.io docker-compose

# Clone your repository
git clone <your-repo>
cd MudRock

# Create cloud configuration
cat > cloud-config.toml << EOF
url = "https://your-cluster.cloud.qdrant.io:6334"
api_key = "your-api-key"
[collections]
vector_size = 384
distance = "Cosine"
[storage]
parquet_base_path = "/data/parquet"
temp_dir = "/tmp"
EOF

# Set environment variables
export QDRANT_URL=https://your-cluster.cloud.qdrant.io:6334
export QDRANT_API_KEY=your-api-key
export CONFIG_PATH=./cloud-config.toml

# Build and run
docker-compose -f docker-compose.qdrant.yml up -d qdrant-orchestrator
```

### 2. Private Cloud with Shakudo VPC

#### Step 1: Set up Shakudo VPC
1. Contact Shakudo for VPC setup
2. Deploy Qdrant Hybrid Cloud in your VPC
3. Configure network access and security

#### Step 2: Deploy Orchestrator
```bash
# Create private cloud configuration
cat > private-cloud-config.toml << EOF
url = "https://your-vpc-qdrant.shakudo.io:6334"
api_key = "your-vpc-api-key"
[collections]
vector_size = 384
distance = "Cosine"
[storage]
parquet_base_path = "/data/parquet"
temp_dir = "/tmp"
EOF

# Deploy with Kubernetes
kubectl apply -f k8s/qdrant-orchestrator.yaml
```

#### Kubernetes Deployment
```yaml
# k8s/qdrant-orchestrator.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: qdrant-orchestrator
spec:
  replicas: 3
  selector:
    matchLabels:
      app: qdrant-orchestrator
  template:
    metadata:
      labels:
        app: qdrant-orchestrator
    spec:
      containers:
      - name: orchestrator
        image: your-registry/qdrant-orchestrator:latest
        ports:
        - containerPort: 8080
        env:
        - name: QDRANT_URL
          value: "https://your-vpc-qdrant.shakudo.io:6334"
        - name: QDRANT_API_KEY
          valueFrom:
            secretKeyRef:
              name: qdrant-secrets
              key: api-key
        - name: CONFIG_PATH
          value: "/app/config/private-cloud-config.toml"
        volumeMounts:
        - name: config
          mountPath: /app/config
        - name: data
          mountPath: /data
      volumes:
      - name: config
        configMap:
          name: qdrant-config
      - name: data
        persistentVolumeClaim:
          claimName: qdrant-data-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: qdrant-orchestrator-service
spec:
  selector:
    app: qdrant-orchestrator
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

### 3. AWS/GCP/Azure Deployment

#### AWS ECS Deployment
```bash
# Create ECS task definition
aws ecs register-task-definition --cli-input-json file://ecs-task-definition.json

# Create ECS service
aws ecs create-service \
  --cluster mudrock-cluster \
  --service-name qdrant-orchestrator \
  --task-definition qdrant-orchestrator:1 \
  --desired-count 2
```

#### Task Definition
```json
{
  "family": "qdrant-orchestrator",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "containerDefinitions": [
    {
      "name": "orchestrator",
      "image": "your-ecr-repo/qdrant-orchestrator:latest",
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "QDRANT_URL",
          "value": "https://your-qdrant-cluster.cloud.qdrant.io:6334"
        },
        {
          "name": "QDRANT_API_KEY",
          "value": "your-api-key"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/qdrant-orchestrator",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

## Configuration Management

### Environment-Specific Configs
```rust
// config/local.toml
url = "http://localhost:6334"
# No API key for local development

// config/cloud.toml
url = "https://your-cluster.cloud.qdrant.io:6334"
api_key = "your-api-key"

// config/private-cloud.toml
url = "https://your-vpc-qdrant.shakudo.io:6334"
api_key = "your-vpc-api-key"
```

### Secrets Management
```bash
# For Kubernetes
kubectl create secret generic qdrant-secrets \
  --from-literal=api-key=your-api-key

# For Docker Compose
echo "your-api-key" | docker secret create qdrant-api-key -
```

## Data Migration

### From Local to Cloud
```bash
# Export local data
curl http://localhost:8080/api/collections > collections.json

# Import to cloud
curl -X POST https://your-cloud-orchestrator/api/import \
  -H "Content-Type: application/json" \
  -d @collections.json
```

### Parquet File Migration
```bash
# Sync Parquet files to cloud storage
aws s3 sync ./data/parquet s3://your-bucket/parquet/

# Update file paths in Qdrant
curl -X POST https://your-cloud-orchestrator/api/update-paths \
  -H "Content-Type: application/json" \
  -d '{"old_prefix": "file://", "new_prefix": "s3://your-bucket/"}'
```

## Security Considerations

### Network Security
- Use VPC for private cloud deployments
- Configure security groups/firewall rules
- Enable TLS/SSL for all communications

### Authentication
```rust
// Add authentication middleware
use axum::middleware::from_fn_with_state;

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Validate JWT or API key
    validate_token(auth_header)?;
    
    Ok(next.run(request).await)
}
```

### Data Encryption
- Encrypt data at rest
- Use TLS for data in transit
- Implement field-level encryption for sensitive data

## Monitoring and Observability

### Health Checks
```bash
# Health check endpoint
curl https://your-cloud-orchestrator/health

# Expected response
{
  "status": "healthy",
  "qdrant_connected": true,
  "collections_ready": true,
  "version": "1.0.0"
}
```

### Metrics and Logging
```rust
// Add Prometheus metrics
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref QUERY_COUNTER: Counter = register_counter!(
        "qdrant_queries_total",
        "Total number of queries"
    ).unwrap();
    
    static ref QUERY_DURATION: Histogram = register_histogram!(
        "qdrant_query_duration_seconds",
        "Query duration in seconds"
    ).unwrap();
}
```

### Alerting
```yaml
# prometheus/alerting.yml
groups:
- name: qdrant-alerts
  rules:
  - alert: QdrantDown
    expr: up{job="qdrant-orchestrator"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Qdrant orchestrator is down"
```

## Cost Optimization

### Resource Scaling
```yaml
# Horizontal Pod Autoscaler
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: qdrant-orchestrator-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: qdrant-orchestrator
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

### Storage Optimization
- Use object storage (S3, GCS) for Parquet files
- Implement data lifecycle policies
- Compress data where appropriate

## Migration Checklist

- [ ] Set up cloud Qdrant cluster
- [ ] Configure orchestrator for cloud environment
- [ ] Set up authentication and authorization
- [ ] Migrate data and Parquet files
- [ ] Update file paths and references
- [ ] Configure monitoring and alerting
- [ ] Test all functionality
- [ ] Set up backup and disaster recovery
- [ ] Document deployment procedures
- [ ] Train team on cloud operations

## Troubleshooting

### Common Issues
1. **Connection Timeouts**: Check network connectivity and firewall rules
2. **Authentication Errors**: Verify API keys and permissions
3. **Data Not Found**: Ensure Parquet files are accessible in cloud storage
4. **Performance Issues**: Monitor resource usage and scale accordingly

### Debug Commands
```bash
# Check Qdrant connection
curl -H "api-key: your-key" https://your-cluster.cloud.qdrant.io:6333/collections

# Check orchestrator logs
kubectl logs -f deployment/qdrant-orchestrator

# Test query processing
curl -X POST https://your-cloud-orchestrator/api/query \
  -H "Content-Type: application/json" \
  -d '{"query": "test query"}'
```

This deployment guide provides a comprehensive path from local development to production cloud deployment, with specific considerations for security, scalability, and cost optimization. 