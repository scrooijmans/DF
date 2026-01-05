# Local-First Automerge Chart Architecture Migration

## Overview

This document tracks the migration of MudRock's chart management system from a PostgreSQL-primary architecture to a **local-first CRDT architecture** using Automerge-Repo.

### Goals

1. **Offline-first**: Charts work without network connectivity
2. **Low latency**: Immediate UI response, no round-trip delays
3. **Automatic persistence**: Changes persist to IndexedDB immediately
4. **Tab synchronization**: Real-time sync between browser tabs
5. **Future multi-device sync**: Foundation for WebSocket-based server sync
6. **Conflict resolution**: Automatic CRDT merging for concurrent edits

---

## Progress Tracker

### ✅ Completed

#### Phase 1: Foundation (Complete)

- [x] **Vite WASM Configuration**
  - Added `vite-plugin-wasm` and `vite-plugin-top-level-await` plugins
  - Enables Automerge's WebAssembly module to load properly
  - File: `vite.config.js`

- [x] **Package Installation**
  - `@automerge/automerge-repo@2.5.1` - Core repository management
  - `@automerge/automerge-repo-svelte-store@2.5.1` - Reactive Svelte bindings
  - `@automerge/automerge-repo-storage-indexeddb@2.5.1` - Browser persistence
  - `@automerge/automerge-repo-network-broadcastchannel@2.5.1` - Tab sync

- [x] **Chart Repo Singleton**
  - Created `src/lib/charts/repo/chart-repo.ts`
  - Lazy initialization (avoids SSR issues)
  - IndexedDB storage adapter with database name "mudrock-charts"
  - BroadcastChannel network adapter for tab-to-tab sync

- [x] **ChartStore Refactoring**
  - Updated `src/lib/charts/stores/chart-store.svelte.ts`
  - Uses `AutomergeDocumentStore<ChartSpec>` for each chart
  - `doc.change((d) => {...})` pattern for all mutations
  - Automatic persistence on every change
  - Debounced PostgreSQL backup sync (2s delay)

- [x] **ChartStoreProvider Integration**
  - Updated `src/lib/charts/components/ChartStoreProvider.svelte`
  - Sets up `automerge-repo` context via `setContextRepo()`
  - Bridges existing `PostgresChartsState` with new CRDT store
  - Bidirectional sync: ChartStore ↔ SciChart visible ranges

- [x] **Guidelines Documentation**
  - Updated `.cursor/rules/automerge.mdc` with automerge-repo patterns
  - Svelte 5 runes integration examples
  - PostgreSQL backup strategy

#### Phase 2: Settings & Visible Range Sync (Complete)

- [x] **Visible Range Persistence**
  - `updateVisibleRanges()` and `updateVisibleRangesDebounced()` methods
  - Saves zoom/pan state to Automerge document
  - Flushes on chart switch (`flushVisibleRangeUpdates()`)

- [x] **Settings Components Restyling**
  - All settings components use `shadcn-svelte` and `STYLE_CONSTANTS`
  - Components: `AxisSettings`, `RangeInput`, `SelectInput`, `ColorPicker`
  - Components: `ChartSettingsPanel`, `ScatterChartSettings`, `ModifierSettings`
  - Components: `DepthRulerSettings`, `MultiSelect`, `MapChartSettings`, `WellCorrelationSettings`
  - Custom `Slider` component using `bits-ui`

---

### 🔄 In Progress

#### Phase 3: Full Migration

- [ ] **Remove PostgresChartsState Dependency**
  - Gradually phase out `src/lib/state/postgres/postgres-charts-state.svelte.ts`
  - ChartStore becomes the single source of truth
  - Keep PostgreSQL as backup/discovery layer only

- [ ] **Automerge URL Storage in PostgreSQL**
  - Add `automerge_url` column to `charts` table
  - Tauri commands: `get_chart_automerge_url`, `set_chart_automerge_url`
  - Enables document discovery across devices

---

### 📋 Pending

#### Phase 4: Backend Integration

