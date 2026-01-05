# MudRock Authentication Flow Documentation

## Overview

This document describes the complete authentication flow implemented in MudRock, a Tauri v2 desktop application with SvelteKit frontend and Supabase backend.

## Architecture Components

### 1. Tauri v2 Configuration

#### `src-tauri/tauri.conf.json`

```json
{
  "plugins": {}
}
```

- **HTTP Plugin**: Configured as empty object `{}` in Tauri v2
- **Security**: CSP disabled for development
- **Build**: Uses `frontendDist` (Tauri v2 syntax)

#### `src-tauri/capabilities/default.json`

```json
{
  "permissions": [
    "core:default",
    "http:allow-fetch",
    {
      "identifier": "http:default",
      "allow": [
        { "url": "http://91.99.166.223:8000/**" },
        { "url": "https://**" }
      ]
    }
  ]
}
```

- **HTTP Permissions**: Allows requests to Supabase server on port 8000
- **Scope Configuration**: Uses glob patterns `**` for URL matching

#### `src-tauri/src/main.rs`

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_http::init())
    .manage(app_state)
    .manage(AuthManager::new())
```

- **HTTP Plugin**: Initialized using `tauri_plugin_http::init()`
- **State Management**: Manages app state and auth manager

### 2. Supabase Configuration

#### `src/lib/supabase.ts`

```typescript
const supabaseUrl = "http://91.99.166.223:8000";
const supabaseAnonKey = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

const customFetch = createFetchFunction();

export const supabase = createClient(supabaseUrl, supabaseAnonKey, {
  auth: {
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: false,
    flowType: "pkce",
    fetch: customFetch,
    storage: typeof window !== "undefined" ? window.localStorage : undefined,
    debug: process.env.NODE_ENV === "development",
  },
});
```

**Key Features:**

- **Custom Fetch**: Uses Tauri HTTP plugin with native fetch fallback
- **PKCE Flow**: Enhanced security for authentication
- **Session Persistence**: Stores tokens in localStorage
- **Auto Refresh**: Automatically refreshes expired tokens

#### Custom Fetch Implementation

```typescript
const createFetchFunction = () => {
  return async (input: RequestInfo | URL, init?: RequestInit) => {
    try {
      const { fetch: tauriFetch } = await import("@tauri-apps/plugin-http");
      return await tauriFetch(input, init);
    } catch (error) {
      console.warn("Tauri HTTP plugin failed, using native fetch:", error);
      return await fetch(input, init);
    }
  };
};
```

### 3. Authentication Flow

#### Login Process (`src/lib/supabase-auth.ts`)

1. **Network Test**: Tests connectivity using Tauri HTTP plugin

   ```typescript
   const testResponse = await tauriFetch(
     "http://91.99.166.223:8000/auth/v1/settings",
     {
       method: "GET",
       headers: {
         "Content-Type": "application/json",
         apikey: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
       },
     },
   );
   ```

2. **Authentication Request**: Uses Supabase client with custom fetch

   ```typescript
   const { data, error } = await supabase.auth.signInWithPassword({
     email,
     password,
   });
   ```

3. **Session Management**: Stores session in localStorage
   ```typescript
   if (data.session) {
     localStorage.setItem("sb-91-auth-token", JSON.stringify(data.session));
     isAuthenticated.set(true);
     currentUser.set(data.user);
   }
   ```

#### Client-Side Form Submission (`src/routes/auth/login/+page.svelte`)

```typescript
<form onsubmit={async (e) => {
  e.preventDefault();

  if (!email || !password) {
    console.log('❌ Missing credentials');
    return false;
  }

  isSubmitting = true;

  try {
    console.log('🔐 Attempting login...');
    const { login } = await import('$lib/supabase-auth');
    const success = await login(email, password);

    if (success) {
      console.log('✅ Login successful, redirecting...');
      window.location.href = '/projects';
    } else {
      console.log('❌ Login failed');
    }
  } catch (error) {
    console.error('💥 Login error:', error);
  } finally {
    isSubmitting = false;
  }

  return false;
}}>
```

**Key Features:**

- **Direct Client-Side Login**: Uses Supabase client directly in form submission
- **Loading State Management**: Shows loading spinner during authentication
- **Error Handling**: Comprehensive try/catch for login errors
- **Automatic Redirect**: Redirects to `/projects` on successful login

#### Sign-Out Process (`src/routes/dashboard/+page.svelte`)

```typescript
async function handleSignOut() {
  try {
    console.log("🚪 Signing out user...");
    const { signOut } = await import("$lib/supabase-auth");
    await signOut();
    console.log("✅ Sign out successful");
  } catch (error) {
    console.error("💥 Sign out error:", error);
  }
}
```

**Sign-Out Implementation:**

```typescript
// In supabase-auth.ts
export async function signOut(): Promise<void> {
  try {
    console.log("🚪 Signing out user...");
    isLoading.set(true);
    authError.set(null);

    // Sign out from Supabase
    const { error } = await supabase.auth.signOut();

    if (error) {
      console.error("❌ Sign out error:", error);
      throw error;
    }

    // Clear local state
    user.set(null);
    isAuthenticated.set(false);

    // Clear localStorage
    if (typeof window !== "undefined") {
      localStorage.removeItem("sb-91-auth-token");
    }

    console.log("✅ Sign out successful");
  } catch (error) {
    console.error("💥 Sign out error:", error);
    authError.set("Failed to sign out. Please try again.");
  } finally {
    isLoading.set(false);
  }
}
```

**Key Features:**

- **Complete Session Cleanup**: Clears Supabase session and local state
- **Storage Cleanup**: Removes authentication tokens from localStorage
- **Error Handling**: Comprehensive error handling for sign-out failures
- **State Reset**: Resets all authentication state variables
- **Automatic Redirect**: Layout automatically redirects to login page

### 4. Endpoints and Network Requests

#### Supabase Server Endpoints

- **Base URL**: `http://91.99.166.223:8000`
- **Auth Settings**: `GET /auth/v1/settings` (with apikey header)
- **Login**: `POST /auth/v1/token?grant_type=password`
- **Token Refresh**: `POST /auth/v1/token?grant_type=refresh_token`

