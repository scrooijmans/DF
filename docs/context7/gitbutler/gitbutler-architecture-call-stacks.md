# GitButler Architecture and User-Driven Call Stacks

This document describes GitButler's tech stack and illustrates the most common user-driven call stacks, showing how frontend user actions relate to backend Rust processes and provide feedback to users.

## Tech Stack Overview

### Frontend
- **Framework**: React/TypeScript (inferred from Tauri patterns)
- **UI Framework**: Modern web technologies (HTML/CSS/JS)
- **Desktop Framework**: **Tauri v2** (recently upgraded from v1)
- **WebView**: System-native WebView (WebKit on macOS/Linux, WebView2 on Windows)
- **Developer Tools**: Built-in DevTools for debugging

### Backend
- **Language**: **Rust**
- **CLI Tool**: `but` (GitButler CLI)
- **Git Operations**: Direct Git library integration
- **IPC**: Tauri IPC for frontend-backend communication
- **State Management**: TOML files for configuration (`.git/gitbutler/`)

### Key Technologies
- **Tauri v2**: Desktop application framework
- **libwebkit2gtk-4.1**: WebView dependency (Linux)
- **Git**: Core Git operations via Rust Git libraries
- **TOML**: Configuration file format
- **MCP (Model Context Protocol)**: For AI integration

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    GitButler Desktop App                     │
│                      (Tauri v2)                              │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────┐      ┌──────────────────────┐    │
│  │   Frontend (React)   │◄────►│  Backend (Rust)      │    │
│  │   - UI Components    │ IPC  │  - Git Operations    │    │
│  │   - User Actions     │      │  - Virtual Branches  │    │
│  │   - State Management │      │  - Commit Management │    │
│  └──────────────────────┘      │  - Branch Stacking   │    │
│                                 └──────────────────────┘    │
│                                         │                    │
│                                         ▼                    │
│                                 ┌──────────────────────┐    │
│                                 │   Git Repository     │    │
│                                 │   - .git/            │    │
│                                 │   - .git/gitbutler/  │    │
│                                 │     - operations-log │    │
│                                 │     - virtual_branches│   │
│                                 └──────────────────────┘    │
│                                         │                    │
│                                         ▼                    │
│                                 ┌──────────────────────┐    │
│                                 │   Remote Repos       │    │
│                                 │   - GitHub           │    │
│                                 │   - GitLab           │    │
│                                 │   - Self-hosted      │    │
│                                 └──────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Common User-Driven Call Stacks

### 1. Creating a Virtual Branch