- [ ] **Tauri Commands for URL Management**

  ```rust
  #[tauri::command]
  async fn get_chart_automerge_url(chart_id: String) -> Result<Option<String>, String>

  #[tauri::command]
  async fn set_chart_automerge_url(chart_id: String, url: String) -> Result<(), String>

  #[tauri::command]
  async fn create_chart_with_automerge_url(spec: ChartSpec, url: String) -> Result<(), String>
  ```

- [ ] **Database Migration**
  ```sql
  ALTER TABLE charts ADD COLUMN automerge_url TEXT;
  CREATE INDEX idx_charts_automerge_url ON charts(automerge_url);
  ```

#### Phase 5: Multi-Device Sync (Future)

- [ ] **WebSocket Network Adapter**
  - Add `@automerge/automerge-repo-network-websocket`
  - Connect to sync server for multi-device synchronization
  - Remote heads gossiping for presence awareness

- [ ] **Sync Server Deployment**
  - Deploy `automerge-repo-sync-server` or custom Rust implementation
  - Handle document routing and storage

#### Phase 6: Cleanup

- [ ] **Remove Legacy Code**
  - `src/lib/state/postgres/chart-states/` directory
  - `src/lib/charts/services/chart-spec-sync-service.ts` (bridge layer)
  - Realtime subscription for chart updates (replaced by CRDT)

- [ ] **Update Documentation**
  - Remove outdated markdown guides
  - Update component documentation

---

## Package Assessment

### Currently Using

| Package                                              | Purpose                  | Status       |
| ---------------------------------------------------- | ------------------------ | ------------ |
| `@automerge/automerge-repo`                          | Core CRDT repository     | ✅ Installed |
| `@automerge/automerge-repo-svelte-store`             | Reactive Svelte bindings | ✅ Installed |
| `@automerge/automerge-repo-storage-indexeddb`        | Browser persistence      | ✅ Installed |
| `@automerge/automerge-repo-network-broadcastchannel` | Tab-to-tab sync          | ✅ Installed |

### Future Consideration

| Package                                            | Purpose                            | When to Add                 |
| -------------------------------------------------- | ---------------------------------- | --------------------------- |
| `@automerge/automerge-repo-network-websocket`      | Server sync                        | Phase 5 (multi-device)      |
| `@automerge/automerge-repo-network-messagechannel` | Efficient tab/worker communication | If performance issues arise |

### Not Needed

| Package                      | Reason                                     |
| ---------------------------- | ------------------------------------------ |
| `@automerge/vanillajs`       | Convenience bundle; we import individually |
| `@automerge/automerge-react` | React-specific; we use Svelte              |

---

## Architecture Comparison

### Before (PostgreSQL-Primary)

```
┌─────────────────────────────────────────────┐
│  Frontend                                   │
│  ┌─────────────────────────────────────┐   │
│  │  PostgresChartsState (global state)  │   │
│  │  ├── XYChartState                    │   │
│  │  ├── MapChartState                   │   │
│  │  └── WellCorrelationChartState       │   │
│  └─────────────────────────────────────┘   │
│             ↓ Realtime subscription         │
└─────────────────────────────────────────────┘
              ↓ Network round-trip
┌─────────────────────────────────────────────┐
│  PostgreSQL (primary storage)              │
│  └── charts table                          │
└─────────────────────────────────────────────┘

Issues:
- High latency (network round-trip for every change)
- Offline unusable
- Complex realtime subscription management
- Race conditions during chart creation
```

### After (Local-First CRDT)

