# Grafana Mutation Control Architecture: State Management and Immutability Patterns

## Executive Summary

Grafana implements a sophisticated mutation control system that prevents unintended side effects through **edit/view mode separation**, **copy-on-write patterns**, **version-based optimistic locking**, and **immutable version history**. The architecture ensures that dashboard state, panel configurations, and query definitions can only be mutated under controlled conditions, with clear boundaries between mutable working state and immutable persisted state.

---

## 1. Core Mutation Control Principles

### 1.1 Edit Mode vs. View Mode: State Isolation

Grafana maintains a strict separation between **View Mode** and **Edit Mode**, creating two distinct operational states:

#### View Mode (Read-Only)
- **Purpose**: Display dashboards without modification capability
- **State**: Dashboard JSON is loaded from database and rendered
- **Mutability**: No mutations allowed
- **UI**: Clean, uncluttered interface focused on visualization

#### Edit Mode (Mutable Working State)
- **Purpose**: Allow users to modify dashboard configuration
- **State**: Dashboard model is cloned into a working copy
- **Mutability**: Changes are staged in memory, not persisted
- **UI**: Edit controls, panel editors, and configuration options become available

**Key Architectural Pattern**: The dashboard exists in two parallel states:
1. **Persisted State**: Immutable version stored in database
2. **Working State**: Mutable copy in memory during edit mode

### 1.2 Copy-on-Write Pattern: `getSaveModelClone()`

Grafana uses a **copy-on-write** pattern to prevent mutations from affecting the persisted dashboard until explicitly saved.

#### Implementation Pattern

```typescript
// Dashboard model maintains original state
class DashboardModel {
  private originalDashboard: DashboardJSON;
  private workingDashboard: DashboardJSON;
  
  // Create a clone for saving - original remains untouched
  getSaveModelClone(): DashboardJSON {
    // Deep clone of working state
    return JSON.parse(JSON.stringify(this.workingDashboard));
  }
  
  // Mutations only affect working state
  updatePanel(panelId: number, changes: Partial<Panel>) {
    // Modify working copy, not original
    const panel = this.workingDashboard.panels.find(p => p.id === panelId);
    Object.assign(panel, changes);
  }
}
```

**Critical Characteristics:**
- **Original State Preserved**: The persisted dashboard JSON is never directly mutated
- **Working Copy**: All edits operate on a cloned copy in memory
- **Save Operation**: Only when user clicks "Save dashboard" is the clone serialized and persisted
- **Discard Capability**: User can discard all changes, reverting to original state

### 1.3 Immutable Version History

**All saved dashboard versions are immutable** - once created, they cannot be modified:

- **Version Creation**: Each save creates a new version entry in the database
- **Immutable Versions**: Previous versions are never overwritten or modified
- **Version Table**: Separate database table stores complete JSON snapshots
- **Restore Creates New Version**: Restoring a previous version creates a new version (e.g., restoring v5 creates v10), preserving the original v5

**Database Structure:**
```
dashboard_versions:
  - id (primary key)
  - dashboard_id (foreign key)
  - version (integer, incrementing)
  - data (JSON, complete dashboard snapshot)
  - created (timestamp)
  - created_by (user ID)
  - message (optional commit message)
```

---

## 2. Dashboard State Mutation Control

### 2.1 Dashboard Loading and State Initialization

When a dashboard is loaded:

1. **Database Query**: Dashboard JSON is retrieved from database
2. **Schema Migration**: Automatic migration to current schema version if needed
3. **Model Creation**: `DashboardModel` instance created with dashboard JSON
4. **View Mode**: Dashboard rendered in read-only view mode by default

**State at Load Time:**
```typescript
// Dashboard loaded from database
const dashboardJSON = await fetchDashboardFromDB(uid);

// Create model - original state is preserved
const dashboard = new DashboardModel(dashboardJSON);

// Dashboard is in view mode - no mutations possible
dashboard.isEditing = false;
dashboard.originalState = dashboardJSON;  // Immutable reference
dashboard.workingState = dashboardJSON;    // Initially same as original
```

### 2.2 Entering Edit Mode: Creating Working Copy