**User Action**: User clicks "Create Virtual Branch" in the UI

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 1. User clicks "Create Virtual Branch" button                │
│ 2. Component calls: invoke('create_virtual_branch', {        │
│      name: "feature-branch",                                 │
│      target: "main"                                          │
│    })                                                        │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 3. #[tauri::command]                                         │
│    fn create_virtual_branch(                                 │
│      name: String,                                           │
│      target: String                                          │
│    ) -> Result<VirtualBranch, Error>                        │
│                                                               │
│ 4. Create branch in Git repository:                          │
│    - Generate UUID for branch ID                             │
│    - Create refs/gitbutler/{branch-name}                     │
│    - Initialize branch state                                 │
│                                                               │
│ 5. Update virtual_branches.toml:                             │
│    - Add branch configuration                                │
│    - Set branch metadata (created_at, updated_at)            │
│                                                               │
│ 6. Update operations-log.toml:                               │
│    - Record CreateBranch operation                           │
│    - Create commit in operations log                         │
│                                                               │
│ 7. Return VirtualBranch struct to frontend                   │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<VirtualBranch>
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 8. Receive VirtualBranch data                                │
│ 9. Update UI state:                                          │
│    - Add branch to branch list                               │
│    - Show branch in vertical lane                            │
│    - Update branch selector                                  │
│                                                               │
│ 10. Display success feedback:                                │
│     - Show branch in UI                                      │
│     - Highlight new branch                                   │
│     - Update branch count                                    │
└─────────────────────────────────────────────────────────────┘
```

**Feedback to User**:
- ✅ New branch appears in the UI immediately
- ✅ Branch is highlighted/selected
- ✅ Branch count updates
- ✅ Branch lane appears in the virtual branches view

---

### 2. Committing Changes to a Virtual Branch

**User Action**: User stages changes and commits to a virtual branch

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 1. User selects files in staging area                        │
│ 2. User enters commit message                                │
│ 3. User clicks "Commit" button                               │
│ 4. Component calls: invoke('commit_to_virtual_branch', {     │
│      branch_id: "uuid-123",                                  │
│      message: "Add feature",                                 │
│      file_paths: ["src/file1.rs", "src/file2.rs"]           │
│    })                                                        │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 5. #[tauri::command]                                         │
│    fn commit_to_virtual_branch(                              │
│      branch_id: String,                                      │
│      message: String,                                        │
│      file_paths: Vec<String>                                 │
│    ) -> Result<Commit, Error>                               │
│                                                               │
│ 6. Load virtual branch state:                                │
│    - Read virtual_branches.toml                              │
│    - Find branch by ID                                       │
│    - Get current branch HEAD                                 │
│                                                               │
│ 7. Stage files in Git:                                       │
│    - git add {file_paths}                                    │
│    - Update Git index                                        │
│                                                               │
│ 8. Create commit:                                            │
│    - git commit -m "{message}"                               │
│    - Create commit object                                    │
│    - Update branch HEAD                                      │
│                                                               │
│ 9. Update virtual branch:                                    │
│    - Update virtual_branches.toml                            │
│    - Update branch head SHA                                  │
│    - Update updated_timestamp_ms                             │
│                                                               │
│ 10. Update operations log:                                   │
│     - Record CommitToBranch operation                        │
│     - Create commit in operations log                        │
│                                                               │
│ 11. Update workspace base (if needed):                       │
│     - UpdateWorkspaceBase operation                          │
│     - Merge virtual branches into workspace                  │
│                                                               │
│ 12. Return Commit struct to frontend                         │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<Commit>
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 13. Receive Commit data                                      │
│ 14. Update UI state:                                         │
│     - Remove staged files from staging area                  │
│     - Add commit to branch history                           │
│     - Update branch HEAD indicator                           │
│     - Clear commit message input                             │
│                                                               │
│ 15. Display success feedback:                                │
│     - Show commit in branch timeline                         │
│     - Update file tree (remove staged files)                 │
│     - Show commit SHA                                        │
│     - Display success notification                           │
└─────────────────────────────────────────────────────────────┘
```

**Feedback to User**:
- ✅ Staged files are removed from staging area
- ✅ Commit appears in branch history
- ✅ Branch HEAD indicator updates
- ✅ Success notification shown
- ✅ File tree updates to reflect committed state

---

### 3. Pushing Virtual Branch to Remote

**User Action**: User clicks "Push" button for a virtual branch

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 1. User clicks "Push" button on virtual branch               │
│ 2. Component calls: invoke('push_virtual_branch', {          │
│      branch_id: "uuid-123",                                  │
│      remote: "origin",                                       │
│      upstream_name: "feature-branch"                         │
│    })                                                        │
│ 3. Show loading indicator                                    │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 4. #[tauri::command]                                         │
│    async fn push_virtual_branch(                             │
│      branch_id: String,                                      │
│      remote: String,                                         │
│      upstream_name: String                                   │
│    ) -> Result<PushResult, Error>                           │
│                                                               │
│ 5. Load virtual branch:                                      │
│    - Read virtual_branches.toml                              │
│    - Get branch HEAD SHA                                     │
│    - Get branch ref: refs/gitbutler/{branch-name}            │
│                                                               │
│ 6. Convert virtual branch to regular branch:                 │
│    - Create refs/heads/{upstream_name}                       │
│    - Point to branch HEAD                                    │
│                                                               │
│ 7. Push to remote:                                           │
│    - git push {remote} refs/heads/{upstream_name}            │
│    - Handle authentication (SSH/HTTPS)                       │
│    - Handle network errors                                   │
│                                                               │
│ 8. Update branch upstream:                                   │
│    - Update virtual_branches.toml                            │
│    - Set upstream = "refs/remotes/{remote}/{upstream_name}"  │
│    - Set upstream_head = {remote_sha}                        │
│                                                               │
│ 9. Emit progress events (if streaming):                      │
│    - app.emit("push-progress", progress)                     │
│                                                               │
│ 10. Return PushResult to frontend                            │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<PushResult>
       │ Emit events (push-progress)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 11. Listen for progress events:                              │
