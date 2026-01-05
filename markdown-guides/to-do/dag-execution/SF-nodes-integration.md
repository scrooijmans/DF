# Svelte Flow Nodes Integration Plan - Type-Safe DAG Editor

## 🎯 Overview

This document outlines the plan for implementing type-safe node operations in the Svelte Flow DAG editor, ensuring schema compatibility validation at the UI level and preventing invalid connections between nodes.

**Status**: Planning Phase  
**Last Updated**: 2025-10-30

---

## 📋 Table of Contents

1. [Node Type Architecture](#node-type-architecture)
2. [Node Structure & Contents](#node-structure--contents)
3. [Schema Tracking & Validation](#schema-tracking--validation)
4. [Connection Validation Strategy](#connection-validation-strategy)
5. [Best Practices from Other DAG Software](#best-practices-from-other-dag-software)
6. [Implementation Plan](#implementation-plan)
7. [Type Safety Layers](#type-safety-layers)

---

## 🏗️ Node Type Architecture

### Decision: Three Distinct Node Types ✅

**Answer**: Yes, we should have **separate node types** for Ingestion, Operator, and Output.

**Rationale**:

- **Clear separation of concerns**: Ingestion nodes fetch data, Operator nodes transform, Output nodes export
- **Different connection rules**: Ingestion nodes have no inputs, Output nodes have no outputs
- **Better UX**: Users can visually distinguish data sources, transformations, and destinations
- **Matches backend**: Our Rust `NodeType` enum already defines these three types

### Node Type Definitions

```typescript
// From dag-types.ts - already defined ✅

type NodeType =
  | {
      type: "ingestion";
      source_type: IngestionSourceType; // well_logs, surface, well_markers, postgres_table
      filters: IngestionFilters;
      output_schema: SchemaDefinition; // ✅ Always has output schema
    }
  | {
      type: "operator";
      operator_id: string;
      operator_version: string;
      parameters: Record<string, any>;
      input_schema?: SchemaDefinition; // ✅ Optional (can be looked up from registry)
      output_schema?: SchemaDefinition; // ✅ Optional (can be looked up from registry)
      version_strategy?: VersionStrategy;
    }
  | {
      type: "output";
      output_type: OutputType; // 'frontend' | 'storage' | 'api'
      input_schema: SchemaDefinition; // ✅ Always has input schema
    };
```

**Key Points**:

- ✅ **Ingestion nodes** have `source_type` that references data sources (wells, surfaces, tables)
- ✅ **Operator nodes** reference operators via `operator_id` + `operator_version`
- ✅ **Output nodes** specify where results go (`frontend`, `storage`, `api`)
- ✅ **All nodes** have schema definitions for type safety

---

## 📦 Node Structure & Contents

### What a Node Should Contain

Based on our current `DagNode` structure and best practices:

```typescript
type DagNode = {
  // Core Identity
  id: string; // UUID - unique node identifier
  name: string; // Human-readable name (e.g., "Ingest Well Logs")

  // Node Type & Configuration
  node_type: NodeType; // Ingestion | Operator | Output (see above)

  // Graph Structure
  dependencies: string[]; // Array of upstream node IDs (sources)
  // Empty for Ingestion nodes, non-empty for Operator/Output

  // Visual Metadata
  metadata: {
    position?: [number, number]; // [x, y] coordinates for Svelte Flow
    cache_enabled?: boolean; // Whether to cache this node's output
    timeout_seconds?: number; // Execution timeout
    tags?: string[]; // Categorization tags
    description?: string; // User notes
  };
};
```

### Ingestion Data References ✅

**Question**: Does our schema handle ingestion data references?

**Answer**: ✅ **Yes**, via `source_type` in Ingestion nodes:

```typescript
type IngestionSourceType =
  | {
      source: "well_logs";
      project_id: string; // ✅ References project
      well_ids: string[]; // ✅ References specific wells
      curve_names: string[]; // ✅ References specific curves
      depth_range?: [number, number];
    }
  | {
      source: "surface";
      project_id: string;
      surface_name: string; // ✅ References surface by name
    }
  | {
      source: "well_markers";
      project_id: string;
      well_ids: string[]; // ✅ References wells
    }
  | {
      source: "postgres_table";
      table_name: string; // ✅ References table
      filters?: Record<string, any>;
    };
```

**Database Schema Support** (`02-pipeline-schema.sql`):

- ✅ `pipeline_nodes.node_config` (JSONB) stores the full `NodeType` enum
- ✅ `pipeline_nodes.input_schema` and `output_schema` (JSONB) store `SchemaDefinition`
- ✅ Ingestion nodes store `source_type` in `node_config`, which includes all data references

**Conclusion**: ✅ **No schema changes needed** - current design fully supports ingestion data references.

---

## 🔍 Schema Tracking & Validation

### Current State: ✅ Already Implemented

**Backend** (`operator_registry` table):

```sql
CREATE TABLE operator_registry (
    ...
    input_schema JSONB NOT NULL,   -- ✅ Required input SchemaDefinition
    output_schema JSONB NOT NULL,  -- ✅ Expected output SchemaDefinition
    parameters_schema JSONB NOT NULL, -- ✅ JSON Schema for parameters
    ...
);
```

**Rust Core** (`crates/dags/core/src/schema.rs`):

```rust
impl SchemaDefinition {
    /// Check if this schema is compatible with another (for node connections)
    pub fn validate_compatible(
        &self,        // Source schema (output of upstream node)
        other: &SchemaDefinition,  // Target schema (input of downstream node)
    ) -> Result<(), SchemaValidationError> {
        // Rule 1: All required columns in 'other' must exist in 'self'
        // Rule 2: Column types should match (when arrow_schema_json is parsed)
        ...
    }
}
```

**Frontend** (`dag-types.ts`):

```typescript
export type SchemaDefinition = {
  schema_id: string; // e.g., "well_logs_standard"
  arrow_schema_json: string; // Arrow schema JSON
  required_columns: string[]; // Must be present
  optional_columns: string[]; // May be present
};
```

### Schema Lookup Flow

**For Operator Nodes**:

1. **Reference-based** (preferred): `input_schema` and `output_schema` are `undefined` → Lookup from `operator_registry` using `operator_id` + `operator_version`
2. **Inline** (backward compatibility): Schemas stored directly in `node_type` → Use inline schemas

**For Ingestion Nodes**:

- Always have `output_schema` (inferred from selected wells/curves)

**For Output Nodes**:

- Always have `input_schema` (matches upstream node's output)

---

## 🔗 Connection Validation Strategy

### Connection Rules Matrix

| Source Node Type | Target Node Type | Valid? | Validation Rule                                                    |
| ---------------- | ---------------- | ------ | ------------------------------------------------------------------ |
| **Ingestion**    | **Operator**     | ✅ Yes | `ingestion.output_schema` compatible with `operator.input_schema`  |
| **Ingestion**    | **Output**       | ✅ Yes | `ingestion.output_schema` compatible with `output.input_schema`    |
| **Operator**     | **Operator**     | ✅ Yes | `operator1.output_schema` compatible with `operator2.input_schema` |
| **Operator**     | **Output**       | ✅ Yes | `operator.output_schema` compatible with `output.input_schema`     |
| **Output**       | **Any**          | ❌ No  | Output nodes cannot have outgoing connections                      |
| **Any**          | **Ingestion**    | ❌ No  | Ingestion nodes cannot have incoming connections                   |
| **Ingestion**    | **Ingestion**    | ❌ No  | Ingestion nodes cannot connect to each other                       |

### Svelte Flow `isValidConnection` Implementation

**File**: `src/lib/components/SF/node-connection-validator.ts` (NEW)

```typescript
import type { Connection } from "@xyflow/svelte";
import type { DagNode, SchemaDefinition } from "$lib/services/dag-types";

/**
 * Node connection validator for Svelte Flow
 * Validates schema compatibility before allowing connections
 */
export class NodeConnectionValidator {
  private nodes: Map<string, DagNode> = new Map();

  /**
   * Update the node registry (called when DAG loads/changes)
   */
  updateNodes(nodes: DagNode[]): void {
    this.nodes.clear();
    nodes.forEach((node) => {
      this.nodes.set(node.id, node);
    });
  }

  /**
   * Get output schema from a source node
   */
  private getOutputSchema(nodeId: string): SchemaDefinition | null {
    const node = this.nodes.get(nodeId);
    if (!node) return null;

    switch (node.node_type.type) {
      case "ingestion":
        return node.node_type.output_schema;
      case "operator":
        // If inline schema exists, use it; otherwise lookup from registry
        return node.node_type.output_schema ?? null; // TODO: Lookup from registry
      case "output":
        return null; // Output nodes have no output schema
    }
  }

  /**
   * Get input schema from a target node
   */
  private getInputSchema(nodeId: string): SchemaDefinition | null {
    const node = this.nodes.get(nodeId);
    if (!node) return null;

    switch (node.node_type.type) {
      case "ingestion":
        return null; // Ingestion nodes have no input schema
      case "operator":
        return node.node_type.input_schema ?? null; // TODO: Lookup from registry
      case "output":
        return node.node_type.input_schema;
    }
  }

  /**
   * Check if two schemas are compatible
   * Matches backend validate_compatible() logic
   */
  private isSchemaCompatible(
    sourceSchema: SchemaDefinition,
    targetSchema: SchemaDefinition,
  ): boolean {
    // Rule 1: All required columns in target must exist in source
    const sourceAllColumns = new Set([
      ...sourceSchema.required_columns,
      ...sourceSchema.optional_columns,
    ]);

    for (const requiredCol of targetSchema.required_columns) {
      if (!sourceAllColumns.has(requiredCol)) {
        return false; // Missing required column
      }
    }

    // Rule 2: Schema IDs should match (if both have schema_id)
    // This is a heuristic - full type checking happens at runtime
    if (sourceSchema.schema_id && targetSchema.schema_id) {
      if (sourceSchema.schema_id === targetSchema.schema_id) {
        return true; // Same schema ID = compatible
      }
      // Different schema IDs might still be compatible if columns match
      // (e.g., "well_logs_standard" vs "well_logs_extended")
    }

    return true; // Column check passed
  }

  /**
   * Validate a connection between two nodes
   * This is the main function used by Svelte Flow's isValidConnection prop
   */
  isValidConnection(connection: Connection): boolean {
    const { source, target } = connection;

    // Get node types
    const sourceNode = this.nodes.get(source);
    const targetNode = this.nodes.get(target);

    if (!sourceNode || !targetNode) {
      console.warn("[NodeConnectionValidator] Node not found:", {
        source,
        target,
      });
      return false;
    }

    // Rule 1: Output nodes cannot have outgoing connections
    if (sourceNode.node_type.type === "output") {
      return false;
    }

    // Rule 2: Ingestion nodes cannot have incoming connections
    if (targetNode.node_type.type === "ingestion") {
      return false;
    }

    // Rule 3: Ingestion nodes cannot connect to each other
    if (
      sourceNode.node_type.type === "ingestion" &&
      targetNode.node_type.type === "ingestion"
    ) {
      return false;
    }

    // Rule 4: Schema compatibility check
    const sourceOutputSchema = this.getOutputSchema(source);
    const targetInputSchema = this.getInputSchema(target);

    if (!sourceOutputSchema || !targetInputSchema) {
      // If schemas are missing, allow connection but warn
      // (schemas might be looked up from registry later)
      console.warn("[NodeConnectionValidator] Missing schema:", {
        source: sourceOutputSchema ? "ok" : "missing",
        target: targetInputSchema ? "ok" : "missing",
      });
      return true; // Allow connection, backend will validate
    }

    return this.isSchemaCompatible(sourceOutputSchema, targetInputSchema);
  }

  /**
   * Get validation error message (for UI feedback)
   */
  getValidationError(connection: Connection): string | null {
    if (this.isValidConnection(connection)) {
      return null;
    }

    const sourceNode = this.nodes.get(connection.source);
    const targetNode = this.nodes.get(connection.target);

    if (!sourceNode || !targetNode) return "Unknown validation error";

    // Provide specific error messages
    if (sourceNode.node_type.type === "output") {
      return "Output nodes cannot have outgoing connections";
    }
    if (targetNode.node_type.type === "ingestion") {
      return "Ingestion nodes cannot have incoming connections";
    }

    const sourceOutputSchema = this.getOutputSchema(connection.source);
    const targetInputSchema = this.getInputSchema(connection.target);

    if (!sourceOutputSchema || !targetInputSchema) {
      return "Missing schema information";
    }

    // Find missing columns
    const sourceAllColumns = new Set([
      ...sourceOutputSchema.required_columns,
      ...sourceOutputSchema.optional_columns,
    ]);
    const missingColumns = targetInputSchema.required_columns.filter(
      (col) => !sourceAllColumns.has(col),
    );

    if (missingColumns.length > 0) {
      return `Missing required columns: ${missingColumns.join(", ")}`;
    }

    return "Schema incompatibility";
  }
}

// Singleton instance
export const nodeConnectionValidator = new NodeConnectionValidator();
```

### Integration with `SF-pipeline-flow.svelte`

```typescript
// In SF-pipeline-flow.svelte

<script lang="ts">
  import { nodeConnectionValidator } from './node-connection-validator';
  import type { DagNode } from '$lib/services/dag-types';
  import type { IsValidConnection } from '@xyflow/svelte';

  // ... existing code ...

  // Update validator when nodes change
  $effect(() => {
    // Convert SvelteFlowNode[] back to DagNode[] for validation
    // (We need to store the original DagNode[] in content-dag-editor.svelte)
    // For now, we'll pass nodes as a prop or use a global state
  });

  // Create isValidConnection function
  const isValidConnection: IsValidConnection = (connection) => {
    return nodeConnectionValidator.isValidConnection(connection);
  };
</script>

<SvelteFlow
  nodes={nodes}
  edges={edges}
  {isValidConnection}
  ...
/>
```

---

## 🌟 Best Practices from Other DAG Software

### Apache Airflow

**Node Types**:

- **Operators**: Task nodes (PythonOperator, BashOperator, etc.)
- **Sensors**: Wait for external conditions (similar to our Ingestion nodes)
- **XComs**: Data passing between tasks (similar to our schema validation)

**Connection Rules**:

- ✅ Tasks can connect to tasks
- ✅ Sensors can connect to tasks
- ❌ Tasks cannot connect to sensors (sensors are sources)

**Type Safety**:

- Uses Python type hints for task inputs/outputs
- XCom keys for data passing (less strict than our schema approach)

**Takeaway**: ✅ Our three-node-type approach aligns with Airflow's Operator/Sensor distinction.

### Node-RED

**Node Types**:

- **Input nodes**: Inject, MQTT In, HTTP In (similar to Ingestion)
- **Function nodes**: Transform data (similar to Operator)
- **Output nodes**: Debug, MQTT Out, HTTP Out (similar to Output)

**Connection Rules**:

- ✅ Input → Function → Output
- ✅ Function → Function (chaining)
- Uses `msg` object structure (less strict than our schema validation)

**Type Safety**:

- Runtime validation only
- No compile-time schema checking

**Takeaway**: ✅ Our schema-based validation is more robust than Node-RED's approach.

### Prefect

**Node Types**:

- **Tasks**: Atomic units of work (similar to our Operator nodes)
- **Flows**: Orchestration containers
- **Parameters**: Inputs to flows (similar to our Ingestion configuration)

**Type Safety**:

- Uses Pydantic models for task inputs/outputs
- Type validation at runtime
- Schema serialization for caching

**Takeaway**: ✅ Our `SchemaDefinition` approach is similar to Prefect's Pydantic models.

### Dagster

**Node Types**:

- **Assets**: Data assets (inputs/outputs)
- **Ops**: Operations that produce assets
- **Graphs**: Compositions of ops

**Type Safety**:

- **Dagster Types**: Type system for assets (similar to our `SchemaDefinition`)
- **IOManagers**: Handle serialization/deserialization
- **Metadata**: Rich metadata attached to assets

**Takeaway**: ✅ Our `SchemaDefinition` + `operator_registry` approach aligns with Dagster's asset type system.

### Summary: Our Approach vs Others

| Feature               | MudRock                       | Airflow                | Node-RED                  | Prefect               | Dagster                   |
| --------------------- | ----------------------------- | ---------------------- | ------------------------- | --------------------- | ------------------------- |
| **Node Types**        | 3 (Ingestion/Operator/Output) | 2 (Sensor/Operator)    | 3 (Input/Function/Output) | 1 (Task)              | 2 (Asset/Op)              |
| **Schema Validation** | ✅ Compile-time + Runtime     | ❌ Runtime only        | ❌ None                   | ✅ Runtime (Pydantic) | ✅ Compile-time (Types)   |
| **Connection Rules**  | ✅ Schema-based               | ✅ Task dependencies   | ✅ Flow-based             | ✅ Task dependencies  | ✅ Asset dependencies     |
| **Type Safety**       | ✅ Strong (Arrow schemas)     | ⚠️ Weak (Python types) | ❌ None                   | ✅ Strong (Pydantic)  | ✅ Strong (Dagster Types) |

**Conclusion**: ✅ Our approach is **more type-safe** than Airflow/Node-RED and **comparable** to Prefect/Dagster.

---

## 🛠️ Implementation Plan

### Phase 1: Node Connection Validator ✅ **PRIORITY**

**Files to Create**:

1. `src/lib/components/SF/node-connection-validator.ts` - Connection validation logic
2. `src/lib/services/operator-schema-service.ts` - Operator schema lookup from registry

**Tasks**:

- [ ] Create `NodeConnectionValidator` class
- [ ] Implement `isValidConnection()` method
- [ ] Implement `getValidationError()` for user feedback
- [ ] Add operator schema lookup (from `operator_registry` table or Tauri command)
- [ ] Unit tests for schema compatibility logic

**Integration**:

- [ ] Wire `isValidConnection` to `SF-pipeline-flow.svelte`
- [ ] Update `nodeConnectionValidator.updateNodes()` when DAG loads/changes
- [ ] Visual feedback: Green edge = valid, Red edge = invalid

### Phase 2: Custom Node Types & Styling

**Files to Create**:

1. `src/lib/components/SF/nodes/ingestion-node.svelte` - Custom Ingestion node component
2. `src/lib/components/SF/nodes/operator-node.svelte` - Custom Operator node component
3. `src/lib/components/SF/nodes/output-node.svelte` - Custom Output node component

**Tasks**:

- [ ] Create custom node components with distinct styling:
  - **Ingestion**: Blue background, "database" icon
  - **Operator**: Green background, "gear" icon
  - **Output**: Orange background, "export" icon
- [ ] Add node type badges/labels
- [ ] Show schema info on hover (required columns, schema_id)
- [ ] Register custom node types in `SF-pipeline-flow.svelte`

**Svelte Flow Integration**:

```typescript
// In SF-pipeline-flow.svelte
import IngestionNode from './nodes/ingestion-node.svelte';
import OperatorNode from './nodes/operator-node.svelte';
import OutputNode from './nodes/output-node.svelte';

const nodeTypes = {
  ingestion: IngestionNode,
  operator: OperatorNode,
  output: OutputNode,
};

<SvelteFlow
  nodeTypes={nodeTypes}
  ...
/>
```

### Phase 3: Operator Schema Lookup Service

**Files to Create**:

1. `src/lib/services/operator-schema-service.ts` - Frontend service for operator schema lookup

**Tasks**:

- [ ] Create `getOperatorSchema(operatorId, version)` function
- [ ] Fetch from Supabase `operator_registry` table OR call Tauri `get_operator_schema` command
- [ ] Cache operator schemas to avoid repeated lookups
- [ ] Handle missing schemas gracefully (fallback to inline schemas)

**Backend Support** (if needed):

- [ ] Add Tauri command `get_operator_schema(operator_id, version)` → Returns `{ input_schema, output_schema }`
- [ ] Or use existing `get_available_operators` and filter by `operator_id` + `version`

### Phase 4: Enhanced Connection Validation UI

**Tasks**:

- [ ] Show validation errors in tooltip when hovering over invalid connection
- [ ] Disable connection handles for incompatible nodes
- [ ] Visual indicators:
  - ✅ Green handle = compatible
  - ❌ Red handle = incompatible
  - ⚠️ Yellow handle = schema missing (will be validated at runtime)
- [ ] Connection preview: Show schema compatibility before completing connection

**Svelte Flow Features**:

- Use `onconnectstart` to show preview
- Use `onbeforeconnect` to validate before creating edge
- Use Handle `isValidConnection` prop for per-handle validation

### Phase 5: Schema Compatibility Helper

**Files to Create**:

1. `src/lib/services/schema-compatibility.ts` - Schema compatibility utilities

**Tasks**:

- [ ] Implement `validateSchemaCompatible()` matching Rust `validate_compatible()`
- [ ] Implement `getMissingColumns()` to list incompatible columns
- [ ] Implement `suggestCompatibleOperators()` - Given an output schema, suggest operators that accept it
- [ ] Add schema merging logic (for multiple inputs to one operator)

---

## 🔒 Type Safety Layers

### Layer 1: Frontend UI Validation (Svelte Flow)

**Location**: `src/lib/components/SF/node-connection-validator.ts`

**Purpose**: Prevent invalid connections at the UI level

**Implementation**:

- `isValidConnection()` prop on `<SvelteFlow>`
- Validates schema compatibility before allowing edge creation
- Visual feedback (red/green edges)

**Benefits**:

- ✅ Immediate user feedback
- ✅ Prevents invalid DAGs from being created
- ✅ Reduces backend validation load

### Layer 2: Frontend Save Validation

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor.svelte`

**Purpose**: Validate entire DAG before saving

**Implementation**:

- Call `validateSchemaCompatibility()` on all edges before `savePipeline()`
- Show validation errors in UI
- Disable "Save" button if validation fails

**Benefits**:

- ✅ Catches edge cases missed by `isValidConnection`
- ✅ Validates entire DAG structure (not just individual connections)
- ✅ Prevents saving invalid DAGs

### Layer 3: Backend Validation (Rust)

**Location**: `crates/dags/validator/src/lib.rs`

**Purpose**: Final validation before execution

**Implementation**:

- `validate_structure()` - Checks cycles, orphans
- `validate_types()` - Validates all node connections are schema-compatible (TODO)
- Called in `save_pipeline` Tauri command

**Benefits**:

- ✅ Server-side validation (cannot be bypassed)
- ✅ Ensures data integrity in database
- ✅ Prevents execution of invalid DAGs

### Layer 4: Runtime Validation (Execution)

**Location**: `crates/dags/executor/src/executor.rs`

**Purpose**: Validate schemas at execution time

**Implementation**:

- `execute_node()` validates input schema matches expected schema
- Uses Arrow schema validation
- Throws error if schema mismatch

**Benefits**:

- ✅ Catches schema mismatches that passed compile-time checks
- ✅ Handles dynamic schemas (e.g., Parquet files with varying columns)
- ✅ Provides detailed error messages for debugging

---

## 📊 Schema Compatibility Rules

### Rule 1: Required Columns

**Source schema** must contain **all required columns** from target schema.

```typescript
// Source (Ingestion output)
{
  required_columns: ['well_id', 'depth'],
  optional_columns: ['gr', 'rhob']
}

// Target (Operator input) - VALID ✅
{
  required_columns: ['well_id', 'depth', 'gr'],  // 'gr' exists in optional_columns
  optional_columns: []
}

// Target (Operator input) - INVALID ❌
{
  required_columns: ['well_id', 'depth', 'sonic'],  // 'sonic' missing
  optional_columns: []
}
```

### Rule 2: Schema ID Matching

**Heuristic**: If both schemas have `schema_id`, matching IDs indicate compatibility.

```typescript
// Same schema ID = likely compatible ✅
source: { schema_id: 'well_logs_standard', ... }
target: { schema_id: 'well_logs_standard', ... }

// Different schema IDs = check columns
source: { schema_id: 'well_logs_standard', ... }
target: { schema_id: 'well_logs_extended', ... }  // May still be compatible if columns match
```

### Rule 3: Column Type Matching (Future)

**TODO**: Parse `arrow_schema_json` and validate column types match.

```typescript
// Current: Only checks column names
// Future: Check types too
source: {
  columns: [{ name: "depth", type: "Float64" }];
}
target: {
  columns: [{ name: "depth", type: "Int64" }];
} // Type mismatch ❌
```

---

## 🎨 Custom Node Components

### Ingestion Node

**Visual Design**:

- **Color**: Blue (`bg-blue-500`)
- **Icon**: Database/Import icon (Lucide)
- **Label**: Node name + "(Ingestion)"
- **Badge**: Source type (e.g., "Well Logs", "Surface")

**Handles**:

- ✅ **Source handle** (right): Can connect to Operator/Output nodes
- ❌ **Target handle** (left): None (Ingestion nodes have no inputs)

**Configuration Panel** (on double-click):

- Source type selector (well_logs, surface, well_markers, postgres_table)
- Well/curve selectors (if well_logs)
- Depth range picker
- Schema preview (output_schema)

### Operator Node

**Visual Design**:

- **Color**: Green (`bg-green-500`)
- **Icon**: Gear/Cog icon (Lucide)
- **Label**: Node name + operator name (e.g., "VShale (Larionov)")
- **Badge**: Category (e.g., "Subsurface", "Signal")

**Handles**:

- ✅ **Source handle** (right): Can connect to Operator/Output nodes
- ✅ **Target handle** (left): Can receive from Ingestion/Operator nodes

**Configuration Panel** (on double-click):

- Operator selector (from `operator_registry`)
- Parameter form (generated from `parameters_schema` JSON Schema)
- Schema preview (input_schema, output_schema)
- Version strategy selector (Pinned/Latest/Compatible)

### Output Node

**Visual Design**:

- **Color**: Orange (`bg-orange-500`)
- **Icon**: Export/Download icon (Lucide)
- **Label**: Node name + output type (e.g., "Frontend Output")
- **Badge**: Output type ("Frontend", "Storage", "API")

**Handles**:

- ❌ **Source handle** (right): None (Output nodes have no outputs)
- ✅ **Target handle** (left): Can receive from Ingestion/Operator nodes

**Configuration Panel** (on double-click):

- Output type selector (frontend, storage, api)
- Storage path (if storage type)
- Table name (if Postgres output)
- Schema preview (input_schema)

---

## 🔄 Data Flow & Schema Propagation

### Example: Well Logs → VShale → Output

```
1. Ingestion Node (Well Logs)
   ├─ source_type: { source: 'well_logs', well_ids: ['well-1'], curve_names: ['GR'] }
   ├─ output_schema: {
   │     schema_id: 'well_logs_standard',
   │     required_columns: ['well_id', 'depth'],
   │     optional_columns: ['gr']
   │   }
   └─ dependencies: []  (no inputs)

2. Operator Node (VShale Larionov)
   ├─ operator_id: 'shale_volume_larionov'
   ├─ operator_version: '1.0.0'
   ├─ input_schema: {  // Looked up from operator_registry
   │     schema_id: 'well_logs_standard',
   │     required_columns: ['well_id', 'depth', 'gr'],
   │     optional_columns: []
   │   }
   ├─ output_schema: {  // Looked up from operator_registry
   │     schema_id: 'vshale_output',
   │     required_columns: ['well_id', 'depth', 'vsh'],
   │     optional_columns: ['gr']
   │   }
   └─ dependencies: ['ingestion-node-id']  (depends on Ingestion node)

3. Output Node (Frontend)
   ├─ output_type: 'frontend'
   ├─ input_schema: {
   │     schema_id: 'vshale_output',
   │     required_columns: ['well_id', 'depth', 'vsh'],
   │     optional_columns: []
   │   }
   └─ dependencies: ['operator-node-id']  (depends on Operator node)
```

**Validation Flow**:

1. ✅ Ingestion → Operator: `well_logs_standard` (with optional 'gr') compatible with `well_logs_standard` (requires 'gr') → **VALID**
2. ✅ Operator → Output: `vshale_output` compatible with `vshale_output` → **VALID**

---

## 📝 Implementation Checklist

### Immediate (This Week)

- [ ] **Create `node-connection-validator.ts`**
  - [ ] Implement `NodeConnectionValidator` class
  - [ ] Implement `isValidConnection()` method
  - [ ] Implement `getValidationError()` method
  - [ ] Add unit tests

- [ ] **Wire to `SF-pipeline-flow.svelte`**
  - [ ] Import `nodeConnectionValidator`
  - [ ] Create `isValidConnection` function
  - [ ] Pass to `<SvelteFlow>` component
  - [ ] Update validator when nodes change

- [ ] **Create operator schema lookup service**
  - [ ] `operator-schema-service.ts`
  - [ ] Fetch from Supabase `operator_registry` table
  - [ ] Cache operator schemas
  - [ ] Handle missing schemas

### Short-term (Next 2 Weeks)

- [ ] **Create custom node components**
  - [ ] `ingestion-node.svelte`
  - [ ] `operator-node.svelte`
  - [ ] `output-node.svelte`
  - [ ] Register in `SF-pipeline-flow.svelte`

- [ ] **Enhanced validation UI**
  - [ ] Visual feedback (red/green edges)
  - [ ] Tooltip with validation errors
  - [ ] Disable incompatible handles
  - [ ] Connection preview

- [ ] **Schema compatibility helper**
  - [ ] `schema-compatibility.ts`
  - [ ] `validateSchemaCompatible()`
  - [ ] `getMissingColumns()`
  - [ ] `suggestCompatibleOperators()`

### Medium-term (Next Month)

- [ ] **Backend validation enhancement**
  - [ ] Add `validate_types()` to `dag-validator`
  - [ ] Call in `save_pipeline` Tauri command
  - [ ] Return detailed validation errors

- [ ] **Runtime schema validation**
  - [ ] Validate schemas in `execute_node()`
  - [ ] Arrow schema parsing and type checking
  - [ ] Detailed error messages

- [ ] **Schema evolution support**
  - [ ] Handle schema versioning
  - [ ] Migration suggestions for incompatible schemas
  - [ ] Schema compatibility matrix UI

---

## 🎯 Key Decisions Summary

### ✅ Confirmed Decisions

1. **Three Node Types**: Ingestion, Operator, Output (separate types)
2. **Schema-Based Validation**: Use `SchemaDefinition` for type safety
3. **Connection Rules**: Ingestion → Operator/Output, Operator → Operator/Output, Output → None
4. **Frontend Validation**: Use Svelte Flow's `isValidConnection` prop
5. **Schema Lookup**: Operator schemas can be looked up from `operator_registry` table
6. **Ingestion References**: Already supported via `source_type` in `NodeType.Ingestion`

### 🔄 Open Questions (Resolved)

1. **Q**: Should we have separate Ingestion nodes or just Operator nodes with ingestion option?  
   **A**: ✅ **Separate Ingestion nodes** - Better UX, clearer semantics, matches backend

2. **Q**: Does schema handle ingestion data references?  
   **A**: ✅ **Yes** - Via `source_type` in Ingestion nodes, stored in `node_config` JSONB

3. **Q**: Do we track operator input/output types?  
   **A**: ✅ **Yes** - `operator_registry` table has `input_schema` and `output_schema` columns

4. **Q**: How to prevent invalid connections in Svelte Flow?  
   **A**: ✅ **Use `isValidConnection` prop** - Implement `NodeConnectionValidator` class

---

## 📚 References

- **Backend Schema**: `crates/dags/core/src/schema.rs` - `validate_compatible()`
- **Database Schema**: `db/init/02-pipeline-schema.sql` - `operator_registry` table
- **Frontend Types**: `src/lib/services/dag-types.ts` - `NodeType`, `SchemaDefinition`
- **Svelte Flow Docs**: `docs/libraries/svelte/svelte-flow/` - `isValidConnection`, `Handle`
- **DAG Execution Plan**: `markdown-guides/to-do/dag-execution/dag-execution.md`
- **WLIP Compatibility**: `markdown-guides/future-plans/WLIP/WLIP_MudRock_Compatibility_and_Security.md`

---

## 🚀 Next Steps

1. **Create `node-connection-validator.ts`** with schema compatibility logic
2. **Wire `isValidConnection` to `SF-pipeline-flow.svelte`**
3. **Create custom node components** for Ingestion/Operator/Output
4. **Add operator schema lookup service** for fetching schemas from registry
5. **Implement visual feedback** (red/green edges, tooltips)

**Priority**: Start with **Phase 1** (Node Connection Validator) as it's the foundation for all type safety features.
