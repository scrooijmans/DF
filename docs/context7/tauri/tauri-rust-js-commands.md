# Tauri: Calling Rust Functions from JavaScript

## Overview

Tauri provides a command system that allows JavaScript/TypeScript frontends to call Rust functions (commands) and receive return values. This is essential for your MVP where Rust UDFs (User Defined Functions) need to be called from your SvelteKit frontend.

## Core Concepts

- **Commands**: Rust functions marked with `#[tauri::command]` that can be invoked from JavaScript
- **Invoke**: JavaScript function to call Rust commands
- **Serialization**: Automatic JSON serialization/deserialization via Serde
- **Error Handling**: Rust `Result` types map to JavaScript promises (resolve/reject)

## Basic Setup

### 1. Define Rust Command

```rust
// src-tauri/src/main.rs or src-tauri/src/lib.rs

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

### 2. Register Command

```rust
// src-tauri/src/main.rs

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Call from JavaScript

```typescript
import { invoke } from '@tauri-apps/api/core';

// Call the command
const message = await invoke('greet', { name: 'World' });
console.log(message); // "Hello, World! You've been greeted from Rust!"
```

## Command Arguments

### Simple Arguments

```rust
#[tauri::command]
fn calculate_shale_volume(
    gamma_ray: f64,
    gr_min: f64,
    gr_max: f64,
    method: String, // "larionov" | "clavier" | "steiber"
) -> f64 {
    // Your petrophysical calculation
    match method.as_str() {
        "larionov" => {
            let igr = (gamma_ray - gr_min) / (gr_max - gr_min);
            0.33 * (2.0_f64.powf(2.0 * igr) - 1.0)
        }
        "clavier" => {
            let igr = (gamma_ray - gr_min) / (gr_max - gr_min);
            1.7 - (3.38 - (igr + 0.7).powi(2)).sqrt()
        }
        _ => 0.0,
    }
}
```

```typescript
import { invoke } from '@tauri-apps/api/core';

const vsh = await invoke<number>('calculate_shale_volume', {
    gammaRay: 75.5,
    grMin: 20.0,
    grMax: 120.0,
    method: 'larionov',
});

console.log(`Shale Volume: ${vsh}`);
```

### Complex Arguments (Structs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct PorosityInput {
    neutron: f64,
    density: f64,
    matrix_density: f64,
    fluid_density: f64,
    method: String,
}

#[tauri::command]
fn calculate_porosity(input: PorosityInput) -> f64 {
    match input.method.as_str() {
        "neutron_density" => {
            // Neutron-Density porosity calculation
            (input.neutron + input.density) / 2.0
        }
        "density_only" => {
            (input.matrix_density - input.density) 
                / (input.matrix_density - input.fluid_density)
        }
        _ => 0.0,
    }
}
```

```typescript
import { invoke } from '@tauri-apps/api/core';

interface PorosityInput {
    neutron: number;
    density: number;
    matrixDensity: number;
    fluidDensity: number;
    method: string;
}

const porosity = await invoke<number>('calculate_porosity', {
    neutron: 0.25,
    density: 2.35,
    matrixDensity: 2.65,
    fluidDensity: 1.0,
    method: 'neutron_density',
} as PorosityInput);
```

### Array Arguments

```rust
#[tauri::command]
fn process_curve(depths: Vec<f64>, values: Vec<f64>) -> Vec<f64> {
    // Process curve data (e.g., smoothing, filtering)
    values
        .iter()
        .map(|v| v * 1.1) // Example: scale values
        .collect()
}
```

```typescript
import { invoke } from '@tauri-apps/api/core';

const processed = await invoke<number[]>('process_curve', {
    depths: [1000.0, 1001.0, 1002.0],
    values: [50.0, 52.0, 48.0],
});
```

## Return Values

### Simple Return Types

```rust
#[tauri::command]
fn get_operator_list() -> Vec<String> {
    vec![
        "shale_volume".to_string(),
        "porosity".to_string(),
        "saturation".to_string(),
    ]
}
```

```typescript
const operators = await invoke<string[]>('get_operator_list');
```

### Complex Return Types

```rust
#[derive(Serialize)]
struct OperatorInfo {
    name: String,
    category: String,
    description: String,
    parameters: Vec<Parameter>,
}