│     - listen('push-progress', (event) => {                   │
│         updateProgressBar(event.payload)                     │
│       })                                                      │
│                                                               │
│ 12. Receive PushResult data                                  │
│ 13. Update UI state:                                         │
│     - Update branch upstream indicator                       │
│     - Show remote branch link                                │
│     - Update branch status (synced/unsynced)                 │
│     - Hide loading indicator                                 │
│                                                               │
│ 14. Display success feedback:                                │
│     - Show "Pushed to origin/feature-branch"                 │
│     - Display remote branch URL (if GitHub/GitLab)           │
│     - Show "Create Pull Request" button                      │
│     - Success notification                                   │
└─────────────────────────────────────────────────────────────┘
```

**Feedback to User**:
- ✅ Loading indicator during push
- ✅ Progress updates (if streaming)
- ✅ Branch upstream indicator updates
- ✅ Remote branch link appears
- ✅ "Create Pull Request" button shown (if GitHub)
- ✅ Success notification

---

### 4. Creating Stacked Branches

**User Action**: User creates a stack of dependent branches

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 1. User creates first branch "refactor"                      │
│ 2. User creates second branch "api-endpoint"                 │
│ 3. User drags "api-endpoint" above "refactor"                │
│ 4. Component detects stack relationship                      │
│ 5. Component calls: invoke('create_stacked_branch', {        │
│      base_branch_id: "refactor-uuid",                        │
│      new_branch_id: "api-endpoint-uuid",                     │
│      order: 1                                                │
│    })                                                        │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 6. #[tauri::command]                                         │
│    fn create_stacked_branch(                                 │
│      base_branch_id: String,                                 │
│      new_branch_id: String,                                  │
│      order: usize                                            │
│    ) -> Result<StackedBranch, Error>                        │
│                                                               │
│ 7. Load branch states:                                       │
│    - Read virtual_branches.toml                              │
│    - Get base branch HEAD                                    │
│    - Get new branch HEAD                                     │
│                                                               │
│ 8. Create dependency relationship:                           │
│    - Set new branch base to base branch HEAD                 │
│    - Update branch order                                     │
│    - Create branch dependency graph                          │
│                                                               │
│ 9. Update virtual_branches.toml:                             │
│    - Update branch order field                               │
│    - Set branch dependencies                                 │
│    - Update branch metadata                                  │
│                                                               │
│ 10. Update operations log:                                   │
│     - Record CreateStackedBranch operation                   │
│     - Create commit in operations log                        │
│                                                               │
│ 11. Return StackedBranch struct to frontend                  │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<StackedBranch>
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 12. Receive StackedBranch data                               │
│ 13. Update UI state:                                         │
│     - Reorder branches in stack view                         │
│     - Show dependency arrows                                 │
│     - Update branch order indicators                         │
│     - Highlight stack relationship                           │
│                                                               │
│ 14. Display success feedback:                                │
│     - Show stacked branch visualization                      │
│     - Display stack order (1, 2, 3...)                       │
│     - Show "Create Stacked PRs" option                       │
│     - Success notification                                   │
└─────────────────────────────────────────────────────────────┘
```

**Feedback to User**:
- ✅ Branches reorder in stack view
- ✅ Dependency arrows appear
- ✅ Stack order numbers displayed
- ✅ "Create Stacked PRs" button enabled
- ✅ Visual stack relationship highlighted

---

### 5. Fetching from Remote

