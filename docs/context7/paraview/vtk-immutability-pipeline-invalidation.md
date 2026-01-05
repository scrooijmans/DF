# ParaView/VTK Immutability Model: Pipeline Output Immutability and State Invalidation

## Executive Summary

ParaView/VTK enforces a strict **immutability model** for pipeline outputs. Once a `vtkDataObject` is set as an output and cached, it becomes **immutable**—its contents cannot be modified. Instead of mutating existing data objects when parameters change, ParaView **invalidates the pipeline state**, causing downstream algorithms to recompute and create new data objects. This design ensures data integrity, enables safe concurrent access, and maintains reproducibility.

**Key Principles**:
- **Output Immutability**: Cached pipeline outputs are immutable after `RequestData()` completes
- **No Mutation**: Filters must never modify cached output data objects
- **State Invalidation**: Parameter changes invalidate pipeline state, not data objects
- **Recomputation**: Invalidated pipelines create new data objects on next `Update()`
- **Reference Safety**: Immutability ensures safe sharing of data objects across the pipeline

---

## 1. The Immutability Contract

### What is Immutable?

In ParaView/VTK, **pipeline output data objects are immutable** once they are:
1. Created in `RequestData()`
2. Set in the output port
3. Cached by the pipeline

**Immutability means**:
- The data object's structure cannot change (points, cells, arrays)
- The data object's content cannot change (point coordinates, array values)
- The data object's metadata cannot change (extents, time steps, bounds)

### The Immutability Contract

```cpp
// IMMUTABILITY CONTRACT:
// 1. Once RequestData() completes and returns, the output data object is IMMUTABLE
// 2. No code should modify a cached output data object
// 3. Parameter changes invalidate the pipeline, causing recomputation
// 4. New data objects are created for each recomputation
```

### Why Immutability?

**1. Safe Sharing**:
```cpp
// Multiple filters can safely reference the same upstream output
source->Update();
vtkPolyData* sharedOutput = source->GetOutput();

filterA->SetInputConnection(source->GetOutputPort());
filterB->SetInputConnection(source->GetOutputPort());
filterC->SetInputConnection(source->GetOutputPort());

// All three filters reference the same immutable data object
// No risk of one filter modifying data used by others
```

**2. Cache Integrity**:
```cpp
// Cached outputs remain valid until explicitly invalidated
vtkPolyData* cached = filter->GetOutput();
// cached is immutable - safe to use indefinitely
// (until filter->Modified() is called)
```

**3. Concurrent Access**:
```cpp
// Immutable objects can be safely accessed from multiple threads
// (VTK itself is not fully thread-safe, but immutability helps)
```

**4. Reproducibility**:
```cpp
// Immutable outputs ensure deterministic behavior
// Same pipeline state always produces same output content
```

---

## 2. Mutation is Disallowed: Enforcement Mechanisms

### Compile-Time vs. Runtime Enforcement

