# Apache Superset: Schema Inference, Column Typing, and Type Safety

## Executive Summary

Apache Superset handles schema inference and column typing through a **database engine spec system** that maps database-specific column types to Superset's generic data types (STRING, NUMERIC, TEMPORAL, BOOLEAN). Schema inference occurs automatically when datasets are created or refreshed, using SQLAlchemy's introspection capabilities. Column types are validated through regex-based mappings in `BaseEngineSpec`, with database-specific overrides. Visualization plugins consume typed query results through a standardized `queriesData` format that includes column metadata, ensuring type safety through TypeScript interfaces and runtime validation.

---

## 1. Schema Inference

### 1.1 Automatic Schema Detection

When a dataset is created or refreshed, Superset automatically infers the schema from the underlying database table or view:

```python
from superset.daos.dataset import DatasetDAO

# Automatic schema detection
dataset = DatasetDAO.get_or_create_dataset(
    database_id=1,
    table_name="customer_orders",
    schema="public"
)

# Schema is automatically detected:
# - Column names
# - Column types (from database)
# - Nullability
# - Default values
```

**Inference Process:**
1. **Connect to Database**: Using SQLAlchemy connection
2. **Introspect Table**: Query database metadata (INFORMATION_SCHEMA, system tables)
3. **Extract Columns**: Get column names, types, constraints
4. **Map Types**: Convert database types to Superset types (via engine spec)
5. **Store Metadata**: Save column information in `table_columns` table

### 1.2 Schema Refresh

Schemas can be manually refreshed to sync with database changes:

```bash
# API endpoint
POST /api/v1/dataset/{id}/refresh
```

**Refresh Response:**
```json
{
  "message": "Dataset schema refreshed",
  "columns_added": ["new_column"],
  "columns_removed": ["deleted_column"],
  "columns_updated": ["modified_column"]
}
```

**Refresh Process:**
1. **Query Current Schema**: Get current columns from database
2. **Compare with Stored**: Compare with `table_columns` records
3. **Identify Changes**: Added, removed, updated columns
4. **Update Metadata**: Sync `table_columns` with database
5. **Return Summary**: Report what changed

### 1.3 Schema Storage

**Database Schema:**
- **Table**: `table_columns`
- **Columns**: `id`, `dataset_id`, `column_name`, `type`, `is_dttm`, `filterable`, `groupby`, `is_active`

**Column Metadata:**
```python
class TableColumn(Model):
    __tablename__ = "table_columns"
    
    id = Column(Integer, primary_key=True)
    dataset_id = Column(Integer, ForeignKey('tables.id'))
    column_name = Column(String(255))
    type = Column(String(32))  # Database-specific type string
    is_dttm = Column(Boolean)  # Is datetime/temporal
    filterable = Column(Boolean)
    groupby = Column(Boolean)
    is_active = Column(Boolean)
```

**Example Stored Column:**
```python
{
    "id": 101,
    "dataset_id": 15,
    "column_name": "order_date",
    "type": "DATE",  # Database-specific type
    "is_dttm": True,  # Mapped to TEMPORAL
    "filterable": True,
    "groupby": True,
    "is_active": True
}
```

---

## 2. Column Type Mapping

### 2.1 Generic Data Types

Superset uses **four generic data types** to abstract database-specific types:

```python
class GenericDataType:
    STRING = "STRING"      # Text, varchar, char, etc.
    NUMERIC = "NUMERIC"    # Integer, float, decimal, etc.
    TEMPORAL = "TEMPORAL"  # Date, datetime, timestamp, etc.
    BOOLEAN = "BOOLEAN"    # Boolean, bit, etc.
```

**Purpose:**
- **Abstraction**: Hide database-specific type differences
- **Consistency**: Same visualization logic across databases
- **Validation**: Type checking for chart compatibility

### 2.2 Base Engine Spec Type Mappings

All database engines inherit default type mappings from `BaseEngineSpec`:

```python
from sqlalchemy import types
import re

class BaseEngineSpec:
    _default_column_type_mappings: tuple[ColumnTypeMapping, ...] = (
        (
            re.compile(r"^string", re.IGNORECASE),
            types.String(),
            GenericDataType.STRING,
        ),
        (
            re.compile(r"^float", re.IGNORECASE),
            types.Float(),
            GenericDataType.NUMERIC,
        ),
        (
            re.compile(r"^date", re.IGNORECASE),
            types.Date(),
            GenericDataType.TEMPORAL,
        ),
        (
            re.compile(r"^bool(ean)?", re.IGNORECASE),
            types.Boolean(),
            GenericDataType.BOOLEAN,
        ),
        # ... more mappings
    )
```

**Mapping Structure:**
- **Regex Pattern**: Matches database type name (case-insensitive)
- **SQLAlchemy Type**: Type object for SQL generation
- **Generic Type**: Superset's generic data type

**Example Mappings:**
| Database Type | Regex Match | SQLAlchemy Type | Generic Type |
|--------------|-------------|-----------------|--------------|
| `VARCHAR(255)` | `^string` | `String()` | `STRING` |
| `FLOAT` | `^float` | `Float()` | `NUMERIC` |
| `DATE` | `^date` | `Date()` | `TEMPORAL` |
| `BOOLEAN` | `^bool(ean)?` | `Boolean()` | `BOOLEAN` |

### 2.3 Database-Specific Type Mappings

Database engines can override or extend default mappings:

```python
from sqlalchemy.dialects.mssql.base import SMALLDATETIME

class MssqlEngineSpec(BaseEngineSpec):
    column_type_mappings = (
        (
            re.compile(r"^smalldatetime.*", re.IGNORECASE),
            SMALLDATETIME(),
            GenericDataType.TEMPORAL,
        ),
        # Inherits defaults from BaseEngineSpec
    )
```

**Override Behavior:**
- **Database-specific mappings** are checked first
- **Base mappings** are used as fallback
- **Most specific match** wins (longer regex patterns first)

### 2.4 Type Mapping Process

**When Column Type is Determined:**

1. **Schema Inference**: Database returns type string (e.g., `"VARCHAR(255)"`)
2. **Engine Spec Lookup**: Find matching engine spec for database
3. **Pattern Matching**: Test type string against regex patterns
4. **Generic Type Assignment**: Map to `GenericDataType`
5. **Storage**: Store both database type and generic type

**Example Flow:**
```
Database Column: "order_date" type "DATE"
  ↓
PostgreSQL Engine Spec
  ↓
Pattern Match: r"^date" → matches
  ↓
Generic Type: TEMPORAL
  ↓
Stored: {type: "DATE", generic_type: "TEMPORAL", is_dttm: true}
```

---

## 3. Type Validation Across Databases

### 3.1 Database Engine Specs

Each database has an **engine spec** that defines:
- Type mappings
- SQL dialect support
- Feature capabilities
- Error handling

```python
from superset.db_engine_specs import get_available_engine_specs

available_engines = get_available_engine_specs()

for engine_spec in available_engines:
    print(f"Engine: {engine_spec.engine_name}")
    print(f"  Supports File Upload: {engine_spec.supports_file_upload}")
    print(f"  Supports Dynamic Schema: {engine_spec.supports_dynamic_schema}")
    print(f"  Supports Catalog: {engine_spec.supports_catalog}")
```

**Engine Spec Capabilities:**
- **Type Mapping**: Database types → Generic types
- **SQL Generation**: Dialect-specific SQL syntax
- **Feature Detection**: Subqueries, JOINs, window functions
- **Error Mapping**: Database errors → Superset errors

### 3.2 Type Validation During Query Building

**Column Type Validation:**
- **Existence Check**: Column exists in dataset
- **Type Compatibility**: Column type supports requested operation
- **Metric Validation**: Metrics reference valid columns with compatible types

**Example Validation:**
```python
# Chart configuration
{
    "groupby": ["region"],  # STRING column - valid
    "metrics": ["sum__sales"],  # NUMERIC column - valid
    "time_range": "Last 30 days"  # Requires TEMPORAL column
}

# Validation checks:
# 1. "region" column exists and is STRING → ✓
# 2. "sales" column exists and is NUMERIC → ✓
# 3. Dataset has TEMPORAL column for time_range → ✓
```

