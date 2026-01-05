# Tauri Performance & Authentication Best Practices for MVP

This document outlines best practices for improving application loading states and implementing modern authentication patterns in Tauri applications, specifically tailored for the MudRock MVP.

## Table of Contents

1. [Performance & Loading States](#performance--loading-states)
2. [Authentication Patterns](#authentication-patterns)
3. [Implementation Recommendations for MVP](#implementation-recommendations-for-mvp)

---

## Performance & Loading States

### 1. Deferred Window Visibility

**Best Practice**: Hide the window initially and show it only after the app is ready.

#### Implementation

```rust
// src-tauri/src/main.rs
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Hide window initially
            window.set_visible(false)?;

            // Initialize app state asynchronously
            tauri::async_runtime::spawn(async move {
                // Perform initialization tasks
                initialize_database().await;
                load_user_session().await;

                // Show window when ready
                window.set_visible(true).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Benefits

- ✅ Prevents showing blank/loading UI to users
- ✅ Better perceived performance
- ✅ Allows background initialization
- ✅ Smoother user experience

### 2. Splash Screen Pattern

**Best Practice**: Use a splash screen window that shows immediately, then transition to main window.

#### Implementation

```rust
// src-tauri/src/main.rs
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Create splash screen window
            let splash_window = tauri::WebviewWindowBuilder::new(
                app,
                "splash",
                tauri::WebviewUrl::App("splash.html".into())
            )
            .title("MudRock")
            .inner_size(400.0, 300.0)
            .decorations(false)
            .transparent(true)
            .visible(true)
            .build()?;

            // Create main window (hidden initially)
            let main_window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into())
            )
            .visible(false)
            .build()?;

            // Initialize app
            tauri::async_runtime::spawn(async move {
                // Initialize database
                initialize_database().await;

                // Load session
                load_user_session().await;

                // Close splash, show main
                splash_window.close().unwrap();
                main_window.set_visible(true).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Frontend Splash Screen

```svelte
<!-- src/splash.html or src/routes/splash/+page.svelte -->
<script>
  import { onMount } from 'svelte';

  onMount(() => {
    // Listen for ready signal from backend
    window.addEventListener('tauri://ready', () => {
      // Window will be closed by backend
    });
  });
</script>

<div class="splash">
  <div class="logo">MudRock</div>
  <div class="spinner"></div>
  <p>Initializing...</p>
</div>

<style>
  .splash {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .spinner {
    border: 3px solid rgba(255, 255, 255, 0.3);
    border-top: 3px solid white;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    animation: spin 1s linear infinite;
    margin: 20px 0;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>
```

### 3. State Initialization in `setup()` Hook

**Best Practice**: Initialize application state before showing the window.

#### Implementation

```rust
// src-tauri/src/main.rs
use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
    db: Arc<Mutex<Option<Connection>>>,
    auth: Arc<Mutex<AuthManager>>,
    initialized: Arc<tokio::sync::Notify>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_visible(false)?;

            // Create app state
            let app_state = AppState {
                db: Arc::new(Mutex::new(None)),
                auth: Arc::new(Mutex::new(AuthManager::new())),
                initialized: Arc::new(tokio::sync::Notify::new()),
            };

            app.manage(app_state.clone());

            // Initialize asynchronously
            let window_clone = window.clone();
            let initialized = app_state.initialized.clone();

            tauri::async_runtime::spawn(async move {
                // 1. Initialize database
                let db = initialize_database().await.unwrap();
                *app_state.db.lock().await = Some(db);

                // 2. Load session
                let session = load_session(&app_state).await;

                // 3. Signal ready
                initialized.notify_one();

                // 4. Show window
                window_clone.set_visible(true).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_session,
            // ... other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Command that waits for initialization
#[tauri::command]
async fn get_session(
    state: tauri::State<'_, AppState>
) -> Result<SessionState, String> {
    // Wait for initialization
    state.initialized.notified().await;

    // Now safe to access state
    let auth = state.auth.lock().await;
    auth.get_session().await
}
```

### 4. Frontend Loading States

**Best Practice**: Show loading UI while backend initializes.

#### Svelte Implementation

```svelte
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import * as authStore from '$lib/stores/auth.svelte';

  let appReady = $state(false);
  let initializationError = $state<string | null>(null);

  onMount(async () => {
    try {
      // Wait for backend initialization
      await invoke('wait_for_initialization');

      // Initialize auth state
      await authStore.initialize();

      appReady = true;

      // Handle routing based on auth state
      if (authStore.isAuthenticated) {
        goto('/projects');
      } else {
        goto('/auth/login');
      }
    } catch (error) {
      console.error('Initialization failed:', error);
      initializationError = error instanceof Error ? error.message : String(error);
    }
  });
</script>

{#if !appReady}
  <div class="loading-screen">
    {#if initializationError}
      <div class="error">
        <h2>Initialization Failed</h2>
        <p>{initializationError}</p>
        <button on:click={() => location.reload()}>Retry</button>
      </div>
    {:else}
      <div class="spinner"></div>
      <p>Loading MudRock...</p>
    {/if}
  </div>
{:else}
  <slot />
{/if}

<style>
  .loading-screen {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: var(--bg-primary);
  }

  .spinner {
    border: 3px solid var(--border);
    border-top: 3px solid var(--primary);
    border-radius: 50%;
    width: 40px;
    height: 40px;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>
```

### 5. Optimize Bundle Size

**Best Practice**: Minimize frontend bundle size for faster loading.

#### Vite Configuration

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [sveltekit()],
	build: {
		// Enable code splitting
		rollupOptions: {
			output: {
				manualChunks: {
					vendor: ['svelte', '@tauri-apps/api'],
					charts: ['scichart']
				}
			}
		},
		// Optimize chunk size
		chunkSizeWarningLimit: 1000
	},
	// Optimize dependencies
	optimizeDeps: {
		include: ['@tauri-apps/api/core']
	}
});
```

---

## Authentication Patterns

### 1. Session Management with Tauri State

**Best Practice**: Store session state in Rust backend, not just frontend.

#### Backend Implementation

```rust
// src-tauri/src/auth.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub account_id: String,
    pub email: String,
    pub name: String,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    session: Arc<RwLock<Option<Session>>>,
    db: Arc<Mutex<Connection>>,
}

