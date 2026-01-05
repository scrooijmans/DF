# Svelte 5 Context API for Component Communication

This document explains how to use the Context API in Svelte 5 to communicate properties and data between components without prop drilling.

## Overview

The Context API allows components to share data across the component tree without passing props through every intermediate component. This is particularly useful for:
- Avoiding prop drilling
- Sharing theme/configuration data
- Providing API functions to child components
- Managing authentication state
- Sharing reactive state across component hierarchies

## Basic Context API

### setContext and getContext

The fundamental pattern uses `setContext` in a parent component and `getContext` in child components.

#### Parent Component (Setting Context)

```svelte
<!-- file: Parent.svelte -->
<script>
  import { setContext } from 'svelte';
  import Child from './Child.svelte';

  // Set context with a key and value
  setContext('my-context', 'hello from Parent.svelte');
</script>

<Child />
```

#### Child Component (Getting Context)

```svelte
<!-- file: Child.svelte -->
<script>
  import { getContext } from 'svelte';

  // Retrieve context using the same key
  const message = getContext('my-context');
</script>

<h1>{message}, inside Child.svelte</h1>
```

### Multiple Contexts

You can set multiple contexts with different keys:

```svelte
<!-- file: Parent.svelte -->
<script>
  import { setContext } from 'svelte';
  import Child from './Child.svelte';

  const theme = {
    primaryColor: '#ff3e00',
    secondaryColor: '#676778'
  };

  const api = {
    fetchUser: (id) => fetch(`/api/users/${id}`).then(r => r.json()),
    updateUser: (id, data) => fetch(`/api/users/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data)
    })
  };

  // Set multiple contexts with different keys
  setContext('theme', theme);
  setContext('api', api);
</script>

<Child />
```

```svelte
<!-- file: Child.svelte -->
<script>
  import { getContext } from 'svelte';

  const theme = getContext('theme');
  const api = getContext('api');

  async function loadUser() {
    const user = await api.fetchUser(123);
    console.log('User:', user);
  }
</script>

<div style="color: {theme.primaryColor}">
  Themed content
</div>

<button onclick={loadUser}>Load User</button>
```

## Using Context with Reactive State

### Storing Reactive State in Context

You can store reactive state (using `$state` in Svelte 5) in context:

```svelte
<!-- file: Parent.svelte -->
<script>
  import { setContext } from 'svelte';
  import Child from './Child.svelte';

  // Create reactive state
  let counter = $state({
    count: 0
  });

  // Set context with reactive state
  setContext('counter', counter);
</script>

<button onclick={() => counter.count += 1}>
  Increment
</button>

<Child />
<Child />
<Child />
```

```svelte
<!-- file: Child.svelte -->
<script>
  import { getContext } from 'svelte';

  // Get reactive context
  const counter = getContext('counter');
</script>

<div>Count: {counter.count}</div>
<button onclick={() => counter.count += 1}>Increment from Child</button>
```

### Important: Don't Reassign Context Values

When using reactive state in context, **don't reassign the entire object**. Instead, update its properties:

```svelte
<!-- ❌ BAD: This breaks the context link -->
<script>
  let counter = $state({ count: 0 });
  setContext('counter', counter);
</script>

<button onclick={() => counter = { count: 0 }}>
  Reset (breaks context!)
</button>
```

```svelte
<!-- ✅ GOOD: Update properties instead -->
<script>
  let counter = $state({ count: 0 });
  setContext('counter', counter);
</script>

<button onclick={() => counter.count = 0}>
  Reset (maintains context)
</button>
```

Svelte will warn you if you try to reassign a context value that's being used reactively.

## Type-Safe Context with createContext

For better type safety and cleaner code, use `createContext` which returns a getter/setter pair:

### Creating Type-Safe Context

```typescript
/// file: context.ts
import { createContext } from 'svelte';

interface Theme {
  mode: 'light' | 'dark';
  primaryColor: string;
  secondaryColor: string;
}

// Create type-safe context getter/setter pair
export const [getTheme, setTheme] = createContext<Theme>();
```

### Using Type-Safe Context in Parent

```svelte
<!-- file: Parent.svelte -->
<script>
  import { setTheme } from './context.js';
  import Child from './Child.svelte';

  // Create reactive theme state
  const theme = $state({
    mode: 'dark',
    primaryColor: '#ff3e00',
    secondaryColor: '#676778'
  });

  // Set context using the setter
  setTheme(theme);
</script>

