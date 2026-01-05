# ColaNode Authentication

This document details ColaNode's authentication system, including account verification methods, OAuth integration, and the account-workspace-user model.

## Overview

ColaNode uses a server-based authentication system where the server authenticates users, stores data, runs background tasks, and broadcasts changes to connected devices. The authentication system supports multiple verification methods and optional OAuth integration.

## Architecture

### Server Role

The ColaNode server is responsible for:

- Authenticating users
- Storing data in PostgreSQL
- Running background tasks
- Broadcasting changes to connected devices

### Account-Workspace-User Model

ColaNode uses a three-tier identity model:

1. **Account**: Top-level identity that can belong to multiple workspaces
2. **Workspace**: Container that hosts multiple users
3. **User**: Workspace-scoped identity created when an account joins a workspace

**Key Characteristics:**

- A server can authenticate multiple accounts
- A server can host multiple workspaces
- An account may belong to multiple workspaces
- When an account joins a workspace, ColaNode creates a distinct user identity with a unique ID scoped to that workspace
- Each user belongs to exactly one workspace
- This per-workspace user model simplifies authorization, auditing, and data partitioning

## Account Verification Methods

ColaNode supports three account verification methods, configurable via the `ACCOUNT_VERIFICATION_TYPE` environment variable:

### 1. Automatic Verification (`automatic`)

- **Default setting**
- New accounts are automatically marked as verified without additional steps
- No user action required
- Suitable for trusted environments or internal deployments

### 2. Manual Verification (`manual`)

- Administrators manually verify accounts
- Verification is done by setting the `status` column to `1` in the `accounts` table within the PostgreSQL database
- Provides full administrative control over account activation
- Suitable for organizations requiring strict access control

**Database Operation:**

```sql
UPDATE accounts SET status = 1 WHERE id = '<account_id>';
```

### 3. Email Verification (`email`)

- A verification code is sent to the user's email address
- Requires SMTP to be enabled and configured
- Users must enter the verification code to activate their account
- The timeout for the one-time password (OTP) is configurable via `ACCOUNT_OTP_TIMEOUT` environment variable
- Default timeout: 600 seconds (10 minutes)

**Required Environment Variables:**

- `ACCOUNT_VERIFICATION_TYPE=email`
- `ACCOUNT_OTP_TIMEOUT=600` (optional, defaults to 600 seconds)
- SMTP configuration variables (see SMTP Configuration section)

## Google OAuth Integration

ColaNode supports optional Google OAuth authentication, currently available only on the web client.

### Configuration

To enable Google OAuth:

1. Set `ACCOUNT_GOOGLE_ENABLED` to `true`
2. Provide Google OAuth credentials:
   - `ACCOUNT_GOOGLE_CLIENT_ID`: Your Google OAuth client ID
   - `ACCOUNT_GOOGLE_CLIENT_SECRET`: Your Google OAuth client secret

### Limitations

- Currently only supported on the web client
- Desktop clients do not support Google OAuth (as of current documentation)

## Environment Variables

### Authentication Configuration

| Variable                       | Description                                                    | Default     | Required                      |
| ------------------------------ | -------------------------------------------------------------- | ----------- | ----------------------------- |
| `ACCOUNT_VERIFICATION_TYPE`    | Account verification method: `automatic`, `manual`, or `email` | `automatic` | No                            |
| `ACCOUNT_OTP_TIMEOUT`          | OTP timeout in seconds (for email verification)                | `600`       | No                            |
| `ACCOUNT_GOOGLE_ENABLED`       | Enable Google OAuth authentication                             | `false`     | No                            |
| `ACCOUNT_GOOGLE_CLIENT_ID`     | Google OAuth client ID                                         | -           | Yes (if Google OAuth enabled) |
| `ACCOUNT_GOOGLE_CLIENT_SECRET` | Google OAuth client secret                                     | -           | Yes (if Google OAuth enabled) |

### SMTP Configuration (for Email Verification)

When using email verification, the following SMTP variables must be configured:

| Variable        | Description              | Required |
| --------------- | ------------------------ | -------- |
| `SMTP_HOST`     | SMTP server hostname     | Yes      |
| `SMTP_PORT`     | SMTP server port         | Yes      |
| `SMTP_USER`     | SMTP username            | Yes      |
| `SMTP_PASSWORD` | SMTP password            | Yes      |
| `SMTP_FROM`     | Email sender address     | Yes      |
| `SMTP_SECURE`   | Use TLS/SSL (true/false) | No       |

## Database Schema

### Accounts Table

The `accounts` table stores account information:

```sql
CREATE TABLE accounts (
  id TEXT PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT,  -- NULL if using OAuth
  status INTEGER DEFAULT 0,  -- 0 = unverified, 1 = verified
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Status Values:**

- `0`: Unverified account
- `1`: Verified account (active)

### Users Table

The `users` table stores workspace-scoped user identities:

```sql
CREATE TABLE users (
  id TEXT PRIMARY KEY,
  account_id TEXT REFERENCES accounts(id),
  workspace_id TEXT REFERENCES workspaces(id),
  name TEXT,
  attributes JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(account_id, workspace_id)
);
```

## Authentication Flow

### Standard Authentication Flow

1. User registers with email and password
2. Account is created with `status = 0` (unverified)
3. Based on `ACCOUNT_VERIFICATION_TYPE`:
   - **Automatic**: Account is immediately set to `status = 1`
   - **Manual**: Admin must set `status = 1` in database
   - **Email**: Verification code sent via email, user enters code to verify
4. Once verified, user can log in
5. When user joins a workspace, a user identity is created in that workspace

### Google OAuth Flow

1. User clicks "Sign in with Google" on web client
2. Redirected to Google OAuth consent screen
3. User grants permissions
4. Google redirects back with authorization code
5. Server exchanges code for access token
6. Server retrieves user info from Google
7. Account is created/updated with Google ID
8. Account verification follows same rules based on `ACCOUNT_VERIFICATION_TYPE`

## Security Considerations

### Password Storage

- Passwords are hashed before storage (never stored in plain text)
- Hash algorithm not specified in public documentation (likely bcrypt or similar)

### Session Management

- Session management details not fully documented
- Server handles session tokens/cookies for authenticated users
- Sessions likely tied to account ID

### Workspace Isolation

- User identities are workspace-scoped
- Authorization checks ensure users can only access their workspace data
- Per-workspace user model simplifies data partitioning and auditing

## Self-Hosting Configuration

For self-hosted deployments, authentication configuration is typically set in the `hosting/docker/docker-compose.yaml` file or as environment variables in the Docker container.

### Example Configuration

```yaml
environment:
  # Account verification
  ACCOUNT_VERIFICATION_TYPE: email
  ACCOUNT_OTP_TIMEOUT: 600

  # Google OAuth (optional)
  ACCOUNT_GOOGLE_ENABLED: true
  ACCOUNT_GOOGLE_CLIENT_ID: your-client-id
  ACCOUNT_GOOGLE_CLIENT_SECRET: your-client-secret

  # SMTP (for email verification)
  SMTP_HOST: smtp.example.com
  SMTP_PORT: 587
  SMTP_USER: noreply@example.com
  SMTP_PASSWORD: your-smtp-password
  SMTP_FROM: noreply@example.com
  SMTP_SECURE: true
```

## References

- [ColaNode GitHub Repository](https://github.com/colanode/colanode)
- [ColaNode Architecture Documentation](https://colanode.com/docs/overview/architecture/)
- [ColaNode Self-Hosting Configuration](https://colanode.com/docs/self-hosting/configuration/)
