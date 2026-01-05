# EventQueue Applicability Assessment

## Overview

This document assesses where EventQueue (offline-first event queuing system) can be applied in MudRock's realtime processes. EventQueue provides:

- ✅ **Offline-first behavior**: Operations work without network connectivity
- ✅ **Batched processing**: Multiple operations combined into efficient batches
- ✅ **Retry logic**: Automatic retry with exponential backoff
- ✅ **Optimistic updates**: Immediate UI feedback without waiting for database

## Assessment Criteria

**EventQueue is applicable when:**

1. ✅ **User-initiated mutations** (CREATE, UPDATE, DELETE operations)
2. ✅ **Operations that can be queued** (not time-sensitive)
3. ✅ **Operations that benefit from batching** (multiple similar operations)
4. ✅ **Operations that need offline support** (field work scenarios)

**EventQueue is NOT applicable when:**

1. ❌ **Read-only operations** (SELECT queries)
2. ❌ **Time-sensitive operations** (real-time synchronization)
3. ❌ **Operations requiring immediate confirmation** (critical transactions)
4. ❌ **Database-driven operations** (realtime subscriptions, triggers)

---

## 1. Chart Data Source Management ✅ **APPLICABLE** (IMPLEMENTED)

**Current Implementation**: `chart-data-source-service.ts`

**Process Flow**:

```
User selects/deselects data → addDataSourcesToChart() → Direct database write
```

**EventQueue Benefits**:

- ✅ **Offline support**: Users can add/remove chart data sources without network
- ✅ **Batching**: Multiple selection changes batched into single database update
- ✅ **Performance**: Reduces database round-trips (currently one per selection change)
- ✅ **Retry logic**: Automatic retry if network fails

**Current Issues Solved**:

- ✅ **Reactive loop issues**: EventQueue decouples UI from database writes
- ✅ **Excessive database calls**: Batched processing reduces calls
- ✅ **Offline capability**: Operations queued locally, synced when online

**Implementation Status**: ✅ **COMPLETE**

- Frontend: `chart-data-source-service.ts` uses EventQueue
- Backend: `event_queue_commands.rs` processes chart_data_source events
- Integration: Fully integrated with chart data selectors

---

## 2. Chart Creation ✅ **APPLICABLE** (RECOMMENDED)

**Current Implementation**: `chart-service.ts` → `createChart()`

**Process Flow** (from `add-chart-to-project.md`):

```
User clicks chart type → createChart() → Direct database INSERT → Realtime INSERT event
```

**EventQueue Benefits**:

- ✅ **Offline chart creation**: Users can create charts without network
- ✅ **Batched creation**: Multiple charts created in single batch
- ✅ **Name validation**: Can be deferred to batch processing (more efficient)
- ✅ **Retry logic**: Automatic retry if creation fails

**Current Issues**:

- ⚠️ **Direct database dependency**: Chart creation fails without network
- ⚠️ **No offline capability**: Cannot create charts in field work scenarios
- ⚠️ **Name validation overhead**: Each creation requires database query

**Recommendation**: ✅ **IMPLEMENT**

**Implementation Plan**:

1. **Frontend**: Update `createChart()` to use EventQueue

   ```typescript
   await eventQueue.enqueue({
     type: 'add',
     entity: 'chart',
     entityId: projectId,
     payload: { chartTypeId, displayName, ... },
   });
   ```

2. **Backend**: Add `process_chart_event()` in `event_queue_commands.rs`
   - Handle name validation and uniqueness checking
   - Batch multiple chart creations
   - Return created chart IDs for optimistic UI updates

3. **Realtime**: Keep Supabase Realtime for UI updates (complementary, not replacement)

**Priority**: **HIGH** (enables offline chart creation)

---

## 3. Chart Deletion ✅ **APPLICABLE** (RECOMMENDED)

**Current Implementation**: `chart-service.ts` → `deleteChart()`

**Process Flow** (from `realtime-chart-db-sync-instantiation.md`):

```
User deletes chart → deleteChart() → Direct database DELETE → Optimistic update → Realtime DELETE event
```

**EventQueue Benefits**:

- ✅ **Offline deletion**: Users can delete charts without network
- ✅ **Batched deletion**: Multiple deletions processed in single batch
- ✅ **Retry logic**: Automatic retry if deletion fails
- ✅ **Optimistic updates**: Already implemented, EventQueue enhances it

**Current Issues**:

- ⚠️ **Direct database dependency**: Deletion fails without network
- ⚠️ **No offline capability**: Cannot delete charts in field work scenarios

**Recommendation**: ✅ **IMPLEMENT**

**Implementation Plan**:

1. **Frontend**: Update `deleteChart()` to use EventQueue

   ```typescript
   // Optimistic update (already implemented)
   chartsState.charts = chartsState.charts.filter((c) => c.id !== chartId);

   // Queue deletion event
   await eventQueue.enqueue({
     type: "remove",
     entity: "chart",
     entityId: chartId,
     payload: {},
   });
   ```

2. **Backend**: Add chart deletion processing in `event_queue_commands.rs`
   - Handle CASCADE DELETE for related records
   - Batch multiple deletions
   - Return success/failure for rollback

**Priority**: **MEDIUM** (complements existing optimistic updates)

---

## 4. Node Creation ❌ **NOT APPLICABLE** (Keep Direct Writes)

**Current Implementation**: `node-service.ts` → `createNodeFromUdfAndAddToPipeline()`

**Process Flow** (from `realtime-node-creation-add-to-active-pipeline.md`):

```
User clicks UDF → createNodeFromUdf() → Direct INSERT → addNodeToPipeline() → Direct UPDATE → Realtime events
```

**Why NOT Applicable**:

- ❌ **Complex transaction**: Requires both node INSERT and pipeline UPDATE
- ❌ **Immediate feedback needed**: Users expect node to appear immediately in visual graph
- ❌ **Position management**: Node positions need immediate persistence for visual consistency
- ❌ **Realtime synchronization**: Multiple users need immediate updates for collaborative editing

**Current Architecture is Optimal**:

- ✅ **Direct writes**: Ensures immediate consistency
- ✅ **Optimistic updates**: Already implemented via realtime subscriptions
- ✅ **Transaction safety**: Database transactions ensure atomicity

**Recommendation**: ❌ **DO NOT IMPLEMENT** (current approach is better)

---

## 5. Chart Settings Updates ✅ **APPLICABLE** (RECOMMENDED)

**Current Implementation**: Chart state classes (`xy-chart-state.svelte.ts`, etc.)

**Process Flow**:

```
User changes chart settings → setXAxisLabel() → Direct database UPDATE → Realtime UPDATE event
```

**EventQueue Benefits**:

- ✅ **Offline settings changes**: Users can modify chart settings without network
- ✅ **Batched updates**: Multiple setting changes batched into single update
- ✅ **Performance**: Reduces database round-trips (currently one per setting change)
- ✅ **Debouncing**: Natural debouncing through batching

**Current Issues**:

- ⚠️ **Excessive database calls**: Each setting change triggers immediate database write
- ⚠️ **No offline capability**: Settings changes fail without network
- ⚠️ **Race conditions**: Realtime updates can overwrite user input

**Recommendation**: ✅ **IMPLEMENT**

**Implementation Plan**:

1. **Frontend**: Update chart state classes to use EventQueue

   ```typescript
   // In xy-chart-state.svelte.ts
   setXAxisLabel(label: string): void {
     this.xAxisLabel = label; // Optimistic update

     // Queue database update
     eventQueue.enqueue({
       type: 'update',
       entity: 'chart_config',
       entityId: this.chartId,
       payload: { xAxisLabel: label },
     });
   }
   ```

2. **Backend**: Add chart_config update processing
   - Batch multiple config updates
   - Merge updates intelligently (avoid overwriting concurrent changes)
   - Return success/failure for rollback

**Priority**: **MEDIUM** (improves performance and offline capability)

---

## 6. Well/Polygon/Curve Data Updates ✅ **APPLICABLE** (FUTURE)

**Current Implementation**: Various state classes and services

**EventQueue Benefits**:

- ✅ **Offline data editing**: Users can edit well/polygon/curve data without network
- ✅ **Batched updates**: Multiple edits batched into single transaction
- ✅ **Field work support**: Critical for offline field data entry

