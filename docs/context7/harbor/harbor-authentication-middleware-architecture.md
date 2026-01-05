# Harbor Authentication Middleware Architecture: Design & Call Stacks (Context7 Summary)

This document dives deep into **Harbor's architecture and design for authentication middleware**, focusing on how Harbor handles token-based authentication, JWT validation, authorization checks, and integration with Docker Distribution registry. It covers the complete call stacks, middleware patterns, and system design.

It builds on:

- `harbor-object-storage-architecture.md`
- `harbor-blob-upload-presigned-url-architecture.md`

and provides detailed call stacks and implementation patterns for authentication middleware.

---

## 1. High-Level Architecture: Multi-Layer Authentication

### 1.1 Authentication Components

Harbor uses a **multi-layer authentication architecture**:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Harbor Authentication Architecture                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 1: Nginx Reverse Proxy                                │  │
│  │  - Routes requests to appropriate backend services           │  │
│  │  - SSL/TLS termination                                       │  │
│  │  - Request forwarding                                        │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              │                                       │
│                              ▼                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 2: Harbor Core API (Token Service)                    │  │
│  │  - Token generation (JWT signed with private key)            │  │
│  │  - User authentication (DB, LDAP/AD, OIDC)                   │  │
│  │  - Permission checking                                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              │                                       │
│                              ▼                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 3: Docker Distribution Registry                       │  │
│  │  - Token validation (JWT verified with public key)           │  │
│  │  - Scope validation (repository:name:action)                 │  │
│  │  - Request authorization                                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              │                                       │
│                              ▼                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 4: Storage Backend                                    │  │
│  │  - Blob storage operations                                   │  │
│  │  - Metadata operations                                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.2 Authentication Flow Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│  Client Request Flow                                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. Client → Nginx → Harbor Core API                                │
│     - Request without token → 401 Unauthorized                      │
│     - Request with token → Validate token                           │
│                                                                      │
│  2. Harbor Core API → Token Service                                 │
│     - Authenticate user (DB/LDAP/OIDC)                             │
│     - Check permissions                                             │
│     - Generate JWT token (signed with private key)                  │
│                                                                      │
│  3. Client → Docker Distribution Registry                           │
│     - Request with Bearer token                                     │
│     - Registry validates token (public key)                         │
│     - Registry checks scope (repository:name:action)                │
│                                                                      │
│  4. Registry → Storage Backend                                      │
│     - Authorized operations proceed                                 │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Token Service Architecture

### 2.1 Token Generation Flow

Harbor's token service generates **JWT tokens** signed with a private key:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Token Generation: Complete Call Stack                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Client Requests Token                               │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  GET /service/token?service=harbor-registry            │  │  │
│  │  │    &scope=repository:library/mysql:pull,push           │  │  │
│  │  │  Authorization: Basic <base64(username:password)>      │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Nginx forwards to Harbor Core API
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Harbor Core API: Extract Credentials               │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Parse Authorization header                           │  │  │
│  │  │  - Decode base64(username:password)                     │  │  │
│  │  │  - Extract username and password                        │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Authenticate user
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: User Authentication                                │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  IF auth_mode == 'db_auth':                            │  │  │
│  │  │    - Query PostgreSQL: SELECT * FROM harbor_user       │  │  │
│  │  │      WHERE username = ?                                │  │  │
│  │  │    - Verify password hash (bcrypt/argon2)              │  │  │
│  │  │                                                         │  │  │
│  │  │  ELSE IF auth_mode == 'ldap_auth':                     │  │  │
│  │  │    - Connect to LDAP/AD server                         │  │  │
│  │  │    - Bind with username/password                       │  │  │
│  │  │    - Query user attributes                             │  │  │
│  │  │                                                         │  │  │
│  │  │  ELSE IF auth_mode == 'oidc_auth':                     │  │  │
│  │  │    - Validate OIDC token with identity provider        │  │  │
│  │  │    - Extract user claims                               │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Authentication successful
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Permission Check                                   │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Parse scope: repository:library/mysql:pull,push     │  │  │
│  │  │  - Extract repository: library/mysql                    │  │  │
│  │  │  - Extract actions: pull, push                          │  │  │
│  │  │  - Query PostgreSQL:                                    │  │  │
│  │  │    SELECT role FROM project_member                      │  │  │
│  │  │    WHERE project_id = ? AND user_id = ?                │  │  │
│  │  │  - Check if role has required permissions              │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Permissions valid
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 5: Generate JWT Token                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  JWT Payload:                                          │  │  │
│  │  │  {                                                     │  │  │
│  │  │    "iss": "harbor-token-service",                      │  │  │
│  │  │    "sub": "username",                                  │  │  │
│  │  │    "aud": "harbor-registry",                           │  │  │
│  │  │    "exp": <expiration_timestamp>,                      │  │  │
│  │  │    "iat": <issued_at_timestamp>,                       │  │  │
│  │  │    "access": [                                         │  │  │
│  │  │      {                                                 │  │  │
│  │  │        "type": "repository",                           │  │  │
│  │  │        "name": "library/mysql",                        │  │  │
│  │  │        "actions": ["pull", "push"]                     │  │  │
│  │  │      }                                                 │  │  │
│  │  │    ]                                                   │  │  │
│  │  │  }                                                     │  │  │
│  │  │                                                         │  │  │
│  │  │  - Sign JWT with private key (RS256)                   │  │  │
│  │  │  - Return token to client                              │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Token Service Implementation