When user clicks "Edit":

1. **Mode Switch**: `isEditing` flag set to `true`
2. **Working Copy Created**: Deep clone of dashboard JSON created
3. **UI Changes**: Edit controls, panel editors become available
4. **State Tracking**: System tracks that changes are unsaved

**Edit Mode Activation:**
```typescript
enterEditMode() {
  this.isEditing = true;
  
  // Create working copy - original remains untouched
  this.workingState = this.getSaveModelClone();
  
  // Track that we have unsaved changes
  this.hasUnsavedChanges = false;  // Initially no changes
  
  // Enable edit UI
  this.enableEditControls();
}
```

### 2.3 Mutations During Edit Mode

All mutations during edit mode affect **only the working copy**:

#### Dashboard-Level Mutations
- **Title changes**: `dashboard.title = "New Title"`
- **Time range changes**: `dashboard.time.from = "now-1h"`
- **Panel additions**: `dashboard.panels.push(newPanel)`
- **Panel removals**: `dashboard.panels = dashboard.panels.filter(...)`
- **Panel reordering**: Array manipulation in `dashboard.panels`
- **Template variable changes**: Modifications to `dashboard.templating.list`

**Mutation Pattern:**
```typescript
// All mutations operate on working state
updateDashboardTitle(newTitle: string) {
  // Only working state is modified
  this.workingState.title = newTitle;
  
  // Original state remains unchanged
  // this.originalState.title is still the old value
  
  // Mark as having unsaved changes
  this.hasUnsavedChanges = true;
}
```

### 2.4 Save Operation: Persisting Changes

When user clicks "Save dashboard":

1. **Validation**: Dashboard title and other required fields validated
2. **Clone Creation**: `getSaveModelClone()` creates a clean copy
3. **Version Check**: Current dashboard version verified (optimistic locking)
4. **API Call**: Save request sent to backend with version number
5. **Version Increment**: On success, dashboard version incremented
6. **State Sync**: Working state becomes new original state

**Save Process:**
```typescript
async saveDashboard(message?: string) {
  // Validate before saving
  if (!this.workingState.title?.trim()) {
    throw new Error('Dashboard title is required');
  }
  
  // Create clean clone for serialization
  const saveModel = this.getSaveModelClone();
  
  // Prepare save options
  const options = {
    dashboard: saveModel,
    message: message,
    overwrite: this.version > 0,  // Update existing
    folderId: this.meta.folderId,
    folderUid: this.meta.folderUid,
    version: this.version,  // Current version for optimistic locking
  };
  
  // Save to backend
  const result = await dashboardAPI.saveDashboard(options);
  
  // Update model with new version
  this.version = result.version;
  this.id = result.id;
  
  // Working state becomes new original
  this.originalState = this.workingState;
  this.hasUnsavedChanges = false;
  
  return result;
}
```

**Optimistic Locking:**
- **Version Field**: Each save includes current `version` number
- **Conflict Detection**: Backend checks if version matches current database version
- **412 Precondition Failed**: If version mismatch, save fails with error:
  ```json
  {
    "message": "The dashboard has been changed by someone else",
    "status": "version-mismatch"
  }
  ```
- **User Action**: User must reload dashboard and reapply changes

### 2.5 Discard Changes: Reverting to Original

Users can discard all unsaved changes:

1. **State Reset**: Working state replaced with original state
2. **Change Tracking Reset**: `hasUnsavedChanges` set to `false`
3. **UI Update**: Dashboard reverts to last saved state

**Discard Implementation:**
```typescript
discardChanges() {
  // Replace working state with original
  this.workingState = this.getSaveModelClone(this.originalState);
  
  // Reset change tracking
  this.hasUnsavedChanges = false;
  
  // Update UI to reflect original state
  this.refreshDashboard();
}
```

### 2.6 Exit Edit Mode: State Cleanup

When exiting edit mode:

1. **Unsaved Changes Check**: If changes exist, user prompted to save or discard
2. **Mode Switch**: `isEditing` set to `false`
3. **UI Cleanup**: Edit controls hidden
4. **State Persistence**: If saved, new state persists; if discarded, original state restored

---

