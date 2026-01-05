# DataFusion UDF Best Practices and Performance Optimizations

This document outlines the best practices for implementing User-Defined Functions (UDFs) in DataFusion, based on our implementation of vectorized petrophysical calculations in MudRock.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Performance Optimization Strategies](#performance-optimization-strategies)
3. [Code Organization and Reusability](#code-organization-and-reusability)
4. [Type Safety and Error Handling](#type-safety-and-error-handling)
5. [Memory Management](#memory-management)
6. [Testing and Validation](#testing-and-validation)
7. [Registry and Discovery](#registry-and-discovery)
8. [UI Integration Guidelines](#ui-integration-guidelines)

## Architecture Overview

### Centralized Registry System

Our implementation uses a centralized UDF registry that ensures:

- **Single source of truth** for all UDF definitions
- **No duplication** between vectorized and non-vectorized versions
- **Consistent naming** and categorization
- **Easy discovery** for UI integration

```rust
// Central registry with metadata
pub struct UdfDefinition {
    pub name: String,
    pub description: String,
    pub input_types: Vec<DataType>,
    pub output_type: DataType,
    pub volatility: Volatility,
    pub category: UdfCategory,
    pub tags: Vec<String>,
}
```

### Vectorized-First Approach

All UDFs are implemented using vectorized operations for maximum performance:

- **Arrow compute kernels** for SIMD operations
- **Batch processing** instead of element-wise operations
- **Memory reuse** to minimize allocations
- **Parallel execution** through DataFusion's execution engine

## Performance Optimization Strategies

### 1. Use Arrow Compute Kernels

**✅ Good:**

```rust
// Vectorized calculation using Arrow compute
let result = compute::binary(
    gr_array,
    gr_clean_array,
    |gr, gr_clean| gr - gr_clean,
)?;
```

**❌ Avoid:**

```rust
// Element-wise processing (slower)
let mut result = Vec::new();
for i in 0..array.len() {
    result.push(array.value(i) - clean.value(i));
}
```

### 2. Implement VectorizedCalculation Trait

Create a generic trait for all petrophysical calculations:

```rust
pub trait VectorizedCalculation {
    fn calculate_single(&self, args: &[f64]) -> Result<f64>;
    fn calculate_vectorized(&self, args: &[ArrayRef]) -> Result<ArrayRef>;
    fn name(&self) -> &str;
    fn input_count(&self) -> usize;
}
```

**Benefits:**

- **Code reuse** across different calculation types
- **Consistent interface** for all UDFs
- **Easy testing** with single-value inputs
- **Automatic vectorization** through trait implementation

### 3. Optimize for Common Cases

**Array + Scalar operations:**

```rust
// Special case for array + scalar (very common)
match (base, exp) {
    (ColumnarValue::Array(base_array), ColumnarValue::Scalar(exp)) => {
        let result = compute::unary(base_array, |base| base.powf(exp))?;
        Ok(ColumnarValue::Array(Arc::new(result)))
    }
    // ... other cases
}
```

**Array + Array operations:**

```rust
// Use binary compute for array operations
let result = compute::binary(
    array1,
    array2,
    |a, b| a + b,
)?;
```

### 4. Memory Management

**Reuse Arrow arrays when possible:**

```rust
// Try to reuse memory
match compute::unary_mut(owned_array, |x| x * 2.0) {
    Ok(result) => Ok(Arc::new(result)), // Reused memory
    Err(original) => {
        // Fall back to new allocation
        let result = compute::unary(&original, |x| x * 2.0)?;
        Ok(Arc::new(result))
    }
}
```

**Avoid unnecessary clones:**

```rust
// ✅ Good - reference only
let array_ref = as_float64_array(&args[0])?;

// ❌ Avoid - unnecessary clone
let array = args[0].clone();
```

## Code Organization and Reusability

### 1. Separate Concerns

**Calculation Logic (petrophysics crate):**

```rust
// Pure calculation functions
pub fn linear_vsh(gr: f64, gr_clean: f64, gr_shale: f64) -> f64 {
    if gr_shale <= gr_clean {
        return 0.0;
    }
    (gr - gr_clean) / (gr_shale - gr_clean)
}
```

**UDF Wrapper (udf-core crate):**

```rust
// UDF registration and DataFusion integration
let calc = VectorizedShaleVolume::new(ShaleVolumeMethod::Linear);
let udf = create_udf(
    "linear_vsh",
    vec![DataType::Float64, DataType::Float64, DataType::Float64],
    DataType::Float64,
    Volatility::Immutable,
    Arc::new(create_vectorized_udf(calc)),
);
```

### 2. Generic UDF Factory

Create a generic factory for vectorized UDFs:

```rust
pub fn create_vectorized_udf<C: VectorizedCalculation + Send + Sync + 'static>(
    calc: C,
) -> impl Fn(&[ColumnarValue]) -> Result<ColumnarValue> {
    move |args: &[ColumnarValue]| {
        let arrays: Result<Vec<ArrayRef>> = args.iter()
            .map(|arg| match arg {
                ColumnarValue::Array(arr) => Ok(arr.clone()),
                ColumnarValue::Scalar(scalar) => {
                    // Convert scalar to array
                    match scalar {
                        ScalarValue::Float64(Some(val)) => {
                            Ok(Arc::new(Float64Array::from(vec![*val])) as ArrayRef)
                        }
                        _ => Err(PetrophysicsError::InvalidInput("Unsupported scalar type".to_string())),
                    }
                }
            })
            .collect();

        let arrays = arrays?;
        let result = calc.calculate_vectorized(&arrays)?;
        Ok(ColumnarValue::Array(result))
    }
}
```

### 3. Registry-Based Registration

Use a centralized registry to avoid duplication:

```rust
// Single registry with all UDF definitions
let registry = create_default_registry();
registry.register_all_udfs(ctx)?;

// Easy discovery for UI
let udf_names = get_udf_names();
let udf_by_category = get_udf_names_by_category();
```

## Type Safety and Error Handling

### 1. Leverage DataFusion's Type System

**Use proper DataType definitions:**

```rust
// ✅ Good - explicit types
vec![DataType::Float64, DataType::Float64, DataType::Float64]

// ❌ Avoid - generic types
vec![DataType::Float64; 3]
```

**Validate input types:**

```rust
fn calculate_vectorized(&self, args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != self.input_count() {
        return Err(PetrophysicsError::InvalidInput(
            format!("Expected {} arguments, got {}", self.input_count(), args.len())
        ));
    }

    // Type checking is handled by DataFusion
    let array = as_float64_array(&args[0])?;
    // ...
}
```

### 2. Comprehensive Error Handling

**Use custom error types:**

```rust
#[derive(Debug, thiserror::Error)]
pub enum PetrophysicsError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Math error: {0}")]
    MathError(String),
    #[error("Data quality error: {0}")]
    DataQualityError(String),
}
```

**Handle edge cases:**

```rust
// Validate inputs
if gr_shale <= gr_clean {
    return Err(PetrophysicsError::MathError(
        "GR_shale must be greater than GR_clean".to_string()
    ));
}

// Handle NaN and infinite values
if result.is_nan() || result.is_infinite() {
    return Ok(ColumnarValue::Scalar(ScalarValue::Float64(None)));
}
```

## Memory Management

### 1. Minimize Allocations

**Reuse Arrow arrays:**

```rust
// Try to reuse existing memory
match compute::unary_mut(owned_array, |x| x * 2.0) {
    Ok(result) => Ok(Arc::new(result)),
    Err(original) => {
        // Fall back to new allocation
        let result = compute::unary(&original, |x| x * 2.0)?;
        Ok(Arc::new(result))
    }
}
```

**Use Arc for sharing:**

```rust
// Share arrays without copying
let result = Arc::new(compute::binary(array1, array2, |a, b| a + b)?);
```

### 2. Batch Processing

**Process entire arrays at once:**

```rust
// ✅ Good - vectorized processing
let result = compute::unary(input_array, |x| x * 2.0)?;

// ❌ Avoid - element-wise processing
let mut result = Vec::new();
for i in 0..input_array.len() {
    result.push(input_array.value(i) * 2.0);
}
```

## Testing and Validation

### 1. Test Both Single and Vectorized Paths

```rust
#[test]
fn test_vectorized_calculation() {
    let calc = VectorizedShaleVolume::new(ShaleVolumeMethod::Linear);

    // Test single calculation
    let result = calc.calculate_single(&[60.0, 20.0, 100.0]).unwrap();
    assert_relative_eq!(result, 0.5, epsilon = 1e-6);

    // Test vectorized calculation
    let gr = Arc::new(Float64Array::from(vec![20.0, 60.0, 100.0]));
    let gr_clean = Arc::new(Float64Array::from(vec![20.0, 20.0, 20.0]));
    let gr_shale = Arc::new(Float64Array::from(vec![100.0, 100.0, 100.0]));

    let result = calc.calculate_vectorized(&[gr, gr_clean, gr_shale]).unwrap();
    let result_array = result.as_any().downcast_ref::<Float64Array>().unwrap();

    assert_relative_eq!(result_array.value(0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(result_array.value(1), 0.5, epsilon = 1e-6);
    assert_relative_eq!(result_array.value(2), 1.0, epsilon = 1e-6);
}
```

### 2. Performance Benchmarks

```rust
#[test]
fn test_performance_benchmark() {
    let calc = VectorizedShaleVolume::new(ShaleVolumeMethod::Linear);

    // Create large arrays
    let size = 1_000_000;
    let gr = Arc::new(Float64Array::from((0..size).map(|i| i as f64).collect::<Vec<_>>()));
    let gr_clean = Arc::new(Float64Array::from(vec![20.0; size]));
    let gr_shale = Arc::new(Float64Array::from(vec![100.0; size]));

    let start = std::time::Instant::now();
    let _result = calc.calculate_vectorized(&[gr, gr_clean, gr_shale]).unwrap();
    let duration = start.elapsed();

    println!("Processed {} elements in {:?}", size, duration);
    assert!(duration.as_millis() < 100); // Should be fast
}
```

## Registry and Discovery

### 1. Centralized UDF Registry

```rust
// Single registry with all UDF metadata
pub struct UdfRegistry {
    definitions: Vec<UdfDefinition>,
}

impl UdfRegistry {
    pub fn register_all_udfs(&self, ctx: &SessionContext) -> Result<()> {
        // Register all UDFs from definitions
        for def in &self.definitions {
            let udf = self.create_udf_from_definition(def)?;
            ctx.register_udf(udf);
        }
        Ok(())
    }
}
```

### 2. UI-Friendly Discovery

```rust
// Get UDF names for UI dropdown
pub fn get_udf_names() -> Vec<String> {
    let registry = create_default_registry();
    registry.definitions.iter().map(|def| def.name.clone()).collect()
}

// Get UDFs by category for organized UI
pub fn get_udf_names_by_category() -> HashMap<UdfCategory, Vec<String>> {
    let registry = create_default_registry();
    let mut result = HashMap::new();

    for category in [UdfCategory::ShaleVolume, UdfCategory::Porosity, ...] {
        let names: Vec<String> = registry.get_by_category(category)
            .iter()
            .map(|def| def.name.clone())
            .collect();
        result.insert(category, names);
    }

    result
}
```

## UI Integration Guidelines

### 1. Function Signatures for UI

**Display format:**

```
linear_vsh(gr: Float64, gr_clean: Float64, gr_shale: Float64) -> Float64
```

**Implementation:**

```rust
pub struct UdfDefinition {
    pub name: String,
    pub description: String,
    pub input_types: Vec<DataType>,
    pub output_type: DataType,
    // ... other fields
}

// Generate UI-friendly signature
impl UdfDefinition {
    pub fn signature(&self) -> String {
        let inputs = self.input_types.iter()
            .enumerate()
            .map(|(i, dt)| format!("arg{}: {}", i, dt))
            .collect::<Vec<_>>()
            .join(", ");

        format!("{}({}) -> {}", self.name, inputs, self.output_type)
    }
}
```

### 2. Categorization for UI

```rust
pub enum UdfCategory {
    ShaleVolume,
    Porosity,
    Saturation,
    Statistics,
    UnitConversion,
    Quality,
}

// UI can organize functions by category
let shale_volume_udfs = get_udf_names_by_category().get(&UdfCategory::ShaleVolume);
```

### 3. Parameter Validation

**UI should validate parameters based on UDF definition:**

```rust
// UI can check parameter types before sending to DataFusion
fn validate_udf_call(udf_name: &str, args: &[Value]) -> Result<()> {
    let registry = create_default_registry();
    let definition = registry.get_by_name(udf_name)
        .ok_or_else(|| Error::UdfNotFound(udf_name.to_string()))?;

    if args.len() != definition.input_types.len() {
        return Err(Error::InvalidArgumentCount {
            expected: definition.input_types.len(),
            actual: args.len(),
        });
    }

    // Validate types
    for (i, (arg, expected_type)) in args.iter().zip(definition.input_types.iter()).enumerate() {
        if !arg.is_compatible_with(expected_type) {
            return Err(Error::InvalidArgumentType {
                index: i,
                expected: expected_type.clone(),
                actual: arg.get_type(),
            });
        }
    }

    Ok(())
}
```

## Summary

Our DataFusion UDF implementation follows these key principles:

1. **Vectorized-first**: All UDFs use Arrow compute kernels for maximum performance
2. **Centralized registry**: Single source of truth for all UDF definitions
3. **Type safety**: Leverage DataFusion's type system for compile-time safety
4. **Code reuse**: Generic traits and factories eliminate duplication
5. **UI-friendly**: Rich metadata for easy discovery and validation
6. **Memory efficient**: Minimize allocations and reuse Arrow arrays
7. **Comprehensive testing**: Test both single and vectorized paths

This approach ensures high performance, maintainability, and ease of integration with user interfaces while providing a solid foundation for petrophysical analysis workflows.

## Using UDFs as DAG Operator Nodes (Integration with dag-execution.md)

To expose your existing DataFusion UDFs as first-class DAG operator nodes:

1. Operator registry (backend)

- Create `crates/dags/operators` with `schemars` to generate JSON Schemas for operator parameters.
- Map UDF metadata (name, input/output types) to `OperatorDefinition` entries. Example categories: `subsurface`, `signal`, `wells`, `quality`.

2. Discovery for UI

- Tauri command `get_available_operators(category?)` should surface operator list built from the registry (or DB table `operator_registry`).
- Reuse existing UDF discovery (see `list-registered-udfs-in-svelte-component.md`) to populate the DAG operator palette.

3. Execution mapping

- In `DagExecutor::execute_node` for `NodeType::Operator`, route to DataFusion by registering relevant UDFs in a `SessionContext` and building a projection/expr using the operator parameters.
- Optionally support `implementation_type: "native_rust" | "udf" | "wasm"` in the operator definition; for UDFs use `udf-core` registration before evaluation.

4. Parameter validation

- Use `schemars`-derived JSON Schemas in the UI to render/validate forms for operator parameters (e.g., method selection, numeric thresholds).

5. Storage alignment

- If operators materialize outputs, use the OpenDAL storage pattern described in `opendal-mudrock-storage-manager.md`. For transient execution, return Arrow `RecordBatch` to the next node.

For the full modular architecture and wiring, see `markdown-guides/to-do/dag-execution.md` (operators crate, validator, storage, executor, and Tauri commands).