### 3.3 Error Mapping

Database-specific errors are mapped to Superset error types:

```python
import re
from superset.db_engine_specs.exceptions import SupersetErrorType

COLUMN_DOES_NOT_EXIST_REGEX = re.compile("no such column: (?P<column_name>.+)")

class SqliteEngineSpec(BaseEngineSpec):
    custom_errors: Dict[Pattern[str], Tuple[str, SupersetErrorType, Dict[str, Any]]] = {
        COLUMN_DOES_NOT_EXIST_REGEX: (
            __('We can\'t seem to resolve the column "%(column_name)s"'),
            SupersetErrorType.COLUMN_DOES_NOT_EXIST_ERROR,
            {},
        ),
    }
```

**Error Mapping Benefits:**
- **Consistent Errors**: Same error types across databases
- **User-Friendly Messages**: Actionable error messages
- **Error Handling**: Structured error responses

### 3.4 Type Validation in SQL Generation

**Type-Aware SQL Generation:**
- **Temporal Columns**: Time range filters use proper date functions
- **Numeric Columns**: Aggregations use appropriate functions
- **String Columns**: String operations use correct syntax

**Example:**
```sql
-- Temporal column with time range
SELECT * FROM orders
WHERE order_date >= '2024-01-01'
  AND order_date < '2024-02-01'

-- Numeric column with aggregation
SELECT region, SUM(sales) AS total_sales
FROM sales_data
GROUP BY region

-- String column with filtering
SELECT * FROM customers
WHERE name LIKE 'John%'
```

---

## 4. Query Result Format

### 4.1 Standardized Result Structure

Query results follow a consistent structure regardless of database:

```json
{
  "result": [
    {
      "query": "SELECT name, SUM(num) AS sum__num FROM ...",
      "language": "sql",
      "status": "success",
      "rowcount": 100,
      "data": [
        {"name": "Michael", "sum__num": 214350},
        {"name": "Christopher", "sum__num": 185832}
      ],
      "columns": [
        {
          "name": "name",
          "type": "VARCHAR(255)",
          "is_dttm": false
        },
        {
          "name": "sum__num",
          "type": "BIGINT",
          "is_dttm": false
        }
      ],
      "selected_columns": [
        {
          "column_name": "name",
          "type": "VARCHAR(255)"
        }
      ],
      "cache_key": "abc123...",
      "applied_filters": [
        {"column": "gender"}
      ],
      "rejected_filters": []
    }
  ]
}
```

**Key Components:**
- **data**: Array of row objects (column names as keys)
- **columns**: Metadata for all returned columns (name, type, is_dttm)
- **selected_columns**: Metadata for explicitly selected columns
- **query**: Generated SQL query
- **status**: Execution status

### 4.2 Column Metadata in Results

**Column Information:**
```json
{
  "name": "order_date",
  "type": "DATE",           // Database-specific type
  "is_dttm": true,          // Is temporal type
  "column_name": "order_date"  // Alias for selected_columns
}
```

**Type Information Flow:**
```
Database Column Type: "DATE"
  ↓
Engine Spec Mapping: TEMPORAL
  ↓
Stored in table_columns: {type: "DATE", is_dttm: true}
  ↓
Query Result: {name: "order_date", type: "DATE", is_dttm: true}
  ↓
Plugin Receives: Typed column metadata
```

### 4.3 Data Type Consistency

**Type Preservation:**
- **Numeric Values**: Returned as numbers (not strings)
- **Temporal Values**: Returned as ISO 8601 strings or timestamps
- **String Values**: Returned as strings
- **Boolean Values**: Returned as booleans

**Example:**
```json
{
  "data": [
    {
      "id": 1,                    // number
      "name": "Product A",        // string
      "price": 29.99,             // number
      "is_active": true,          // boolean
      "created_at": "2024-01-15T10:30:00Z"  // ISO 8601 string
    }
  ]
}
```