## 3. Panel Configuration Mutation Control

### 3.1 Panel State Management

Panels are part of the dashboard's `panels` array, and their mutation follows the same copy-on-write pattern:

#### Panel Reference Model
- **Panel ID**: Unique identifier within dashboard (not globally unique)
- **Panel JSON**: Complete panel configuration stored in dashboard JSON
- **Panel State**: Panel state is part of dashboard working state

**Panel Structure in Dashboard:**
```json
{
  "panels": [
    {
      "id": 1,
      "type": "graph",
      "title": "CPU Usage",
      "datasource": {
        "type": "prometheus",
        "uid": "kLtEtcRGk"
      },
      "targets": [
        {
          "expr": "cpu_usage_percent",
          "refId": "A"
        }
      ],
      "gridPos": { "x": 0, "y": 0, "w": 24, "h": 8 }
    }
  ]
}
```

### 3.2 Panel Editor: Isolated Editing Context

When user opens panel editor:

1. **Panel Selection**: Specific panel identified by ID
2. **Panel Clone**: Panel configuration cloned for editing
3. **Editor State**: Panel editor maintains its own working copy
4. **Changes Staged**: Modifications staged in editor, not yet in dashboard

**Panel Editor Pattern:**
```typescript
openPanelEditor(panelId: number) {
  // Find panel in working state
  const panel = this.workingState.panels.find(p => p.id === panelId);
  
  // Clone panel for editor
  const panelClone = JSON.parse(JSON.stringify(panel));
  
  // Open editor with cloned panel
  this.panelEditor.open(panelClone, {
    onSave: (updatedPanel) => {
      // Only update when user clicks "Save" in editor
      this.updatePanelInWorkingState(panelId, updatedPanel);
    },
    onDiscard: () => {
      // Discard editor changes, panel remains unchanged
      this.panelEditor.close();
    }
  });
}
```

### 3.3 Panel Editor Actions

The panel editor provides three distinct actions:

#### 1. "Back to dashboard" (Apply Changes)
- **Action**: Apply panel changes to dashboard working state
- **Persistence**: Changes NOT saved to database yet
- **State**: Dashboard remains in edit mode with unsaved changes
- **Use Case**: Continue editing other panels before saving dashboard

#### 2. "Discard panel changes"
- **Action**: Discard all changes made in panel editor
- **State**: Panel reverts to state when editor was opened
- **Dashboard State**: Dashboard working state unchanged
- **Use Case**: User made mistakes, wants to start over

#### 3. "Save dashboard"
- **Action**: Save entire dashboard (including panel changes) to database
- **Persistence**: All changes (dashboard + panels) persisted
- **State**: Dashboard exits edit mode, changes committed
- **Use Case**: User finished editing, ready to commit

**Panel Editor State Flow:**
```
Panel Editor Opened
    ↓
Panel Cloned for Editing
    ↓
User Makes Changes (staged in editor)
    ↓
User Clicks "Back to dashboard"
    ↓
Changes Applied to Dashboard Working State
    ↓
Dashboard Still in Edit Mode (unsaved)
    ↓
User Clicks "Save dashboard"
    ↓
All Changes Persisted to Database
```

### 3.4 Panel Mutations: What Can Be Changed

During panel editing, the following can be mutated:

#### Panel Configuration
- **Title**: `panel.title = "New Title"`
- **Description**: `panel.description = "Panel description"`
- **Type**: `panel.type = "stat"` (visualization type)
- **Grid Position**: `panel.gridPos = { x, y, w, h }`
- **Panel Options**: Visualization-specific settings

#### Query Definitions (Targets)
- **Query Addition**: `panel.targets.push(newQuery)`
- **Query Modification**: `panel.targets[0].expr = "new_query"`
- **Query Removal**: `panel.targets = panel.targets.filter(...)`
- **Data Source Change**: `panel.datasource.uid = "new_uid"`

#### Field Configuration
- **Field Overrides**: `panel.fieldConfig.overrides = [...]`
- **Field Options**: `panel.fieldConfig.defaults = {...}`

**All mutations are staged in working state until dashboard is saved.**

---

## 4. Query Definition Mutation Control

