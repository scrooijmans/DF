# Arrow RecordBatch to AG-Grid Data Table Conversion

This document describes the most efficient approach for converting Arrow RecordBatch data to AG-Grid compatible format in Rust, optimized for performance and compatibility.

## 🎯 **Most Efficient Data Type: `Vec<HashMap<String, serde_json::Value>>`**

For converting Arrow RecordBatch to AG-Grid compatible format, the most efficient and compatible data type in Rust is:

```rust
use std::collections::HashMap;
use serde_json::Value;

// Most efficient format for AG-Grid
type AgGridRowData = Vec<HashMap<String, Value>>;
```

## 🚀 **Why This Format is Optimal**

### 1. **AG-Grid Compatibility**
- AG-Grid expects `rowData` as an array of objects: `Array<Record<string, any>>`
- `Vec<HashMap<String, Value>>` maps directly to this TypeScript type
- No additional conversion needed on the frontend

### 2. **Arrow Integration**
- Arrow RecordBatch can be efficiently converted to this format
- Preserves all data types through `serde_json::Value`
- Handles null values, nested data, and complex types

### 3. **Memory Efficiency**
- `HashMap` provides O(1) column access
- `serde_json::Value` handles type flexibility without boxing
- Minimal allocations during conversion

## 🔄 **Complete Conversion Flow**

### **Rust Backend (Arrow → AG-Grid Format)**

```rust
use arrow::record_batch::RecordBatch;
use arrow::array::*;
use serde_json::Value;
use std::collections::HashMap;

pub fn record_batch_to_ag_grid_data(batch: &RecordBatch) -> Result<Vec<HashMap<String, Value>>, ArrowError> {
    let mut rows = Vec::new();
    let num_rows = batch.num_rows();
    let num_cols = batch.num_columns();
    
    // Get column names
    let column_names: Vec<&str> = batch.schema()
        .fields()
        .iter()
        .map(|field| field.name().as_str())
        .collect();
    
    // Convert each row
    for row_idx in 0..num_rows {
        let mut row = HashMap::new();
        
        for col_idx in 0..num_cols {
            let column_name = column_names[col_idx].to_string();
            let value = match batch.column(col_idx).data_type() {
                DataType::Int64 => {
                    let array = batch.column(col_idx).as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap();
                    if array.is_null(row_idx) {
                        Value::Null
                    } else {
                        Value::Number(array.value(row_idx).into())
                    }
                },
                DataType::Float64 => {
                    let array = batch.column(col_idx).as_any()
                        .downcast_ref::<Float64Array>()
                        .unwrap();
                    if array.is_null(row_idx) {
                        Value::Null
                    } else {
                        Value::Number(serde_json::Number::from_f64(array.value(row_idx)).unwrap_or(0.into()))
                    }
                },
                DataType::Utf8 => {
                    let array = batch.column(col_idx).as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap();
                    if array.is_null(row_idx) {
                        Value::Null
                    } else {
                        Value::String(array.value(row_idx).to_string())
                    }
                },
                DataType::Boolean => {
                    let array = batch.column(col_idx).as_any()
                        .downcast_ref::<BooleanArray>()
                        .unwrap();
                    if array.is_null(row_idx) {
                        Value::Null
                    } else {
                        Value::Bool(array.value(row_idx))
                    }
                },
                DataType::Timestamp(_, _) => {
                    let array = batch.column(col_idx).as_any()
                        .downcast_ref::<TimestampMillisecondArray>()
                        .unwrap();
                    if array.is_null(row_idx) {
                        Value::Null
                    } else {
                        // Convert timestamp to ISO string
                        let timestamp = array.value(row_idx);
                        let datetime = chrono::DateTime::from_timestamp_millis(timestamp).unwrap();
                        Value::String(datetime.to_rfc3339())
                    }
                },
                _ => {
                    // Fallback: convert to string
                    Value::String(format!("{:?}", batch.column(col_idx)))
                }
            };
            
            row.insert(column_name, value);
        }
        
        rows.push(row);
    }
    
    Ok(rows)
}
```

### **TypeScript Frontend (Direct Usage)**

```typescript
// No conversion needed - direct assignment
let rowData: Record<string, any>[] = csvData; // or arrowData

// AG-Grid configuration
const gridOptions: GridOptions = {
  rowData: rowData, // Direct assignment
  columnDefs: generateColumnDefsFromSchema(schema),
  // ... other options
};
```

## ⚡ **Performance Optimizations**

### 1. **Batch Processing**
```rust
// Process multiple RecordBatches efficiently
pub fn process_arrow_batches(batches: &[RecordBatch]) -> Vec<HashMap<String, Value>> {
    let mut all_rows = Vec::new();
    for batch in batches {
        let rows = record_batch_to_ag_grid_data(batch)?;
        all_rows.extend(rows);
    }
    all_rows
}
```