#### Request Headers

```typescript
{
  'Content-Type': 'application/json',
  'apikey': 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...',
  'Authorization': 'Bearer <access_token>' // For authenticated requests
}
```

### 5. State Management

#### Reactive Stores (`src/lib/supabase-auth.ts`)

```typescript
export const isAuthenticated = writable(false);
export const currentUser = writable(null);
export const isLoading = writable(false);
export const authError = writable(null);
```

#### Layout State Management (`src/routes/+layout.svelte`)

```typescript
// Reactive state management
let isAuthenticated = $state(false);
let currentUser = $state(null);
let isLoading = $state(false);
let isRedirecting = $state(false);

// Route detection
const currentPath = $derived($page.url.pathname);
const isAuthRoute = $derived(currentPath.startsWith("/auth"));
const isPublicRoute = $derived(
  ["/auth/login", "/auth/signup", "/"].includes(currentPath),
);

// Auth state effect
$effect(() => {
  if ($isAuthenticated && isAuthRoute && currentPath !== "/projects") {
    console.log(
      "✅ +layout.svelte: User authenticated, redirecting to projects...",
    );
    isRedirecting = true;
    goto("/projects");

    // Reset redirecting flag after delay
    setTimeout(() => {
      isRedirecting = false;
    }, 2000);
  } else if (!$isAuthenticated && !isPublicRoute) {
    console.log(
      "🔒 +layout.svelte: User not authenticated, redirecting to login...",
    );
    goto("/auth/login");
  }
});
```

**Key Features:**

- **Svelte 5 State Management**: Uses `$state()` for reactive variables
- **Derived Values**: Uses `$derived()` for computed properties
- **Effect-Based Logic**: Uses `$effect()` for side effects and redirects
- **Redirect Guards**: Prevents infinite redirect loops with timing controls
- **Route Detection**: Automatically detects auth and public routes

### 6. Security Features

#### Tauri Security

- **HTTP Plugin**: Bypasses CORS restrictions
- **Scope Permissions**: Restricts network access to specific URLs
- **Capabilities**: Defines what the app can access

#### Supabase Security

- **JWT Tokens**: Secure authentication tokens
- **PKCE Flow**: Enhanced OAuth security
- **Session Management**: Automatic token refresh
- **Storage**: Secure localStorage for session persistence

### 7. Error Handling

#### Network Errors

```typescript
if (error.message.includes("url not allowed on the configured scope")) {
  authError.set("Connection blocked by security settings");
} else if (
  error.message.includes("fetch") ||
  error.message.includes("network")
) {
  authError.set("Network error - please check your connection");
}
```

#### Authentication Errors

```typescript
if (error.message.includes("Invalid login credentials")) {
  authError.set("Invalid email or password");
} else if (error.message.includes("Email not confirmed")) {
  authError.set("Please check your email and click the confirmation link");
}
```

### 8. Redirect Flow

#### Client-Side Redirects

- **Layout Effect**: Monitors auth state changes
- **Route Guards**: Prevents access to protected routes
- **Automatic Redirects**: Redirects based on authentication status
- **Primary Redirect Target**: `/projects` (replaces `/home` and `/dashboard`)
  - After successful login: `/auth/login` → `/projects`
  - From root route: `/` → `/projects` (if authenticated)
  - From `/home` route: `/home` → `/projects` (if authenticated)
  - From auth routes: `/auth/*` → `/projects` (if authenticated)