<Child />
```

### Using Type-Safe Context in Child

```svelte
<!-- file: Child.svelte -->
<script>
  import { getTheme } from './context.js';

  // Get context - fully typed!
  const theme = getTheme();

  $effect(() => {
    console.log('Current theme:', theme.mode);
  });
</script>

<div class={theme.mode}>
  <p style="color: {theme.primaryColor}">
    Themed content
  </p>
</div>
```

**Benefits of `createContext`:**
- Type safety (TypeScript support)
- No need to remember string keys
- Better IDE autocomplete
- Prevents key collisions

## Context Utilities

### hasContext - Check if Context Exists

Check whether a context exists before attempting to retrieve it:

```svelte
<script>
  import { hasContext, getContext } from 'svelte';

  let theme;

  if (hasContext('theme')) {
    theme = getContext('theme');
    console.log('Theme context available');
  } else {
    // Use default theme
    theme = { 
      primaryColor: '#000', 
      secondaryColor: '#666' 
    };
    console.log('Using default theme');
  }
</script>
```

### getAllContexts - Retrieve All Contexts

Get all available contexts as an object map:

```svelte
<script>
  import { getAllContexts } from 'svelte';

  // Get all contexts to pass to dynamically created component
  const contexts = getAllContexts();

  // Later, when creating component programmatically
  import { mount } from 'svelte';
  import DynamicComponent from './DynamicComponent.svelte';

  const instance = mount(DynamicComponent, {
    target: someElement,
    context: contexts // Pass contexts down
  });
</script>
```

## Common Patterns

### Pattern 1: Theme Provider

```typescript
/// file: theme-context.ts
import { createContext } from 'svelte';

export interface Theme {
  mode: 'light' | 'dark';
  colors: {
    primary: string;
    secondary: string;
    background: string;
    text: string;
  };
}

export const [getTheme, setTheme] = createContext<Theme>();
```

```svelte
<!-- file: ThemeProvider.svelte -->
<script>
  import { setTheme } from './theme-context.js';
  
  const theme = $state({
    mode: 'dark',
    colors: {
      primary: '#ff3e00',
      secondary: '#676778',
      background: '#1a1a1a',
      text: '#ffffff'
    }
  });

  setTheme(theme);

  function toggleTheme() {
    theme.mode = theme.mode === 'light' ? 'dark' : 'light';
    // Update colors based on mode
    if (theme.mode === 'dark') {
      theme.colors = {
        primary: '#ff3e00',
        secondary: '#676778',
        background: '#1a1a1a',
        text: '#ffffff'
      };
    } else {
      theme.colors = {
        primary: '#0066cc',
        secondary: '#666666',
        background: '#ffffff',
        text: '#000000'
      };
    }
  }
</script>

<button onclick={toggleTheme}>
  Toggle {theme.mode === 'dark' ? 'Light' : 'Dark'} Mode
</button>

<slot />
```

```svelte
<!-- file: ThemedComponent.svelte -->
<script>
  import { getTheme } from './theme-context.js';

  const theme = getTheme();
</script>

<div 
  style="background-color: {theme.colors.background}; color: {theme.colors.text};"
>
  <h1 style="color: {theme.colors.primary}">
    Themed Heading
  </h1>
  <p>Current theme: {theme.mode}</p>
</div>
```

### Pattern 2: API Provider

```typescript
/// file: api-context.ts
import { createContext } from 'svelte';

export interface ApiContext {
  fetchUser: (id: number) => Promise<User>;
  updateUser: (id: number, data: Partial<User>) => Promise<User>;
  deleteUser: (id: number) => Promise<void>;
}