**Recommendation**: ✅ **IMPLEMENT** (Future Phase)

**Priority**: **LOW** (not currently causing issues, but valuable for field work)

---

## 7. Realtime Subscriptions ❌ **NOT APPLICABLE** (Keep Direct)

**Current Implementation**: Supabase Realtime subscriptions

**Why NOT Applicable**:

- ❌ **Read-only operations**: Realtime subscriptions are for receiving updates, not mutations
- ❌ **Time-sensitive**: Updates must be received immediately for UI consistency
- ❌ **Database-driven**: Events originate from database, not user actions

**Recommendation**: ❌ **DO NOT IMPLEMENT** (current approach is optimal)

---

## Summary Table

| Process                          | Current Approach              | EventQueue Applicable? | Priority | Status                |
| -------------------------------- | ----------------------------- | ---------------------- | -------- | --------------------- |
| **Chart Data Source Management** | Direct database writes        | ✅ Yes                 | HIGH     | ✅ **IMPLEMENTED**    |
| **Chart Creation**               | Direct database INSERT        | ✅ Yes                 | HIGH     | ⏳ **RECOMMENDED**    |
| **Chart Deletion**               | Direct database DELETE        | ✅ Yes                 | MEDIUM   | ⏳ **RECOMMENDED**    |
| **Chart Settings Updates**       | Direct database UPDATE        | ✅ Yes                 | MEDIUM   | ⏳ **RECOMMENDED**    |
| **Node Creation**                | Direct database INSERT/UPDATE | ❌ No                  | -        | ❌ **NOT APPLICABLE** |
| **Well/Polygon/Curve Updates**   | Direct database writes        | ✅ Yes                 | LOW      | ⏳ **FUTURE**         |
| **Realtime Subscriptions**       | Supabase Realtime             | ❌ No                  | -        | ❌ **NOT APPLICABLE** |

---

## Implementation Priority

### Phase 1: ✅ **COMPLETE**

- ✅ Chart Data Source Management (add/remove data sources)

### Phase 2: **HIGH PRIORITY** (Recommended Next Steps)

1. **Chart Creation** - Enables offline chart creation
2. **Chart Deletion** - Enhances existing optimistic updates

### Phase 3: **MEDIUM PRIORITY**

3. **Chart Settings Updates** - Improves performance and offline capability

### Phase 4: **LOW PRIORITY** (Future)

4. **Well/Polygon/Curve Updates** - Field work support

---

## Key Benefits Summary

### ✅ **Benefits Achieved** (Chart Data Source Management)

1. **Offline-First**: Users can add/remove chart data sources without network
2. **Batched Processing**: Multiple selection changes processed efficiently
3. **Retry Logic**: Automatic retry with exponential backoff
4. **Performance**: Reduced database round-trips
5. **Reactive Loop Prevention**: Decouples UI from database writes

### 🎯 **Potential Benefits** (Future Implementations)

1. **Chart Creation**: Offline chart creation for field work
2. **Chart Deletion**: Enhanced offline deletion capability
3. **Chart Settings**: Batched settings updates, reduced database calls
4. **Field Work Support**: Full offline capability for data entry

---

## Architecture Notes

### **Hybrid Approach** (Current Implementation)

- **Frontend (EventQueue)**: Immediate UI updates, offline event queuing
- **Rust Backend**: Batch processing, reliable database sync
- **Supabase Realtime**: UI synchronization (complementary, not replacement)

### **When to Use EventQueue**

✅ **Use EventQueue for**:

- User-initiated mutations (CREATE, UPDATE, DELETE)
- Operations that can be queued (not time-sensitive)
- Operations that benefit from batching
- Operations that need offline support

❌ **Do NOT use EventQueue for**:

- Read-only operations (SELECT queries)
- Time-sensitive operations (real-time synchronization)
- Operations requiring immediate confirmation
- Database-driven operations (realtime subscriptions)

---

## Conclusion

EventQueue is **highly applicable** for chart-related mutations (data sources, creation, deletion, settings) but **not applicable** for node creation or realtime subscriptions. The current implementation for chart data source management demonstrates the benefits, and extending it to chart creation and deletion would provide significant value for offline field work scenarios.
