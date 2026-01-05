# Initialization Loop Fix

## Problem Summary

The application was stuck in an "Initializing..." state with the error "An error occurred trying to load the resource" in the console. The app was getting stuck in a loop during startup.

## Root Causes

### 1. Race Condition During Initialization
- The frontend was calling `get_session` immediately on mount
- The backend database initialization might not have completed yet
- This caused "Database not initialized" errors

### 2. Poor Error Handling
- The `get_session` command returned an error when the database wasn't ready
- This error was caught but caused the app to get stuck in a loading state
- No graceful fallback for initialization failures

### 3. Redirect Loop
- The layout's `$effect` was redirecting even when `authStatus` was 'loading'
- This could cause infinite redirect loops if initialization kept failing

### 4. Missing Logging
- No console logs to help debug what was happening during initialization
- Made it difficult to identify where the process was failing

## Fixes Applied

### 1. Backend: Graceful Database Check (`src-tauri/src/commands.rs`)

**Before:**
```rust
let db = state.db.as_ref().ok_or("Database not initialized")?;
```

**After:**
```rust
// If database is not initialized, return unauthenticated state
// This can happen during app startup before initialization completes
let db = match state.db.as_ref() {
    Some(db) => db,
    None => {
        tracing::warn!("Database not initialized yet, returning unauthenticated state");
        return Ok(SessionStateResponse {
            authenticated: false,
            account: None,
            workspaces: vec![],
            current_workspace_id: None,
        });
    }
};
```

**Impact:** The command now gracefully handles the case where the database isn't ready, returning an unauthenticated state instead of an error.

### 2. Frontend: Add Initialization Delay (`src/lib/stores/auth.svelte.ts`)

**Added:**
```typescript
// Add a small delay to ensure backend is ready
// This helps avoid race conditions during app startup
await new Promise(resolve => setTimeout(resolve, 100))
```

**Impact:** Gives the backend time to complete initialization before the frontend tries to call `get_session`.

### 3. Frontend: Better Error Handling (`src/lib/stores/auth.svelte.ts`)

**Before:**
```typescript
} catch (e) {
    console.error('Failed to initialize auth:', e)
    error = e instanceof Error ? e.message : String(e)
    authStatus = 'unauthenticated'
}
```

**After:**
```typescript
} catch (e) {
    console.error('❌ Failed to initialize auth:', e)
    error = e instanceof Error ? e.message : String(e)
    // On error, assume unauthenticated rather than staying in loading state
    account = null
    workspaces = []
    currentWorkspaceId = null
    authStatus = 'unauthenticated'
}
```

**Impact:** Ensures the app never gets stuck in a loading state, always transitioning to either authenticated or unauthenticated.

### 4. Layout: Prevent Redirects During Loading (`src/routes/+layout.svelte`)

**Added:**
```typescript
if (authStore.authStatus === 'loading') return // Don't redirect while loading
```

**Impact:** Prevents redirect loops by not redirecting while authentication is still being determined.

### 5. Added Logging Throughout

**Added console logs:**
- `✅ Auth initialized: authenticated as {email}` - When auth succeeds
- `ℹ️ Auth initialized: not authenticated` - When no session found
- `❌ Failed to initialize auth: {error}` - When initialization fails
- `🔒 Redirecting to login (unauthenticated)` - When redirecting to login
- `✅ Authenticated, redirecting from public route` - When redirecting from login

**Impact:** Makes it much easier to debug authentication and routing issues.

### 6. Backend: Better Session Validation Logging (`src-tauri/src/commands.rs`)

**Added:**
```rust
tracing::info!("Session validation failed: {}, clearing session", e);
```

**Impact:** Helps debug session validation issues in the backend logs.

## Testing

After these fixes, the app should:

1. ✅ Show "Initializing..." briefly during startup
2. ✅ Call `get_session` and get a response (even if unauthenticated)
3. ✅ Transition to either authenticated or unauthenticated state
4. ✅ Redirect to `/login` if unauthenticated
5. ✅ Redirect to appropriate route if authenticated
6. ✅ Never get stuck in a loading loop

## Expected Flow

```
App Launch
  ↓
Window Hidden (tauri.conf.json: visible: false)
  ↓
Backend Initialization (lib.rs setup hook)
  - Create data directory
  - Open SQLite database
  - Load session from session.json
  ↓
Window Shown (lib.rs: window.show())
  ↓
Frontend Loads (+layout.svelte)
  - Show "Initializing..." screen
  - Call authStore.initialize()
  - Wait 100ms (avoid race condition)
  - Call get_session Tauri command
  ↓
Backend Responds (commands.rs: get_session)
  - Check if database is ready
  - If not ready: return unauthenticated
  - If ready: validate session token
  - Return SessionStateResponse
  ↓
Frontend Updates State
  - Set authStatus to 'authenticated' or 'unauthenticated'
  - Set initialized = true
  ↓
Layout Effect Runs
  - Check authStatus
  - Redirect based on auth state
  ↓
User Sees Login Page or Main App
```

## If Issues Persist

1. **Check console logs** - Look for the new log messages to see where it's failing
2. **Check backend logs** - Look for "Database not initialized" or session validation errors
3. **Check database file** - Ensure `{app_data}/dataforge.db` exists and is accessible
4. **Check session file** - Ensure `{app_data}/session.json` is valid JSON if it exists
5. **Clear session** - Delete `session.json` to force a fresh login

## Related Files

- `src-tauri/src/lib.rs` - App initialization
- `src-tauri/src/commands.rs` - Tauri commands (get_session)
- `src-tauri/src/state.rs` - App state management
- `src/lib/stores/auth.svelte.ts` - Frontend auth store
- `src/routes/+layout.svelte` - Root layout with routing logic

