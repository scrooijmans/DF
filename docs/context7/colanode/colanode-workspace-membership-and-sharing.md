# ColaNode Workspace Membership & Data Sharing

This document summarizes how ColaNode links users to workspaces so they can share data, based on the
account–workspace–user model and the node graph schema.

It complements `colanode-sync-server-configuration.md`, which covers **where** ColaNode syncs, by
focusing on **who** can see and change workspace data.

---

## 1. Identity Model Recap

From `colanode-authentication.md` and `colanode-database-schema.md`:

- **Account** (`accounts` table)
  - Top-level identity (email/password or OAuth).
  - Can belong to multiple workspaces.

- **Workspace** (`workspaces` and `nodes` with `type='workspace'`)
  - Collaboration container (team/org).
  - Owns spaces, discussions, pages, databases, etc.

- **User (workspace-scoped)** (`users` table and `nodes` with `type='user'`)

  ```sql
  CREATE TABLE users (
    id TEXT PRIMARY KEY,
    account_id TEXT REFERENCES accounts(id),
    workspace_id TEXT REFERENCES workspaces(id),
    name TEXT,
    attributes JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (account_id, workspace_id)
  );
  ```

  - Each `(account_id, workspace_id)` pair gets a **distinct user identity**.
  - This identity is also represented as a `nodes` row with `type='user'`, allowing it to
    participate in the graph (ownership, interactions, reactions, etc.).

- **Workspace membership** (`workspace_members` table, plus node graph)
  - Ties a `user` / user node to a workspace with a role (e.g. `owner`, `admin`, `member`).
  - Used by queries like “all users in workspace”, “who reacted to this node?”, etc.

**Key idea:**  
An **account does not directly own workspace data**; instead, it owns one or more **per-workspace
user identities**, and those user identities participate in the workspace graph.

---

## 2. How Users Are Linked to Workspaces

Linking a user to a workspace typically happens in three cases:

1. **Account creates a new workspace** (becomes the owner).
2. **Account joins an existing workspace** (via invite link / admin action).
3. **Workspace is provisioned externally** (e.g. via admin tooling) and user is attached.

In all cases, the end state is:

- A `workspaces` row exists.
- A `users` row exists for `(account_id, workspace_id)`.
- A `workspace_members` row exists tying the user to the workspace with a role.
- A `nodes` row of `type='user'` exists and is associated to the workspace.

### 2.1 Database Relationships (Conceptual)

```sql
-- Accounts (global identity)
CREATE TABLE accounts (...);

-- Workspaces (containers)
CREATE TABLE workspaces (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  ...
);

-- Users (workspace-scoped identity)
CREATE TABLE users (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL REFERENCES accounts(id),
  workspace_id TEXT NOT NULL REFERENCES workspaces(id),
  name TEXT,
  attributes JSONB,
  ...
  UNIQUE (account_id, workspace_id)
);

-- Workspace membership / role
CREATE TABLE workspace_members (
  workspace_id TEXT NOT NULL REFERENCES workspaces(id),
  user_id TEXT NOT NULL REFERENCES users(id),
  role TEXT NOT NULL, -- 'owner', 'admin', 'member', 'viewer'
  joined_at TIMESTAMPTZ DEFAULT NOW(),
  PRIMARY KEY (workspace_id, user_id)
);
```

On top of this, ColaNode also uses the `nodes` table:

- `nodes` with `type='workspace'` represent workspaces in the graph.
- `nodes` with `type='user'` represent user identities in the workspace.
- Other entities (spaces, discussions, pages, database rows) attach to the workspace and/or user
  nodes as parents/owners.

---

## 3. UX Workflow: Adding a User to a Workspace

### 3.1 “Create Workspace” Flow (Owner Creates Workspace)

**User experience**

1. User signs up or logs in.
2. On first login (no workspaces), they see: “Create your first workspace”.
3. They enter a workspace name (and optional description).
4. They are taken directly into the new workspace (spaces/discussions/pages view).

**High-level call stack**

1. **Frontend** (`WorkspaceCreatePage`):
   - Validates form.
   - Calls `POST /api/workspaces` with `{ name, description? }`.

2. **API Controller** (`POST /api/workspaces`):
   - Reads `account_id` from the authenticated session.
   - Delegates to `WorkspaceService.createWorkspace(accountId, payload)`.

3. **WorkspaceService.createWorkspace(accountId, payload)`**:

   Pseudocode:

   ```ts
   async function createWorkspace(accountId: string, payload: CreateWorkspacePayload) {
     // 1. Create workspace record
     const workspaceId = ulid();
     await db.insert('workspaces', {
       id: workspaceId,
       name: payload.name,
       description: payload.description ?? null,
       owner_account_id: accountId,
       created_at: now(),
     });

     // 2. Create workspace node (for graph / hierarchy)
     await db.insert('nodes', {
       id: workspaceId,
       type: 'workspace',
       parent_id: null,
       attributes: jsonb({ name: payload.name, description: payload.description }),
     });

     // 3. Ensure a user identity exists for (account, workspace)
     const userId = ulid();
     await db.insert('users', {
       id: userId,
       account_id: accountId,
       workspace_id: workspaceId,
       name: payload.displayName ?? null,
       attributes: jsonb({ role: 'owner' }),
     });

     // 4. Create workspace_members entry
     await db.insert('workspace_members', {
       workspace_id: workspaceId,
       user_id: userId,
       role: 'owner',
       joined_at: now(),
     });

     // 5. Optionally create a 'user' node in the graph
     await db.insert('nodes', {
       id: userId,
       type: 'user',
       parent_id: workspaceId,
       attributes: jsonb({ name: payload.displayName, email: account.email }),
     });

     return { workspaceId, userId, role: 'owner' };
   }
   ```

4. **API Controller** returns workspace + membership info.
5. **Frontend**:
   - Stores `currentWorkspaceId`.
   - Navigates to `/w/:workspaceId` (workspace home).

Result: the user’s account is now linked to the workspace via a dedicated user identity and
membership entry, and all new nodes created under that workspace are visible to all members.

---

### 3.2 “Join Workspace via Invite Link” Flow

**User experience**

1. User receives an invite link (e.g. `https://app.colanode.com/invite/:token`).
2. They click the link; if not logged in, they are prompted to log in or sign up.
3. After authentication, they see a “Join workspace X?” confirmation.
4. Once confirmed, they are taken into the workspace.