```
┌─────────────────────────────────────────────────────┐
│  Frontend                                           │
│  ┌───────────────────────────────────────────────┐ │
│  │  Automerge Repo                                │ │
│  │  ├── IndexedDB Storage (immediate persist)     │ │
│  │  └── BroadcastChannel (tab sync)               │ │
│  └───────────────────────────────────────────────┘ │
│  ┌───────────────────────────────────────────────┐ │
│  │  ChartStore                                    │ │
│  │  ├── Map<chartId, AutomergeDocumentStore>      │ │
│  │  └── doc.change() → immediate UI + persist     │ │
│  └───────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
              ↓ Background backup (debounced 2s)
┌─────────────────────────────────────────────────────┐
│  PostgreSQL (backup + discovery)                   │
│  ├── charts table                                  │
│  └── automerge_url column                          │
└─────────────────────────────────────────────────────┘

Benefits:
- Zero latency (local-first)
- Works offline
- Automatic tab sync
- No race conditions
- CRDT conflict resolution
```

---

## Key Files

### New Files (Automerge Integration)

| File                                    | Description               |
| --------------------------------------- | ------------------------- |
| `src/lib/charts/repo/chart-repo.ts`     | Automerge Repo singleton  |
| `src/lib/charts/repo/chart-document.ts` | Document type definitions |
| `src/lib/charts/repo/index.ts`          | Module exports            |

### Modified Files

| File                                                  | Changes                         |
| ----------------------------------------------------- | ------------------------------- |
| `vite.config.js`                                      | Added WASM plugins              |
| `src/lib/charts/stores/chart-store.svelte.ts`         | Full refactor to automerge-repo |
| `src/lib/charts/components/ChartStoreProvider.svelte` | setContextRepo + bridge logic   |
| `.cursor/rules/automerge.mdc`                         | Updated guidelines              |

### Files to Remove (After Migration)

| File                                                 | Reason                  |
| ---------------------------------------------------- | ----------------------- |
| `src/lib/state/postgres/chart-states/*.svelte.ts`    | Replaced by ChartStore  |
| `src/lib/charts/services/chart-spec-sync-service.ts` | Bridge no longer needed |

---

## API Changes

### Creating a Chart

**Before:**

```typescript
const { data: chart } = await supabase.from("charts").insert(chartData);
chartsState.setSelectedChartId(chart.id); // Race condition!
```

**After:**

```typescript
const spec = createChartSpec(chartData);
await chartStore.createChart(spec); // Immediate local + IndexedDB persist
chartStore.selectChart(spec.id); // Instant, no race
```

### Updating Chart State

**Before:**

```typescript
await xyPlotState.setXVisibleRange({ min, max });
await supabase.from("charts").update({ chart_config: config });
```

**After:**

```typescript
chartStore.updateVisibleRanges(chartId, xRange, yRange);
// Persists to IndexedDB immediately
// PostgreSQL backup happens after 2s debounce
```

### Reading Chart State

**Before:**

```typescript
const chart = postgresChartsState.getSelectedChart();
const config = chart?.chart_config;
```

**After:**

```typescript
const chart = chartStore.selectedChart; // From Automerge doc
const config = chart?.config;
```

---

## Testing Checklist

### Phase 1-2 (Current)

- [x] Chart creation persists to IndexedDB
- [x] Chart visible range saves on zoom/pan
- [x] Chart state persists across page refresh
- [x] Tab A changes appear in Tab B automatically
- [x] Settings changes update ChartStore

### Phase 3-4 (Upcoming)

- [ ] Chart loads from Automerge URL stored in PostgreSQL
- [ ] New charts register URL in PostgreSQL
- [ ] Offline chart creation works
- [ ] Charts sync when coming back online

### Phase 5 (Future)