### 4.1 Query Structure

Queries are stored in the `targets` array of each panel:

```json
{
  "targets": [
    {
      "refId": "A",
      "expr": "cpu_usage_percent",
      "datasource": {
        "type": "prometheus",
        "uid": "kLtEtcRGk"
      }
    }
  ]
}
```

### 4.2 Query Editor: Mutation Isolation

Query editors provide isolated editing context:

1. **Query Selection**: Specific query identified by `refId`
2. **Query Clone**: Query configuration cloned for editing
3. **Editor State**: Changes staged in editor
4. **Apply to Panel**: Changes applied to panel when editor closed/saved

**Query Editor Pattern:**
```typescript
openQueryEditor(panelId: number, refId: string) {
  // Get panel from working state
  const panel = this.workingState.panels.find(p => p.id === panelId);
  
  // Get query
  const query = panel.targets.find(t => t.refId === refId);
  
  // Clone query for editing
  const queryClone = JSON.parse(JSON.stringify(query));
  
  // Open query editor
  this.queryEditor.open(queryClone, {
    onApply: (updatedQuery) => {
      // Update query in panel
      const queryIndex = panel.targets.findIndex(t => t.refId === refId);
      panel.targets[queryIndex] = updatedQuery;
      
      // Mark panel as modified
      this.hasUnsavedChanges = true;
    }
  });
}
```

### 4.3 Query Mutations: Controlled Changes

#### Query Property Mutations
- **Query Expression**: `query.expr = "new_promql_query"`
- **Data Source**: `query.datasource.uid = "new_datasource_uid"`
- **Query Options**: Type-specific query parameters
- **Reference ID**: `query.refId = "B"` (must be unique within panel)

#### Query Array Mutations
- **Add Query**: `panel.targets.push(newQuery)`
- **Remove Query**: `panel.targets = panel.targets.filter(q => q.refId !== refId)`
- **Reorder Queries**: Array manipulation

**Important**: Query mutations are **not persisted** until:
1. Panel editor is closed with "Back to dashboard" (applies to dashboard working state)
2. Dashboard is saved (persists to database)

### 4.4 Query History: Immutable Query Records

Grafana maintains a **query history** that is separate from dashboard queries:

- **Purpose**: Track queries executed in Explore mode
- **Storage**: Separate database table
- **Mutability**: Query history entries are **immutable** once created
- **Update Capability**: Only comments can be updated via PATCH API

**Query History Structure:**
```
query_history:
  - uid (primary key)
  - datasource_uid
  - queries (JSON array, immutable)
  - created_by
  - created_at
  - comment (mutable via PATCH)
  - starred (mutable)
```

**Key Point**: Query history is **not** the same as panel queries. Panel queries are part of dashboard configuration and follow dashboard mutation rules.

---

## 5. Special Cases: Provisioned Dashboards and Library Panels

### 5.1 Provisioned Dashboards: Controlled Mutability

Provisioned dashboards are managed via configuration files (Git Sync or file provisioning):

#### Mutability Modes

**1. Read-Only (`allowUiUpdates: false`)**
- **State**: Dashboard is immutable from UI
- **Save Attempt**: Shows "Cannot save provisioned dashboard" dialog
- **Mutation Control**: All UI mutations blocked
- **Update Method**: Only via provisioning source (Git/file)

**2. UI Updates Allowed (`allowUiUpdates: true`)**
- **State**: Dashboard can be edited in UI
- **Save Behavior**: Changes saved to database
- **Provisioning Overwrite**: Next provisioning sync **overwrites** UI changes
- **Warning**: Grafana ignores `version` property in provisioning file, always overwrites

**Provisioning Overwrite Pattern:**
```
Provisioned Dashboard (v5 in database)
    ↓
User Edits in UI, Saves (v6 in database)
    ↓
Provisioning Source Updated
    ↓
Grafana Syncs Provisioning
    ↓
Dashboard Overwritten (v5 from file, v6 lost)
```

**Critical Behavior**: 
- Provisioned dashboards can be **overwritten** by provisioning source
- UI changes are **not** automatically synced back to provisioning source
- Version numbers in provisioning files are **ignored** during sync