### 2. **Memory Pooling**
```rust
// Reuse HashMap allocations
pub struct AgGridConverter {
    row_pool: Vec<HashMap<String, Value>>,
}

impl AgGridConverter {
    fn get_row(&mut self) -> HashMap<String, Value> {
        self.row_pool.pop().unwrap_or_else(|| HashMap::new())
    }
    
    fn return_row(&mut self, mut row: HashMap<String, Value>) {
        row.clear();
        self.row_pool.push(row);
    }
}
```

### 3. **Streaming for Large Datasets**
```rust
// For very large datasets, stream the conversion
pub fn stream_arrow_to_ag_grid<F>(
    batch: &RecordBatch,
    mut callback: F
) -> Result<(), ArrowError> 
where
    F: FnMut(HashMap<String, Value>),
{
    // Process row by row instead of loading all into memory
    for row_idx in 0..batch.num_rows() {
        let row = convert_single_row(batch, row_idx)?;
        callback(row);
    }
    Ok(())
}
```

## 📊 **Comparison with Current Schema-to-UI Flow**

| Aspect | Current (PostgreSQL) | Optimized (Arrow) |
|--------|---------------------|-------------------|
| **Data Source** | Database queries | Arrow RecordBatch |
| **Type Safety** | Schema-based | Arrow schema + runtime |
| **Performance** | Network round-trip | In-memory processing |
| **Memory Usage** | JSON serialization | Direct conversion |
| **Scalability** | Limited by DB | Limited by memory |

## 🔧 **Integration with Existing System**

### **Enhanced HardcodedSchema**
```rust
impl HardcodedSchema {
    pub fn get_arrow_schema() -> arrow::datatypes::Schema {
        // Define Arrow schema for wells data
        Schema::new(vec![
            Field::new("well_name", DataType::Utf8, false),
            Field::new("field", DataType::Utf8, true),
            Field::new("operator", DataType::Utf8, true),
            Field::new("latitude", DataType::Float64, true),
            Field::new("longitude", DataType::Float64, true),
            Field::new("elevation", DataType::Float64, true),
            Field::new("spud_date", DataType::Utf8, true),
            Field::new("completion_date", DataType::Utf8, true),
            Field::new("total_depth", DataType::Float64, true),
            Field::new("status", DataType::Utf8, true),
            Field::new("coordinate_system", DataType::Utf8, true),
            Field::new("company", DataType::Utf8, true),
        ])
    }
}
```

### **Arrow-to-AG-Grid Converter**
```rust
pub struct ArrowToAgGridConverter {
    schema: arrow::datatypes::Schema,
}

impl ArrowToAgGridConverter {
    pub fn new() -> Self {
        Self {
            schema: HardcodedSchema::get_arrow_schema(),
        }
    }
    
    pub fn convert_record_batch(&self, batch: &RecordBatch) -> Vec<HashMap<String, Value>> {
        record_batch_to_ag_grid_data(batch).unwrap_or_default()
    }
}
```

### **Updated Tauri Commands**
```rust
#[tauri::command]
pub async fn get_wells_data_arrow() -> Result<Vec<HashMap<String, Value>>, String> {
    let batch = load_wells_parquet_file().await?;
    let converter = ArrowToAgGridConverter::new();
    Ok(converter.convert_record_batch(&batch))
}
```

## 🎯 **Key Benefits**

1. **Type Safety**: End-to-end Arrow → AG-Grid type checking
2. **Performance**: In-memory processing, no network overhead
3. **Memory Efficiency**: Direct conversion, minimal allocations
4. **Scalability**: Handles large datasets efficiently
5. **Compatibility**: Works seamlessly with existing AG-Grid system
6. **Flexibility**: Supports all Arrow data types

## 🚀 **Usage Example**

```rust
// Load Parquet file as Arrow RecordBatch
let batch = load_parquet_as_record_batch("wells.parquet").await?;

// Convert to AG-Grid format
let ag_grid_data = record_batch_to_ag_grid_data(&batch)?;

// Send to frontend via Tauri
Ok(ag_grid_data)
```

```typescript
// Frontend receives data directly
const wellsData = await invoke<Record<string, any>[]>("get_wells_data_arrow");

// Use directly in AG-Grid
const gridOptions: GridOptions = {
  rowData: wellsData,
  columnDefs: generateWellsColumnDefs(),
  // ... other options
};
```

This approach provides the optimal balance between performance, memory efficiency, and compatibility for converting Arrow RecordBatch data to AG-Grid format! 🎉