---

## 5. Visualization Plugin Data Consumption

### 5.1 SuperChart Component

Plugins receive data through the `SuperChart` component:

```javascript
<SuperChart
  chartType="bar"
  width={600}
  height={400}
  formData={{
    groupby: ["region"],
    metrics: ["sum__sales"]
  }}
  queriesData={[{
    data: [
      {"region": "US-West", "sum__sales": 10000},
      {"region": "US-East", "sum__sales": 15000}
    ],
    colnames: ["region", "sum__sales"],
    coltypes: ["STRING", "NUMERIC"]
  }]}
/>
```

**Props:**
- **chartType**: Visualization type identifier
- **formData**: Chart configuration (filters, metrics, etc.)
- **queriesData**: Array of query results with data and metadata

### 5.2 QueriesData Structure

**Standard Format:**
```typescript
interface QueriesData {
  data: Array<Record<string, any>>;  // Row objects
  colnames: string[];                // Column names
  coltypes: string[];                // Generic types (STRING, NUMERIC, etc.)
  rowcount: number;                  // Number of rows
  query?: string;                    // Generated SQL
}
```

**Example:**
```javascript
queriesData = [{
  data: [
    {"region": "US-West", "sum__sales": 10000, "order_date": "2024-01-15"},
    {"region": "US-East", "sum__sales": 15000, "order_date": "2024-01-16"}
  ],
  colnames: ["region", "sum__sales", "order_date"],
  coltypes: ["STRING", "NUMERIC", "TEMPORAL"],
  rowcount: 2
}]
```

### 5.3 Type Safety in Plugins

**TypeScript Interfaces:**
```typescript
interface ChartProps {
  chartId: number;
  datasource: Datasource;
  formData: FormData;
  queriesResponse: QueriesResponse;
  vizType: string;
  width: number;
  height: number;
}

interface QueriesResponse {
  queries: Array<{
    data: Array<Record<string, any>>;
    columns: Array<{
      name: string;
      type: string;
      is_dttm: boolean;
    }>;
    rowcount: number;
  }>;
}
```

**Type Checking:**
- **Compile-time**: TypeScript interfaces catch type errors
- **Runtime**: Column metadata validates data structure
- **Generic Types**: `coltypes` array provides type information

### 5.4 Plugin Transform Function

Plugins use a `transformProps` function to process data:

```typescript
function transformProps(chartProps: ChartProps) {
  const { formData, queriesData } = chartProps;
  const data = queriesData[0].data;
  const coltypes = queriesData[0].coltypes;
  
  // Type-safe data processing
  const numericColumns = coltypes
    .map((type, idx) => type === 'NUMERIC' ? colnames[idx] : null)
    .filter(Boolean);
  
  // Process data with type awareness
  const processedData = data.map(row => ({
    x: row[formData.groupby[0]],  // STRING column
    y: row[formData.metrics[0]]   // NUMERIC column
  }));
  
  return {
    data: processedData,
    // ... other props
  };
}
```

**Type Safety Benefits:**
- **Column Type Awareness**: Plugins know which columns are numeric, temporal, etc.
- **Validation**: Can validate data structure matches expected types
- **Error Prevention**: Type mismatches caught early

### 5.5 Handling Different Data Types

**Numeric Data:**
```typescript
// Extract numeric columns
const numericData = data.map(row => ({
  value: parseFloat(row[metricColumn]) || 0  // Ensure number
}));
```

**Temporal Data:**
```typescript
// Parse temporal columns
const temporalData = data.map(row => ({
  date: new Date(row[temporalColumn]),  // Parse ISO 8601
  value: row[metricColumn]
}));
```

**String Data:**
```typescript
// String columns for grouping
const groupedData = data.reduce((acc, row) => {
  const key = row[stringColumn];
  acc[key] = (acc[key] || 0) + row[metricColumn];
  return acc;
}, {});
```

### 5.6 Type Validation in Plugins

