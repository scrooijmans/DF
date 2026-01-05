# Finding Your DataForge SQLite Database

## Database Location

When you register or use DataForge, all your data is stored locally in a SQLite database. Here's where to find it:

### macOS (Development & Production)

**Location**: `~/Library/Application Support/com.dataforge.app/dataforge.db`

**Full Path Example**:

```
/Users/your-username/Library/Application Support/com.dataforge.app/dataforge.db
```

### Other Files in the Same Directory

- **`dataforge.db`** - The main SQLite database (contains all your data)
- **`session.json`** - Your current session token and selected workspace

## Quick Access Commands

### Open the Directory in Finder (macOS)

```bash
open ~/Library/Application\ Support/com.dataforge.app/
```

### View Database with SQLite CLI

```bash
sqlite3 ~/Library/Application\ Support/com.dataforge.app/dataforge.db
```

### View Your Account Record

```bash
sqlite3 ~/Library/Application\ Support/com.dataforge.app/dataforge.db "SELECT id, email, name, status, created_at FROM accounts;"
```

### View All Tables

```bash
sqlite3 ~/Library/Application\ Support/com.dataforge.app/dataforge.db ".tables"
```

### View Database Schema

```bash
sqlite3 ~/Library/Application\ Support/com.dataforge.app/dataforge.db ".schema"
```

## Database Structure

After registration, your account is stored in the `accounts` table:

```sql
CREATE TABLE accounts (
    id TEXT PRIMARY KEY,           -- UUID of your account
    email TEXT UNIQUE NOT NULL,    -- Your email address
    password_hash TEXT,            -- Argon2 hashed password (not readable)
    name TEXT NOT NULL,            -- Your display name
    avatar_url TEXT,               -- Optional avatar URL
    status INTEGER NOT NULL DEFAULT 0,  -- 0=unverified, 1=verified, 2=suspended
    created_at TEXT NOT NULL,      -- Registration timestamp
    updated_at TEXT NOT NULL       -- Last update timestamp
);
```

## Useful SQL Queries

### View Your Account

```sql
SELECT id, email, name, status, created_at FROM accounts;
```

### View Your Workspaces

```sql
SELECT w.id, w.name, w.description, w.created_at
FROM workspaces w
JOIN workspace_members wm ON w.id = wm.workspace_id
JOIN accounts a ON wm.account_id = a.id
WHERE a.email = 'your-email@example.com';
```

### View Your Sessions

```sql
SELECT s.id, s.device_name, s.created_at, s.expires_at, s.last_active_at
FROM sessions s
JOIN accounts a ON s.account_id = a.id
WHERE a.email = 'your-email@example.com';
```

### View Session Token (from session.json)

```bash
cat ~/Library/Application\ Support/com.dataforge.app/session.json
```

## Using a GUI Tool

### Quick Open Command

**Open with DB Browser for SQLite:**

```bash
open -a "DB Browser for SQLite" ~/Library/Application\ Support/com.dataforge.app/dataforge.db
```

**Or use the helper script:**

```bash
./scripts/open-db.sh
```

### GUI Tools Available

1. **DB Browser for SQLite** (Free, cross-platform)
   - Download: https://sqlitebrowser.org/
   - Open command: `open -a "DB Browser for SQLite" ~/Library/Application\ Support/com.dataforge.app/dataforge.db`
   - Or drag and drop the `.db` file onto the DB Browser icon

2. **TablePlus** (macOS, paid with free tier)
   - Download: https://tableplus.com/
   - Open: `~/Library/Application Support/com.dataforge.app/dataforge.db`
   - Or use: `open -a TablePlus ~/Library/Application\ Support/com.dataforge.app/dataforge.db`

3. **VS Code Extension**
   - Install: "SQLite Viewer" extension
   - Open the `.db` file directly in VS Code

## Important Notes

⚠️ **Security**:

- The database contains your password hash (not the actual password)
- Session tokens are stored in `session.json` (plain text for local use)
- Never share these files publicly

⚠️ **Development vs Production**:

- In development mode (`tauri dev`), the database is created when you first register
- The location is the same for both dev and production builds
- If you delete the database, you'll need to register again

⚠️ **Backup**:

- To backup your data, simply copy the entire `com.dataforge.app` directory
- To restore, copy it back to the same location

## Troubleshooting

### Database Not Found?

1. **Check if the app has run**: The database is only created after first launch
2. **Check the exact path**: Make sure you're using the correct app identifier
3. **Check permissions**: Ensure you have read access to `~/Library/Application Support/`

### View Logs to Find Database Path

When the app starts, it logs the data directory. Check the terminal where you ran `tauri dev`:

```
🚀 Initializing DataForge with data directory: /Users/your-username/Library/Application Support/com.dataforge.app
```

This will show you the exact path being used.