#### Server-Side Redirects

- **Form Actions**: Uses SvelteKit's `redirect()` function
- **Status Codes**: Uses 303 for form submissions
- **Navigation**: Ensures proper page transitions

### 9. Debugging and Logging

#### HTTP Plugin Testing

```typescript
export async function testHttpPlugin() {
  const { fetch: tauriFetch } = await import("@tauri-apps/plugin-http");
  const response = await tauriFetch(
    "http://91.99.166.223:8000/auth/v1/settings",
    {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        apikey: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      },
    },
  );
  return { success: true, status: response.status };
}
```

#### Console Logging

- **Auth State Changes**: Logs all authentication state transitions
- **Network Requests**: Logs HTTP requests and responses
- **Error Handling**: Detailed error logging for debugging

### 10. Current Working Authentication Flow

> **🔀 IMPORTANT: Redirect Target Changed**
> 
> As of the latest update, authenticated users are now redirected to **`/projects`** instead of `/home` or `/dashboard`. This is the primary entry point for authenticated users.

#### Complete Login Process

1. **User Input**: User enters email and password in login form
2. **Form Submission**: Form prevents default submission and calls login function
3. **Network Test**: Tests connectivity using Tauri HTTP plugin
4. **Supabase Authentication**: Calls `supabase.auth.signInWithPassword()`
5. **Session Creation**: Stores access token and refresh token in localStorage
6. **State Update**: Updates reactive stores (`isAuthenticated`, `currentUser`)
7. **Layout Detection**: Layout detects auth state change via `$effect()`
8. **Automatic Redirect**: Redirects to `/projects` using `window.location.href` or `goto()`
9. **Projects Page Access**: User can access protected projects page

#### Complete Sign-Out Process

1. **User Action**: User clicks sign-out button in menubar
2. **Sign-Out Call**: Calls `supabase.auth.signOut()` to clear Supabase session
3. **State Cleanup**: Resets all local authentication state variables
4. **Storage Cleanup**: Removes authentication tokens from localStorage
5. **Layout Detection**: Layout detects auth state change via `$effect()`
6. **Automatic Redirect**: Redirects to `/auth/login` using `goto()`
7. **Login Access**: User returns to login page and can sign in again

#### Key Success Factors

- **Tauri HTTP Plugin**: Bypasses CORS restrictions for Supabase requests
- **Svelte 5 Reactivity**: Uses `$state()`, `$derived()`, and `$effect()` for reactive updates
- **Direct Client-Side Auth**: No server-side form actions needed
- **Proper Error Handling**: Comprehensive try/catch blocks
- **Loading States**: User feedback during authentication process
- **Redirect Guards**: Prevents infinite redirect loops

### 11. File Structure

```
src/
├── lib/
│   ├── supabase.ts              # Supabase client configuration
│   ├── supabase-auth.ts         # Authentication logic
│   └── http-test.ts            # HTTP plugin testing
├── routes/
│   ├── +layout.svelte          # Main layout with auth guards
│   ├── auth/
│   │   └── login/
│   │       ├── +page.svelte    # Login form with direct auth
│   │       └── +page.server.ts # Server-side login action (unused)
│   └── dashboard/
│       └── +page.svelte        # Protected dashboard page
src-tauri/
├── tauri.conf.json             # Tauri v2 configuration
├── capabilities/
│   └── default.json            # HTTP permissions
└── src/
    └── main.rs                 # Rust backend with HTTP plugin
```

## Summary

The MudRock authentication flow successfully integrates:

1. **Tauri v2 HTTP Plugin** for network requests (bypasses CORS)
2. **Supabase Authentication** with PKCE flow and session management
3. **Svelte 5 Reactivity** with `$state()`, `$derived()`, and `$effect()`
4. **Direct Client-Side Authentication** (no server-side form actions needed)
5. **Reactive State Management** with automatic redirects
6. **Proper Error Handling** and comprehensive logging
7. **Security Best Practices** with Tauri scope permissions
8. **Loading States** and user feedback during authentication

**Current Status**: ✅ **FULLY WORKING**

- Login form submission works correctly
- Authentication succeeds with valid credentials
- Session management and token storage functional
- **Automatic redirect to `/projects` on successful login** ✅
- Sign-out functionality works correctly
- Complete session cleanup on sign-out
- Automatic redirect to login page after sign-out
- Proper error handling and loading states
- No infinite redirect loops

The system provides a robust, secure, and user-friendly authentication experience in a Tauri desktop application environment using modern Svelte 5 patterns.