**Runtime Type Checking:**
```typescript
function validateDataTypes(queriesData: QueriesData) {
  const { colnames, coltypes, data } = queriesData[0];
  
  // Validate column types match expected
  const requiredTypes = {
    'region': 'STRING',
    'sales': 'NUMERIC',
    'date': 'TEMPORAL'
  };
  
  for (const [colName, expectedType] of Object.entries(requiredTypes)) {
    const idx = colnames.indexOf(colName);
    if (idx === -1) {
      throw new Error(`Column ${colName} not found`);
    }
    if (coltypes[idx] !== expectedType) {
      throw new Error(
        `Column ${colName} has type ${coltypes[idx]}, expected ${expectedType}`
      );
    }
  }
  
  return true;
}
```

---

## 6. Type Safety Mechanisms

### 6.1 Backend Type Validation

**Schema Validation:**
- **Marshmallow Schemas**: Validate API request/response types
- **SQLAlchemy Types**: Type checking in ORM models
- **Engine Spec Validation**: Column type compatibility checks

**Example:**
```python
from marshmallow import Schema, fields

class DatasetSchema(Schema):
    table_name = fields.String(required=True)
    columns = fields.List(fields.Dict(), required=True)
    metrics = fields.List(fields.Dict())
```

### 6.2 Frontend Type Safety

**TypeScript:**
- **Interfaces**: Define data structures
- **Type Guards**: Runtime type checking
- **Generic Types**: Reusable type definitions

**Example:**
```typescript
interface ColumnMetadata {
  name: string;
  type: 'STRING' | 'NUMERIC' | 'TEMPORAL' | 'BOOLEAN';
  is_dttm: boolean;
}

function isNumericColumn(col: ColumnMetadata): boolean {
  return col.type === 'NUMERIC';
}
```

### 6.3 Runtime Type Validation

**Data Validation:**
- **Column Existence**: Verify columns exist in dataset
- **Type Compatibility**: Check types support operations
- **Value Validation**: Validate data values match types

**Example:**
```python
# Backend validation
def validate_metric(metric: str, dataset: Dataset) -> bool:
    # Check metric exists
    if metric not in [m.metric_name for m in dataset.metrics]:
        return False
    
    # Check metric expression is valid
    # Check referenced columns exist and are compatible types
    return True
```

---

## 7. Cross-Database Type Consistency

### 7.1 Generic Type Abstraction

**Benefits:**
- **Consistent Visualizations**: Same chart logic works across databases
- **Simplified Plugin Development**: Plugins work with generic types
- **Type Safety**: Validation based on generic types, not database types

**Example:**
```typescript
// Plugin doesn't need to know about VARCHAR vs TEXT vs CHAR
// Just knows it's a STRING type
if (coltype === 'STRING') {
  // Handle as string
}
```

### 7.2 Database-Specific Handling

**When Needed:**
- **SQL Generation**: Database-specific syntax
- **Type Conversion**: Database-specific type casting
- **Feature Support**: Database-specific capabilities

**Example:**
```python
class PostgresEngineSpec(BaseEngineSpec):
    def convert_dttm(self, target_type, dttm):
        if target_type.upper() == 'DATE':
            return f"DATE '{dttm.date()}'"
        elif target_type.upper() == 'DATETIME':
            return f"TIMESTAMP '{dttm}'"
```

### 7.3 Type Mapping Examples

**PostgreSQL:**
```
VARCHAR(255) → STRING
INTEGER → NUMERIC
DATE → TEMPORAL
BOOLEAN → BOOLEAN
```

**MySQL:**
```
VARCHAR(255) → STRING
INT → NUMERIC
DATE → TEMPORAL
TINYINT(1) → BOOLEAN
```

**SQLite:**
```
TEXT → STRING
INTEGER → NUMERIC
DATE → TEMPORAL
BOOLEAN → BOOLEAN
```

**All map to same generic types for plugin consumption.**

---

## 8. Best Practices

### 8.1 Plugin Development

**Type Safety:**
- Use TypeScript interfaces for all data structures
- Validate column types at runtime
- Handle type mismatches gracefully