```go
package token

import (
    "crypto/rsa"
    "crypto/x509"
    "encoding/pem"
    "time"
    "github.com/golang-jwt/jwt/v5"
)

type TokenService struct {
    privateKey *rsa.PrivateKey
    publicKey  *rsa.PublicKey
    issuer     string
    expiration time.Duration
    db         *sql.DB
    ldapClient *ldap.Client
    oidcConfig *oidc.Config
}

// TokenClaims represents JWT token claims
type TokenClaims struct {
    jwt.RegisteredClaims
    Access []AccessEntry `json:"access"`
}

// AccessEntry represents repository access scope
type AccessEntry struct {
    Type    string   `json:"type"`    // "repository"
    Name    string   `json:"name"`    // "library/mysql"
    Actions []string `json:"actions"` // ["pull", "push"]
}

/**
 * Generate token for authenticated user
 */
func (ts *TokenService) GenerateToken(
    ctx context.Context,
    username string,
    password string,
    service string,
    scopes []string,
) (string, error) {
    // Step 1: Authenticate user
    user, err := ts.authenticateUser(ctx, username, password)
    if err != nil {
        return "", fmt.Errorf("authentication failed: %w", err)
    }

    // Step 2: Parse and validate scopes
    accessEntries, err := ts.parseScopes(scopes)
    if err != nil {
        return "", fmt.Errorf("invalid scopes: %w", err)
    }

    // Step 3: Check permissions for each scope
    for _, entry := range accessEntries {
        if err := ts.checkPermission(ctx, user, entry); err != nil {
            return "", fmt.Errorf("permission denied: %w", err)
        }
    }

    // Step 4: Generate JWT token
    now := time.Now()
    claims := TokenClaims{
        RegisteredClaims: jwt.RegisteredClaims{
            Issuer:    ts.issuer,
            Subject:   user.Username,
            Audience:  []string{service},
            ExpiresAt: jwt.NewNumericDate(now.Add(ts.expiration)),
            IssuedAt:  jwt.NewNumericDate(now),
            NotBefore: jwt.NewNumericDate(now),
        },
        Access: accessEntries,
    }

    token := jwt.NewWithClaims(jwt.SigningMethodRS256, claims)
    tokenString, err := token.SignedString(ts.privateKey)
    if err != nil {
        return "", fmt.Errorf("failed to sign token: %w", err)
    }

    return tokenString, nil
}

/**
 * Authenticate user based on configured auth mode
 */
func (ts *TokenService) authenticateUser(
    ctx context.Context,
    username string,
    password string,
) (*User, error) {
    authMode := ts.getAuthMode()

    switch authMode {
    case "db_auth":
        return ts.authenticateDB(ctx, username, password)
    case "ldap_auth":
        return ts.authenticateLDAP(ctx, username, password)
    case "oidc_auth":
        return ts.authenticateOIDC(ctx, username, password)
    default:
        return nil, fmt.Errorf("unsupported auth mode: %s", authMode)
    }
}

/**
 * Database authentication
 */
func (ts *TokenService) authenticateDB(
    ctx context.Context,
    username string,
    password string,
) (*User, error) {
    var user User
    err := ts.db.QueryRowContext(
        ctx,
        `SELECT user_id, username, password, email, realname,
                sysadmin_flag, deleted
         FROM harbor_user
         WHERE username = ? AND deleted = false`,
        username,
    ).Scan(
        &user.ID, &user.Username, &user.PasswordHash,
        &user.Email, &user.Realname, &user.SysAdmin, &user.Deleted,
    )
    if err != nil {
        return nil, fmt.Errorf("user not found: %w", err)
    }

    // Verify password hash
    if err := bcrypt.CompareHashAndPassword(
        []byte(user.PasswordHash),
        []byte(password),
    ); err != nil {
        return nil, fmt.Errorf("invalid password: %w", err)
    }

    return &user, nil
}

/**
 * LDAP/AD authentication
 */
func (ts *TokenService) authenticateLDAP(
    ctx context.Context,
    username string,
    password string,
) (*User, error) {
    // Connect to LDAP server
    conn, err := ts.ldapClient.Connect()
    if err != nil {
        return nil, fmt.Errorf("LDAP connection failed: %w", err)
    }
    defer conn.Close()

    // Bind with username/password
    bindDN := fmt.Sprintf("cn=%s,%s", username, ts.ldapBaseDN)
    if err := conn.Bind(bindDN, password); err != nil {
        return nil, fmt.Errorf("LDAP bind failed: %w", err)
    }

    // Search for user attributes
    searchRequest := ldap.NewSearchRequest(
        ts.ldapBaseDN,
        ldap.ScopeWholeSubtree,
        ldap.NeverDerefAliases,
        0, 0, false,
        fmt.Sprintf("(cn=%s)", username),
        []string{"cn", "mail", "displayName"},
        nil,
    )

    result, err := conn.Search(searchRequest)
    if err != nil {
        return nil, fmt.Errorf("LDAP search failed: %w", err)
    }

    if len(result.Entries) == 0 {
        return nil, fmt.Errorf("user not found in LDAP")
    }

    entry := result.Entries[0]
    return &User{
        Username: entry.GetAttributeValue("cn"),
        Email:    entry.GetAttributeValue("mail"),
        Realname: entry.GetAttributeValue("displayName"),
    }, nil
}

/**
 * Check permission for repository access
 */
func (ts *TokenService) checkPermission(
    ctx context.Context,
    user *User,
    access AccessEntry,
) error {
    // System admin has all permissions
    if user.SysAdmin {
        return nil
    }

    // Parse repository name: "library/mysql" -> project="library", repo="mysql"
    parts := strings.Split(access.Name, "/")
    if len(parts) != 2 {
        return fmt.Errorf("invalid repository name: %s", access.Name)
    }
    projectName := parts[0]
    repoName := parts[1]

    // Query project membership
    var role string
    err := ts.db.QueryRowContext(
        ctx,
        `SELECT role FROM project_member pm
         JOIN project p ON p.project_id = pm.project_id
         WHERE p.name = ? AND pm.user_id = ? AND p.deleted = false`,
        projectName, user.ID,
    ).Scan(&role)

    if err != nil {
        // Check if project is public
        var isPublic bool
        err2 := ts.db.QueryRowContext(
            ctx,
            `SELECT public FROM project WHERE name = ? AND deleted = false`,
            projectName,
        ).Scan(&isPublic)

        if err2 != nil || !isPublic {
            return fmt.Errorf("permission denied: not a member of project %s", projectName)
        }

        // Public project: only pull is allowed
        for _, action := range access.Actions {
            if action != "pull" {
                return fmt.Errorf("permission denied: %s requires project membership", action)
            }
        }
        return nil
    }

    // Check if role has required permissions
    for _, action := range access.Actions {
        if !ts.roleHasPermission(role, action) {
            return fmt.Errorf("permission denied: role %s cannot %s", role, action)
        }
    }

    return nil
}

/**
 * Check if role has permission for action
 */
func (ts *TokenService) roleHasPermission(role string, action string) bool {
    permissions := map[string][]string{
        "projectAdmin": {"pull", "push", "delete", "scan"},
        "developer":    {"pull", "push", "scan"},
        "guest":        {"pull"},
        "maintainer":   {"pull", "push", "scan"},
    }

    allowedActions, ok := permissions[role]
    if !ok {
        return false
    }

    for _, allowed := range allowedActions {
        if allowed == action {
            return true
        }
    }

    return false
}
```

