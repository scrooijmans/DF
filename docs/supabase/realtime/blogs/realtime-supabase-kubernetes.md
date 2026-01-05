Complete Guide: Setting up Supabase in Kubernetes with Real-time Examples
ThamizhElango Natarajan
ThamizhElango Natarajan

Follow
15 min read
·
Jul 29, 2025
1






Supabase has become a popular open-source alternative to Firebase, offering a complete backend-as-a-service solution with PostgreSQL, real-time subscriptions, authentication, and storage. While Supabase provides hosted solutions, running it on your own Kubernetes cluster gives you complete control over your data and infrastructure.

In this comprehensive guide, we’ll walk through setting up a complete Supabase stack on Kubernetes with real-time examples and production-ready configurations.

Prerequisites
Before we begin, ensure you have:

A running Kubernetes cluster (v1.20+)
kubectl configured to communicate with your cluster
Helm 3.x installed
At least 8GB RAM and 4 CPU cores available in your cluster
A storage class configured for persistent volumes
Architecture Overview
Our Supabase deployment will include:

PostgreSQL: The core database with required extensions
PostgREST: Automatic REST API generation
Realtime: WebSocket server for real-time subscriptions
GoTrue: Authentication server
Storage: File storage service
Kong: API Gateway for routing and rate limiting
Supabase Studio: Web-based database management interface
Step 1: Create Namespace and ConfigMaps
First, let’s create a dedicated namespace and configuration:

# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: supabase
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: supabase-config
  namespace: supabase
data:
  POSTGRES_DB: "postgres"
  POSTGRES_USER: "postgres"
  POSTGRES_HOST: "supabase-db"
  POSTGRES_PORT: "5432"
  JWT_SECRET: "your-super-secret-jwt-token-with-at-least-32-characters-long"
  ANON_KEY: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0NTcxNDY2M30.pRkUhyIhKD9eQj6QBn5-NMB4bx8UzJmw3C4UYcLgQHA"
  SERVICE_ROLE_KEY: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaWF0IjoxNjQ1NzE0NjYzfQ.YNdeYBGWGZqw4oI-2k6EewWKcAm0jCYnRIFHfP3WQ4s"
  API_EXTERNAL_URL: "http://localhost:8000"
  SUPABASE_URL: "http://localhost:8000"
  SUPABASE_PUBLIC_URL: "http://localhost:8000"
---
apiVersion: v1
kind: Secret
metadata:
  name: supabase-secrets
  namespace: supabase
type: Opaque
stringData:
  POSTGRES_PASSWORD: "your-super-secret-password"
  DASHBOARD_USERNAME: "admin"
  DASHBOARD_PASSWORD: "admin-password"
Step 2: PostgreSQL Database Setup
Let’s deploy PostgreSQL with the required extensions:

# postgres-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-db
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-db
  template:
    metadata:
      labels:
        app: supabase-db
    spec:
      containers:
      - name: postgres
        image: supabase/postgres:14.1.0.21
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_DB
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: POSTGRES_DB
        - name: POSTGRES_USER
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: POSTGRES_USER
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: supabase-secrets
              key: POSTGRES_PASSWORD
        - name: JWT_SECRET
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: JWT_SECRET
        - name: JWT_EXP
          value: "3600"
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
      volumes:
      - name: postgres-storage
        persistentVolumeClaim:
          claimName: postgres-pvc
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: postgres-pvc
  namespace: supabase
spec:
  accessModes:
  - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-db
  namespace: supabase
spec:
  selector:
    app: supabase-db
  ports:
  - port: 5432
    targetPort: 5432
  type: ClusterIP
Step 3: Kong API Gateway
Kong acts as the API gateway, handling routing and authentication:

# kong-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-kong
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-kong
  template:
    metadata:
      labels:
        app: supabase-kong
    spec:
      containers:
      - name: kong
        image: kong:2.8.1
        ports:
        - containerPort: 8000
        - containerPort: 8001
        env:
        - name: KONG_DATABASE
          value: "off"
        - name: KONG_DECLARATIVE_CONFIG
          value: "/var/lib/kong/kong.yml"
        - name: KONG_DNS_ORDER
          value: "LAST,A,CNAME"
        - name: KONG_PLUGINS
          value: "request-size-limiting,cors,key-auth,acl"
        - name: KONG_NGINX_PROXY_PROXY_BUFFER_SIZE
          value: "160k"
        - name: KONG_NGINX_PROXY_PROXY_BUFFERS
          value: "64 160k"
        volumeMounts:
        - name: kong-config
          mountPath: /var/lib/kong
      volumes:
      - name: kong-config
        configMap:
          name: kong-config
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: kong-config
  namespace: supabase
