# Nodes Input/Output Curves: End-to-End Flow

This document describes the complete frontend-to-backend flow for displaying and managing input/output curve options for operator nodes in the DAG editor.

## Overview

When a user selects an operator node in `content-node-editor.svelte`, the system:

1. **Loads operator metadata** from `operator_registry` table
2. **Fetches available input curves** matching the operator's input requirements
3. **Displays curves** in `curve-options-AG-table.svelte` for selection
4. **Manages output curve mappings** per input curve via `output-curves.svelte`
5. **Stores mappings** in `nodes.input_to_output_curve_mapping` JSONB column

## Component Roles

### `content-node-editor.svelte`

**Role**: Main orchestrator component

- Manages tab state (Settings, Input curves, Output curves, Chart Connections, Info)
- Extracts `operator_id` from selected node's `node_config` JSONB
- Loads input curve options when operator/project changes
- Handles input curve row clicks to update `nodes.input_curve_ids`
- Passes data to child components

### `curve-options-AG-table.svelte`

**Role**: Reusable AG Grid table for displaying curve options

- Displays curves with columns: Curve ID, Well, Curve Name, Subcurve, Mnemonic, Display Name, Main Type, Units, Parquet Path
- Highlights selected curves (light green background)
- Supports multi-row selection via click
- Handles row clicks to toggle curve selection
- Used for both input and output curve selection

### `output-curves.svelte` ⭐ **ACTIVE COMPONENT**

**Role**: Per-input-curve output mapping management

- Displays each selected input curve
- For each input curve, provides:
  - Radio buttons: "Create New" (with suffix input) or "Overwrite Existing"
  - If "Overwrite Existing": Shows `CurveOptionsAGTable` filtered by input curve's well
- Manages local state for each input curve's mapping mode and selected output curve
- Saves `input_to_output_curve_mapping` to database on "Save Mappings" click
- Ensures output curves are from the same well as input curves

### `output-curve-options.svelte` ❌ **NOT USED**

**Role**: Legacy component (not imported in `content-node-editor.svelte`)

- This component exists but is not actively used
- It was replaced by `output-curves.svelte` which provides per-input-curve mapping
- Can be removed in future cleanup

## End-to-End Flow

### Phase 1: Node Selection & Operator Loading

```
User selects node in pipeline
  ↓
content-node-editor.svelte: selectedNodeId changes
  ↓
Extract operator_id from node.node_config JSONB
  ↓
Load operator metadata from operator_registry table
```

**Functions Called**:

- `getContentPipelineState()` → `currentSelectedNodeForSelectedPipelineData`
- Extract `operator_id` from `node_type` (which contains `node_config` JSONB)

### Phase 2: Input Curve Options Loading

```
Operator ID + Project ID available
  ↓
$effect() triggers loadInputCurveOptions()
  ↓
getCurveOptionsForOperator(operatorId, projectId)
  ↓
Query operator_registry for input_schema.curve_metadata_types
  ↓
Query curves_view filtered by:
  - well_id IN (project wells)
  - curve_mnemonic IN (operator's input curve metadata types)
  ↓
Transform to CurveOption[] format
  ↓
Display in CurveOptionsAGTable
```

**Functions Called**:

- `getCurveOptionsForOperator(operatorId, projectId)` in `curve-options-provider.ts`
- Supabase query: `operator_registry` → `input_schema.curve_metadata_types`
- Supabase query: `curves_view` filtered by `well_id` and `curve_mnemonic`
- Transform to `CurveOption` interface

**Type Safety**:

- ✅ Operator's `input_schema.curve_metadata_types` validated against `curve_metadata` table
- ✅ Only curves matching operator requirements are returned
- ✅ Well filtering ensures curves belong to current project

### Phase 3: Input Curve Selection

```
User clicks row in CurveOptionsAGTable (Input Curves tab)
  ↓
handleInputCurveRowClick(curveId)
  ↓
Toggle curve ID in nodes.input_curve_ids array
  ↓
Update nodes table via Supabase
  ↓
Realtime service detects change
  ↓
selectedInputCurveIds reactive variable updates
  ↓
CurveOptionsAGTable highlights selected rows (light green)
```

**Functions Called**:

- `handleInputCurveRowClick(curveId)` in `content-node-editor.svelte`
- `supabase.from('nodes').update({ input_curve_ids })`
- `realtimeNodesService` updates `currentSelectedNodeForSelectedPipelineData`

**Type Safety**:

- ✅ Curve IDs validated against `curves` table
- ✅ Only curves from current project's wells can be selected
- ✅ Multi-row selection supported (array of curve IDs)

### Phase 4: Output Curve Mapping

```
selectedInputCurveIds changes
  ↓
output-curves.svelte: $effect() triggers loadInputCurveMappings()
  ↓
For each selected input curve:
  - Check existing mapping in input_to_output_curve_mapping
  - Initialize mapping mode (create-new or overwrite-existing)
  - Load output curve options filtered by input curve's well_id
  ↓
getOutputCurveOptionsForOperatorByWell(operatorId, projectId, wellId)
  ↓
Query operator_registry for output_schema.curve_metadata_type
  ↓
Query curves_view filtered by:
  - well_id = input_curve.well_id (SAME WELL ONLY)
  - curve_mnemonic = operator's output curve metadata type
  ↓
Display in CurveOptionsAGTable (if overwrite mode)
  ↓
User selects output curve or enters suffix
  ↓
Click "Save Mappings"
  ↓
Build input_to_output_curve_mapping JSONB:
  { "input_curve_id_1": "output_curve_id_1" | null }
  ↓
Update nodes table
```

**Functions Called**:

- `loadInputCurveMappings()` in `output-curves.svelte`
- `getOutputCurveOptionsForOperatorByWell(operatorId, projectId, wellId)` in `curve-options-provider.ts`
- Supabase query: `operator_registry` → `output_schema.curve_metadata_type`
- Supabase query: `curves_view` filtered by `well_id` and `curve_mnemonic`
- `handleSave()` → `supabase.from('nodes').update({ input_to_output_curve_mapping })`

**Type Safety**:

- ✅ Output curves must match operator's `output_schema.curve_metadata_type`
- ✅ Output curves must be from the same well as input curve (enforced by filtering)
- ✅ Mapping structure validated: `{ "input_curve_id": "output_curve_id" | null }`
- ✅ Database constraint ensures curve IDs exist in `curves` table

## Type Safety Architecture

### Database Level

- **Foreign Keys**: `curves.curve_metadata_id` → `curve_metadata.id`
- **UNIQUE Constraints**: `curve_metadata.curve_mnemonic` ensures no duplicates
- **JSONB Validation**: `input_to_output_curve_mapping` keys/values validated against `curves` table

### Runtime Level (Frontend)

- **TypeScript Interfaces**: `CurveOption` interface ensures consistent data structure
- **Operator Schema Validation**: `operator_registry.input_schema/output_schema` validated against `curve_metadata`
- **Well Filtering**: Output curves filtered by input curve's well_id (same well constraint)

### UI Level

- **AG Grid Selection**: Only valid curves displayed (filtered by operator requirements)
- **Visual Feedback**: Selected curves highlighted in light green
- **Disabled States**: Output curve selection disabled until input curves selected

## Key Functions Reference

### Frontend Functions

**`curve-options-provider.ts`**:

- `getCurveOptionsForOperator(operatorId, projectId)` - Get input curve options
- `getOutputCurveOptionsForOperator(operatorId, projectId)` - Get all output curve options
- `getOutputCurveOptionsForOperatorByWell(operatorId, projectId, wellId)` - Get output curves for specific well

**`content-node-editor.svelte`**:

- `loadInputCurveOptions()` - Load input curve options when operator changes
- `handleInputCurveRowClick(curveId)` - Toggle input curve selection

**`output-curves.svelte`**:

- `loadInputCurveMappings()` - Initialize mappings for selected input curves
- `loadOutputCurveOptionsForWell(...)` - Load output curve options per input curve
- `handleSave()` - Save `input_to_output_curve_mapping` to database

### Database Queries

**Operator Metadata**:

```sql
SELECT input_schema, output_schema
FROM operator_registry
WHERE operator_id = ? AND implementation_type = 'udf' AND is_latest = true
```

**Input Curve Options**:

```sql
SELECT * FROM curves_view
WHERE well_id IN (SELECT id FROM wells WHERE project_id = ?)
  AND curve_mnemonic IN (SELECT jsonb_array_elements_text(input_schema->'curve_metadata_types'))
ORDER BY well_name, curve_name
```

**Output Curve Options (by well)**:

```sql
SELECT * FROM curves_view
WHERE well_id = ?
  AND curve_mnemonic = (SELECT output_schema->>'curve_metadata_type' FROM operator_registry WHERE operator_id = ?)
ORDER BY curve_name
```

## Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    USER INTERACTION                          │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         content-node-editor.svelte                           │
│  - Extracts operator_id from node.node_config               │
│  - Loads input curve options                                │
│  - Handles input curve selection                            │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┴───────────────────┐
        │                                       │
        ▼                                       ▼
┌──────────────────────┐          ┌──────────────────────────┐
│ Input Curves Tab     │          │ Output Curves Tab         │
│                      │          │                           │
│ CurveOptionsAGTable  │          │ output-curves.svelte      │
│ - Display curves     │          │ - Per-input mapping       │
│ - Row click → toggle │          │ - Create new / Overwrite  │
│ - Highlight selected │          │ - Well-filtered options   │
└──────────────────────┘          └──────────────────────────┘
        │                                       │
        └───────────────────┬───────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         curve-options-provider.ts                            │
│  - getCurveOptionsForOperator()                             │
│  - getOutputCurveOptionsForOperatorByWell()                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Supabase Queries                                │
│  - operator_registry (metadata)                             │
│  - curves_view (curve options)                              │
│  - nodes (update input_curve_ids, input_to_output_mapping)  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         PostgreSQL Database                                  │
│  - operator_registry table                                  │
│  - curves_view (wells + curves + curve_metadata join)      │
│  - nodes table (input_curve_ids, input_to_output_mapping)  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         Realtime Updates                                     │
│  - realtimeNodesService detects changes                     │
│  - Updates currentSelectedNodeForSelectedPipelineData       │
│  - UI reactively updates                                    │
└─────────────────────────────────────────────────────────────┘
```

## Summary

**Input Curves Flow**:

1. Operator selected → Load operator metadata → Query input curve options → Display in AG Grid → User selects → Update `nodes.input_curve_ids`

**Output Curves Flow**:

1. Input curves selected → For each input curve → Load output options (same well) → User chooses create/overwrite → Save `input_to_output_curve_mapping`

**Type Safety**:

- Database constraints ensure curve IDs exist
- Operator schemas validated against curve_metadata
- Well filtering ensures output curves match input curve's well
- Frontend TypeScript interfaces ensure type consistency

**Components**:

- ✅ `content-node-editor.svelte` - Orchestrator
- ✅ `curve-options-AG-table.svelte` - Reusable table component
- ✅ `output-curves.svelte` - Per-input-curve mapping (ACTIVE)
- ❌ `output-curve-options.svelte` - Legacy (NOT USED)