- [ ] Multi-device sync via WebSocket
- [ ] Conflict resolution on concurrent edits
- [ ] Presence awareness (who's editing what)

---

## Migration Risks & Mitigations

| Risk                          | Mitigation                                                 |
| ----------------------------- | ---------------------------------------------------------- |
| IndexedDB storage limits      | Monitor storage usage; implement cleanup for old documents |
| WASM loading failures         | Fallback to PostgreSQL-only mode if WASM fails             |
| Tab sync message storms       | BroadcastChannel is local; minimal overhead                |
| Conflict resolution surprises | Test concurrent edit scenarios thoroughly                  |

> **Note:** Data loss during this early development stage is acceptable. We can always re-create charts from scratch.

---

## Performance Expectations

| Metric                 | Before               | After              |
| ---------------------- | -------------------- | ------------------ |
| Chart creation latency | 200-500ms            | <10ms              |
| Visible range save     | 200-500ms            | <10ms              |
| Page refresh recovery  | ~1s (network fetch)  | <100ms (IndexedDB) |
| Tab sync latency       | N/A (manual refresh) | <50ms              |
| Offline capability     | None                 | Full               |

---

## How to Monitor IndexedDB Persistence

### Browser DevTools (Recommended)

1. **Open Chrome/Safari DevTools** → `Application` tab (Chrome) or `Storage` tab (Safari)
2. **Navigate to:** `IndexedDB` → `mudrock-charts` database → `documents` object store
3. **What to look for:**
   - Each document has a key array like `["automerge:...", "snapshot"]`
   - Binary data (`binary` field) contains the Automerge document state
   - Document count increases when charts are created
   - Data persists after page refresh

### Console Logging (Quick Check)

Add this to browser console to verify IndexedDB has data:

```javascript
// Check if mudrock-charts database exists and has documents
const request = indexedDB.open("mudrock-charts", 1);
request.onsuccess = (e) => {
  const db = e.target.result;
  const tx = db.transaction("documents", "readonly");
  const store = tx.objectStore("documents");
  const countReq = store.count();
  countReq.onsuccess = () => {
    console.log("📊 IndexedDB document count:", countReq.result);
  };
};
```

### Tab Sync Verification

1. Open two browser tabs with the same chart
2. In Tab A: Zoom/pan the chart
3. In Tab B: The visible range should update automatically (within ~50ms)
4. Check console for: `[ChartStoreProvider] Setup visible range listeners for chart: ...`

### What Indicates Success

| Indicator      | Expected Behavior                                                        |
| -------------- | ------------------------------------------------------------------------ |
| Console log    | `[ChartStoreProvider] Mounted with IndexedDB storage via automerge-repo` |
| IndexedDB      | `mudrock-charts` database with `documents` store                         |
| Document count | Increases when charts are created/modified                               |
| Tab sync       | Changes in one tab appear in another                                     |
| Page refresh   | Chart state preserved after refresh                                      |

### Troubleshooting

| Issue                        | Cause                            | Solution                                |
| ---------------------------- | -------------------------------- | --------------------------------------- |
| No `mudrock-charts` database | Repo not initialized             | Check `getChartRepo()` is called        |
| Empty `documents` store      | No charts created via new system | Create a chart after the migration      |
| Tab sync not working         | BroadcastChannel issue           | Check browser supports BroadcastChannel |
| WASM error                   | Vite plugins not configured      | Verify `vite-plugin-wasm` in config     |

---

## Next Steps (Priority Order)

1. **Test current implementation** - Verify IndexedDB persistence and tab sync work correctly
2. **Add PostgreSQL URL storage** - Database migration + Tauri commands
3. **Migrate chart loading** - Load from Automerge URL instead of PostgreSQL chart_config
4. **Remove bridge layer** - Phase out `chart-spec-sync-service.ts`
5. **Clean up legacy code** - Remove `chart-states/*.svelte.ts` files

---

## References

- [Automerge-Repo Documentation](https://automerge.org/docs/repositories/)
- [IndexedDB Storage Adapter Source](../../docs/libraries/js/automerge/automerge-repo/packages/automerge-repo-storage-indexeddb/src/index.ts)
- [Svelte Store Adapter Source](../../docs/libraries/js/automerge/automerge-repo/packages/automerge-repo-svelte-store/src/lib/index.ts)
- [BroadcastChannel Adapter Source](../../docs/libraries/js/automerge/automerge-repo/packages/automerge-repo-network-broadcastchannel/src/index.ts)
- [WebSocket Protocol Spec](../../docs/libraries/js/automerge/automerge-repo/packages/automerge-repo-network-websocket/README.md)
- [MudRock Automerge Guidelines](.cursor/rules/automerge.mdc)