VTK/ParaView does **not** enforce immutability at compile-time (C++ doesn't have built-in immutability). Instead, immutability is enforced through:

1. **Convention and Documentation**: Strong guidelines that filters must not modify outputs
2. **Pipeline Architecture**: Outputs are cached and replaced, not modified
3. **Best Practices**: Well-established patterns that prevent mutation

### What Happens if You Mutate?

**Dangerous Mutation Example** (DO NOT DO THIS):
```cpp
// BAD: Mutating a cached output
int vtkBadFilter::RequestData(...) {
    vtkPolyData* output = vtkPolyData::SafeDownCast(
        outputVector->GetInformationObject(0)->Get(
            vtkDataObject::DATA_OBJECT()));
    
    // Create output
    output->SetPoints(points);
    // ... set other data ...
    
    return 1;  // Output is now cached and immutable
}

// Later, somewhere else (WRONG!):
vtkPolyData* cached = filter->GetOutput();
cached->GetPoints()->SetPoint(0, newPoint);  // MUTATION! BREAKS IMMUTABILITY!
// This corrupts the cached output and breaks downstream filters
```

**Problems with Mutation**:
1. **Cache Corruption**: Other filters using the same output see corrupted data
2. **Non-Reproducible**: Same pipeline state produces different results
3. **Race Conditions**: Concurrent access becomes unsafe
4. **Downstream Errors**: Filters expecting immutable data may fail

### Correct Pattern: Create New Output

**Correct Approach**:
```cpp
// GOOD: Always create new output data
int vtkGoodFilter::RequestData(...) {
    // Get input (read-only, immutable)
    vtkPolyData* input = vtkPolyData::SafeDownCast(
        inputVector[0]->GetInformationObject(0)->Get(
            vtkDataObject::DATA_OBJECT()));
    
    // Get or create output
    vtkPolyData* output = vtkPolyData::SafeDownCast(
        outputVector->GetInformationObject(0)->Get(
            vtkDataObject::DATA_OBJECT()));
    
    if (!output) {
        // Create new output object
        output = vtkPolyData::New();
        outputVector->GetInformationObject(0)->Set(
            vtkDataObject::DATA_OBJECT(), output);
        output->Delete();  // Pipeline now owns it
    }
    
    // Create new geometry (don't modify input)
    vtkNew<vtkPoints> newPoints;
    vtkPoints* inputPoints = input->GetPoints();
    
    // Transform points into new array
    for (vtkIdType i = 0; i < input->GetNumberOfPoints(); i++) {
        double point[3];
        inputPoints->GetPoint(i, point);
        // Transform...
        newPoints->InsertNextPoint(transformedPoint);
    }
    
    // Set new data (output is mutable during RequestData)
    output->SetPoints(newPoints);
    output->SetPolys(input->GetPolys());  // Shallow copy (safe, input is immutable)
    
    // Once RequestData() returns, output becomes immutable
    return 1;
}
```

### Input Immutability

**Inputs are also treated as immutable**:

```cpp
// CORRECT: Read input, create new output
int vtkFilter::RequestData(...) {
    vtkPolyData* input = ...;  // Input is immutable
    
    // GOOD: Read from input
    vtkPoints* inputPoints = input->GetPoints();
    double point[3];
    inputPoints->GetPoint(0, point);
    
    // BAD: Don't modify input
    // input->GetPoints()->SetPoint(0, newPoint);  // WRONG!
    
    // Create new output instead
    vtkNew<vtkPolyData> output;
    // ... populate output ...
}
```

**Why Input Immutability?**
- Inputs may be shared by multiple filters
- Inputs are cached outputs from upstream filters
- Modifying inputs corrupts upstream caches

---

## 3. Parameter Changes and Pipeline Invalidation

### The Invalidation Model

When a filter parameter changes, ParaView/VTK does **not** mutate existing data objects. Instead, it:

1. **Marks the algorithm as modified** (`Modified()`)
2. **Invalidates the pipeline state** (marks outputs as stale)
3. **Triggers recomputation** on next `Update()`
4. **Creates new data objects** (old ones are released when no longer referenced)

### Modified() and MTime

**Modification Time Tracking**:
```cpp
class vtkAlgorithm {
    vtkTimeStamp MTime;  // Modification time
    vtkTimeStamp OutputTime;  // When output was last computed
    
    void Modified() {
        this->MTime.Modified();  // Increment modification time
        // Propagate to downstream algorithms
    }
    
    bool NeedToExecute() {
        // Output is stale if algorithm was modified after output was computed
        return this->OutputTime < this->MTime;
    }
};
```

**Parameter Change Flow**:
```cpp
// 1. User changes parameter
filter->SetRadius(2.0);  // This calls Modified() internally

// 2. Modified() increments MTime
void vtkSphereSource::SetRadius(double radius) {
    if (this->Radius != radius) {
        this->Radius = radius;
        this->Modified();  // Marks algorithm as modified
    }
}

// 3. Pipeline state is now invalid
//    OutputTime < MTime, so NeedToExecute() returns true

// 4. Next Update() triggers recomputation
filter->Update();

// 5. RequestData() creates NEW output data object
int vtkSphereSource::RequestData(...) {
    // Create new vtkPolyData (not modifying old one)
    vtkPolyData* output = vtkPolyData::New();
    // ... populate with new sphere geometry ...
    outputVector->GetInformationObject(0)->Set(
        vtkDataObject::DATA_OBJECT(), output);
    output->Delete();
    
    // Update OutputTime to current MTime
    this->OutputTime.Modified();
    
    return 1;
}

// 6. Old output data object is released (if no other references)
//    New output is cached and immutable
```

### Hierarchical Invalidation

**Downstream Propagation**:
```cpp
// Pipeline: Source -> FilterA -> FilterB -> Mapper

// 1. Change source parameter
source->SetRadius(2.0);
source->Modified();

// 2. Invalidation propagates downstream
//    FilterA's input changed, so FilterA is invalid
//    FilterB's input (FilterA's output) changed, so FilterB is invalid
//    Mapper's input (FilterB's output) changed, so Mapper is invalid

// 3. All downstream algorithms mark themselves as needing execution
void vtkAlgorithm::Modified() {
    this->MTime.Modified();
    
    // Propagate to downstream
    for (auto& connection : this->Outputs) {
        for (auto& consumer : connection->Consumers) {
            consumer->Modified();  // Recursive invalidation
        }
    }
}

// 4. Next Update() triggers recomputation of entire pipeline
mapper->Update();
// Source->RequestData() creates new output
// FilterA->RequestData() creates new output (using Source's new output)
// FilterB->RequestData() creates new output (using FilterA's new output)
// Mapper uses FilterB's new output
```

### Cache Invalidation vs. Data Mutation

**Key Distinction**:

```cpp
// WRONG APPROACH (mutation):
void mutateOutput(vtkPolyData* output) {
    // Modify existing cached output
    output->GetPoints()->SetPoint(0, newPoint);
    // This breaks immutability contract!
}

// CORRECT APPROACH (invalidation + recomputation):
void changeParameter(vtkAlgorithm* filter) {
    // Change parameter
    filter->SetParameter(newValue);
    filter->Modified();  // Invalidate pipeline state
    
    // Next Update() will create new output
    filter->Update();
    // New output created, old output released
}
```

### Example: Parameter Change Sequence

**Complete Example**:
```cpp
// Initial state
vtkNew<vtkSphereSource> source;
source->SetRadius(1.0);
source->Update();

vtkPolyData* output1 = source->GetOutput();
// output1 is cached and immutable
// Memory address: 0x12345678

// Change parameter
source->SetRadius(2.0);
// Internally calls Modified()
// MTime incremented
// OutputTime < MTime, so output is stale

// At this point:
// - output1 still exists (may be referenced elsewhere)
// - output1 is still immutable (its contents haven't changed)
// - But output1 is "stale" (doesn't reflect current parameters)

// Trigger recomputation
source->Update();

// RequestData() executes:
int vtkSphereSource::RequestData(...) {
    // Get existing output (will be replaced)
    vtkPolyData* output = ...;
    
    // Create NEW geometry
    vtkNew<vtkPoints> newPoints;
    // ... create sphere with radius 2.0 ...
    
    // Replace output's data (output is mutable during RequestData)
    output->SetPoints(newPoints);
    // ... set other data ...
    
    // Update OutputTime
    this->OutputTime.Modified();
    
    return 1;
}

vtkPolyData* output2 = source->GetOutput();
// output2 is the NEW output
// Memory address: 0x87654321 (different from output1!)

// output1 may still exist if referenced elsewhere
// But output1 is no longer the "current" output
// output2 is now the cached, immutable output
```

**Important Points**:
1. **Old output is not mutated** - it remains unchanged
2. **New output is created** - fresh data object with new geometry
3. **Old output may persist** - if still referenced (reference counting)
4. **New output is immutable** - once RequestData() completes

---

## 4. Pipeline State vs. Data Object State

### Two Separate Concepts

**Pipeline State**:
- Algorithm parameters (radius, center, etc.)
- Pipeline topology (connections)
- Modification times (MTime)
- Cache validity (OutputTime vs. MTime)

**Data Object State**:
- Geometry (points, cells)
- Attributes (arrays, scalars, vectors)
- Metadata (extents, bounds, time steps)
- Immutable once cached

### State Invalidation, Not Data Mutation

```cpp
// Pipeline state change
source->SetRadius(2.0);  // Changes pipeline state
source->Modified();      // Invalidates pipeline state

// Pipeline state is now:
// - Parameter: radius = 2.0
// - MTime: incremented
// - OutputTime: stale (old)
// - Cache: invalid

// Data object state is unchanged:
vtkPolyData* oldOutput = source->GetOutput();
// oldOutput still has radius 1.0 geometry
// oldOutput is immutable (cannot be changed)

// Recomputation creates new data object with new state:
source->Update();
vtkPolyData* newOutput = source->GetOutput();
// newOutput has radius 2.0 geometry
// newOutput is immutable (new object, different from oldOutput)
```

### State File Representation

**State files capture pipeline state, not data object state**:

```xml
<!-- Pipeline state (captured in .pvsm file) -->
<Item id="0" name="Sphere1" group="sources">
  <Property name="Radius" value="2.0"/>  <!-- Pipeline parameter -->
  <Property name="Center" value="0.0 0.0 0.0"/>
</Item>

<!-- Data object state is NOT captured -->
<!-- Data must be regenerated from pipeline state + input data -->
```

**Reproducibility**:
- **Pipeline state** (parameters) → captured in state file
- **Data object state** (geometry) → regenerated from pipeline state
- **Same pipeline state** → same data object content (but different object identity)

---

## 5. Practical Implications

### For Filter Developers

**1. Never Modify Cached Outputs**:
```cpp
// WRONG
void someFunction(vtkPolyData* output) {
    output->GetPoints()->SetPoint(0, newPoint);  // DON'T DO THIS
}

// CORRECT
void changePipeline(vtkAlgorithm* filter) {
    filter->SetParameter(newValue);
    filter->Modified();  // Invalidate, don't mutate
    filter->Update();    // Recompute
}
```

**2. Always Create New Output in RequestData()**:
```cpp
int vtkMyFilter::RequestData(...) {
    // Get output (may be existing or new)
    vtkPolyData* output = ...;
    
    // Create new data (output is mutable during RequestData)
    vtkNew<vtkPoints> newPoints;
    // ... populate newPoints ...
    output->SetPoints(newPoints);
    
    // Once RequestData() returns, output becomes immutable
    return 1;
}
```

**3. Never Modify Inputs**:
```cpp
int vtkMyFilter::RequestData(...) {
    vtkPolyData* input = ...;  // Input is immutable
    
    // GOOD: Read from input
    double point[3];
    input->GetPoints()->GetPoint(0, point);
    
    // BAD: Don't modify input
    // input->GetPoints()->SetPoint(0, newPoint);  // WRONG!
}
```

### For Application Developers

**1. Understand Cache Validity**:
```cpp
// Get output
vtkPolyData* output = filter->GetOutput();

// Change parameter
filter->SetParameter(newValue);
filter->Modified();

// output is now stale (doesn't reflect new parameter)
// But output is still immutable (its contents haven't changed)

// Recompute to get new output
filter->Update();
vtkPolyData* newOutput = filter->GetOutput();
// newOutput is different object with new data
```

**2. Don't Store Raw Pointers to Outputs**:
```cpp
// BAD: Raw pointer may become stale
vtkPolyData* cached = filter->GetOutput();
filter->Modified();
filter->Update();
// cached still points to old output!

// GOOD: Always get fresh reference
vtkSmartPointer<vtkPolyData> getCurrentOutput(vtkAlgorithm* filter) {
    filter->Update();  // Ensure up-to-date
    return filter->GetOutput();  // Returns smart pointer
}
```

**3. Use Connections, Not Direct Data**:
```cpp
// GOOD: Connection-based (respects immutability)
filter->SetInputConnection(source->GetOutputPort());

// AVOID: Direct data (unless necessary)
vtkSmartPointer<vtkPolyData> data = ...;
filter->SetInputData(data);  // Only if not in pipeline
```

### Common Mistakes

**Mistake 1: Mutating Cached Output**:
```cpp
// WRONG
vtkPolyData* output = filter->GetOutput();
output->GetPoints()->SetPoint(0, newPoint);  // Breaks immutability!
```

**Mistake 2: Assuming Output Updates Automatically**:
```cpp
// WRONG
vtkPolyData* output = filter->GetOutput();
filter->SetParameter(newValue);
// output still has old data! Must call Update()
```

**Mistake 3: Modifying Input**:
```cpp
// WRONG
vtkPolyData* input = filter->GetInput();
input->GetPoints()->SetPoint(0, newPoint);  // Corrupts upstream cache!
```

---

## 6. Advanced Topics

### ShallowCopy and Immutability

**ShallowCopy shares data, but immutability is preserved**:

```cpp
int vtkMyFilter::RequestData(...) {
    vtkPolyData* input = ...;
    vtkPolyData* output = ...;
    
    // ShallowCopy shares arrays (efficient)
    output->GetPointData()->ShallowCopy(input->GetPointData());
    
    // Both input and output reference same arrays
    // But both are immutable (cannot modify shared arrays)
    // If you need to modify, must DeepCopy first
}
```

**When to Use ShallowCopy**:
- When output can share data with input
- When data won't be modified
- For memory efficiency

**When to Use DeepCopy**:
- When output needs independent data
- When data will be modified
- When input may be modified elsewhere

### Streaming and Immutability

**Streaming maintains immutability per piece**:

```cpp
// Large dataset processed in pieces
for (int piece = 0; piece < numPieces; piece++) {
    // Request specific piece
    request->Set(vtkStreamingDemandDrivenPipeline::UPDATE_PIECE_NUMBER(), piece);
    filter->Update(request);
    
    // Each piece is immutable once computed
    vtkPolyData* pieceOutput = filter->GetOutput();
    // pieceOutput is immutable for this piece
}
```

### Temporal Data and Immutability

**Each time step produces immutable output**:

```cpp
// Request specific time step
request->Set(vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP(), 0.5);
filter->Update(request);

vtkPolyData* output = filter->GetOutput();
// output is immutable for time step 0.5

// Request different time step
request->Set(vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP(), 1.0);
filter->Update(request);

vtkPolyData* newOutput = filter->GetOutput();
// newOutput is different object (may be same address if reused)
// But represents different time step
// Both are immutable
```

---

## 7. Summary: The Immutability Model

### Core Principles

1. **Output Immutability**:
   - Pipeline outputs are immutable once `RequestData()` completes
   - Cached outputs cannot be modified
   - New outputs are created for each recomputation

2. **No Mutation**:
   - Filters must never modify cached outputs
   - Filters must never modify inputs
   - Mutation breaks the immutability contract

3. **State Invalidation**:
   - Parameter changes invalidate pipeline state (MTime)
   - Invalidated pipelines recompute on next `Update()`
   - New data objects are created, old ones are released

4. **Safe Sharing**:
   - Immutability enables safe sharing of data objects
   - Multiple filters can reference same upstream output
   - No risk of concurrent modification

### Benefits

- **Data Integrity**: Immutable outputs prevent corruption
- **Cache Safety**: Cached outputs remain valid until invalidated
- **Reproducibility**: Same pipeline state produces same output content
- **Concurrent Access**: Immutable objects are safer for multi-threading
- **Memory Efficiency**: Reference counting with immutability enables sharing

### Key Takeaways

1. **Pipeline outputs are immutable** once cached
2. **Parameter changes invalidate state**, not data objects
3. **Recomputation creates new outputs**, doesn't mutate old ones
4. **Never modify cached outputs** or inputs
5. **Always create new output** in `RequestData()`

This immutability model is fundamental to ParaView/VTK's architecture, ensuring data integrity, reproducibility, and safe concurrent access while maintaining efficient memory usage through reference counting and caching.