data:
  kong.yml: |
    _format_version: "1.1"
    consumers:
      - username: anon
        keyauth_credentials:
          - key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0NTcxNDY2M30.pRkUhyIhKD9eQj6QBn5-NMB4bx8UzJmw3C4UYcLgQHA
      - username: service_role
        keyauth_credentials:
          - key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaWF0IjoxNjQ1NzE0NjYzfQ.YNdeYBGWGZqw4oI-2k6EewWKcAm0jCYnRIFHfP3WQ4s
    acls:
      - consumer: anon
        group: anon
      - consumer: service_role
        group: service_role
    services:
      - name: auth-v1-open
        url: http://supabase-auth:9999/verify
        routes:
          - name: auth-v1-open
            strip_path: true
            paths:
              - /auth/v1/verify
        plugins:
          - name: cors
      - name: auth-v1-open-callback
        url: http://supabase-auth:9999/callback
        routes:
          - name: auth-v1-open-callback
            strip_path: true
            paths:
              - /auth/v1/callback
        plugins:
          - name: cors
      - name: auth-v1
        url: http://supabase-auth:9999/
        routes:
          - name: auth-v1-all
            strip_path: true
            paths:
              - /auth/v1/
        plugins:
          - name: cors
          - name: key-auth
            config:
              hide_credentials: false
      - name: rest-v1
        url: http://supabase-rest:3000/
        routes:
          - name: rest-v1-all
            strip_path: true
            paths:
              - /rest/v1/
        plugins:
          - name: cors
          - name: key-auth
            config:
              hide_credentials: true
      - name: realtime-v1
        url: http://supabase-realtime:4000/socket/
        routes:
          - name: realtime-v1-all
            strip_path: true
            paths:
              - /realtime/v1/
        plugins:
          - name: cors
          - name: key-auth
            config:
              hide_credentials: false
      - name: storage-v1
        url: http://supabase-storage:5000/
        routes:
          - name: storage-v1-all
            strip_path: true
            paths:
              - /storage/v1/
        plugins:
          - name: cors
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-kong
  namespace: supabase
spec:
  selector:
    app: supabase-kong
  ports:
  - name: proxy
    port: 8000
    targetPort: 8000
  - name: admin
    port: 8001
    targetPort: 8001
  type: LoadBalancer
Step 4: PostgREST API Server
PostgREST automatically generates a REST API from your PostgreSQL schema:

# postgrest-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-rest
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-rest
  template:
    metadata:
      labels:
        app: supabase-rest
    spec:
      containers:
      - name: postgrest
        image: postgrest/postgrest:v10.1.1
        ports:
        - containerPort: 3000
        env:
        - name: PGRST_DB_URI
          value: "postgres://authenticator:your-super-secret-password@supabase-db:5432/postgres"
        - name: PGRST_DB_SCHEMAS
          value: "public,storage,graphql_public"
        - name: PGRST_DB_ANON_ROLE
          value: "anon"
        - name: PGRST_JWT_SECRET
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: JWT_SECRET
        - name: PGRST_DB_USE_LEGACY_GUCS
          value: "false"
        - name: PGRST_APP_SETTINGS_JWT_SECRET
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: JWT_SECRET
        - name: PGRST_APP_SETTINGS_JWT_EXP
          value: "3600"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-rest
  namespace: supabase
spec:
  selector:
    app: supabase-rest
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
Step 5: Realtime Server
The Realtime server enables WebSocket connections for live data updates:

# realtime-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-realtime
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-realtime
  template:
    metadata:
      labels:
        app: supabase-realtime
    spec:
      containers:
      - name: realtime
        image: supabase/realtime:v2.10.1
        ports:
        - containerPort: 4000
        env:
        - name: PORT
          value: "4000"
        - name: DB_HOST
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: POSTGRES_HOST
        - name: DB_PORT
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: POSTGRES_PORT
        - name: DB_USER
          value: "supabase_realtime_admin"
        - name: DB_PASSWORD
          valueFrom:
            secretKeyRef:
              name: supabase-secrets
              key: POSTGRES_PASSWORD
        - name: DB_NAME
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: POSTGRES_DB
        - name: DB_AFTER_CONNECT_QUERY
          value: "SET search_path TO _realtime"
        - name: DB_ENC_KEY
          value: "supabaserealtime"
        - name: API_JWT_SECRET
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: JWT_SECRET
        - name: FLY_ALLOC_ID
          value: "fly123"
        - name: FLY_APP_NAME
          value: "realtime"
        - name: SECRET_KEY_BASE
          value: "UpNVntn3cDxHJpq99YMc1T1AQgQpc8kfYTuRgBiYa15BLrx8etQoXz3gZv1/u2oq"
        - name: ERL_AFLAGS
          value: "-proto_dist inet_tcp"
        - name: ENABLE_TAILSCALE
          value: "false"
        - name: DNS_NODES
          value: "''"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-realtime
  namespace: supabase
spec:
  selector:
    app: supabase-realtime
  ports:
  - port: 4000
    targetPort: 4000
  type: ClusterIP
Step 6: GoTrue Authentication Server
GoTrue handles user authentication and management:

# auth-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-auth
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-auth
  template:
    metadata:
      labels:
        app: supabase-auth
    spec:
      containers:
      - name: gotrue
        image: supabase/gotrue:v2.10.0
        ports:
        - containerPort: 9999
        env:
        - name: GOTRUE_API_HOST
          value: "0.0.0.0"
        - name: GOTRUE_API_PORT
          value: "9999"
        - name: API_EXTERNAL_URL
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: API_EXTERNAL_URL
        - name: GOTRUE_DB_DRIVER
          value: "postgres"
        - name: GOTRUE_DB_DATABASE_URL
          value: "postgres://supabase_auth_admin:your-super-secret-password@supabase-db:5432/postgres"
        - name: GOTRUE_SITE_URL
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: SUPABASE_PUBLIC_URL
        - name: GOTRUE_URI_ALLOW_LIST
          value: "*"
        - name: GOTRUE_DISABLE_SIGNUP
          value: "false"
        - name: GOTRUE_JWT_ADMIN_ROLES
          value: "service_role"
        - name: GOTRUE_JWT_AUD
          value: "authenticated"
        - name: GOTRUE_JWT_DEFAULT_GROUP_NAME
          value: "authenticated"
        - name: GOTRUE_JWT_EXP
          value: "3600"
        - name: GOTRUE_JWT_SECRET
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: JWT_SECRET
        - name: GOTRUE_EXTERNAL_EMAIL_ENABLED
          value: "true"
        - name: GOTRUE_MAILER_AUTOCONFIRM
          value: "false"
        - name: GOTRUE_SMTP_ADMIN_EMAIL
          value: "admin@example.com"
        - name: GOTRUE_SMTP_HOST
          value: "supabase-inbucket"
        - name: GOTRUE_SMTP_PORT
          value: "2500"
        - name: GOTRUE_SMTP_USER
          value: "fake_mail_user"
        - name: GOTRUE_SMTP_PASS
          value: "fake_mail_password"
        - name: GOTRUE_SMTP_SENDER_NAME
          value: "fake_sender"
        - name: GOTRUE_MAILER_URLPATHS_INVITE
          value: "/auth/v1/verify"
        - name: GOTRUE_MAILER_URLPATHS_CONFIRMATION
          value: "/auth/v1/verify"
        - name: GOTRUE_MAILER_URLPATHS_RECOVERY
          value: "/auth/v1/verify"
        - name: GOTRUE_MAILER_URLPATHS_EMAIL_CHANGE
          value: "/auth/v1/verify"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-auth
  namespace: supabase
spec:
  selector:
    app: supabase-auth
  ports:
  - port: 9999
    targetPort: 9999
  type: ClusterIP
Step 7: Storage Service
The storage service handles file uploads and management:

# storage-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-storage
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-storage
  template:
    metadata:
      labels:
        app: supabase-storage
    spec:
      containers:
      - name: storage
        image: supabase/storage-api:v0.40.4
        ports:
        - containerPort: 5000
        env:
        - name: ANON_KEY
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: ANON_KEY
        - name: SERVICE_KEY
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: SERVICE_ROLE_KEY
        - name: POSTGREST_URL
          value: "http://supabase-rest:3000"
        - name: PGRST_JWT_SECRET
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: JWT_SECRET
        - name: DATABASE_URL
          value: "postgres://supabase_storage_admin:your-super-secret-password@supabase-db:5432/postgres"
        - name: FILE_SIZE_LIMIT
          value: "52428800"
        - name: STORAGE_BACKEND
          value: "file"
        - name: FILE_STORAGE_BACKEND_PATH
          value: "/var/lib/storage"
        - name: TENANT_ID
          value: "stub"
        - name: REGION
          value: "stub"
        - name: GLOBAL_S3_BUCKET
          value: "stub"
        - name: ENABLE_IMAGE_TRANSFORMATION
          value: "true"
        - name: IMGPROXY_URL
          value: "http://supabase-imgproxy:5001"
        volumeMounts:
        - name: storage-data
          mountPath: /var/lib/storage
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
      volumes:
      - name: storage-data
        persistentVolumeClaim:
          claimName: storage-pvc
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: storage-pvc
  namespace: supabase
spec:
  accessModes:
  - ReadWriteOnce
  resources:
    requests:
      storage: 5Gi
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-storage
  namespace: supabase
spec:
  selector:
    app: supabase-storage
  ports:
  - port: 5000
    targetPort: 5000
  type: ClusterIP
Step 8: Supabase Studio (Dashboard)
The Studio provides a web interface for managing your database:

# studio-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: supabase-studio
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: supabase-studio
  template:
    metadata:
      labels:
        app: supabase-studio
    spec:
      containers:
      - name: studio
        image: supabase/studio:20230803-a91625f
        ports:
        - containerPort: 3000
        env:
        - name: STUDIO_PG_META_URL
          value: "http://supabase-meta:8080"
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: supabase-secrets
              key: POSTGRES_PASSWORD
        - name: DEFAULT_ORGANIZATION_NAME
          value: "Default Organization"
        - name: DEFAULT_PROJECT_NAME
          value: "Default Project"
        - name: SUPABASE_URL
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: SUPABASE_URL
        - name: SUPABASE_PUBLIC_URL
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: SUPABASE_PUBLIC_URL
        - name: SUPABASE_ANON_KEY
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: ANON_KEY
        - name: SUPABASE_SERVICE_KEY
          valueFrom:
            configMapKeyRef:
              name: supabase-config
              key: SERVICE_ROLE_KEY
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: supabase-studio
  namespace: supabase
spec:
  selector:
    app: supabase-studio
  ports:
  - port: 3000
    targetPort: 3000
  type: LoadBalancer
Step 9: Deploy All Components
Now let’s deploy everything:

# Apply all configurations
kubectl apply -f namespace.yaml
kubectl apply -f postgres-deployment.yaml
kubectl apply -f kong-deployment.yaml
kubectl apply -f postgrest-deployment.yaml
kubectl apply -f realtime-deployment.yaml
kubectl apply -f auth-deployment.yaml
kubectl apply -f storage-deployment.yaml
kubectl apply -f studio-deployment.yaml

# Check the status
kubectl get pods -n supabase
kubectl get services -n supabase
Step 10: Real-time Example Application
Let’s create a real-time chat application to demonstrate Supabase’s capabilities:

Database Setup
First, create the necessary tables in your database:

-- Connect to your database and run these commands
CREATE TABLE IF NOT EXISTS messages (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  content TEXT NOT NULL,
  user_email TEXT NOT NULL,
  room_id TEXT NOT NULL DEFAULT 'general'
);

-- Enable real-time
ALTER TABLE messages REPLICA IDENTITY FULL;
ALTER PUBLICATION supabase_realtime ADD TABLE messages;

-- Enable RLS (Row Level Security)
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;

-- Create policies
CREATE POLICY "Enable read access for all users" ON messages FOR SELECT USING (true);
CREATE POLICY "Enable insert for authenticated users only" ON messages FOR INSERT WITH CHECK (auth.role() = 'authenticated');
Frontend Application
Create a simple HTML page to test real-time functionality:

<!DOCTYPE html>
<html>
<head>
    <title>Supabase Real-time Chat</title>
    <script src="https://unpkg.com/@supabase/supabase-js@2"></script>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        #messages { height: 400px; overflow-y: scroll; border: 1px solid #ccc; padding: 10px; margin: 10px 0; }
        #messageForm { display: flex; gap: 10px; }
        #messageInput { flex: 1; padding: 10px; }
        button { padding: 10px 20px; }
        .message { margin: 5px 0; padding: 5px; background: #f0f0f0; border-radius: 5px; }
        .timestamp { font-size: 0.8em; color: #666; }
    </style>
</head>
<body>
    <h1>Supabase Real-time Chat</h1>
    
    <div id="auth-section">
        <input type="email" id="email" placeholder="Enter your email">
        <input type="password" id="password" placeholder="Enter password">
        <button onclick="signUp()">Sign Up</button>
        <button onclick="signIn()">Sign In</button>
        <button onclick="signOut()">Sign Out</button>
    </div>
    
    <div id="chat-section" style="display: none;">
        <div id="messages"></div>
        <form id="messageForm">
            <input type="text" id="messageInput" placeholder="Type your message..." required>
            <button type="submit">Send</button>
        </form>
    </div>

    <script>
        // Initialize Supabase client
        const supabaseUrl = 'http://localhost:8000' // Your Kong gateway URL
        const supabaseKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0NTcxNDY2M30.pRkUhyIhKD9eQj6QBn5-NMB4bx8UzJmw3C4UYcLgQHA'
        const supabase = supabase.createClient(supabaseUrl, supabaseKey)

        let currentUser = null;

        // Authentication functions
        async function signUp() {
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;
            
            const { data, error } = await supabase.auth.signUp({
                email: email,
                password: password,
            });
            
            if (error) {
                alert('Error: ' + error.message);
            } else {
                alert('Check your email for verification link!');
            }
        }

        async function signIn() {
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;
            
            const { data, error } = await supabase.auth.signInWithPassword({
                email: email,
                password: password,
            });
            
            if (error) {
                alert('Error: ' + error.message);
            } else {
                currentUser = data.user;
                showChatSection();
                loadMessages();
                setupRealtime();
            }
        }

        async function signOut() {
            const { error } = await supabase.auth.signOut();
            if (error) {
                alert('Error: ' + error.message);
            } else {
                hideChatSection();
            }
        }

        function showChatSection() {
            document.getElementById('auth-section').style.display = 'none';
            document.getElementById('chat-section').style.display = 'block';
        }

        function hideChatSection() {
            document.getElementById('auth-section').style.display = 'block';
            document.getElementById('chat-section').style.display = 'none';
            currentUser = null;
        }

        // Chat functions
        async function loadMessages() {
            const { data, error } = await supabase
                .from('messages')
                .select('*')
                .order('created_at', { ascending: true })
                .limit(50);

            if (error) {
                console.error('Error loading messages:', error);
                return;
            }

            const messagesDiv = document.getElementById('messages');
            messagesDiv.innerHTML = '';
            
            data.forEach(message => {
                displayMessage(message);
            });
        }

        function displayMessage(message) {
            const messagesDiv = document.getElementById('messages');
            const messageEl = document.createElement('div');
            messageEl.className = 'message';
            
            const timestamp = new Date(message.created_at).toLocaleTimeString();
            messageEl.innerHTML = `
                <strong>${message.user_email}:</strong> ${message.content}
                <div class="timestamp">${timestamp}</div>
            `;
            
            messagesDiv.appendChild(messageEl);
            messagesDiv.scrollTop = messagesDiv.scrollHeight;
        }

        async function sendMessage(content) {
            if (!currentUser) return;

            const { data, error } = await supabase
                .from('messages')
                .insert([
                    {
                        content: content,
                        user_email: currentUser.email,
                        room_id: 'general'
                    }
                ]);

            if (error) {
                console.error('Error sending message:', error);
                alert('Error sending message: ' + error.message);
            }
        }

        function setupRealtime() {
            // Subscribe to real-time changes
            const channel = supabase
                .channel('messages')
                .on('postgres_changes', 
                    { 
                        event: 'INSERT', 
                        schema: 'public', 
                        table: 'messages' 
                    }, 
                    (payload) => {
                        displayMessage(payload.new);
                    }
                )
                .subscribe();
        }

        // Form submission
        document.getElementById('messageForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            const input = document.getElementById('messageInput');
            const content = input.value.trim();
            
            if (content) {
                await sendMessage(content);
                input.value = '';
            }
        });

        // Check if user is already logged in
        supabase.auth.onAuthStateChange((event, session) => {
            if (session) {
                currentUser = session.user;
                showChatSection();
                loadMessages();
                setupRealtime();
            } else {
                hideChatSection();
            }
        });
    </script>
</body>
</html>
Step 11: Testing and Monitoring
Health Checks
Create a simple health check script:

#!/bin/bash
# health-check.sh

NAMESPACE="supabase"
KONG_URL="http://localhost:8000"

echo "Checking Supabase services health..."

# Check if all pods are running
echo "Pod Status:"
kubectl get pods -n $NAMESPACE

# Check services
echo -e "\nService Status:"
kubectl get services -n $NAMESPACE

# Test API endpoints
echo -e "\nAPI Health Checks:"

# Test REST API
echo "Testing REST API..."
curl -s -o /dev/null -w "REST API: %{http_code}\n" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0NTcxNDY2M30.pRkUhyIhKD9eQj6QBn5-NMB4bx8UzJmw3C4UYcLgQHA" \
  "$KONG_URL/rest/v1/"

# Test Auth API
echo "Testing Auth API..."
curl -s -o /dev/null -w "Auth API: %{http_code}\n" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0NTcxNDY2M30.pRkUhyIhKD9eQj6QBn5-NMB4bx8UzJmw3C4UYcLgQHA" \
  "$KONG_URL/auth/v1/settings"

# Test Storage API
echo "Testing Storage API..."
curl -s -o /dev/null -w "Storage API: %{http_code}\n" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0NTcxNDY2M30.pRkUhyIhKD9eQj6QBn5-NMB4bx8UzJmw3C4UYcLgQHA" \
  "$KONG_URL/storage/v1/buckets"

echo -e "\nHealth check completed!"
Monitoring with Prometheus
Create monitoring configurations:

# monitoring.yaml
apiVersion: v1
kind: ServiceMonitor
metadata:
  name: supabase-monitoring
  namespace: supabase
spec:
  selector:
    matchLabels:
      monitor: supabase
  endpoints:
  - port: metrics
    interval: 30s
    path: /metrics
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: grafana-dashboard
  namespace: supabase
data:
  supabase-dashboard.json: |
    {
      "dashboard": {
        "title": "Supabase Monitoring",
        "panels": [
          {
            "title": "API Requests",
            "type": "graph",
            "targets": [
              {
                "expr": "rate(kong_http_requests_total[5m])",
                "legendFormat": "{{service}}"
              }
            ]
          },
          {
            "title": "Database Connections",
            "type": "graph",
            "targets": [
              {
                "expr": "pg_stat_activity_count",
                "legendFormat": "Active Connections"
              }
            ]
          },
          {
            "title": "Real-time Connections",
            "type": "graph",
            "targets": [
              {
                "expr": "realtime_connected_clients",
                "legendFormat": "WebSocket Connections"
              }
            ]
          }
        ]
      }
    }
Step 12: Production Considerations
Security Hardening
# security-policies.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: supabase-network-policy
  namespace: supabase
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: supabase
    - podSelector: {}
  - from: []
    ports:
    - protocol: TCP
      port: 8000  # Kong gateway
    - protocol: TCP
      port: 3000  # Studio
  egress:
  - to: []
    ports:
    - protocol: TCP
      port: 53   # DNS
    - protocol: UDP
      port: 53   # DNS
    - protocol: TCP
      port: 443  # HTTPS
    - protocol: TCP
      port: 80   # HTTP
  - to:
    - namespaceSelector:
        matchLabels:
          name: supabase
---
apiVersion: v1
kind: Secret
metadata:
  name: supabase-tls
  namespace: supabase
type: kubernetes.io/tls
data:
  tls.crt: LS0tLS1CRUdJTi... # Your TLS certificate
  tls.key: LS0tLS1CRUdJTi... # Your TLS private key
Backup Configuration
# backup-cronjob.yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: supabase
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: postgres-backup
            image: postgres:14
            command:
            - /bin/bash
            - -c
            - |
              export PGPASSWORD="$POSTGRES_PASSWORD"
              pg_dump -h supabase-db -U postgres -d postgres > /backup/postgres-$(date +%Y%m%d-%H%M%S).sql
              # Keep only last 7 backups
              ls -t /backup/postgres-*.sql | tail -n +8 | xargs rm -f
            env:
            - name: POSTGRES_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: supabase-secrets
                  key: POSTGRES_PASSWORD
            volumeMounts:
            - name: backup-storage
              mountPath: /backup
          volumes:
          - name: backup-storage
            persistentVolumeClaim:
              claimName: backup-pvc
          restartPolicy: OnFailure
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: backup-pvc
  namespace: supabase
spec:
  accessModes:
  - ReadWriteOnce
  resources:
    requests:
      storage: 50Gi
Scaling Configuration
# hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: supabase-rest-hpa
  namespace: supabase
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: supabase-rest
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: supabase-realtime-hpa
  namespace: supabase
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: supabase-realtime
  minReplicas: 2
  maxReplicas: 5
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
Step 13: Advanced Real-time Examples
Real-time Collaborative Document Editor
// collaborative-editor.js
class CollaborativeEditor {
    constructor(supabase, documentId) {
        this.supabase = supabase;
        this.documentId = documentId;
        this.editor = document.getElementById('editor');
        this.cursors = new Map();
        this.init();
    }

    async init() {
        await this.loadDocument();
        this.setupRealtime();
        this.setupEventListeners();
    }

    async loadDocument() {
        const { data, error } = await this.supabase
            .from('documents')
            .select('content')
            .eq('id', this.documentId)
            .single();

        if (data) {
            this.editor.value = data.content;
        }
    }

    setupRealtime() {
        // Listen for document changes
        this.supabase
            .channel(`document:${this.documentId}`)
            .on('postgres_changes', 
                { 
                    event: 'UPDATE', 
                    schema: 'public', 
                    table: 'documents',
                    filter: `id=eq.${this.documentId}`
                }, 
                (payload) => {
                    this.handleRemoteChange(payload.new);
                }
            )
            .on('presence', { event: 'sync' }, () => {
                this.updateCursors();
            })
            .on('presence', { event: 'join' }, ({ key, newPresences }) => {
                this.showUserJoined(newPresences[0]);
            })
            .on('presence', { event: 'leave' }, ({ key, leftPresences }) => {
                this.hideUserLeft(leftPresences[0]);
            })
            .subscribe(async (status) => {
                if (status === 'SUBSCRIBED') {
                    await this.channel.track({
                        user_id: this.supabase.auth.user().id,
                        cursor_position: 0
                    });
                }
            });
    }

    async saveDocument(content) {
        const { error } = await this.supabase
            .from('documents')
            .update({ 
                content: content,
                updated_at: new Date().toISOString()
            })
            .eq('id', this.documentId);

        if (error) {
            console.error('Error saving document:', error);
        }
    }

    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    setupEventListeners() {
        // Debounced save
        const debouncedSave = this.debounce((content) => {
            this.saveDocument(content);
        }, 1000);

        this.editor.addEventListener('input', (e) => {
            debouncedSave(e.target.value);
        });

        // Cursor position tracking
        this.editor.addEventListener('selectionchange', () => {
            const position = this.editor.selectionStart;
            this.channel.track({
                user_id: this.supabase.auth.user().id,
                cursor_position: position
            });
        });
    }
}
Real-time Analytics Dashboard
// analytics-dashboard.js
class AnalyticsDashboard {
    constructor(supabase) {
        this.supabase = supabase;
        this.charts = {};
        this.init();
    }

    async init() {
        await this.loadInitialData();
        this.setupRealtime();
        this.startPeriodicUpdates();
    }

    async loadInitialData() {
        // Load page views
        const { data: pageViews } = await this.supabase
            .from('page_views')
            .select('*')
            .gte('timestamp', new Date(Date.now() - 24*60*60*1000).toISOString());

        this.updatePageViewsChart(pageViews);

        // Load user sessions
        const { data: sessions } = await this.supabase
            .from('user_sessions')
            .select('*')
            .eq('active', true);

        this.updateActiveUsersCount(sessions.length);
    }

    setupRealtime() {
        // Real-time page views
        this.supabase
            .channel('analytics')
            .on('postgres_changes', 
                { 
                    event: 'INSERT', 
                    schema: 'public', 
                    table: 'page_views' 
                }, 
                (payload) => {
                    this.addPageView(payload.new);
                }
            )
            .on('postgres_changes', 
                { 
                    event: '*', 
                    schema: 'public', 
                    table: 'user_sessions' 
                }, 
                (payload) => {
                    this.updateUserSessions(payload);
                }
            )
            .subscribe();
    }

    updatePageViewsChart(data) {
        const ctx = document.getElementById('pageViewsChart').getContext('2d');
        
        if (this.charts.pageViews) {
            this.charts.pageViews.destroy();
        }

        this.charts.pageViews = new Chart(ctx, {
            type: 'line',
            data: {
                labels: data.map(d => new Date(d.timestamp).toLocaleTimeString()),
                datasets: [{
                    label: 'Page Views',
                    data: data.map(d => d.count),
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.1
                }]
            },
            options: {
                responsive: true,
                scales: {
                    y: {
                        beginAtZero: true
                    }
                }
            }
        });
    }

    addPageView(newView) {
        if (this.charts.pageViews) {
            const chart = this.charts.pageViews;
            chart.data.labels.push(new Date(newView.timestamp).toLocaleTimeString());
            chart.data.datasets[0].data.push(newView.count);
            
            // Keep only last 100 data points
            if (chart.data.labels.length > 100) {
                chart.data.labels.shift();
                chart.data.datasets[0].data.shift();
            }
            
            chart.update('none');
        }
    }

    updateActiveUsersCount(count) {
        document.getElementById('activeUsers').textContent = count;
    }

    startPeriodicUpdates() {
        // Update metrics every 30 seconds
        setInterval(async () => {
            const { data } = await this.supabase
                .from('user_sessions')
                .select('*')
                .eq('active', true);
            
            this.updateActiveUsersCount(data.length);
        }, 30000);
    }
}
Troubleshooting Guide
Common Issues and Solutions
Kong Gateway Not Starting
# Check logs
kubectl logs -n supabase deployment/supabase-kong

# Common fix: Recreate kong config
kubectl delete configmap kong-config -n supabase
kubectl apply -f kong-deployment.yaml
2. PostgreSQL Connection Issues

# Check if PostgreSQL is running
kubectl exec -it -n supabase deployment/supabase-db -- psql -U postgres

# Check network connectivity
kubectl exec -it -n supabase deployment/supabase-rest -- nslookup supabase-db
3. Real-time Not Working

# Check realtime logs
kubectl logs -n supabase deployment/supabase-realtime

# Verify database publication
kubectl exec -it -n supabase deployment/supabase-db -- psql -U postgres -c "SELECT * FROM pg_publication;"
4. Authentication Issues

# Check GoTrue logs
kubectl logs -n supabase deployment/supabase-auth

# Verify JWT secret consistency
kubectl get configmap supabase-config -n supabase -o yaml
Performance Optimization
Database Optimization
-- Add indexes for better performance
CREATE INDEX idx_messages_created_at ON messages(created_at);
CREATE INDEX idx_messages_room_id ON messages(room_id);

-- Configure PostgreSQL settings
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET maintenance_work_mem = '64MB';
SELECT pg_reload_conf()
2. Connection Pooling

# Add PgBouncer for connection pooling
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pgbouncer
  namespace: supabase
spec:
  replicas: 1
  selector:
    matchLabels:
      app: pgbouncer
  template:
    metadata:
      labels:
        app: pgbouncer
    spec:
      containers:
      - name: pgbouncer
        image: pgbouncer/pgbouncer:latest
        ports:
        - containerPort: 5432
        env:
        - name: DATABASES_HOST
          value: "supabase-db"
        - name: DATABASES_PORT
          value: "5432"
        - name: DATABASES_USER
          value: "postgres"
        - name: DATABASES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: supabase-secrets
              key: POSTGRES_PASSWORD
        - name: POOL_MODE
          value: "transaction"
        - name: MAX_CLIENT_CONN
          value: "100"
Conclusion
You now have a complete Supabase installation running on Kubernetes with real-time capabilities. This setup provides:

Scalability: All components can be scaled independently
High Availability: Multiple replicas and proper health checks
Security: Network policies, TLS, and proper secret management
Monitoring: Health checks and metrics collection
Real-time Features: WebSocket connections for live updates
Production Ready: Backup strategies and performance optimization
The real-time examples demonstrate how to build collaborative applications, live dashboards, and interactive user experiences using Supabase’s powerful real-time capabilities.

Remember to:

Regularly backup your data
Monitor resource usage and scale accordingly
Keep your secrets secure and rotate them periodically
Update images to the latest stable versions
Test your disaster recovery procedures
This setup gives you the foundation to build modern, real-time applications with the reliability and scalability of Kubernetes.