#[derive(Serialize)]
struct Parameter {
    name: String,
    param_type: String, // "f64" | "String" | "Vec<f64>"
    required: bool,
}

#[tauri::command]
fn get_operator_info(operator_name: String) -> OperatorInfo {
    OperatorInfo {
        name: operator_name.clone(),
        category: "subsurface".to_string(),
        description: "Calculates shale volume from gamma ray".to_string(),
        parameters: vec![
            Parameter {
                name: "gamma_ray".to_string(),
                param_type: "f64".to_string(),
                required: true,
            },
            Parameter {
                name: "gr_min".to_string(),
                param_type: "f64".to_string(),
                required: true,
            },
        ],
    }
}
```

```typescript
interface OperatorInfo {
    name: string;
    category: string;
    description: string;
    parameters: Array<{
        name: string;
        paramType: string;
        required: boolean;
    }>;
}

const info = await invoke<OperatorInfo>('get_operator_info', {
    operatorName: 'shale_volume',
});
```

## Error Handling

### Using Result Type

```rust
#[tauri::command]
fn calculate_saturation(
    porosity: f64,
    water_resistivity: f64,
    formation_resistivity: f64,
    cementation_exponent: f64,
) -> Result<f64, String> {
    if porosity <= 0.0 || porosity >= 1.0 {
        return Err("Porosity must be between 0 and 1".to_string());
    }
    
    if water_resistivity <= 0.0 || formation_resistivity <= 0.0 {
        return Err("Resistivity values must be positive".to_string());
    }
    
    // Archie equation
    let sw = (water_resistivity / formation_resistivity 
        / porosity.powf(cementation_exponent)).sqrt();
    
    Ok(sw.min(1.0).max(0.0))
}
```

```typescript
import { invoke } from '@tauri-apps/api/core';

try {
    const sw = await invoke<number>('calculate_saturation', {
        porosity: 0.25,
        waterResistivity: 0.1,
        formationResistivity: 5.0,
        cementationExponent: 2.0,
    });
    console.log(`Water Saturation: ${sw}`);
} catch (error) {
    console.error('Calculation error:', error);
    // Error is the string from Rust: "Porosity must be between 0 and 1"
}
```

### Custom Error Types

```rust
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
enum CalculationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Division by zero in {0}")]
    DivisionByZero(String),
    #[error("Out of range: {0}")]
    OutOfRange(String),
}

#[tauri::command]
fn calculate_permeability(
    porosity: f64,
    irreducible_water: f64,
) -> Result<f64, CalculationError> {
    if porosity <= 0.0 {
        return Err(CalculationError::InvalidInput(
            "Porosity must be positive".to_string(),
        ));
    }
    
    if irreducible_water <= 0.0 {
        return Err(CalculationError::DivisionByZero(
            "Irreducible water saturation".to_string(),
        ));
    }
    
    // Timur equation
    let perm = 0.136 * porosity.powf(4.4) / irreducible_water.powf(2.0);
    Ok(perm)
}
```

```typescript
interface CalculationError {
    kind: 'InvalidInput' | 'DivisionByZero' | 'OutOfRange';
    message: string;
}

try {
    const perm = await invoke<number>('calculate_permeability', {
        porosity: 0.25,
        irreducibleWater: 0.15,
    });
} catch (error: CalculationError) {
    switch (error.kind) {
        case 'InvalidInput':
            // Handle invalid input
            break;
        case 'DivisionByZero':
            // Handle division by zero
            break;
    }
}
```

## Async Commands

### Async Rust Functions

```rust
use tokio::time::{sleep, Duration};

#[tauri::command]
async fn process_large_dataset(
    data: Vec<f64>,
    window_size: usize,
) -> Vec<f64> {
    // Simulate long-running computation
    sleep(Duration::from_millis(100)).await;
    
    // Moving average filter
    data.windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window_size as f64)
        .collect()
}
```

```typescript
// Async commands return promises automatically
const filtered = await invoke<number[]>('process_large_dataset', {
    data: [1.0, 2.0, 3.0, 4.0, 5.0],
    windowSize: 3,
});
```

### Async with Database/File Operations

```rust
use std::fs;

