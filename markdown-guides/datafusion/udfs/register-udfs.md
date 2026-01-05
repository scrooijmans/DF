# DataFusion UDF Registration API

This document describes our current API for registering vectorized User-Defined Functions (UDFs) in DataFusion, specifically for petrophysical calculations.

## Overview

Our UDF registration system is built around a centralized registry that ensures only high-performance, vectorized versions of petrophysical functions are exposed to end users. This eliminates code duplication and provides a consistent interface for UDF management.

## Architecture

### Core Components

1. **`UdfRegistry`** - Centralized registry for managing UDF definitions
2. **`VectorizedCalculation` trait** - Generic trait for vectorized petrophysical calculations
3. **Vectorized implementations** - High-performance implementations of petrophysical functions
4. **UDF definitions** - Metadata about each UDF including input/output types and categories

### Key Design Principles

- **Only vectorized UDFs are registered** - No unvectorized versions are exposed
- **Centralized management** - All UDFs are managed through a single registry
- **Type safety** - Strong typing ensures compatibility between operations
- **Performance optimization** - Leverages Arrow's compute kernels and SIMD operations
- **Error handling** - Comprehensive error conversion between petrophysics and DataFusion errors

## API Reference

### UdfRegistry

The main registry for managing UDF definitions and registration.

```rust
pub struct UdfRegistry {
    definitions: Vec<UdfDefinition>,
}

impl UdfRegistry {
    /// Create a new UDF registry
    pub fn new() -> Self;

    /// Get all UDF definitions
    pub fn get_definitions(&self) -> &[UdfDefinition];

    /// Get UDF definitions by category
    pub fn get_by_category(&self, category: &UdfCategory) -> Vec<&UdfDefinition>;

    /// Get UDF definition by name
    pub fn get_by_name(&self, name: &str) -> Option<&UdfDefinition>;

    /// Register all UDFs with DataFusion context
    pub fn register_all_udfs(&self, ctx: &SessionContext) -> Result<()>;
}
```

### UDF Categories

UDFs are organized into categories for better management and UI organization:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UdfCategory {
    ShaleVolume,    // Shale volume calculations
    Porosity,       // Porosity calculations
    Saturation,     // Water saturation calculations
    Statistics,     // Statistical analysis
    UnitConversion, // Unit conversions
    Quality,        // Data quality assessment
}
```

### UDF Definition

Each UDF is defined with comprehensive metadata:

```rust
#[derive(Debug, Clone)]
pub struct UdfDefinition {
    pub name: String,                    // Function name
    pub description: String,             // Human-readable description
    pub input_types: Vec<DataType>,      // Input parameter types
    pub output_type: DataType,           // Return type
    pub volatility: Volatility,          // Function volatility (Immutable/Stable/Volatile)
    pub category: UdfCategory,           // UDF category
    pub tags: Vec<String>,               // Searchable tags
}
```

## Vectorized Calculation Trait

All petrophysical functions implement the `VectorizedCalculation` trait:

```rust
pub trait VectorizedCalculation {
    /// Calculate the function for a single value
    fn calculate_single(&self, args: &[f64]) -> Result<f64>;

    /// Calculate the function for an array of values (vectorized)
    fn calculate_vectorized(&self, args: &[ArrayRef]) -> Result<ArrayRef>;

    /// Get the function name
    fn name(&self) -> &str;

