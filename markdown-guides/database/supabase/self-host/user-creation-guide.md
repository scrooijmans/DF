# Supabase User Creation Guide

This guide covers all the methods available for creating users in your self-hosted Supabase instance.

## 🎯 **Available Methods**

### ✅ **Method 1: Auth Admin API (Recommended for Admin Operations)**

**Best for**: Creating users programmatically as an admin without user interaction.

**Endpoint**: `POST /auth/v1/admin/users`

**Headers Required**:
- `Authorization: Bearer <SERVICE_ROLE_KEY>`
- `apikey: <SERVICE_ROLE_KEY>`
- `Content-Type: application/json`

**Example**:
```bash
curl -X POST http://91.99.166.223:8000/auth/v1/admin/users \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaXNzIjoic3VwYWJhc2UtbXVkcm9jayIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJpYXQiOjE3NTQ2NjE0MTQsImV4cCI6MjA3MDIzNzQxNH0.ipR05YNdP7Ux72VTNqrJHIBKhQW5jvmt20rYNPp_scs" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaXNzIjoic3VwYWJhc2UtbXVkcm9jayIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJpYXQiOjE3NTQ2NjE0MTQsImV4cCI6MjA3MDIzNzQxNH0.ipR05YNdP7Ux72VTNqrJHIBKhQW5jvmt20rYNPp_scs" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "securepassword123",
    "email_confirm": true,
    "user_metadata": {
      "first_name": "John",
      "last_name": "Doe"
    }
  }'
```

**Response**: Returns the created user object with ID, email, and metadata.

**Advantages**:
- ✅ No email confirmation required (can set `email_confirm: true`)
- ✅ Full control over user creation
- ✅ Can set custom metadata
- ✅ Works without user interaction

### ✅ **Method 2: Regular Signup API (For User Registration)**

**Best for**: Allowing users to register themselves through your application.

**Endpoint**: `POST /auth/v1/signup`

**Headers Required**:
- `apikey: <ANON_KEY>`
- `Content-Type: application/json`

**Example**:
```bash
curl -X POST http://91.99.166.223:8000/auth/v1/signup \
  -H "Content-Type: application/json" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlzcyI6InN1cGFiYXNlLW11ZHJvY2siLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwiaWF0IjoxNzU0NjYxNDE0LCJleHAiOjIwNzAyMzc0MTR9.pKQ270lrWeeJ_K2Vm0rUyMYMMfc8LUcmRI4igawRL2o" \
  -d '{
    "email": "user@example.com",
    "password": "userpassword123",
    "options": {
      "data": {
        "first_name": "Jane",
        "last_name": "Smith"
      }
    }
  }'
```

**Response**: Returns user object and session (if email confirmation is disabled).

**Advantages**:
- ✅ Standard user registration flow
- ✅ Returns session immediately (if no email confirmation)
- ✅ Can include user metadata

**Note**: If email confirmation is enabled, user needs to verify email before login.

### ✅ **Method 3: Edge Functions (Advanced)**

**Best for**: Complex user creation with custom business logic, role assignments, and validation.

**Implementation**: Create a Supabase Edge Function that uses the Admin API internally.

**Example Edge Function**:
```typescript
import { createClient } from 'jsr:@supabase/supabase-js'

const supabase = createClient(
  Deno.env.get('SUPABASE_URL')!,
  Deno.env.get('SUPABASE_SERVICE_ROLE_KEY')!
)

Deno.serve(async (req) => {
  const { email, password, role_id, metadata } = await req.json()
  
  // Create user
  const { data: user, error } = await supabase.auth.admin.createUser({
    email,
    password,
    email_confirm: true,
    user_metadata: metadata
  })
  
  if (error) return new Response(JSON.stringify({ error }), { status: 400 })
  
  // Assign role (if you have a roles system)
  if (role_id) {
    await supabase.from('user_roles').insert({
      user_id: user.user.id,
      role_id: role_id
    })
  }
  
  return new Response(JSON.stringify({ user }))
})
```

**Advantages**:
- ✅ Custom business logic
- ✅ Role assignments
- ✅ Validation and error handling
- ✅ Can integrate with external systems

### ❌ **Method 4: REST API (Not Available)**

**Note**: The REST API (PostgREST) cannot directly create auth users. It's designed for database operations, not authentication operations.

## 🔧 **Configuration Options**

### Email Confirmation Settings

**Disable Email Confirmation** (for testing/admin creation):
```bash
# In your supabase.env
GOTRUE_MAILER_AUTOCONFIRM=true
```

**Enable Email Confirmation** (for production):
```bash
# In your supabase.env
GOTRUE_MAILER_AUTOCONFIRM=false
```

### User Metadata

You can add custom metadata to users:

```json
{
  "email": "user@example.com",
  "password": "password123",
  "user_metadata": {
    "first_name": "John",
    "last_name": "Doe",
    "company": "Acme Corp",
    "role": "admin"
  },
  "app_metadata": {
    "provider": "email",
    "providers": ["email"]
  }
}
```

## 🚀 **Practical Examples**

### Create Admin User
```bash
curl -X POST http://91.99.166.223:8000/auth/v1/admin/users \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY" \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@mudrock.com",
    "password": "AdminPassword123!",
    "email_confirm": true,
    "user_metadata": {
      "role": "admin",
      "first_name": "Admin",
      "last_name": "User"
    }
  }'
```

### Create Regular User
```bash
curl -X POST http://91.99.166.223:8000/auth/v1/signup \
  -H "apikey: $ANON_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@mudrock.com",
    "password": "UserPassword123!",
    "options": {
      "data": {
        "first_name": "Regular",
        "last_name": "User"
      }
    }
  }'
```

### List All Users
```bash
curl -X GET "http://91.99.166.223:8000/auth/v1/admin/users?page=1&per_page=10" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY" \
  -H "apikey: $SERVICE_ROLE_KEY"
```

## 🔑 **API Keys Reference**

- **SERVICE_ROLE_KEY**: `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaXNzIjoic3VwYWJhc2UtbXVkcm9jayIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJpYXQiOjE3NTQ2NjE0MTQsImV4cCI6MjA3MDIzNzQxNH0.ipR05YNdP7Ux72VTNqrJHIBKhQW5jvmt20rYNPp_scs`
- **ANON_KEY**: `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlzcyI6InN1cGFiYXNlLW11ZHJvY2siLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwiaWF0IjoxNzU0NjYxNDE0LCJleHAiOjIwNzAyMzc0MTR9.pKQ270lrWeeJ_K2Vm0rUyMYMMfc8LUcmRI4igawRL2o`

## 📝 **Summary**

**For Admin Operations**: Use **Method 1 (Auth Admin API)** - it's the most powerful and flexible.

**For User Registration**: Use **Method 2 (Regular Signup API)** - it's the standard flow.

**For Complex Logic**: Use **Method 3 (Edge Functions)** - when you need custom business logic.

**REST API**: Cannot create users directly - it's for database operations only.

All methods are working perfectly with your self-hosted Supabase instance! 🎉
