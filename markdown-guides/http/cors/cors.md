# CORS (Cross-Origin Resource Sharing) Explained

## 🏢 **What is CORS? A Hotel Analogy**

Imagine you're staying at the **"MudRock Hotel"** (your Tauri app running on `localhost:5173`), and you want to order room service from the **"Supabase Restaurant"** (your Supabase instance at `91.99.166.223:8000`).

### **Without CORS (The Problem):**

```
🏨 MudRock Hotel (localhost:5173) → 🚫 BLOCKED → 🍽️ Supabase Restaurant (91.99.166.223:8000)
```

**The Hotel Security Guard (Browser) says:**

> "Sorry, you can't order from that restaurant. It's in a different building (different origin), and we don't have permission to communicate with them."

### **With CORS (The Solution):**

```
🏨 MudRock Hotel (localhost:5173) → ✅ ALLOWED → 🍽️ Supabase Restaurant (91.99.166.223:8000)
```

**The Restaurant Manager (Supabase) says:**

> "Yes, we accept orders from MudRock Hotel! Here's our permission slip (CORS headers) that says you're welcome to dine with us."

## 🌐 **What is an "Origin"?**

An **origin** consists of three parts:

- **Protocol**: `http://` or `https://`
- **Domain**: `localhost` or `91.99.166.223`
- **Port**: `:5173` or `:8000`

### **Examples from Our Setup:**

| Request From             | Request To                  | Same Origin?               | CORS Required? |
| ------------------------ | --------------------------- | -------------------------- | -------------- |
| `http://localhost:5173`  | `http://localhost:5173`     | ✅ Yes                     | ❌ No          |
| `http://localhost:5173`  | `http://91.99.166.223:8000` | ❌ No                      | ✅ Yes         |
| `https://localhost:5173` | `http://localhost:5173`     | ❌ No (different protocol) | ✅ Yes         |

## 🚨 **Why CORS Exists: Security**

CORS is like a **bouncer at a nightclub** - it prevents unauthorized access:

### **The Security Problem CORS Solves:**

```
🌐 Malicious Website (evil.com) → 🚫 BLOCKED → 🏦 Your Bank API (bank.com)
```

**Without CORS:**

- Evil websites could make requests to your bank on your behalf
- They could steal your money, read your emails, access your data
- Your browser would blindly trust any website

**With CORS:**

- Your bank explicitly says: "Only allow requests from `bank.com` and `mobile-app`"
- Evil websites get blocked by the browser
- Your data stays safe

## 🔧 **How CORS Works: The Handshake**

### **Step 1: Preflight Request (The Permission Check)**

```
🏨 MudRock Hotel → 📋 "Can I order from Supabase Restaurant?"
🍽️ Supabase Restaurant → ✅ "Yes, here's what you can order and how"
```

**Technical Details:**

```http
OPTIONS /auth/v1/user HTTP/1.1
Host: 91.99.166.223:8000
Origin: http://localhost:5173
Access-Control-Request-Method: GET
Access-Control-Request-Headers: authorization, apikey
```

**Response:**

```http
HTTP/1.1 200 OK
Access-Control-Allow-Origin: http://localhost:5173
Access-Control-Allow-Methods: GET, POST, PUT, DELETE
Access-Control-Allow-Headers: authorization, apikey, content-type
```

### **Step 2: Actual Request (The Order)**

```
🏨 MudRock Hotel → 🍽️ "I'd like to get my user info, please"
🍽️ Supabase Restaurant → 📦 "Here's your user data"
```

## 🐛 **Our CORS Problem: The Broken Permission Slip**

### **What Was Happening:**

```
🏨 MudRock Hotel (localhost:5173) → 🚫 BLOCKED → 🍽️ Supabase Restaurant (91.99.166.223:8000)
```

**Error Message:**

```
Origin http://localhost:5173 is not allowed by Access-Control-Allow-Origin. Status code: 204
```

**Translation:**

> "The Supabase Restaurant doesn't have a permission slip that allows MudRock Hotel to order from them."

### **Why It Was Broken:**

#### **1. Wrong Supabase Client**