**User Action**: User clicks "Fetch" or auto-fetch triggers

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 1. User clicks "Fetch" button OR                             │
│    Auto-fetch timer triggers (every 15 min by default)       │
│ 2. Component calls: invoke('fetch_from_remote', {            │
│      remote: "origin"                                        │
│    })                                                        │
│ 3. Show loading indicator                                    │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 4. #[tauri::command]                                         │
│    async fn fetch_from_remote(                               │
│      remote: String                                          │
│    ) -> Result<FetchResult, Error>                          │
│                                                               │
│ 5. Execute Git fetch:                                        │
│    - git fetch {remote}                                      │
│    - Handle authentication                                   │
│    - Handle network errors                                   │
│    - Download refs and objects                               │
│                                                               │
│ 6. Update remote refs:                                       │
│    - Update refs/remotes/{remote}/*                          │
│    - Update remote branch tracking                           │
│                                                               │
│ 7. Compare local vs remote:                                  │
│    - Check for new commits on remote                         │
│    - Check for diverged branches                             │
│    - Identify branches to update                             │
│                                                               │
│ 8. Update virtual branches:                                  │
│    - Update upstream_head for tracked branches               │
│    - Update virtual_branches.toml                            │
│    - Mark branches as synced/unsynced                        │
│                                                               │
│ 9. Emit progress events:                                     │
│    - app.emit("fetch-progress", progress)                    │
│    - app.emit("branch-updated", branch_info)                 │
│                                                               │
│ 10. Return FetchResult to frontend                           │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<FetchResult>
       │ Emit events (fetch-progress, branch-updated)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 11. Listen for events:                                       │
│     - listen('fetch-progress', updateProgress)               │
│     - listen('branch-updated', updateBranch)                 │
│                                                               │
│ 12. Receive FetchResult data                                 │
│ 13. Update UI state:                                         │
│     - Update branch sync status                              │
│     - Show new commits indicator                             │
│     - Update branch upstream indicators                      │
│     - Refresh branch list                                    │
│     - Hide loading indicator                                 │
│                                                               │
│ 14. Display feedback:                                        │
│     - Show "Fetched from origin" notification                │
│     - Display new commits count                              │
│     - Highlight updated branches                             │
│     - Show "Pull" button if behind                           │
└─────────────────────────────────────────────────────────────┘
```

**Feedback to User**:
- ✅ Loading indicator during fetch
- ✅ Progress updates
- ✅ Branch sync status updates
- ✅ New commits indicator
- ✅ "Pull" button appears if behind
- ✅ Success notification

---

### 6. Resolving Conflicts

**User Action**: User resolves merge conflicts in a virtual branch

```
┌─────────────┐
│   User      │
│   Action    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 1. User sees conflict indicator on branch                    │
│ 2. User clicks "Resolve Conflicts"                           │
│ 3. Component calls: invoke('get_conflicts', {                │
│      branch_id: "uuid-123"                                   │
│    })                                                        │
│ 4. Display conflict files in UI                              │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 5. #[tauri::command]                                         │
│    fn get_conflicts(                                         │
│      branch_id: String                                       │
│    ) -> Result<Vec<Conflict>, Error>                        │
│                                                               │
│ 6. Load conflict state:                                      │
│    - Read .git/gitbutler/conflicts/                          │
│    - Parse conflict files                                    │
│    - Identify conflicted sections                            │
│                                                               │
│ 7. Return conflict data to frontend                          │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<Vec<Conflict>>
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 8. Display conflict resolution UI:                           │
│    - Show conflicted files                                   │
│    - Show conflict markers                                   │
│    - Provide "Accept Ours/Theirs/Both" buttons               │
│                                                               │
│ 9. User resolves conflicts in UI                             │
│ 10. Component calls: invoke('resolve_conflict', {            │
│       branch_id: "uuid-123",                                 │
│       file_path: "src/file.rs",                              │
│       resolution: "ours" | "theirs" | "both" | "custom"      │
│     })                                                       │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Tauri IPC (invoke)
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust)                                               │
│                                                               │
│ 11. #[tauri::command]                                        │
│     fn resolve_conflict(                                     │
│       branch_id: String,                                     │
│       file_path: String,                                     │
│       resolution: ConflictResolution                         │
│     ) -> Result<(), Error>                                  │
│                                                               │
│ 12. Apply conflict resolution:                               │
│     - Remove conflict markers                                │
│     - Apply chosen resolution                                │
│     - Update file content                                    │
│                                                               │
│ 13. Stage resolved file:                                     │
│     - git add {file_path}                                    │
│     - Update Git index                                       │
│                                                               │
│ 14. Update conflict state:                                   │
│     - Remove from conflicts/                                 │
│     - Update virtual_branches.toml                           │
│                                                               │
│ 15. Check if all conflicts resolved:                         │
│     - If yes, mark branch as ready                           │
│     - If no, return remaining conflicts                      │
│                                                               │
│ 16. Return resolution result to frontend                     │
└──────┬───────────────────────────────────────────────────────┘
       │
       │ Return Result<()>
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Frontend (React/TypeScript)                                  │
│                                                               │
│ 17. Receive resolution result                                │
│ 18. Update UI state:                                         │
│     - Remove conflict indicator from file                    │
│     - Update conflict count                                  │
│     - Mark file as resolved                                  │
│     - Update branch status                                   │
│                                                               │
│ 19. Display feedback:                                        │
│     - Show "Conflict resolved" notification                  │
│     - Update conflict progress bar                           │
│     - Enable "Commit" button if all resolved                 │
│     - Show remaining conflicts count                         │
└─────────────────────────────────────────────────────────────┘
```

**Feedback to User**:
- ✅ Conflict files displayed
- ✅ Conflict markers shown
- ✅ Resolution buttons available
- ✅ File marked as resolved
- ✅ Conflict count updates
- ✅ "Commit" button enabled when all resolved
- ✅ Progress indicator

---

## Data Flow Summary

### State Storage

**Backend State (Rust)**:
- `.git/gitbutler/virtual_branches.toml` - Virtual branch configurations
- `.git/gitbutler/operations-log.toml` - Operations history
- `.git/gitbutler/conflicts/` - Conflict state
- Git refs: `refs/gitbutler/*` - Virtual branch references

**Frontend State (React)**:
- Component state (React hooks)
- UI state (selected branches, staging area, etc.)
- Cache of backend data

### Communication Pattern

1. **User Action** → Frontend component
2. **Frontend** → `invoke('command_name', args)` → Tauri IPC
3. **Backend** → Rust command handler → Git operations
4. **Backend** → Update TOML files → Update Git refs
5. **Backend** → Return result → Emit events (optional)
6. **Frontend** → Update UI state → Display feedback

### Event-Driven Updates

For long-running operations, GitButler uses Tauri events:

```rust
// Backend emits events
app.emit("push-progress", progress).unwrap();
app.emit("fetch-progress", progress).unwrap();
app.emit("branch-updated", branch_info).unwrap();
```

```typescript
// Frontend listens for events
import { listen } from '@tauri-apps/api/event';

listen('push-progress', (event) => {
  updateProgressBar(event.payload);
});
```

## Key Architectural Patterns

### 1. **Separation of Concerns**
- **Frontend**: UI, user interactions, state presentation
- **Backend**: Git operations, state persistence, business logic

### 2. **State Persistence**
- All state stored in Git repository (`.git/gitbutler/`)
- TOML files for human-readable configuration
- Git refs for branch tracking

### 3. **Offline-First**
- All operations work locally
- Remote operations are optional
- State persists in Git repository

### 4. **Event-Driven UI Updates**
- Long-running operations emit progress events
- Frontend updates UI reactively
- Real-time feedback for async operations

### 5. **Error Handling**
- Rust `Result<T, E>` types
- Frontend handles errors gracefully
- User-friendly error messages

## Performance Considerations

1. **Async Operations**: Long-running Git operations use async Rust
2. **Batch Updates**: Multiple operations batched when possible
3. **Lazy Loading**: Branch data loaded on demand
4. **Caching**: Frontend caches backend state
5. **Incremental Updates**: Only changed data sent to frontend

## Security Considerations

1. **Content Security Policy (CSP)**: Strict CSP for webview
2. **Git Authentication**: Handles SSH/HTTPS credentials securely
3. **Local State**: All state stored locally (no cloud sync of repo content)
4. **Permission Model**: Tauri capabilities restrict backend access

This architecture enables GitButler to provide a responsive, native-feeling desktop application while leveraging the power of Rust for Git operations and React for a modern UI experience.