#[tauri::command]
async fn read_parquet_file(file_path: String) -> Result<Vec<u8>, String> {
    tokio::task::spawn_blocking(move || {
        fs::read(&file_path)
            .map_err(|e| format!("Failed to read file: {}", e))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}
```

```typescript
try {
    const fileData = await invoke<number[]>('read_parquet_file', {
        filePath: '/path/to/data.parquet',
    });
    // Process file data
} catch (error) {
    console.error('File read error:', error);
}
```

## UDF Examples for Your MVP

### Petrophysical UDFs

```rust
// src-tauri/src/operators/subsurface/petrophysics.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ShaleVolumeParams {
    gamma_ray: Vec<f64>,
    gr_min: f64,
    gr_max: f64,
    method: String,
}

#[derive(Debug, Serialize)]
struct ShaleVolumeResult {
    vsh: Vec<f64>,
    method_used: String,
}

#[tauri::command]
fn calculate_shale_volume_batch(
    params: ShaleVolumeParams,
) -> Result<ShaleVolumeResult, String> {
    if params.gamma_ray.is_empty() {
        return Err("Gamma ray data is empty".to_string());
    }
    
    let vsh: Vec<f64> = params
        .gamma_ray
        .iter()
        .map(|&gr| {
            let igr = (gr - params.gr_min) / (params.gr_max - params.gr_min);
            match params.method.as_str() {
                "larionov" => 0.33 * (2.0_f64.powf(2.0 * igr) - 1.0),
                "clavier" => {
                    let term = (igr + 0.7).powi(2);
                    1.7 - (3.38 - term).sqrt()
                }
                "steiber" => igr / (3.0 - 2.0 * igr),
                _ => return Err(format!("Unknown method: {}", params.method)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(ShaleVolumeResult {
        vsh,
        method_used: params.method,
    })
}
```

```typescript
// src/lib/operators/shale-volume.ts

import { invoke } from '@tauri-apps/api/core';

interface ShaleVolumeParams {
    gammaRay: number[];
    grMin: number;
    grMax: number;
    method: 'larionov' | 'clavier' | 'steiber';
}

interface ShaleVolumeResult {
    vsh: number[];
    methodUsed: string;
}

export async function calculateShaleVolume(
    params: ShaleVolumeParams,
): Promise<ShaleVolumeResult> {
    return await invoke<ShaleVolumeResult>('calculate_shale_volume_batch', {
        gammaRay: params.gammaRay,
        grMin: params.grMin,
        grMax: params.grMax,
        method: params.method,
    });
}
```

### Signal Processing UDFs

```rust
// src-tauri/src/operators/signal/filters.rs

#[tauri::command]
fn moving_average(
    data: Vec<f64>,
    window_size: usize,
) -> Result<Vec<f64>, String> {
    if window_size == 0 || window_size > data.len() {
        return Err("Invalid window size".to_string());
    }
    
    let result: Vec<f64> = data
        .windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window_size as f64)
        .collect();
    
    // Pad beginning with first value
    let mut padded = vec![result[0]; window_size - 1];
    padded.extend(result);
    
    Ok(padded)
}
```

```typescript
// src/lib/operators/filters.ts

import { invoke } from '@tauri-apps/api/core';

export async function movingAverage(
    data: number[],
    windowSize: number,
): Promise<number[]> {
    return await invoke<number[]>('moving_average', {
        data,
        windowSize,
    });
}
```

### Pipeline Execution

```rust
// src-tauri/src/pipelines/executor.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct PipelineNode {
    id: String,
    operator: String,
    inputs: Vec<String>, // IDs of input nodes
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct Pipeline {
    nodes: Vec<PipelineNode>,
    input_data: std::collections::HashMap<String, Vec<f64>>,
}

#[derive(Debug, Serialize)]
struct PipelineResult {
    outputs: std::collections::HashMap<String, Vec<f64>>,
    execution_time_ms: u64,
}

#[tauri::command]
async fn execute_pipeline(pipeline: Pipeline) -> Result<PipelineResult, String> {
    let start = std::time::Instant::now();
    
    // Execute pipeline nodes in topological order
    let mut results = pipeline.input_data.clone();
    
    for node in pipeline.nodes {
        // Get input data
        let inputs: Vec<Vec<f64>> = node
            .inputs
            .iter()
            .map(|id| {
                results
                    .get(id)
                    .ok_or_else(|| format!("Missing input: {}", id))
                    .cloned()
            })
            .collect::<Result<_, _>>()?;
        
        // Execute operator (simplified)
        let output = match node.operator.as_str() {
            "shale_volume" => {
                // Execute shale volume calculation
                vec![0.5; 100] // Placeholder
            }
            "moving_average" => {
                // Execute moving average
                inputs[0].clone() // Placeholder
            }
            _ => return Err(format!("Unknown operator: {}", node.operator)),
        };
        
        results.insert(node.id, output);
    }
    
    let execution_time = start.elapsed().as_millis() as u64;
    
    Ok(PipelineResult {
        outputs: results,
        execution_time_ms: execution_time,
    })
}
```

```typescript
// src/lib/pipelines/executor.ts

import { invoke } from '@tauri-apps/api/core';

interface PipelineNode {
    id: string;
    operator: string;
    inputs: string[];
    parameters: Record<string, unknown>;
}

interface Pipeline {
    nodes: PipelineNode[];
    inputData: Record<string, number[]>;
}

interface PipelineResult {
    outputs: Record<string, number[]>;
    executionTimeMs: number;
}

export async function executePipeline(
    pipeline: Pipeline,
): Promise<PipelineResult> {
    return await invoke<PipelineResult>('execute_pipeline', {
        nodes: pipeline.nodes,
        inputData: pipeline.inputData,
    });
}
```

## Svelte Integration

### Svelte Component Example

```svelte
<!-- src/lib/components/ShaleVolumeCalculator.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let gammaRay = $state<number[]>([]);
  let grMin = $state(20.0);
  let grMax = $state(120.0);
  let method = $state<'larionov' | 'clavier' | 'steiber'>('larionov');
  let vsh = $state<number[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function calculate() {
    if (gammaRay.length === 0) {
      error = 'Please provide gamma ray data';
      return;
    }

    loading = true;
    error = null;

    try {
      const result = await invoke<{ vsh: number[]; methodUsed: string }>(
        'calculate_shale_volume_batch',
        {
          gammaRay: gammaRay,
          grMin: grMin,
          grMax: grMax,
          method: method,
        }
      );
      vsh = result.vsh;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  }
</script>

<div class="calculator">
  <h2>Shale Volume Calculator</h2>

  <div class="inputs">
    <label>
      GR Min:
      <input type="number" bind:value={grMin} step="0.1" />
    </label>
    <label>
      GR Max:
      <input type="number" bind:value={grMax} step="0.1" />
    </label>
    <label>
      Method:
      <select bind:value={method}>
        <option value="larionov">Larionov</option>
        <option value="clavier">Clavier</option>
        <option value="steiber">Steiber</option>
      </select>
    </label>
  </div>

  <button on:click={calculate} disabled={loading}>
    {loading ? 'Calculating...' : 'Calculate'}
  </button>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if vsh.length > 0}
    <div class="results">
      <h3>Results</h3>
      <p>VSH values: {vsh.length} points</p>
      <p>Average VSH: {(vsh.reduce((a, b) => a + b, 0) / vsh.length).toFixed(3)}</p>
    </div>
  {/if}
</div>
```

## Advanced Patterns

### State Management

```rust
use std::sync::Mutex;
use tauri::State;

struct AppState {
    cache: Mutex<std::collections::HashMap<String, Vec<f64>>>,
}

#[tauri::command]
fn get_cached_result(
    key: String,
    state: State<AppState>,
) -> Option<Vec<f64>> {
    state.cache.lock().unwrap().get(&key).cloned()
}

#[tauri::command]
fn cache_result(
    key: String,
    value: Vec<f64>,
    state: State<AppState>,
) {
    state.cache.lock().unwrap().insert(key, value);
}
```

```rust
// In main.rs
tauri::Builder::default()
    .manage(AppState {
        cache: Mutex::new(std::collections::HashMap::new()),
    })
    .invoke_handler(tauri::generate_handler![
        get_cached_result,
        cache_result
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

### Window Access

```rust
use tauri::Window;

#[tauri::command]
fn emit_progress(window: Window, progress: f64) {
    window.emit("calculation-progress", progress).unwrap();
}

#[tauri::command]
async fn long_running_calculation(
    window: Window,
    data: Vec<f64>,
) -> Vec<f64> {
    let total = data.len();
    let mut result = Vec::new();
    
    for (i, value) in data.iter().enumerate() {
        // Process value
        result.push(value * 2.0);
        
        // Emit progress
        let progress = (i + 1) as f64 / total as f64;
        window.emit("calculation-progress", progress).unwrap();
        
        // Simulate work
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
    
    result
}
```

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Listen for progress events
const unlisten = await listen<number>('calculation-progress', (event) => {
  console.log(`Progress: ${(event.payload * 100).toFixed(1)}%`);
});

// Execute calculation
const result = await invoke<number[]>('long_running_calculation', {
  data: [1, 2, 3, 4, 5],
});

// Clean up listener
unlisten();
```

### Binary Data (Large Arrays)

```rust
use tauri::ipc::Response;

#[tauri::command]
fn get_large_array() -> Response {
    // Generate large array (e.g., seismic data)
    let data: Vec<f32> = (0..1_000_000).map(|i| i as f32 * 0.001).collect();
    
    // Convert to bytes
    let bytes: Vec<u8> = data
        .iter()
        .flat_map(|&f| f.to_le_bytes())
        .collect();
    
    Response::new(bytes)
}
```

```typescript
import { invoke } from '@tauri-apps/api/core';

// Response automatically handles binary data
const binaryData = await invoke<number[]>('get_large_array');

// Convert back to Float32Array if needed
const floatArray = new Float32Array(binaryData);
```

## Type Safety with TypeScript

### Generate Types from Rust

You can use tools like `tauri-specta` to generate TypeScript types from your Rust commands:

```rust
use specta::Type;
use tauri_specta::*;

#[derive(Debug, Serialize, Deserialize, Type)]
struct ShaleVolumeParams {
    gamma_ray: Vec<f64>,
    gr_min: f64,
    gr_max: f64,
    method: String,
}

#[tauri::command]
#[specta::specta]
fn calculate_shale_volume(params: ShaleVolumeParams) -> Result<Vec<f64>, String> {
    // Implementation
}
```

## Best Practices

### 1. Use Descriptive Command Names

```rust
// Good
#[tauri::command]
fn calculate_shale_volume_larionov(...) -> ...

// Bad
#[tauri::command]
fn calc(...) -> ...
```

### 2. Validate Input Early

```rust
#[tauri::command]
fn process_data(data: Vec<f64>) -> Result<Vec<f64>, String> {
    if data.is_empty() {
        return Err("Data cannot be empty".to_string());
    }
    
    if data.len() > 1_000_000 {
        return Err("Data too large".to_string());
    }
    
    // Process data
    Ok(data)
}
```

### 3. Use Appropriate Return Types

```rust
// For simple success/failure
#[tauri::command]
fn save_config(config: Config) -> Result<(), String> {
    // Save config
    Ok(())
}

// For data with potential errors
#[tauri::command]
fn load_data() -> Result<Data, String> {
    // Load data
    Ok(data)
}

// For optional data
#[tauri::command]
fn find_well(well_id: String) -> Option<Well> {
    // Find well
    Some(well)
}
```

### 4. Handle Long-Running Operations

```rust
#[tauri::command]
async fn process_large_dataset(data: Vec<f64>) -> Vec<f64> {
    // Use spawn_blocking for CPU-intensive work
    tokio::task::spawn_blocking(move || {
        // Heavy computation
        data.iter().map(|x| x * 2.0).collect()
    })
    .await
    .unwrap()
}
```

## Summary

Tauri commands enable:

1. **Type-Safe Communication**: Automatic serialization between Rust and JavaScript
2. **Error Handling**: Rust `Result` types map to JavaScript promises
3. **Async Support**: Both sync and async Rust functions work seamlessly
4. **Complex Data**: Structs, arrays, and nested types are fully supported
5. **Performance**: Direct Rust execution for UDFs with minimal overhead

This architecture is perfect for your MVP where Rust UDFs (petrophysical calculations, signal processing, etc.) need to be called from your SvelteKit frontend with full type safety and error handling.

## References

- [Tauri Commands Documentation](https://tauri.app/develop/calling-rust)
- [Tauri API Reference](https://tauri.app/reference/javascript/core)
- [Serde Documentation](https://serde.rs/)