```typescript
// ❌ BROKEN: Custom client without proper CORS handling
function getSupabaseClient() {
  const supabaseUrl = "http://91.99.166.223:8000";
  const supabaseAnonKey = "your-anon-key";
  return createClient(supabaseUrl, supabaseAnonKey); // Basic client
}
```

**Problem:** This client doesn't know how to handle CORS properly in a Tauri environment.

#### **2. Missing Tauri HTTP Plugin**

```typescript
// ❌ BROKEN: Using browser fetch in Tauri
fetch("http://91.99.166.223:8000/auth/v1/user"); // Browser fetch = CORS issues
```

**Problem:** Tauri apps need special HTTP handling, not browser fetch.

## ✅ **Our CORS Solution: The Fixed Permission System**

### **What We Fixed:**

#### **1. Used Shared Supabase Client**

```typescript
// ✅ FIXED: Using shared client with Tauri HTTP plugin
import { supabase } from "$lib/supabase"; // Shared client with custom fetch
```

**From `src/lib/supabase.ts`:**

```typescript
// Custom fetch function that uses Tauri HTTP plugin
const createFetchFunction = () => {
  return async (input: RequestInfo | URL, init?: RequestInit) => {
    try {
      console.log("🔧 Using Tauri HTTP plugin for request to:", input);
      const { fetch: tauriFetch } = await import("@tauri-apps/plugin-http");
      return await tauriFetch(input, init); // ✅ Tauri HTTP plugin
    } catch (error) {
      console.error("❌ HTTP request failed:", error);
      throw error;
    }
  };
};

// Create Supabase client with custom fetch function
export const supabase = createClient(supabaseUrl, supabaseAnonKey, {
  global: {
    fetch: customFetch, // ✅ Uses Tauri HTTP plugin
  },
});
```

#### **2. Tauri HTTP Plugin Bypasses CORS**

```
🏨 MudRock Hotel → 🚀 Tauri HTTP Plugin → 🍽️ Supabase Restaurant
```

**Why This Works:**

- **Tauri HTTP Plugin** = **Hotel's Private Delivery Service**
- It doesn't go through the browser's security checks
- It's like having a direct phone line to the restaurant
- No need for CORS permission slips!

## 🏗️ **Our Implementation: Create Notebook & Create Project**

### **Create Notebook Flow:**

```typescript
// src/lib/services/notebook-creation-service.ts
export async function getUserProjects(): Promise<Project[]> {
  // ✅ Uses shared supabase client with Tauri HTTP plugin
  const {
    data: { user },
    error: userError,
  } = await supabase.auth.getUser();

  // ✅ This request now works because of Tauri HTTP plugin
  const { data: ownedProjects, error: ownedError } = await supabase
    .from("projects")
    .select("id, name, owner_id")
    .eq("owner_id", user.id);
}
```

### **Create Project Flow:**

```typescript
// src/lib/services/project-creation-service.ts
export async function createProjectWithStorageJS(projectData: ProjectFormData) {
  // ✅ Uses same shared supabase client
  const { data: project, error } = await supabase
    .from("projects")
    .insert(projectData)
    .select()
    .single();
}
```

### **Both Use the Same Pattern:**

```
Tauri App → Shared Supabase Client → Tauri HTTP Plugin → Supabase Server
```

## 🌐 **Our Supabase Configuration: The Restaurant Setup**

### **From HETZNER_DOKPLOY_VPS_GUIDE.md:**

#### **Kong API Gateway (The Maître D')**

```
Client Apps → Kong Gateway (Port 8000) → All Supabase Services
```

**Kong Configuration:**

```yaml
# kong.yml - The restaurant's routing system
services:
  - name: auth-v1
    url: http://mudrock-auth:9999
    routes:
      - name: auth-v1-route
        paths:
          - /auth/v1
        strip_path: false
    plugins:
      - name: cors # ✅ CORS plugin enabled
```

#### **CORS Plugin Configuration:**

```yaml
# Kong CORS plugin settings
plugins:
  - name: cors
    config:
      origins:
        - "http://localhost:5173" # ✅ Allow MudRock Hotel
        - "http://localhost:3000" # ✅ Allow Dokploy
      methods:
        - GET
        - POST
        - PUT
        - DELETE
        - OPTIONS
      headers:
        - authorization
        - apikey
        - content-type
```