impl AuthState {
    pub fn new(db: Connection) -> Self {
        Self {
            session: Arc::new(RwLock::new(None)),
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub async fn get_session(&self) -> Option<Session> {
        self.session.read().await.clone()
    }

    pub async fn set_session(&self, session: Session) {
        *self.session.write().await = Some(session);
    }

    pub async fn clear_session(&self) {
        *self.session.write().await = None;
    }

    pub async fn is_authenticated(&self) -> bool {
        let session = self.session.read().await;
        if let Some(ref s) = *session {
            // Check expiration
            chrono::Utc::now() < s.expires_at
        } else {
            false
        }
    }
}
```

#### Tauri Commands

```rust
// src-tauri/src/commands.rs
use tauri::State;

#[tauri::command]
async fn get_session(
    auth: State<'_, AuthState>
) -> Result<Option<Session>, String> {
    Ok(auth.get_session().await)
}

#[tauri::command]
async fn login(
    email: String,
    password: String,
    auth: State<'_, AuthState>
) -> Result<Session, String> {
    // Validate credentials
    let account = validate_credentials(&email, &password).await?;

    // Create session
    let session = Session {
        account_id: account.id,
        email: account.email,
        name: account.name,
        token: generate_token(),
        expires_at: chrono::Utc::now() + chrono::Duration::days(30),
    };

    // Store in state
    auth.set_session(session.clone()).await;

    // Persist to database
    persist_session(&auth.db, &session).await?;

    Ok(session)
}

#[tauri::command]
async fn logout(
    auth: State<'_, AuthState>
) -> Result<(), String> {
    auth.clear_session().await;
    Ok(())
}
```

### 2. Secure Token Storage

**Best Practice**: Store tokens securely using Tauri's secure storage or system keychain.

#### Using Tauri Store Plugin

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-store = "2.0"
```

```rust
// src-tauri/src/main.rs
use tauri_plugin_store::Builder as StoreBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(StoreBuilder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```typescript
// src/lib/auth-store.ts
import { Store } from 'tauri-plugin-store-api';

const store = new Store('.auth.dat');

export async function saveSession(session: Session) {
	await store.set('session', session);
	await store.save();
}

export async function loadSession(): Promise<Session | null> {
	return (await store.get<Session>('session')) || null;
}

export async function clearSession() {
	await store.delete('session');
	await store.save();
}
```

### 3. Cookie-Based Authentication (For Web APIs)

**Best Practice**: Use cookies for web API authentication when available.

#### Backend Cookie Management

```rust
// src-tauri/src/auth.rs
use tauri::Manager;

#[tauri::command]
async fn login_with_cookies(
    email: String,
    password: String,
    app: tauri::AppHandle,
) -> Result<Session, String> {
    // Authenticate with server
    let response = authenticate_with_server(&email, &password).await?;

    // Set cookies in webview
    if let Some(window) = app.get_webview_window("main") {
        for cookie in response.cookies {
            window.cookies().set_cookie(&cookie)?;
        }
    }

    Ok(response.session)
}
```

### 4. Deep Link Authentication (OAuth/SSO)

**Best Practice**: Use deep links for OAuth flows in desktop apps.

#### Using Better Auth Tauri Plugin

```bash
npm install @daveyplate/better-auth-tauri
```

```typescript
// src/lib/auth.ts
import { setupBetterAuthTauri } from '@daveyplate/better-auth-tauri';
import { authClient } from './auth-client';

// Initialize in app entry point
setupBetterAuthTauri({
	authClient,
	scheme: 'mudrock', // Must match tauri.conf.json
	debugLogs: process.env.NODE_ENV === 'development',
	onRequest: (href) => {
		console.log('Auth request:', href);
	},
	onSuccess: (callbackURL) => {
		console.log('Auth successful');
		// Update app state
		window.location.href = callbackURL;
	},
	onError: (error) => {
		console.error('Auth error:', error);
	}
});
```

```json
// src-tauri/tauri.conf.json
{
	"plugins": {
		"deep-link": {
			"desktop": {
				"schemes": ["mudrock"]
			}
		}
	}
}
```

### 5. Session Persistence Across Restarts

**Best Practice**: Restore session on app startup.

#### Implementation

```rust
// src-tauri/src/main.rs
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let auth = app.state::<AuthState>();

            // Restore session on startup
            tauri::async_runtime::spawn(async move {
                // Load session from secure storage
                if let Ok(Some(session)) = load_persisted_session().await {
                    // Validate session
                    if validate_session(&session).await.is_ok() {
                        auth.set_session(session).await;
                    } else {
                        // Session expired, clear it
                        clear_persisted_session().await;
                    }
                }

                // Show window after session check
                window.set_visible(true).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Implementation Recommendations for MVP

### 1. Recommended Architecture

Based on your MVP plan and best practices:

```
┌─────────────────────────────────────────────────────────┐
│  Tauri App Startup                                      │
├─────────────────────────────────────────────────────────┤
│  1. Create hidden main window                           │
│  2. Initialize SQLite database                          │
│  3. Load persisted session (if exists)                  │
│  4. Validate session token                              │
│  5. Show window when ready                              │
└─────────────────────────────────────────────────────────┘
```

### 2. Implementation Steps

#### Step 1: Update `main.rs` with Deferred Visibility

```rust
// src-tauri/src/main.rs
use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Option<rusqlite::Connection>>>,
    auth: Arc<Mutex<AuthState>>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Hide window initially
            window.set_visible(false).unwrap();

            // Initialize app state
            let app_state = AppState {
                db: Arc::new(Mutex::new(None)),
                auth: Arc::new(Mutex::new(AuthState::new())),
            };

            app.manage(app_state.clone());

            // Initialize asynchronously
            let window_clone = window.clone();
            tauri::async_runtime::spawn(async move {
                // 1. Initialize SQLite
                let db_path = app.path().app_data_dir().unwrap().join("mudrock.db");
                let conn = rusqlite::Connection::open(db_path).unwrap();
                *app_state.db.lock().await = Some(conn);

                // 2. Load session from database
                if let Some(session) = load_session_from_db(&app_state).await {
                    // Validate session
                    if validate_session(&session).await.is_ok() {
                        app_state.auth.lock().await.set_session(session).await;
                    }
                }

                // 3. Show window
                window_clone.set_visible(true).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_session,
            login,
            logout,
            register,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Step 2: Update Frontend Loading State

```svelte
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import * as authStore from '$lib/stores/auth.svelte';

  let appReady = $state(false);

  onMount(async () => {
    // Wait for backend to be ready (window will be visible when ready)
    // Then initialize auth state
    await authStore.initialize();

    appReady = true;

    // Route based on auth state
    if (authStore.isAuthenticated) {
      if (authStore.workspaceStatus === 'needs_selection') {
        goto('/workspaces/select');
      } else if (authStore.workspaceStatus === 'needs_creation') {
        goto('/workspaces/create');
      } else {
        goto('/projects');
      }
    } else {
      goto('/auth/login');
    }
  });
</script>

{#if !appReady}
  <div class="loading-screen">
    <div class="spinner"></div>
    <p>Loading MudRock...</p>
  </div>
{:else}
  <slot />
{/if}
```

#### Step 3: Implement Session Persistence

```rust
// src-tauri/src/auth.rs
use dataforge_core::auth::{validate_session, get_session};

async fn load_session_from_db(
    app_state: &AppState
) -> Option<Session> {
    let db = app_state.db.lock().await;
    let db = db.as_ref()?;

    // Load session token from database
    let token: String = db.query_row(
        "SELECT token FROM sessions WHERE account_id = (SELECT id FROM accounts LIMIT 1) ORDER BY created_at DESC LIMIT 1",
        [],
        |row| row.get(0)
    ).ok()?;

    // Validate session
    match validate_session(db, &token) {
        Ok(auth_result) => Some(Session {
            account_id: auth_result.account.id.to_string(),
            email: auth_result.account.email,
            name: auth_result.account.name,
            token,
            workspaces: auth_result.workspaces,
        }),
        Err(_) => None,
    }
}
```

### 3. Modern Practices to Implement

#### ✅ Recommended for MVP

1. **Deferred Window Visibility** - Hide window until ready
2. **Session Persistence** - Store sessions in SQLite
3. **Secure Token Storage** - Use Tauri's secure storage
4. **Loading States** - Show loading UI during initialization
5. **State Management** - Use Tauri's `manage()` for app state

#### ⚠️ Consider for Future

1. **Splash Screen** - If initialization takes > 2 seconds
2. **Deep Link Auth** - If adding OAuth/SSO later
3. **Cookie Management** - If integrating with web APIs
4. **Token Refresh** - Automatic token renewal

### 4. Performance Checklist

- [ ] Window hidden until initialization complete
- [ ] Database initialization in background
- [ ] Session loaded asynchronously
- [ ] Frontend shows loading state
- [ ] Bundle size optimized (< 2MB)
- [ ] Code splitting enabled
- [ ] Lazy loading for routes

### 5. Security Checklist

- [ ] Tokens stored securely (not in localStorage)
- [ ] Session validation on startup
- [ ] Token expiration checked
- [ ] Secure HTTP only cookies (if using)
- [ ] CSRF protection (if using cookies)
- [ ] Password hashing (argon2)

---

## Summary

### Key Takeaways

1. **Performance**: Hide window until ready, initialize state asynchronously
2. **Authentication**: Store sessions in Rust backend, persist across restarts
3. **User Experience**: Show loading states, smooth transitions
4. **Security**: Use secure storage, validate sessions, check expiration

### Next Steps

1. Implement deferred window visibility in `main.rs`
2. Add session persistence to SQLite
3. Update frontend loading states
4. Test initialization flow
5. Measure and optimize bundle size

---

## References

- [Tauri Window Configuration](https://tauri.app/v1/api/js/window)
- [Tauri State Management](https://tauri.app/v1/guides/features/state-management)
- [Better Auth Tauri Plugin](https://github.com/daveyplate/better-auth-tauri)
- [Tauri Store Plugin](https://github.com/tauri-apps/tauri-plugin-store)
