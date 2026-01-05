# 🔐 Complete Authentication Flow Documentation

## Overview

This document describes the complete authentication flow for the MudRock application, including both sign-in and sign-out processes using Supabase with Tauri.

## Architecture

```
User Interface → SvelteKit → Supabase Auth → Tauri Backend → Database
```

## Sign-In Flow

> **🔀 IMPORTANT: Redirect Target Changed**
>
> As of the latest update, authenticated users are now redirected to **`/projects`** instead of `/home` or `/dashboard`. This is the primary entry point for authenticated users.

### 1. **User Access**

- User navigates to `/` (root page)
- `src/routes/+page.svelte` checks authentication state
- If not authenticated, redirects to `/auth/login`

### 2. **Login Page**

- **File**: `src/routes/auth/login/+page.svelte`
- User enters email and password
- Form submission triggers client-side login

### 3. **Client-Side Authentication**

- **File**: `src/lib/supabase-auth.ts`
- `login(email, password)` function called
- Network connectivity test using Tauri HTTP plugin
- Supabase `signInWithPassword()` API call
- JWT token received and stored in localStorage

### 4. **State Management**

- Svelte stores updated:
  - `user.set(userInfo)` - User information
  - `isAuthenticated.set(true)` - Authentication status
  - `isLoading.set(false)` - Loading state
- Auth state change listener activated

### 5. **Server-Side Validation**

- **File**: `src/hooks.server.ts`
- Session validation from cookies or Authorization header
- Supabase client validates JWT token
- Session stored in `event.locals.session`

### 6. **Redirect Logic**

- **File**: `src/routes/+layout.svelte`
- `$effect` watches authentication state changes
- **Authenticated users redirected to `/projects`** ✅
  - After successful login: `/auth/login` → `/projects`
  - From root route: `/` → `/projects`
  - From `/home` route: `/home` → `/projects` (legacy route redirect)
  - From any auth route: `/auth/*` → `/projects`
- Unauthenticated users redirected to `/auth/login`

### 7. **Projects Page Access**

- **File**: `src/routes/projects/+page.svelte`
- Renders `ProjectsPage` component (`src/lib/components/pages/projects/projects-page.svelte`)
- User can access protected projects content

## Sign-Out Flow

### 1. **User Action**

- User clicks "Account" menu in menubar
- Selects "Sign Out" option
- **File**: `src/lib/components/pages/home/menubar-home/menubar-home.svelte`

### 2. **Sign-Out Process**

- `handleMenuAction('sign-out')` called
- `logout()` function from `src/lib/supabase-auth.ts` executed
- Supabase `signOut()` API call made
- All sessions terminated (global scope)

### 3. **State Cleanup**

- Svelte stores cleared:
  - `user.set(null)`
  - `isAuthenticated.set(false)`
  - `authError.set(null)`
- LocalStorage cleared of auth tokens

### 4. **Redirect**

- `goto('/auth/login')` called
- User redirected to login page
- Authentication state reset

### 5. **Session Invalidation**

- Server-side session validation fails
- `hooks.server.ts` detects no valid session
- Future requests require re-authentication

## Key Components

### **Frontend Authentication Client**

```typescript
// src/lib/supabase-auth.ts
export async function login(email: string, password: string): Promise<boolean>;
export async function logout(): Promise<void>;
export async function initializeAuth(): Promise<void>;
export const user = writable<UserInfo | null>(null);
export const isAuthenticated = writable<boolean>(false);
```

### **Server-Side Session Handling**

```typescript
// src/hooks.server.ts
export const handle: Handle = async ({ event, resolve }) => {
  const session = await getSession(event);
  event.locals.session = session;
  // Route protection logic
};
```

### **Layout Authentication Logic**

```svelte
<!-- src/routes/+layout.svelte -->
$effect(() => {
  // Redirect authenticated users away from auth pages to /projects
  if ($isAuthenticated && isAuthRoute && currentPath !== '/projects') {
    goto('/projects')
  }
  // Redirect authenticated users from root to /projects
  else if ($isAuthenticated && currentPath === '/') {
    goto('/projects')
  }
  // Redirect authenticated users from /home to /projects (legacy route)
  else if ($isAuthenticated && currentPath === '/home') {
    goto('/projects')
  }
  // Redirect unauthenticated users to login
  else if (!$isAuthenticated && !isPublicRoute) {
    goto('/auth/login')
  }
})
```

## Security Features

### **Token Management**

- JWT tokens stored securely in localStorage
- Automatic token refresh handled by Supabase
- Token validation on every request
- Expired tokens automatically cleared

### **Session Security**

- PKCE flow enabled for enhanced security
- Auto-refresh tokens enabled
- Session persistence in cookies
- URL session detection enabled

### **Network Security**

- HTTPS communication (when deployed)
- API key authentication
- CORS properly configured
- Input validation and sanitization

## Error Handling

### **Network Errors**

- Graceful handling of offline scenarios
- Cached session fallback
- User-friendly error messages
- Automatic retry mechanisms

### **Authentication Errors**

- Invalid credentials handling
- Email confirmation requirements
- Session expiration handling
- Scope error recovery

### **Redirect Loops Prevention**

- State tracking to prevent infinite redirects
- Timeout mechanisms
- Fallback redirect strategies

## Performance Optimizations

### **Fast Authentication**

- Cached session validation
- Minimal API calls
- Efficient state management
- Optimized redirect logic

### **User Experience**

- Loading states during authentication
- Smooth transitions between pages
- Clear error messages
- Responsive design

## Testing

### **Manual Testing**

1. Navigate to `/auth/login`
2. Enter valid credentials
3. **Verify redirect to `/projects`** ✅
4. Test sign-out from Account menu
5. Verify redirect to `/auth/login`
6. Test direct navigation to `/home` (should redirect to `/projects`)
7. Test direct navigation to `/` (should redirect to `/projects` if authenticated)

### **Error Scenarios**

- Invalid credentials
- Network connectivity issues
- Session expiration
- Server errors

## Configuration

### **Environment Variables**

```typescript
const PUBLIC_SUPABASE_URL = "http://91.99.166.223:3001";
const PUBLIC_SUPABASE_ANON_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
```

### **Supabase Client Configuration**

```typescript
const supabase = createClient(url, key, {
  auth: {
    flowType: "pkce",
    autoRefreshToken: true,
    persistSession: true,
    detectSessionInUrl: true,
  },
});
```

## Troubleshooting

### **Common Issues**

1. **Infinite Redirect Loops**: Check redirect logic in `+layout.svelte`
2. **Session Not Persisting**: Verify localStorage and cookie settings
3. **Network Errors**: Check Supabase URL and API key configuration
4. **Token Validation Failures**: Ensure proper JWT handling

### **Debug Tools**

- Browser DevTools for localStorage inspection
- Network tab for API call monitoring
- Console logs for authentication flow tracking
- Supabase dashboard for session management

## Future Enhancements

### **Planned Features**

- Password reset functionality
- Email verification flow
- Social login integration
- Two-factor authentication
- Session management improvements
- Remember me functionality

### **Security Improvements**

- Enhanced token security
- Rate limiting
- Audit logging
- Advanced session management

## Conclusion

The MudRock authentication system provides a robust, secure, and user-friendly authentication experience using Supabase with Tauri. The system handles both sign-in and sign-out flows efficiently while maintaining security best practices and providing excellent user experience.

The modular architecture allows for easy maintenance and future enhancements while ensuring reliable authentication across the entire application.