## 🔄 **The Complete Flow: How It All Works Together**

### **1. Tauri App Starts**

```
🏨 MudRock Hotel opens for business (localhost:5173)
```

### **2. User Creates Notebook**

```
👤 User clicks "New Notebook" → 🏨 MudRock Hotel → 📋 "Get available projects"
```

### **3. Authentication Request**

```
🏨 MudRock Hotel → 🚀 Tauri HTTP Plugin → 🍽️ Kong Gateway → 🔐 Auth Service
```

**Request:**

```http
GET /auth/v1/user HTTP/1.1
Host: 91.99.166.223:8000
Authorization: Bearer <jwt-token>
apikey: <anon-key>
```

**Response:**

```http
HTTP/1.1 200 OK
Access-Control-Allow-Origin: http://localhost:5173
Content-Type: application/json

{
  "id": "user-uuid",
  "email": "user@example.com"
}
```

### **4. Projects Request**

```
🏨 MudRock Hotel → 🚀 Tauri HTTP Plugin → 🍽️ Kong Gateway → 🗄️ PostgREST API
```

**Request:**

```http
GET /rest/v1/projects?owner_id=eq.user-uuid HTTP/1.1
Host: 91.99.166.223:8000
Authorization: Bearer <jwt-token>
apikey: <anon-key>
```

**Response:**

```http
HTTP/1.1 200 OK
Access-Control-Allow-Origin: http://localhost:5173
Content-Type: application/json

[
  {
    "id": "project-uuid",
    "name": "my-project",
    "owner_id": "user-uuid"
  }
]
```

### **5. Notebook Creation**

```
🏨 MudRock Hotel → 🚀 Tauri HTTP Plugin → 🍽️ Kong Gateway → 🗄️ PostgREST API
```

**Request:**

```http
POST /rest/v1/notebooks HTTP/1.1
Host: 91.99.166.223:8000
Authorization: Bearer <jwt-token>
apikey: <anon-key>
Content-Type: application/json

{
  "project_id": "project-uuid",
  "name": "my-notebook",
  "title": "My Analysis Notebook"
}
```

## 🎯 **Why It Works Now: The Complete Picture**

### **Before (Broken):**

```
Tauri App → Browser Fetch → ❌ CORS Error → Supabase Server
```

### **After (Working):**

```
Tauri App → Shared Supabase Client → Tauri HTTP Plugin → Kong Gateway → Supabase Services
```

### **Key Components:**

1. **🏨 Tauri App**: Your MudRock application
2. **🚀 Tauri HTTP Plugin**: Bypasses browser CORS restrictions
3. **🍽️ Kong Gateway**: Routes requests and handles CORS headers
4. **🔐 Auth Service**: Handles authentication
5. **🗄️ PostgREST API**: Handles database operations

## 📚 **Summary: CORS in Simple Terms**

**CORS is like a VIP list at an exclusive restaurant:**

- **Without CORS**: Anyone can walk in and order
- **With CORS**: Only people on the VIP list can dine
- **Our Problem**: We weren't on the VIP list
- **Our Solution**: We got a VIP pass (Tauri HTTP Plugin) that lets us bypass the list

**The Restaurant (Supabase) says:**

> "We have a VIP list, but if you have our special delivery service (Tauri HTTP Plugin), you can order directly from the kitchen!"

**That's why our notebook creation works now!** 🎉

---

## 🔧 **Technical Implementation Details**

### **Environment Variables:**

```bash
# From HETZNER_DOKPLOY_VPS_GUIDE.md
VITE_SUPABASE_URL=http://91.99.166.223:8000
VITE_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### **Tauri Configuration:**

```toml
# src-tauri/tauri.conf.json
{
  "plugins": {
    "http": {
      "all": true,
      "request": true
    }
  }
}
```

### **CORS Headers in Kong:**

```yaml
Access-Control-Allow-Origin: http://localhost:5173
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: authorization, apikey, content-type
Access-Control-Max-Age: 86400
```

This setup ensures that your Tauri app can communicate seamlessly with your self-hosted Supabase instance without CORS issues! 🚀