### 5.2 Library Panels: Shared Panel Mutation

Library panels are reusable panels shared across multiple dashboards:

#### Library Panel Architecture

**Storage:**
- **Library Panel Table**: Separate database table stores library panel definitions
- **Dashboard Reference**: Dashboards reference library panels by UID
- **Panel Instance**: Each dashboard has an instance of the library panel

**Mutation Control:**

**1. Library Panel Definition (Immutable Until Updated)**
- **Storage**: Stored in `library_panels` table
- **Mutability**: Can be updated, but update propagates to all instances
- **Update Process**: 
  ```
  User Edits Library Panel
      ↓
  Changes Saved to Library Panel Definition
      ↓
  All Dashboard Instances Updated Automatically
  ```

**2. Panel Instance in Dashboard (Can Be Unlinked)**
- **Reference**: Dashboard stores reference to library panel UID
- **Unlinking**: Panel can be "unlinked" from library, becoming independent
- **After Unlinking**: Panel becomes regular panel, no longer receives library updates

**Library Panel Mutation Pattern:**
```typescript
// Library panel is shared resource
const libraryPanel = {
  uid: "lib-panel-123",
  name: "CPU Usage Panel",
  model: { /* panel JSON */ }
};

// Multiple dashboards reference it
dashboard1.panels = [
  { libraryPanel: { uid: "lib-panel-123" } }
];
dashboard2.panels = [
  { libraryPanel: { uid: "lib-panel-123" } }
];

// Update library panel
updateLibraryPanel("lib-panel-123", newModel) {
  // Update library definition
  libraryPanel.model = newModel;
  
  // Propagate to all instances
  dashboards.forEach(dashboard => {
    dashboard.panels.forEach(panel => {
      if (panel.libraryPanel?.uid === "lib-panel-123") {
        panel = mergeLibraryPanel(panel, newModel);
      }
    });
  });
}
```

**Key Characteristics:**
- **Shared State**: Library panel definition is shared across dashboards
- **Propagation**: Updates to library panel automatically update all instances
- **Unlinking**: Instances can be unlinked to become independent
- **Mutation Control**: Library panel updates require dashboard save to persist

---

## 6. Mutation Control Mechanisms Summary

### 6.1 Mutable Objects (During Edit Mode)

| Object | Mutability | Scope | Persistence |
|--------|-----------|-------|-------------|
| Dashboard Working State | Mutable | Edit mode only | On "Save dashboard" |
| Panel Configurations | Mutable | Edit mode only | On "Save dashboard" |
| Query Definitions | Mutable | Edit mode only | On "Save dashboard" |
| Template Variables | Mutable | Edit mode only | On "Save dashboard" |
| Dashboard Time Range | Mutable | Edit mode only | On "Save dashboard" |

### 6.2 Immutable Objects

| Object | Immutability | Reason |
|--------|-------------|--------|
| Dashboard Versions | Immutable | Version history preservation |
| Saved Dashboard JSON | Immutable | Source of truth until new save |
| Query History Entries | Immutable | Audit trail (except comments) |
| Data Source Configurations | Immutable* | Shared resource (*can be updated, but separate from dashboards) |

### 6.3 Mutation Control Points

**1. Edit Mode Gate**
- **Control**: Dashboard must be in edit mode to mutate
- **Enforcement**: UI controls, API validation
- **Purpose**: Prevent accidental mutations

**2. Working State Isolation**
- **Control**: All mutations affect working copy, not original
- **Enforcement**: Copy-on-write pattern
- **Purpose**: Enable discard functionality

**3. Save Operation Gate**
- **Control**: Changes only persist on explicit save
- **Enforcement**: API endpoint, version checking
- **Purpose**: Prevent unintended persistence

**4. Version-Based Locking**
- **Control**: Optimistic locking via version numbers
- **Enforcement**: Backend validation
- **Purpose**: Prevent concurrent modification conflicts

**5. Provisioning Override**
- **Control**: Provisioned dashboards can be overwritten
- **Enforcement**: Provisioning sync process
- **Purpose**: Infrastructure-as-code consistency

---