    /// Get the number of input arguments
    fn input_count(&self) -> usize;
}
```

## Available UDFs

### Shale Volume Calculations

| Function             | Description                           | Input Types               | Output Type |
| -------------------- | ------------------------------------- | ------------------------- | ----------- |
| `linear_vsh`         | Linear shale volume calculation       | Float64, Float64, Float64 | Float64     |
| `larionov_old_vsh`   | Larionov shale volume for old rocks   | Float64, Float64, Float64 | Float64     |
| `larionov_young_vsh` | Larionov shale volume for young rocks | Float64, Float64, Float64 | Float64     |
| `clavier_vsh`        | Clavier shale volume calculation      | Float64, Float64, Float64 | Float64     |
| `steiber_vsh`        | Steiber shale volume calculation      | Float64, Float64, Float64 | Float64     |

### Porosity Calculations

| Function                       | Description                           | Input Types      | Output Type |
| ------------------------------ | ------------------------------------- | ---------------- | ----------- |
| `density_porosity_sandstone`   | Density porosity for sandstone matrix | Float64          | Float64     |
| `density_porosity_limestone`   | Density porosity for limestone matrix | Float64          | Float64     |
| `density_porosity_dolomite`    | Density porosity for dolomite matrix  | Float64          | Float64     |
| `neutron_porosity`             | Neutron porosity calculation          | Float64          | Float64     |
| `sonic_porosity`               | Sonic porosity calculation            | Float64          | Float64     |
| `density_neutron_avg_porosity` | Average density-neutron porosity      | Float64, Float64 | Float64     |
| `density_neutron_rms_porosity` | RMS density-neutron porosity          | Float64, Float64 | Float64     |

### Saturation Calculations

| Function          | Description                   | Input Types               | Output Type |
| ----------------- | ----------------------------- | ------------------------- | ----------- |
| `archie_sw`       | Archie water saturation       | Float64, Float64          | Float64     |
| `simandoux_sw`    | Simandoux water saturation    | Float64, Float64, Float64 | Float64     |
| `indonesia_sw`    | Indonesia water saturation    | Float64, Float64, Float64 | Float64     |
| `waxman_smits_sw` | Waxman-Smits water saturation | Float64, Float64, Float64 | Float64     |

### Statistical Functions

| Function           | Description                          | Input Types      | Output Type |
| ------------------ | ------------------------------------ | ---------------- | ----------- |
| `petro_mean`       | Calculate mean of petrophysical data | Float64          | Float64     |
| `petro_std_dev`    | Calculate standard deviation         | Float64          | Float64     |
| `petro_median`     | Calculate median                     | Float64          | Float64     |
| `petro_percentile` | Calculate percentile                 | Float64, Float64 | Float64     |

### Unit Conversions

| Function                | Description                   | Input Types | Output Type |
| ----------------------- | ----------------------------- | ----------- | ----------- |
| `celsius_to_fahrenheit` | Convert Celsius to Fahrenheit | Float64     | Float64     |
| `feet_to_meters`        | Convert feet to meters        | Float64     | Float64     |
| `gcc_to_lbft3`          | Convert g/cc to lb/ft³        | Float64     | Float64     |

## Usage Examples

### Basic Registration

```rust
use udf_core::register_all_udfs;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create DataFusion context
    let ctx = SessionContext::new();

    // Register all UDFs with a single function call
    register_all_udfs(&ctx)?;

    // Now you can use UDFs in SQL queries
    let df = ctx.sql("SELECT linear_vsh(gr, gr_clean, gr_shale) as vsh FROM well_data").await?;
    df.show().await?;

    Ok(())
}
```

### Advanced Registration (Using Registry)

```rust
use udf_core::{UdfRegistry, create_default_registry};
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create DataFusion context
    let ctx = SessionContext::new();

    // Create and register all UDFs with full control
    let registry = create_default_registry();
    registry.register_all_udfs(&ctx)?;

    // Now you can use UDFs in SQL queries
    let df = ctx.sql("SELECT linear_vsh(gr, gr_clean, gr_shale) as vsh FROM well_data").await?;
    df.show().await?;

    Ok(())
}
```

### Using DataFrame API

```rust
use datafusion::prelude::*;
use udf_core::register_all_udfs;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();
    register_all_udfs(&ctx)?;

    // Register a data source
    ctx.register_parquet("well_data", "data/well_logs.parquet").await?;

    // Use UDFs in DataFrame operations
    let df = ctx.table("well_data").await?
        .select(vec![
            col("depth"),
            col("gr"),
            col("gr_clean"),
            col("gr_shale"),
            // Use UDF in select
            call_fn("linear_vsh", vec![col("gr"), col("gr_clean"), col("gr_shale")])
                .alias("vsh")
        ])?;

    df.show().await?;
    Ok(())
}
```

### Getting UDF Information

```rust
use udf_core::{create_default_registry, get_udf_names, get_udf_names_by_category, UdfCategory};

// Get all UDF names
let all_udfs = get_udf_names();
println!("Available UDFs: {:?}", all_udfs);

// Get UDFs by category
let udfs_by_category = get_udf_names_by_category();
let shale_volume_udfs = udfs_by_category.get(&UdfCategory::ShaleVolume);
println!("Shale volume UDFs: {:?}", shale_volume_udfs);

