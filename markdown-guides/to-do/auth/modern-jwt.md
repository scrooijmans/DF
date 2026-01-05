# Migration to Modern JWT Signing Keys

## Current Implementation (Legacy Symmetric JWT)

### What We're Using Now
- **ANON_KEY**: JWT-based key for client authentication
- **SERVICE_ROLE_KEY**: JWT-based key for server operations
- **JWT_SECRET**: Symmetric key for signing all JWTs
- **Session Management**: Supabase client handles everything automatically

### Current Flow
```
User Login → Supabase Server → JWT signed with JWT_SECRET → Client stores JWT → 
Every API call → Network call to verify JWT → Access granted/denied
```

## Required Changes for Modern JWT

### 1. Update Supabase Configuration

#### Current (Legacy):
```javascript
// src/lib/supabase.ts
const supabaseAnonKey = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'; // JWT-based ANON_KEY
```

#### New (Modern):
```javascript
// src/lib/supabase.ts
const supabasePublishableKey = 'pk_...'; // Non-JWT publishable key
const supabaseSecretKey = 'sk_...'; // Non-JWT secret key
```

### 2. Update Environment Variables

#### Current:
```bash
PUBLIC_SUPABASE_URL=http://91.99.166.223:3001
PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
SUPABASE_SERVICE_ROLE_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
JWT_SECRET=your-symmetric-secret
```

#### New:
```bash
PUBLIC_SUPABASE_URL=http://91.99.166.223:3001
PUBLIC_SUPABASE_PUBLISHABLE_KEY=pk_...
SUPABASE_SECRET_KEY=sk_...
# JWT_SECRET no longer needed - Supabase handles asymmetric keys
```

### 3. Update Supabase Client Creation

#### Current:
```javascript
// src/lib/supabase.ts
export const supabase = createClient(supabaseUrl, supabaseAnonKey, {
  auth: {
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: false,
    flowType: "pkce",
  }
});
```

#### New:
```javascript
// src/lib/supabase.ts
export const supabase = createClient(supabaseUrl, supabasePublishableKey, {
  auth: {
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: false,
    flowType: "pkce",
    // New: Enable asymmetric JWT verification
    verifyJWT: true,
  }
});
```

### 4. Update Server-Side Configuration

#### Current:
```javascript
// src/hooks.server.ts
const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY, {
  auth: {
    flowType: 'pkce',
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: true
  }
});
```

#### New:
```javascript
// src/hooks.server.ts
const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_PUBLISHABLE_KEY, {
  auth: {
    flowType: 'pkce',
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: true,
    // New: Enable local JWT verification
    verifyJWT: true,
  }
});
```

### 5. Update Authentication Store

#### Current:
```javascript
// src/lib/stores/auth.svelte.js
export const supabase = createClient(supabaseUrl, supabaseAnonKey, {
  auth: {
    flowType: 'pkce',
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: true
  }
});
```

#### New:
```javascript
// src/lib/stores/auth.svelte.js
export const supabase = createClient(supabaseUrl, supabasePublishableKey, {
  auth: {
    flowType: 'pkce',
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: true,
    // New: Enable local JWT verification
    verifyJWT: true,
  }
});
```

### 6. Add Local JWT Verification (Optional Enhancement)

#### New Function:
```javascript
// src/lib/jwt-verification.ts
import { createRemoteJWKSet, jwtVerify } from 'jose';

const SUPABASE_JWT_ISSUER = 'http://91.99.166.223:3001/auth/v1';
const SUPABASE_JWT_KEYS = createRemoteJWKSet(
  new URL(SUPABASE_JWT_ISSUER + '/.well-known/jwks.json')
);

export async function verifySupabaseJWT(jwt: string) {
  try {
    const { payload } = await jwtVerify(jwt, SUPABASE_JWT_KEYS, { 
      issuer: SUPABASE_JWT_ISSUER 
    });
    return { success: true, payload };
  } catch (error) {
    return { success: false, error: error.message };
  }
}

// Use in auth store for local verification
export async function isUserAuthenticated(): Promise<boolean> {
  const token = localStorage.getItem('supabase.auth.token');
  if (!token) return false;
  
  const result = await verifySupabaseJWT(token);
  return result.success;
}
```

### 7. Update API Calls

#### Current (Network-dependent):
```javascript
// Every auth check makes a network call
const { data: { user } } = await supabase.auth.getUser();
```

#### New (Local verification):
```javascript
// Can verify JWT locally without network calls
const isAuth = await isUserAuthenticated();
```

### 8. Update Tauri Backend (if needed)

#### Current:
```rust
// src-tauri/src/auth.rs
const SUPABASE_URL: &str = "http://91.99.166.223:3001";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
```

#### New:
```rust
// src-tauri/src/auth.rs
const SUPABASE_URL: &str = "http://91.99.166.223:3001";
const SUPABASE_PUBLISHABLE_KEY: &str = "pk_...";
const SUPABASE_SECRET_KEY: &str = "sk_...";
```

## Migration Steps

### Phase 1: Prepare Supabase Instance
1. **Migrate to JWT Signing Keys** in Supabase dashboard
2. **Generate new key pairs** (public/private)
3. **Get new API keys** (publishable/secret)

### Phase 2: Update Client Code
1. **Update environment variables**
2. **Replace ANON_KEY with PUBLISHABLE_KEY**
3. **Replace SERVICE_ROLE_KEY with SECRET_KEY**
4. **Update all Supabase client configurations**

### Phase 3: Test and Deploy
1. **Test authentication flow**
2. **Verify JWT verification works**
3. **Deploy to production**

### Phase 4: Cleanup (Optional)
1. **Remove old JWT_SECRET** from environment
2. **Add local JWT verification** for better performance
3. **Update documentation**

## Benefits of Modern JWT

### Performance Improvements
- ✅ **No network calls** for auth checks
- ✅ **Faster authentication** verification
- ✅ **Better offline support**

### Security Improvements
- ✅ **Asymmetric key cryptography**
- ✅ **Public key verification** (safer)
- ✅ **Better key rotation** support

### Developer Experience
- ✅ **Local JWT verification** possible
- ✅ **Better debugging** capabilities
- ✅ **More flexible** authentication

## Breaking Changes

### What Will Break
- ❌ **Old ANON_KEY** will stop working
- ❌ **Old SERVICE_ROLE_KEY** will stop working
- ❌ **JWT_SECRET** will be ignored

### What Stays the Same
- ✅ **User login/logout flow**
- ✅ **Svelte store management**
- ✅ **Route protection logic**
- ✅ **API call patterns**

## Rollback Plan

If issues occur:
1. **Revert environment variables** to old keys
2. **Revert code changes** to use ANON_KEY
3. **Disable JWT signing keys** in Supabase dashboard
4. **Restart services**

## Timeline

- **Week 1**: Update Supabase instance and get new keys
- **Week 2**: Update client code and test
- **Week 3**: Deploy and monitor
- **Week 4**: Add local JWT verification (optional)

## Success Metrics

- **Performance**: Auth checks < 10ms (vs current ~100ms)
- **Reliability**: 99.9% uptime for auth operations
- **Security**: Asymmetric key verification working
- **User Experience**: No visible changes to users