---

## 3. Authentication Middleware

### 3.1 Harbor Core API Middleware

Harbor Core API uses middleware to validate tokens on incoming requests:

```go
package middleware

import (
    "context"
    "net/http"
    "strings"
    "github.com/golang-jwt/jwt/v5"
)

type AuthMiddleware struct {
    publicKey  *rsa.PublicKey
    tokenService *TokenService
}

/**
 * Authentication middleware for Harbor Core API
 */
func (am *AuthMiddleware) Authenticate(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        // Step 1: Extract token from request
        token, err := am.extractToken(r)
        if err != nil {
            http.Error(w, "Unauthorized", http.StatusUnauthorized)
            return
        }

        // Step 2: Validate token
        claims, err := am.validateToken(token)
        if err != nil {
            http.Error(w, "Unauthorized: invalid token", http.StatusUnauthorized)
            return
        }

        // Step 3: Add user context to request
        ctx := context.WithValue(r.Context(), "user", claims.Subject)
        ctx = context.WithValue(ctx, "access", claims.Access)
        r = r.WithContext(ctx)

        // Step 4: Continue to next handler
        next.ServeHTTP(w, r)
    })
}

/**
 * Extract token from Authorization header or cookie
 */
func (am *AuthMiddleware) extractToken(r *http.Request) (string, error) {
    // Try Bearer token in Authorization header
    authHeader := r.Header.Get("Authorization")
    if authHeader != "" {
        parts := strings.Split(authHeader, " ")
        if len(parts) == 2 && parts[0] == "Bearer" {
            return parts[1], nil
        }
    }

    // Try cookie
    cookie, err := r.Cookie("harbor_session")
    if err == nil {
        return cookie.Value, nil
    }

    return "", fmt.Errorf("no token found")
}

/**
 * Validate JWT token
 */
func (am *AuthMiddleware) validateToken(tokenString string) (*TokenClaims, error) {
    token, err := jwt.ParseWithClaims(
        tokenString,
        &TokenClaims{},
        func(token *jwt.Token) (interface{}, error) {
            // Verify signing method
            if _, ok := token.Method.(*jwt.SigningMethodRSA); !ok {
                return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
            }
            return am.publicKey, nil
        },
    )

    if err != nil {
        return nil, fmt.Errorf("token validation failed: %w", err)
    }

    claims, ok := token.Claims.(*TokenClaims)
    if !ok || !token.Valid {
        return nil, fmt.Errorf("invalid token claims")
    }

    // Check expiration
    if claims.ExpiresAt != nil && claims.ExpiresAt.Time.Before(time.Now()) {
        return nil, fmt.Errorf("token expired")
    }

    return claims, nil
}
```