**High-level call stack**

1. **Frontend**:

   - On invite page, extracts `inviteToken` from URL.
   - Calls `POST /api/workspaces/join` with `{ inviteToken }`.

2. **API Controller** (`POST /api/workspaces/join`):

   - Reads `account_id` from session.
   - Calls `WorkspaceService.joinWorkspace(accountId, inviteToken)`.

3. **WorkspaceService.joinWorkspace(accountId, inviteToken)`**:

   Pseudocode:

   ```ts
   async function joinWorkspace(accountId: string, inviteToken: string) {
     // 1. Validate invite
     const invite = await db.queryOne('SELECT * FROM workspace_invites WHERE token = ?', [inviteToken]);
     if (!invite || invite.expires_at < now()) throw new Error('Invite expired');

     const workspaceId = invite.workspace_id;

     // 2. Ensure user identity exists for this (account, workspace)
     let user = await db.queryOne(
       'SELECT * FROM users WHERE account_id = ? AND workspace_id = ?',
       [accountId, workspaceId]
     );

     if (!user) {
       const userId = ulid();
       await db.insert('users', {
         id: userId,
         account_id: accountId,
         workspace_id: workspaceId,
         name: invite.suggested_name ?? null,
         attributes: jsonb({ role: invite.default_role ?? 'member' }),
       });
       user = { id: userId, workspace_id: workspaceId, account_id: accountId };

       // Also create a user node inside the workspace
       await db.insert('nodes', {
         id: user.id,
         type: 'user',
         parent_id: workspaceId,
         attributes: jsonb({ name: invite.suggested_name, email: invite.email_hint }),
       });
     }

     // 3. Upsert workspace_members with appropriate role
     await db.upsert('workspace_members', {
       workspace_id: workspaceId,
       user_id: user.id,
       role: invite.default_role ?? 'member',
       joined_at: now(),
     });

     // 4. Optionally mark invite as used
     await db.update('workspace_invites', { used_at: now() }, { id: invite.id });

     return { workspaceId, userId: user.id, role: invite.default_role ?? 'member' };
   }
   ```

4. **API Controller** returns workspace + membership.
5. **Frontend**:
   - Adds the new workspace to the workspace list.
   - Sets `currentWorkspaceId = workspaceId`.
   - Navigates into the workspace home.

Result: the account now has a user identity and membership in the workspace; subsequent sync and
authorization checks use this membership to decide what data the user can see and edit.

---

## 4. How Shared Data is Scoped to Workspaces

Once a user is linked to a workspace:

- All collaborative content (pages, databases, discussions, etc.) is represented as `nodes` with:
  - `parent_id` pointing to the workspace node or a child space node.
  - `workspace_id` (on Yjs storage tables) referencing the workspace.
- When a user edits content:
  - Their user node ID is recorded in `node_interactions`, `node_reactions`, `yjs_operations`,
    etc., enabling:
    - “Seen by” lists.
    - Auditing who changed what.
    - Per‑workspace analytics.

Example: get all users in a workspace (`colanode-database-schema.md`):

```sql
SELECT
  u.id as user_node_id,
  u.attributes->>'name' as name,
  u.attributes->>'email' as email,
  u.attributes->>'avatar_url' as avatar_url,
  wm.role,
  wm.joined_at
FROM nodes u
INNER JOIN workspace_members wm ON u.id = wm.user_node_id
WHERE wm.workspace_id = 'workspace-id-here'
  AND u.type = 'user'
  AND u.deleted_at IS NULL
ORDER BY wm.joined_at;
```

Because everything is keyed by `workspace_id`, **sharing is automatic**: any member with a role in
that workspace can see and collaborate on nodes under that workspace, subject to role‑based
authorization rules.

---

## 5. Summary

- ColaNode links accounts to workspaces via **workspace‑scoped user identities** and
  `workspace_members` entries, not direct account → workspace links.
- Creating or joining a workspace always results in:
  - Row in `workspaces`.
  - Row in `users (account_id, workspace_id)`.
  - Row in `workspace_members (workspace_id, user_id, role)`.
  - `nodes` representation for workspace and user.
- The call stacks for **creating** and **joining** workspaces follow a clear pattern:
  - Authenticated request → service method → DB inserts/updates → frontend updates workspace
    selection and navigation.
- All collaborative data is scoped by `workspace_id`, so sharing is naturally limited to members of
  that workspace and can be audited and secured at that boundary.