**Example:**
```typescript
function transformProps(chartProps: ChartProps): TransformedProps {
  const { queriesData } = chartProps;
  
  // Type guard
  if (!queriesData || queriesData.length === 0) {
    throw new Error('No query data available');
  }
  
  const { data, colnames, coltypes } = queriesData[0];
  
  // Validate types
  const metricIdx = colnames.indexOf('sales');
  if (coltypes[metricIdx] !== 'NUMERIC') {
    throw new Error('Sales column must be NUMERIC');
  }
  
  return { data, /* ... */ };
}
```

### 8.2 Dataset Configuration

**Column Metadata:**
- Keep column metadata up-to-date (refresh schema after DB changes)
- Set appropriate `is_dttm` flags for temporal columns
- Configure `filterable` and `groupby` appropriately

**Type Configuration:**
- Use database-native types (don't override unnecessarily)
- Let engine spec handle type mapping
- Customize only when needed (database-specific types)

### 8.3 Error Handling

**Type Errors:**
- Provide clear error messages with column names and types
- Suggest fixes (e.g., "Column X must be NUMERIC for SUM aggregation")
- Log type mismatches for debugging

**Example:**
```python
if column.generic_type != GenericDataType.NUMERIC:
    raise ValidationError(
        f"Column '{column.column_name}' has type {column.generic_type}, "
        f"but SUM aggregation requires NUMERIC type"
    )
```

---

## 9. Limitations and Considerations

### 9.1 Type Inference Limitations

**Current Limitations:**
- **No Type Refinement**: Generic types are broad (NUMERIC covers int, float, decimal)
- **No Nullability Tracking**: Doesn't track which columns are nullable
- **No Precision/Scale**: NUMERIC type doesn't preserve precision/scale info
- **Limited Validation**: Type validation is basic (existence, compatibility)

### 9.2 Database-Specific Quirks

**Type Variations:**
- **Boolean Representation**: Some databases use TINYINT(1), others use BOOLEAN
- **Date/DateTime**: Some databases distinguish DATE and DATETIME, others don't
- **String Types**: VARCHAR, TEXT, CHAR all map to STRING (no length info)

**Workarounds:**
- Database-specific engine specs handle quirks
- Custom type mappings for edge cases
- Runtime validation for critical operations

### 9.3 Plugin Type Safety

**Runtime vs Compile-Time:**
- **TypeScript**: Compile-time type checking (can be bypassed)
- **Runtime**: Column metadata provides runtime type info
- **No Guarantee**: Data structure could still be wrong at runtime

**Best Practices:**
- Always validate data structure in plugins
- Use type guards for runtime checking
- Handle type mismatches gracefully

---

## 10. Conclusion

Superset's type system provides:

1. **Automatic Schema Inference**: Detects columns and types from databases
2. **Generic Type Abstraction**: Maps database types to four generic types
3. **Database-Specific Handling**: Engine specs handle database quirks
4. **Type-Safe Plugin Consumption**: Standardized data format with type metadata
5. **Validation**: Type checking at multiple layers (backend, frontend, runtime)

**Key Takeaways:**
- Schema inference is automatic via SQLAlchemy introspection
- Column types are mapped to generic types (STRING, NUMERIC, TEMPORAL, BOOLEAN)
- Database engine specs provide database-specific type mappings
- Query results include column metadata for type safety
- Plugins consume typed data through standardized `queriesData` format
- Type safety is enforced at compile-time (TypeScript) and runtime (validation)

**Architecture Strengths:**
- **Abstraction**: Generic types hide database differences
- **Extensibility**: Easy to add new database engines
- **Type Safety**: Multiple layers of type checking
- **Consistency**: Same visualization logic across databases

**Areas for Improvement:**
- More granular type information (precision, scale, nullability)
- Better type refinement (distinguish int vs float)
- Enhanced runtime type validation
- Type inference for virtual datasets (SQL queries)

This architecture prioritizes **simplicity and consistency** over **fine-grained type information**, which aligns with Superset's goal of providing a unified interface across diverse database systems.