### 3.2 Authorization Middleware

After authentication, Harbor checks authorization (permissions):

```go
/**
 * Authorization middleware: Check if user has required permissions
 */
func (am *AuthMiddleware) Authorize(
    requiredAction string,
    projectName string,
) func(http.Handler) http.Handler {
    return func(next http.Handler) http.Handler {
        return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
            // Get user from context
            username, ok := r.Context().Value("user").(string)
            if !ok {
                http.Error(w, "Unauthorized", http.StatusUnauthorized)
                return
            }

            // Get access scopes from context
            access, ok := r.Context().Value("access").([]AccessEntry)
            if !ok {
                http.Error(w, "Forbidden", http.StatusForbidden)
                return
            }

            // Check if user has required permission
            hasPermission := false
            for _, entry := range access {
                if entry.Type == "repository" {
                    parts := strings.Split(entry.Name, "/")
                    if len(parts) == 2 && parts[0] == projectName {
                        for _, action := range entry.Actions {
                            if action == requiredAction {
                                hasPermission = true
                                break
                            }
                        }
                    }
                }
            }

            if !hasPermission {
                // Check database for current permissions (token might be stale)
                user, err := am.tokenService.getUser(username)
                if err != nil {
                    http.Error(w, "Forbidden", http.StatusForbidden)
                    return
                }

                if !am.tokenService.checkPermissionInDB(r.Context(), user, projectName, requiredAction) {
                    http.Error(w, "Forbidden", http.StatusForbidden)
                    return
                }
            }

            // Continue to next handler
            next.ServeHTTP(w, r)
        })
    }
}
```