// Get specific UDF definition
let registry = create_default_registry();
if let Some(udf_def) = registry.get_by_name("linear_vsh") {
    println!("UDF: {}", udf_def.name);
    println!("Description: {}", udf_def.description);
    println!("Input types: {:?}", udf_def.input_types);
    println!("Output type: {:?}", udf_def.output_type);
}
```

## Performance Optimizations

### Vectorized Operations

All UDFs use vectorized operations that leverage:

- **Arrow compute kernels** - High-performance, SIMD-optimized operations
- **Columnar processing** - Process entire columns at once
- **Memory efficiency** - Minimize data copying and allocations
- **Parallel processing** - Automatic parallelization where possible

### Example: Linear Shale Volume

```rust
// Vectorized implementation using Arrow compute kernels
let numerator: Float64Array = compute::binary(
    gr_array,
    gr_clean_array,
    |gr: f64, gr_clean: f64| gr - gr_clean,
)?;
let denominator: Float64Array = compute::binary(
    gr_shale_array,
    gr_clean_array,
    |gr_shale: f64, gr_clean: f64| gr_shale - gr_clean,
)?;
let result: Float64Array = compute::binary(
    &numerator,
    &denominator,
    |num: f64, den: f64| if den != 0.0 { (num / den).clamp(0.0, 1.0) } else { 0.0 },
)?;
```

## Error Handling

The system provides comprehensive error handling with automatic conversion between petrophysics and DataFusion error types:

```rust
fn convert_error(err: PetrophysicsError) -> datafusion::error::DataFusionError {
    datafusion::error::DataFusionError::External(Box::new(err))
}
```

Common error scenarios:

- **Invalid input arguments** - Wrong number or types of arguments
- **Mathematical errors** - Division by zero, invalid ranges
- **Array length mismatches** - Inconsistent array lengths in vectorized operations
- **Type conversion errors** - Unsupported data types

## Type Safety

The system ensures type safety through:

1. **Compile-time type checking** - Rust's type system prevents type mismatches
2. **Runtime validation** - Input validation ensures correct argument types and counts
3. **Arrow type system** - Leverages Arrow's strong typing for data operations
4. **Function signatures** - Each UDF has well-defined input/output types

### Example: Type Safety in Action

```rust
// This will fail at compile time - wrong number of arguments
let result = calc.calculate_single(&[60.0, 20.0]); // Missing third argument

// This will fail at runtime - wrong array lengths
let gr = Float64Array::from(vec![1.0, 2.0, 3.0]);
let gr_clean = Float64Array::from(vec![1.0, 2.0]); // Different length
let result = calc.calculate_vectorized(&[gr, gr_clean, gr_shale])?; // Error!
```

## Best Practices

1. **Always use vectorized UDFs** - Never register unvectorized versions
2. **Handle errors gracefully** - Use proper error handling in your applications
3. **Validate inputs** - Check data quality before processing
4. **Use appropriate categories** - Organize UDFs by their purpose
5. **Leverage the registry** - Use the centralized registry for UDF management
6. **Test thoroughly** - Test UDFs with various input scenarios

## Recent Improvements

### Version Conflict Resolution

We've resolved DataFusion version conflicts by:

1. **Unified DataFusion version** - All crates now use DataFusion v50.2.0
2. **Workspace dependency management** - Centralized version control in workspace Cargo.toml
3. **Patch overrides** - Forced all transitive dependencies to use DataFusion v50.2.0
4. **Eliminated duplicate compilation** - Only one version of DataFusion is compiled

### Enhanced Error Handling

- **Better error context** - More descriptive error messages with UDF context
- **Automatic error conversion** - Seamless conversion between petrophysics and DataFusion errors
- **Type-safe error handling** - Comprehensive error types throughout the stack

### Simplified API

- **Convenience function** - `register_all_udfs(&ctx)` for simple registration
- **Centralized registry** - `UdfRegistry` for advanced UDF management
- **Consistent patterns** - All UDFs follow the same registration pattern

## Future Enhancements

- **Dynamic UDF loading** - Load UDFs at runtime
- **UDF versioning** - Support multiple versions of the same UDF
- **Performance monitoring** - Built-in performance metrics
- **Custom categories** - User-defined UDF categories
- **UDF dependencies** - Support for UDFs that depend on other UDFs
- **WASM sandboxing** - Secure execution of untrusted UDFs
- **UDF catalog UI** - Web interface for browsing and testing UDFs

## DAG Operator Integration

To make UDFs available as DAG operators:

1. Registry bridge

- Mirror UDF metadata into the `OperatorRegistry` (or store in `operator_registry` table). Each UDF becomes an `OperatorDefinition` with `implementation_type = "udf"`.

2. Parameters and schemas

- For UDFs requiring configuration (e.g., method variants), define parameter structs and derive `schemars::JsonSchema` to generate JSON Schema served to the UI.

3. Tauri wiring

- Implement `get_available_operators` to list operators (backed by built-ins or DB). The UI uses this to render the DAG operator library.

4. Execution

- During `DagExecutor::execute`, register UDFs with DataFusion `SessionContext`, build expressions from operator parameters, and evaluate on incoming `RecordBatch`.

See `markdown-guides/to-do/dag-execution.md` for crate structure and Tauri command details.
