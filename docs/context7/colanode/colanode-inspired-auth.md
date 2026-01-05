# ColaNode-Inspired Authentication Implementation

This document outlines the improved authentication implementation for DataForge, following ColaNode's account-workspace-user model and initialization patterns, adapted for offline-first local SQLite storage.

## Key Patterns from ColaNode

### 1. Account-Workspace-User Model
- **Account**: Top-level identity (email + password)
- **Workspace**: Container for projects (team/organization)
- **Workspace Member**: Per-workspace identity with role

### 2. Initialization Flow
- Server/Backend initializes first
- Session is loaded/validated before showing UI
- Window remains hidden until ready
- Frontend queries session state after backend is ready

### 3. Session Management
- Sessions persisted to disk
- Session validation on startup
- Graceful handling of invalid/expired sessions

## Implementation Strategy

Based on ColaNode's pattern and Tauri best practices, we've implemented:

1. **Backend-first initialization** - Complete all setup before showing window
2. **Session validation on startup** - Check session during backend initialization
3. **Readiness check** - Frontend polls `is_ready` before querying session
4. **Graceful error handling** - Never get stuck in loading state

## Implementation Details

### Backend Initialization (`src-tauri/src/lib.rs`)

Following ColaNode's pattern, the backend:

1. **Initializes database** synchronously in `setup()` hook
2. **Loads session** from disk during initialization
3. **Validates session** immediately (before frontend loads)
4. **Clears invalid sessions** automatically
5. **Shows window** only after initialization completes

```rust
.setup(|app| {
    // 1. Initialize database
    state.initialize(data_dir)?;
    
    // 2. Validate session during initialization (ColaNode pattern)
    if let Some(token) = &state.session.token {
        match auth::validate_session(db, token) {
            Ok(_) => info!("✅ Valid session found"),
            Err(_) => {
                // Auto-clear invalid session
                state.clear_session()?;
            }
        }
    }
    
    // 3. Show window only after everything is ready
    window.show()?;
})
```

### Readiness Check (`src-tauri/src/commands.rs`)

Added `is_ready` command following ColaNode's readiness pattern:

```rust
#[tauri::command]
pub fn is_ready(state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let state = state.lock()?;
    Ok(state.db.is_some()) // Backend is ready when DB is initialized
}
```

### Frontend Initialization (`src/lib/stores/auth.svelte.ts`)

Frontend follows ColaNode pattern:

1. **Wait for backend readiness** - Poll `is_ready` (max 5 seconds)
2. **Query session state** - Only after backend confirms readiness
3. **Handle gracefully** - Never stuck in loading state

```typescript
async function initialize(): Promise<void> {
    // 1. Wait for backend to be ready (ColaNode pattern)
    let ready = false
    while (!ready && attempts < maxAttempts) {
        ready = await invoke<boolean>('is_ready')
        if (!ready) await sleep(100)
    }
    
    // 2. Query session state (backend already validated it)
    const session = await invoke<SessionState>('get_session')
    
    // 3. Update frontend state
    if (session.authenticated) {
        // Set authenticated state
    } else {
        // Set unauthenticated state
    }
}
```

## Key Improvements

### 1. Backend Validates Session During Initialization

**Before:** Session validation happened when frontend called `get_session`
**After:** Session validated during backend `setup()`, invalid sessions cleared immediately

**Benefit:** Frontend always gets accurate session state, no race conditions

### 2. Readiness Check Pattern

**Before:** Frontend called `get_session` immediately, causing race conditions
**After:** Frontend polls `is_ready` until backend confirms readiness

**Benefit:** Eliminates race conditions, ensures backend is ready before queries

### 3. Automatic Session Cleanup

**Before:** Invalid sessions could persist
**After:** Invalid sessions automatically cleared during initialization

**Benefit:** Cleaner state, better security

### 4. Better Logging

**Before:** Minimal logging made debugging difficult
**After:** Comprehensive logging at each step

**Benefit:** Easy to debug initialization issues

## Flow Comparison

### ColaNode (Server-based)
```
Server Starts
  ↓
Load Session from DB
  ↓
Validate Session
  ↓
Client Connects
  ↓
Client Queries Session
  ↓
Server Returns Validated Session
```

### DataForge (Local-first, ColaNode-inspired)
```
Tauri App Starts
  ↓
Backend Setup Hook
  ↓
Initialize SQLite DB
  ↓
Load Session from session.json
  ↓
Validate Session (ColaNode pattern)
  ↓
Clear Invalid Sessions
  ↓
Show Window
  ↓
Frontend Loads
  ↓
Frontend Polls is_ready()
  ↓
Backend Confirms Ready
  ↓
Frontend Queries get_session()
  ↓
Backend Returns Validated Session
```

## Benefits of This Approach

1. **No Race Conditions** - Backend always ready before frontend queries
2. **Clean State** - Invalid sessions cleared automatically
3. **Better UX** - Window only shows when ready
4. **Easier Debugging** - Comprehensive logging
5. **ColaNode-Compatible** - Follows proven patterns from ColaNode

## Testing

To verify the implementation:

1. **First Launch** (no session):
   - Backend initializes → No session found → Window shows → Frontend redirects to login

2. **Valid Session**:
   - Backend initializes → Session found → Validated → Window shows → Frontend shows authenticated state

3. **Invalid Session**:
   - Backend initializes → Session found → Validation fails → Session cleared → Window shows → Frontend redirects to login

4. **Database Not Ready** (edge case):
   - Frontend polls `is_ready` → Waits up to 5 seconds → Proceeds anyway → Handles gracefully