---

## 4. Docker Distribution Registry Integration

### 4.1 Registry Token Validation

Docker Distribution registry validates tokens using the public key:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Docker Distribution: Token Validation Call Stack                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Client Request to Registry                         │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  GET /v2/library/mysql/manifests/latest                │  │  │
│  │  │  Authorization: Bearer <token>                          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Registry receives request
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Registry Extracts Token                            │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Parse Authorization header                           │  │  │
│  │  │  - Extract Bearer token                                 │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Validate token signature
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Token Signature Validation                         │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Decode JWT token                                    │  │  │
│  │  │  - Verify signature using public key (RS256)           │  │  │
│  │  │  - Check token expiration                              │  │  │
│  │  │  - Validate issuer (must be "harbor-token-service")    │  │  │
│  │  │  - Validate audience (must be "harbor-registry")       │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Token valid
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Scope Validation                                   │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Extract access scopes from token                    │  │  │
│  │  │  - Parse repository name from request path             │  │  │
│  │  │  - Extract action from request method                  │  │  │
│  │  │    (GET = pull, PUT/POST = push, DELETE = delete)      │  │  │
│  │  │  - Check if scope matches:                             │  │  │
│  │  │    repository:library/mysql:pull                        │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Scope valid
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 5: Authorize Request                                  │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Allow request to proceed                            │  │  │
│  │  │  - Execute registry operation                          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 4.2 Registry Token Validator Implementation