export const [getApi, setApi] = createContext<ApiContext>();
```

```svelte
<!-- file: ApiProvider.svelte -->
<script>
  import { setApi } from './api-context.js';
  
  const api = {
    async fetchUser(id: number) {
      const response = await fetch(`/api/users/${id}`);
      if (!response.ok) throw new Error('Failed to fetch user');
      return response.json();
    },
    
    async updateUser(id: number, data: Partial<User>) {
      const response = await fetch(`/api/users/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
      });
      if (!response.ok) throw new Error('Failed to update user');
      return response.json();
    },
    
    async deleteUser(id: number) {
      const response = await fetch(`/api/users/${id}`, {
        method: 'DELETE'
      });
      if (!response.ok) throw new Error('Failed to delete user');
    }
  };

  setApi(api);
</script>

<slot />
```

```svelte
<!-- file: UserProfile.svelte -->
<script>
  import { getApi } from './api-context.js';
  
  const api = getApi();
  const user = $state<User | null>(null);
  const loading = $state(false);
  const error = $state<string | null>(null);

  async function loadUser(id: number) {
    loading = true;
    error = null;
    try {
      user = await api.fetchUser(id);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Unknown error';
    } finally {
      loading = false;
    }
  }

  async function updateUser(data: Partial<User>) {
    if (!user) return;
    try {
      user = await api.updateUser(user.id, data);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Unknown error';
    }
  }
</script>

{#if loading}
  <p>Loading...</p>
{:else if error}
  <p>Error: {error}</p>
{:else if user}
  <div>
    <h1>{user.name}</h1>
    <p>{user.email}</p>
    <button onclick={() => updateUser({ name: 'New Name' })}>
      Update Name
    </button>
  </div>
{/if}
```

### Pattern 3: Authentication Context

```typescript
/// file: auth-context.ts
import { createContext } from 'svelte';

export interface User {
  id: number;
  name: string;
  email: string;
  role: 'admin' | 'user';
}

export interface AuthContext {
  user: User | null;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  hasRole: (role: string) => boolean;
}

export const [getAuth, setAuth] = createContext<AuthContext>();
```

```svelte
<!-- file: AuthProvider.svelte -->
<script>
  import { setAuth } from './auth-context.js';
  
  const user = $state<User | null>(null);

  const auth = {
    get user() {
      return user;
    },
    
    get isAuthenticated() {
      return user !== null;
    },
    
    async login(email: string, password: string) {
      const response = await fetch('/api/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });
      
      if (!response.ok) {
        throw new Error('Login failed');
      }
      
      user = await response.json();
    },
    
    logout() {
      user = null;
      // Clear any tokens, etc.
    },
    
    hasRole(role: string) {
      return user?.role === role;
    }
  };

  setAuth(auth);
</script>

<slot />
```

```svelte
<!-- file: ProtectedRoute.svelte -->
<script>
  import { getAuth } from './auth-context.js';
  
  const auth = getAuth();
</script>

{#if !auth.isAuthenticated}
  <p>Please log in to access this content.</p>
{:else}
  <slot />
{/if}
```

```svelte
<!-- file: AdminPanel.svelte -->
<script>
  import { getAuth } from './auth-context.js';
  
  const auth = getAuth();
</script>

{#if !auth.hasRole('admin')}
  <p>Access denied. Admin role required.</p>
{:else}
  <div>
    <h1>Admin Panel</h1>
    <p>Welcome, {auth.user?.name}!</p>
    <!-- Admin content -->
  </div>
{/if}
```

### Pattern 4: Form Context

```typescript
/// file: form-context.ts
import { createContext } from 'svelte';

export interface FormContext<T> {
  values: T;
  errors: Partial<Record<keyof T, string>>;
  touched: Partial<Record<keyof T, boolean>>;
  setValue: <K extends keyof T>(field: K, value: T[K]) => void;
  setError: <K extends keyof T>(field: K, error: string) => void;
  setTouched: <K extends keyof T>(field: K, touched: boolean) => void;
  isValid: boolean;
  submit: () => Promise<void>;
}

export function createFormContext<T>() {
  return createContext<FormContext<T>>();
}
```

```svelte
<!-- file: FormProvider.svelte -->
<script lang="ts" generics="T">
  import { createFormContext } from './form-context.js';
  
  const [getForm, setForm] = createFormContext<T>();
  
  const values = $state<T>({} as T);
  const errors = $state<Partial<Record<keyof T, string>>>({});
  const touched = $state<Partial<Record<keyof T, boolean>>>({});
  
  const form = {
    get values() { return values; },
    get errors() { return errors; },
    get touched() { return touched; },
    
    setValue<K extends keyof T>(field: K, value: T[K]) {
      values[field] = value;
      // Clear error when user types
      if (errors[field]) {
        delete errors[field];
      }
    },
    
    setError<K extends keyof T>(field: K, error: string) {
      errors[field] = error;
    },
    
    setTouched<K extends keyof T>(field: K, touchedValue: boolean) {
      touched[field] = touchedValue;
    },
    
    get isValid() {
      return Object.keys(errors).length === 0;
    },
    
    async submit() {
      // Validation and submission logic
    }
  };
  
  setForm(form);
</script>

<slot />
```

## Context vs. Props vs. Stores

### When to Use Context

Use Context when:
- ✅ Data needs to be shared across many component levels
- ✅ You want to avoid prop drilling
- ✅ Data is specific to a component subtree
- ✅ You need to provide configuration/APIs to children
- ✅ You want type-safe, scoped data sharing

### When to Use Props

Use Props when:
- ✅ Data is passed directly from parent to child
- ✅ You need explicit data flow
- ✅ Component is reusable with different data
- ✅ Data flow is simple (1-2 levels)

### When to Use Stores

Use Stores when:
- ✅ Data needs to be shared across unrelated components
- ✅ Data is global application state
- ✅ You need reactive state management
- ✅ Data persists across route changes

### Context vs. Global State

**Important**: Context is **not shared between requests** in SSR. If you mutate state during server-side rendering, it won't affect other users. This makes context safer than global state for SSR applications.

```svelte
<!-- ❌ BAD: Global state shared between requests -->
<script>
  // In state.svelte.js
  export const myGlobalState = $state({
    user: { /* ... */ }
  });
</script>

<!-- ✅ GOOD: Context is request-scoped -->
<script>
  const user = $state({ /* ... */ });
  setContext('user', user);
</script>
```

## Best Practices

### 1. Use createContext for Type Safety

```typescript
// ✅ GOOD: Type-safe
export const [getTheme, setTheme] = createContext<Theme>();

// ⚠️ OK: Works but no type safety
setContext('theme', theme);
const theme = getContext('theme');
```

### 2. Don't Reassign Context Values

```svelte
<!-- ❌ BAD -->
<script>
  let counter = $state({ count: 0 });
  setContext('counter', counter);
</script>
<button onclick={() => counter = { count: 0 }}>Reset</button>

<!-- ✅ GOOD -->
<script>
  let counter = $state({ count: 0 });
  setContext('counter', counter);
</script>
<button onclick={() => counter.count = 0}>Reset</button>
```

### 3. Check Context Existence When Optional

```svelte
<script>
  import { hasContext, getContext } from 'svelte';
  
  let theme;
  if (hasContext('theme')) {
    theme = getContext('theme');
  } else {
    theme = defaultTheme;
  }
</script>
```

### 4. Use Descriptive Context Keys

```typescript
// ✅ GOOD: Clear, descriptive
export const [getUserContext, setUserContext] = createContext<User>();

// ⚠️ OK: Works but less clear
setContext('ctx1', user);
```

### 5. Provide Default Values

```svelte
<script>
  import { getTheme } from './theme-context.js';
  
  // Provide default if context might not exist
  const theme = getTheme() ?? {
    mode: 'light',
    primaryColor: '#000',
    secondaryColor: '#666'
  };
</script>
```

## Complete Example: Multi-Level Component Communication

```svelte
<!-- file: App.svelte -->
<script>
  import ThemeProvider from './ThemeProvider.svelte';
  import ApiProvider from './ApiProvider.svelte';
  import AuthProvider from './AuthProvider.svelte';
  import Dashboard from './Dashboard.svelte';
</script>

<ThemeProvider>
  <ApiProvider>
    <AuthProvider>
      <Dashboard />
    </AuthProvider>
  </ApiProvider>
</ThemeProvider>
```

```svelte
<!-- file: Dashboard.svelte -->
<script>
  import { getTheme } from './theme-context.js';
  import { getApi } from './api-context.js';
  import { getAuth } from './auth-context.js';
  import UserProfile from './UserProfile.svelte';
  import AdminPanel from './AdminPanel.svelte';
  
  const theme = getTheme();
  const api = getApi();
  const auth = getAuth();
</script>

<div style="background: {theme.colors.background}; color: {theme.colors.text};">
  <h1>Dashboard</h1>
  <p>Welcome, {auth.user?.name}!</p>
  
  <UserProfile />
  
  {#if auth.hasRole('admin')}
    <AdminPanel />
  {/if}
</div>
```

## Summary

The Context API in Svelte 5 provides:

1. **Avoid Prop Drilling**: Share data without passing through intermediate components
2. **Type Safety**: Use `createContext` for TypeScript support
3. **Reactive State**: Store `$state` in context for reactive updates
4. **Request-Scoped**: Safe for SSR (not shared between requests)
5. **Flexible**: Can share any JavaScript value (objects, functions, primitives)
6. **Utilities**: `hasContext` and `getAllContexts` for advanced use cases

Context is the recommended solution for sharing data across component hierarchies in Svelte 5, especially when combined with reactive state (`$state`) for powerful, type-safe component communication.