## 7. Preventing Unintended Side Effects

### 7.1 Side Effect Prevention Mechanisms

#### 1. **Isolated Working State**
- **Mechanism**: Copy-on-write ensures original state never mutated
- **Prevention**: User can always discard and return to original state
- **Benefit**: No risk of corrupting saved dashboard

#### 2. **Explicit Save Required**
- **Mechanism**: Changes only persist when user clicks "Save dashboard"
- **Prevention**: Accidental changes don't persist
- **Benefit**: User has full control over when changes commit

#### 3. **Version Conflict Detection**
- **Mechanism**: Optimistic locking prevents overwriting concurrent changes
- **Prevention**: User warned if dashboard changed by another user
- **Benefit**: Prevents lost updates

#### 4. **Edit Mode Isolation**
- **Mechanism**: View mode prevents all mutations
- **Prevention**: No accidental edits in view mode
- **Benefit**: Clear separation between viewing and editing

#### 5. **Panel Editor Isolation**
- **Mechanism**: Panel changes staged in editor, not immediately in dashboard
- **Prevention**: User can discard panel changes without affecting dashboard
- **Benefit**: Granular control over what changes are applied

### 7.2 State Consistency Guarantees

**1. Database Consistency**
- **Guarantee**: Saved dashboard state is always consistent
- **Mechanism**: Atomic save operations, version increments
- **Result**: No partial saves, no corrupted states

**2. Version History Integrity**
- **Guarantee**: All versions are complete, immutable snapshots
- **Mechanism**: Full JSON stored per version
- **Result**: Any version can be restored completely

**3. Working State Consistency**
- **Guarantee**: Working state is always internally consistent
- **Mechanism**: Validation before save, structured mutations
- **Result**: Invalid states cannot be saved

---

## 8. Practical Implications

### 8.1 For Dashboard Editors

**Workflow:**
1. Click "Edit" to enter edit mode
2. Make changes (dashboard, panels, queries)
3. Review changes in working state
4. Click "Save dashboard" to persist, or "Discard" to revert
5. Exit edit mode

**Key Behaviors:**
- Changes are **not** saved automatically
- Can discard all changes at any time
- Must explicitly save to persist changes
- Version conflicts require reload and reapply

### 8.2 For Provisioned Dashboards

**Workflow:**
1. Dashboard provisioned from Git/file
2. If `allowUiUpdates: true`, can edit in UI
3. UI changes saved to database
4. Next provisioning sync may overwrite UI changes
5. UI changes not synced back to source

**Key Behaviors:**
- Provisioned dashboards can be read-only
- UI edits may be overwritten by provisioning
- Version numbers ignored during provisioning sync
- Use Git Sync for bidirectional synchronization

### 8.3 For Library Panels

**Workflow:**
1. Create library panel from regular panel
2. Library panel used in multiple dashboards
3. Edit library panel definition
4. Changes propagate to all instances
5. Instances can be unlinked to become independent

**Key Behaviors:**
- Library panel updates affect all instances
- Unlinking creates independent copy
- Library panel changes require dashboard save
- Shared state enables consistent updates

---

## 9. Summary

Grafana's mutation control architecture provides robust protection against unintended side effects through:

1. **Edit/View Mode Separation**: Clear boundaries between mutable and immutable states
2. **Copy-on-Write Pattern**: Working state isolated from persisted state
3. **Explicit Save Operations**: Changes only persist on user action
4. **Version-Based Locking**: Prevents concurrent modification conflicts
5. **Immutable Version History**: Complete audit trail with restore capability
6. **Provisioning Control**: Infrastructure-as-code with controlled mutability
7. **Library Panel Management**: Shared panels with controlled propagation

This architecture ensures that:
- **No accidental mutations** occur in view mode
- **All changes are reversible** until explicitly saved
- **Concurrent edits are detected** and prevented
- **Version history is preserved** for audit and recovery
- **Provisioned resources maintain consistency** with source of truth
- **Shared resources (library panels) update consistently** across dashboards

The system balances flexibility (allowing extensive customization) with safety (preventing unintended side effects), making Grafana both powerful and reliable for dashboard management.