```go
package registry

import (
    "crypto/rsa"
    "net/http"
    "strings"
    "github.com/docker/distribution/registry/auth/token"
    "github.com/golang-jwt/jwt/v5"
)

type HarborTokenValidator struct {
    publicKey *rsa.PublicKey
}

/**
 * Validate token for Docker Distribution registry
 */
func (htv *HarborTokenValidator) ValidateToken(
    r *http.Request,
    requiredScope string,
) (*token.ResourceActions, error) {
    // Step 1: Extract token
    tokenString, err := htv.extractToken(r)
    if err != nil {
        return nil, fmt.Errorf("no token found: %w", err)
    }

    // Step 2: Parse and validate token
    claims, err := htv.parseToken(tokenString)
    if err != nil {
        return nil, fmt.Errorf("token validation failed: %w", err)
    }

    // Step 3: Parse required scope
    // Format: "repository:library/mysql:pull"
    scopeParts := strings.Split(requiredScope, ":")
    if len(scopeParts) != 3 {
        return nil, fmt.Errorf("invalid scope format: %s", requiredScope)
    }

    requiredType := scopeParts[0]    // "repository"
    requiredName := scopeParts[1]    // "library/mysql"
    requiredAction := scopeParts[2]  // "pull"

    // Step 4: Check if token has required scope
    for _, access := range claims.Access {
        if access.Type == requiredType && access.Name == requiredName {
            for _, action := range access.Actions {
                if action == requiredAction {
                    return &token.ResourceActions{
                        Type:    requiredType,
                        Name:    requiredName,
                        Actions: []string{requiredAction},
                    }, nil
                }
            }
        }
    }

    return nil, fmt.Errorf("token does not have required scope: %s", requiredScope)
}

/**
 * Parse and validate JWT token
 */
func (htv *HarborTokenValidator) parseToken(tokenString string) (*TokenClaims, error) {
    token, err := jwt.ParseWithClaims(
        tokenString,
        &TokenClaims{},
        func(token *jwt.Token) (interface{}, error) {
            // Verify signing method
            if _, ok := token.Method.(*jwt.SigningMethodRSA); !ok {
                return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
            }
            return htv.publicKey, nil
        },
    )

    if err != nil {
        return nil, fmt.Errorf("token parsing failed: %w", err)
    }

    claims, ok := token.Claims.(*TokenClaims)
    if !ok || !token.Valid {
        return nil, fmt.Errorf("invalid token claims")
    }

    // Validate issuer
    if claims.Issuer != "harbor-token-service" {
        return nil, fmt.Errorf("invalid issuer: %s", claims.Issuer)
    }

    // Validate audience
    if !contains(claims.Audience, "harbor-registry") {
        return nil, fmt.Errorf("invalid audience")
    }

    // Check expiration
    if claims.ExpiresAt != nil && claims.ExpiresAt.Time.Before(time.Now()) {
        return nil, fmt.Errorf("token expired")
    }

    return claims, nil
}
```

---

## 5. Complete Authentication Flow: Docker Push Example

### 5.1 Complete Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Client: docker push harbor.example.com/library/app:latest         │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Step 1: Initial request (no token)
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Docker Distribution Registry                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /v2/library/app/blobs/uploads/                        │  │
│  │  (No Authorization header)                                    │  │
│  │                                                               │  │
│  │  Response: 401 Unauthorized                                  │  │
│  │  WWW-Authenticate: Bearer realm="https://harbor.example.com/ │  │
│  │    service/token",service="harbor-registry",                 │  │
│  │    scope="repository:library/app:pull,push"                  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Client requests token
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Harbor Core API: Token Service                                    │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  GET /service/token?service=harbor-registry                  │  │
│  │    &scope=repository:library/app:pull,push                   │  │
│  │  Authorization: Basic <base64(username:password)>            │  │
│  │                                                               │  │
│  │  1. Extract credentials                                      │  │
│  │  2. Authenticate user (DB/LDAP/OIDC)                        │  │
│  │  3. Check permissions                                        │  │
│  │  4. Generate JWT token (signed with private key)             │  │
│  │                                                               │  │
│  │  Response: 200 OK                                            │  │
│  │  {                                                           │  │
│  │    "token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",      │  │
│  │    "expires_in": 3600,                                       │  │
│  │    "issued_at": "2025-01-15T10:00:00Z"                      │  │
│  │  }                                                           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Client retries with token
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Docker Distribution Registry                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /v2/library/app/blobs/uploads/                        │  │
│  │  Authorization: Bearer <token>                                │  │
│  │                                                               │  │
│  │  1. Extract token from Authorization header                  │  │
│  │  2. Validate token signature (public key)                    │  │
│  │  3. Check token expiration                                   │  │
│  │  4. Validate scope: repository:library/app:push              │  │
│  │  5. Authorize request                                        │  │
│  │                                                               │  │
│  │  Response: 202 Accepted                                      │  │
│  │  Location: /v2/library/app/blobs/uploads/{uuid}             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Multi-Authentication Provider Support

### 6.1 Authentication Provider Abstraction

Harbor supports multiple authentication providers through an abstraction:

```go
type AuthProvider interface {
    Authenticate(ctx context.Context, username string, password string) (*User, error)
    GetUser(ctx context.Context, username string) (*User, error)
}

// Database authentication provider
type DBAuthProvider struct {
    db *sql.DB
}

func (p *DBAuthProvider) Authenticate(ctx context.Context, username string, password string) (*User, error) {
    // Query database and verify password hash
    // ...
}

// LDAP authentication provider
type LDAPAuthProvider struct {
    client *ldap.Client
    baseDN string
}

func (p *LDAPAuthProvider) Authenticate(ctx context.Context, username string, password string) (*User, error) {
    // Bind to LDAP server and query user attributes
    // ...
}

// OIDC authentication provider
type OIDCAuthProvider struct {
    verifier *oidc.IDTokenVerifier
}

func (p *OIDCAuthProvider) Authenticate(ctx context.Context, username string, password string) (*User, error) {
    // Validate OIDC token and extract claims
    // ...
}
```

### 6.2 Authentication Provider Factory

```go
func NewAuthProvider(authMode string, config AuthConfig) (AuthProvider, error) {
    switch authMode {
    case "db_auth":
        return &DBAuthProvider{db: config.DB}, nil
    case "ldap_auth":
        return &LDAPAuthProvider{
            client: config.LDAPClient,
            baseDN: config.LDAPBaseDN,
        }, nil
    case "oidc_auth":
        return &OIDCAuthProvider{verifier: config.OIDCVerifier}, nil
    default:
        return nil, fmt.Errorf("unsupported auth mode: %s", authMode)
    }
}
```

---

## 7. Key-Pair Management

### 7.1 Private/Public Key Generation

Harbor generates RSA key pairs for token signing:

```go
package crypto

import (
    "crypto/rand"
    "crypto/rsa"
    "crypto/x509"
    "encoding/pem"
)

/**
 * Generate RSA key pair for token signing
 */
func GenerateKeyPair() (*rsa.PrivateKey, *rsa.PublicKey, error) {
    privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
    if err != nil {
        return nil, nil, err
    }

    publicKey := &privateKey.PublicKey
    return privateKey, publicKey, nil
}

/**
 * Save private key to file
 */
func SavePrivateKey(key *rsa.PrivateKey, path string) error {
    keyBytes := x509.MarshalPKCS1PrivateKey(key)
    keyPEM := pem.EncodeToMemory(&pem.Block{
        Type:  "RSA PRIVATE KEY",
        Bytes: keyBytes,
    })

    return os.WriteFile(path, keyPEM, 0600)
}

/**
 * Save public key to file
 */
func SavePublicKey(key *rsa.PublicKey, path string) error {
    keyBytes, err := x509.MarshalPKIXPublicKey(key)
    if err != nil {
        return err
    }

    keyPEM := pem.EncodeToMemory(&pem.Block{
        Type:  "PUBLIC KEY",
        Bytes: keyBytes,
    })

    return os.WriteFile(path, keyPEM, 0644)
}

/**
 * Load private key from file
 */
func LoadPrivateKey(path string) (*rsa.PrivateKey, error) {
    keyPEM, err := os.ReadFile(path)
    if err != nil {
        return nil, err
    }

    block, _ := pem.Decode(keyPEM)
    if block == nil {
        return nil, fmt.Errorf("failed to decode PEM block")
    }

    key, err := x509.ParsePKCS1PrivateKey(block.Bytes)
    if err != nil {
        return nil, err
    }

    return key, nil
}

/**
 * Load public key from file
 */
func LoadPublicKey(path string) (*rsa.PublicKey, error) {
    keyPEM, err := os.ReadFile(path)
    if err != nil {
        return nil, err
    }

    block, _ := pem.Decode(keyPEM)
    if block == nil {
        return nil, fmt.Errorf("failed to decode PEM block")
    }

    pub, err := x509.ParsePKIXPublicKey(block.Bytes)
    if err != nil {
        return nil, err
    }

    publicKey, ok := pub.(*rsa.PublicKey)
    if !ok {
        return nil, fmt.Errorf("not an RSA public key")
    }

    return publicKey, nil
}
```

### 7.2 Key Distribution

**Harbor Core API** (Token Service):

- Holds **private key** for signing tokens
- Generates JWT tokens signed with private key

**Docker Distribution Registry**:

- Holds **public key** for validating tokens
- Validates token signatures using public key
- Does not need access to private key

**Key Distribution**:

- Keys are generated during Harbor installation
- Private key stored securely (file system, secrets manager)
- Public key shared with registry via configuration
- Keys can be rotated (requires coordination)

---

## 8. Middleware Chain Pattern

### 8.1 Request Processing Pipeline

Harbor uses a middleware chain pattern for request processing:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Request Processing Pipeline                                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Request → [Middleware 1] → [Middleware 2] → [Middleware 3] → Handler
│                                                                      │
│  Middleware Chain:                                                  │
│  1. Logging Middleware (log request)                                │
│  2. CORS Middleware (handle CORS headers)                           │
│  3. Authentication Middleware (validate token)                      │
│  4. Authorization Middleware (check permissions)                    │
│  5. Rate Limiting Middleware (throttle requests)                    │
│  6. Request Handler (process request)                               │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.2 Middleware Implementation

```go
package middleware

import (
    "net/http"
)

type Middleware func(http.Handler) http.Handler

/**
 * Chain multiple middlewares together
 */
func Chain(middlewares ...Middleware) Middleware {
    return func(next http.Handler) http.Handler {
        for i := len(middlewares) - 1; i >= 0; i-- {
            next = middlewares[i](next)
        }
        return next
    }
}

/**
 * Apply middleware chain to handler
 */
func Apply(handler http.Handler, middlewares ...Middleware) http.Handler {
    return Chain(middlewares...)(handler)
}

// Usage example:
func setupRoutes() {
    authMiddleware := &AuthMiddleware{...}
    authzMiddleware := &AuthzMiddleware{...}
    loggingMiddleware := &LoggingMiddleware{...}

    handler := http.HandlerFunc(handleRequest)

    // Apply middleware chain
    finalHandler := Apply(
        handler,
        loggingMiddleware.Log,
        authMiddleware.Authenticate,
        authzMiddleware.Authorize("push", "library"),
    )

    http.Handle("/api/v2.0/...", finalHandler)
}
```

---

## 9. Design Patterns for DataForge

### 9.1 Token-Based Authentication

**Harbor Pattern**:

- JWT tokens signed with private key
- Public key used for validation
- Token contains user identity and scopes

**DataForge Application**:

- Use JWT tokens for API authentication
- Sign tokens with private key
- Validate tokens with public key
- Include workspace and permission scopes in tokens

### 9.2 Middleware Chain

**Harbor Pattern**:

- Chain multiple middlewares together
- Authentication before authorization
- Request logging and rate limiting

**DataForge Application**:

- Use middleware chain for request processing
- Authenticate before authorizing
- Add logging and monitoring middleware

### 9.3 Multi-Provider Authentication

**Harbor Pattern**:

- Abstract authentication providers
- Support DB, LDAP/AD, OIDC
- Factory pattern for provider creation

**DataForge Application**:

- Support multiple authentication methods
- Abstract authentication provider interface
- Easy to add new providers

---

## 10. Summary

### ✅ Key Patterns

1. **Multi-Layer Authentication**: Nginx → Harbor Core → Registry → Storage
2. **Token-Based Auth**: JWT tokens signed with private key, validated with public key
3. **Middleware Chain**: Authentication → Authorization → Request Handler
4. **Scope-Based Permissions**: Repository-level access control with actions (pull, push, delete)
5. **Multi-Provider Support**: DB, LDAP/AD, OIDC authentication providers

### 📋 Best Practices

1. **Separate authentication from authorization** (authenticate first, then authorize)
2. **Use JWT tokens** for stateless authentication
3. **Sign tokens with private key** (only token service has private key)
4. **Validate tokens with public key** (registry validates without private key)
5. **Include scopes in tokens** (repository:name:action format)
6. **Check permissions in middleware** (before request handler)
7. **Support multiple auth providers** (flexible authentication)
8. **Log authentication events** (audit trail)

This architecture provides a secure, scalable authentication system that integrates seamlessly with Docker Distribution registry while supporting multiple authentication providers and fine-grained access control.

