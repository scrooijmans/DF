# VTK Input Type Compatibility Enforcement

This document explains in detail how VTK enforces input type compatibility (`FillInputPortInformation`, `RequestDataObject`, `SafeDownCast` patterns), including the call chain when a filter executes and rejects incompatible `vtkDataObject` types.

## Overview

VTK uses a **multi-stage type checking system** to ensure input type compatibility:

1. **FillInputPortInformation()**: Declares accepted input types (static declaration)
2. **RequestDataObject()**: Creates compatible output objects (dynamic creation)
3. **SafeDownCast()**: Runtime type validation (execution-time checking)
4. **RequestData()**: Final validation and processing

**Key Principles:**

- **Declarative Type System**: Filters declare expected types upfront
- **Runtime Validation**: Type checking occurs at multiple pipeline stages
- **Safe Casting**: `SafeDownCast` prevents invalid type assumptions
- **Early Detection**: Incompatible types detected before expensive processing
- **Error Reporting**: Clear error messages for type mismatches

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│         VTK Pipeline Type Checking System                   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Pipeline Construction                               │   │
│  │  - FillInputPortInformation()                       │   │
│  │  - FillOutputPortInformation()                      │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Pipeline Execution                                  │   │
│  │  1. RequestDataObject()                             │   │
│  │  2. RequestInformation()                             │   │
│  │  3. RequestUpdateExtent()                            │   │
│  │  4. RequestData()                                     │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Type Validation                                      │   │
│  │  - SafeDownCast()                                     │   │
│  │  - Type checking                                      │   │
│  │  - Error handling                                     │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## FillInputPortInformation

### Purpose

`FillInputPortInformation()` declares the expected input data types for each input port. This is a **static declaration** that informs the pipeline about type requirements before execution.

### Method Signature

```cpp
class vtkAlgorithm : public vtkObject
{
public:
    virtual int FillInputPortInformation(int port, vtkInformation* info);
};
```

### Implementation Pattern

**Basic Implementation:**

```cpp
class vtkMyFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Declare required input type
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
                     "vtkImageData");
            // Required input (not optional)
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
        }
        return 1;
    }
};
```

### Type Declaration Options

**Single Type:**

```cpp
// Accept only vtkImageData
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkImageData");
```

**Multiple Types (comma-separated):**

```cpp
// Accept multiple types
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
          "vtkPolyData;vtkUnstructuredGrid");
```

**Base Type:**

```cpp
// Accept any dataset
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");

// Accept any data object
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataObject");
```

**Optional Input:**

```cpp
// Optional input port
info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 1);
```

### Complete Example

```cpp
class vtkContourFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Accept any dataset
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
                     "vtkDataSet");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
            info->Set(vtkAlgorithm::INPUT_PORT_NAME(), "Input");
        }
        return 1;
    }

    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Output is polygonal data
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }
};
```

## RequestDataObject

### Purpose

`RequestDataObject()` creates output data objects of the appropriate type. This method is called **before** `RequestData()` to ensure output objects exist and match expected types.

### Method Signature

```cpp
class vtkAlgorithm : public vtkObject
{
public:
    virtual int RequestDataObject(vtkInformation* request,
                                 vtkInformationVector** inputVector,
                                 vtkInformationVector* outputVector);
};
```

### Default Implementation

**VTK's Default Behavior:**

```cpp
int vtkAlgorithm::RequestDataObject(vtkInformation* request,
                                    vtkInformationVector** inputVector,
                                    vtkInformationVector* outputVector)
{
    // For each output port
    for (int i = 0; i < this->GetNumberOfOutputPorts(); i++)
    {
        vtkInformation* outInfo = outputVector->GetInformationObject(i);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());

        // Get expected output type from FillOutputPortInformation
        const char* dataType = outInfo->Get(vtkDataObject::DATA_TYPE_NAME());

        if (!output || !output->IsA(dataType))
        {
            // Create new output object
            vtkDataObject* newOutput = vtkDataObject::NewInstance(dataType);
            outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
            newOutput->Delete();
        }
    }
    return 1;
}
```

### Custom Implementation

**Filter-Specific Output Creation:**

```cpp
class vtkImageToPolyDataFilter : public vtkAlgorithm
{
    int RequestDataObject(vtkInformation* request,
                         vtkInformationVector** inputVector,
                         vtkInformationVector* outputVector) override
    {
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());

        // Always output vtkPolyData (regardless of input)
        if (!output || !output->IsA("vtkPolyData"))
        {
            vtkPolyData* newOutput = vtkPolyData::New();
            outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
            newOutput->Delete();
        }

        return 1;
    }
};
```

**Input-Dependent Output Creation:**

```cpp
class vtkPassThroughFilter : public vtkAlgorithm
{
    int RequestDataObject(vtkInformation* request,
                         vtkInformationVector** inputVector,
                         vtkInformationVector* outputVector) override
    {
        vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
        vtkDataObject* input = inInfo->Get(vtkDataObject::DATA_OBJECT());

        if (!input)
        {
            return 0;
        }

        // Output type matches input type
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());

        if (!output || !output->IsA(input->GetClassName()))
        {
            vtkDataObject* newOutput = input->NewInstance();
            outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
            newOutput->Delete();
        }

        return 1;
    }
};
```

## SafeDownCast Pattern

### Purpose

`SafeDownCast()` performs **runtime type checking** and safely casts a base class pointer to a derived class pointer. It returns `nullptr` if the cast is invalid.

### Method Signature

```cpp
template<class T>
static T* SafeDownCast(vtkObjectBase* o);
```

### Usage Pattern

**Basic Pattern:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    // Get input data object
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // Safe cast to expected type
    vtkImageData* input = vtkImageData::SafeDownCast(inputData);

    if (!input)
    {
        vtkErrorMacro("Input is not vtkImageData. Got: "
                     << (inputData ? inputData->GetClassName() : "null"));
        return 0;
    }

    // Process input...
    return 1;
}
```

### Multiple Type Checking

**Check for Multiple Types:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // Try multiple types
    vtkPolyData* polyData = vtkPolyData::SafeDownCast(inputData);
    vtkUnstructuredGrid* ugrid = vtkUnstructuredGrid::SafeDownCast(inputData);

    if (!polyData && !ugrid)
    {
        vtkErrorMacro("Input must be vtkPolyData or vtkUnstructuredGrid");
        return 0;
    }

    // Process based on type
    if (polyData)
    {
        // Process polygonal data
    }
    else if (ugrid)
    {
        // Process unstructured grid
    }

    return 1;
}
```

### Base Type Checking

**Check for Base Type:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // Cast to base type
    vtkDataSet* dataset = vtkDataSet::SafeDownCast(inputData);

    if (!dataset)
    {
        vtkErrorMacro("Input must be a vtkDataSet");
        return 0;
    }

    // Process dataset (works for all dataset types)
    return 1;
}
```

## Pipeline Execution Call Chain

### Complete Execution Sequence

**Pipeline Update Sequence:**

```
1. Pipeline Update Request
   ↓
2. RequestDataObject()
   - Creates output data objects
   - Validates output types
   ↓
3. RequestInformation()
   - Gathers metadata about data
   - Determines data structure
   ↓
4. RequestUpdateExtent()
   - Determines required data extent
   - Sets update extent on inputs
   ↓
5. RequestData()
   - Performs actual data processing
   - Uses SafeDownCast for type checking
   ↓
6. Output Available
```

### RequestDataObject Call Chain

**Detailed Flow:**

```cpp
// 1. Pipeline update initiated
pipeline->Update();

// 2. For each filter in pipeline (bottom-up):
void vtkExecutive::Update()
{
    // 2a. Update inputs first
    this->UpdateInputs();

    // 2b. Request output data object
    this->RequestDataObject();
}

// 3. RequestDataObject implementation
int vtkAlgorithm::RequestDataObject(vtkInformation* request,
                                   vtkInformationVector** inputVector,
                                   vtkInformationVector* outputVector)
{
    // 3a. Get output port information
    vtkInformation* outInfo = outputVector->GetInformationObject(0);
    const char* dataType = outInfo->Get(vtkDataObject::DATA_TYPE_NAME());

    // 3b. Check if output exists and is correct type
    vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());

    if (!output || !output->IsA(dataType))
    {
        // 3c. Create new output object
        vtkDataObject* newOutput = vtkDataObject::NewInstance(dataType);
        outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
        newOutput->Delete();
    }

    return 1;
}
```

### RequestData Call Chain

**Detailed Flow:**

```cpp
// 1. RequestData called
int vtkAlgorithm::RequestData(vtkInformation* request,
                             vtkInformationVector** inputVector,
                             vtkInformationVector* outputVector)
{
    // 2. Get input data object
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // 3. Type checking with SafeDownCast
    vtkImageData* input = vtkImageData::SafeDownCast(inputData);

    // 4. Validation
    if (!input)
    {
        // 4a. Type mismatch detected
        vtkErrorMacro("Expected vtkImageData, got: "
                     << (inputData ? inputData->GetClassName() : "null"));
        return 0;  // Execution stops
    }

    // 5. Get output
    vtkInformation* outInfo = outputVector->GetInformationObject(0);
    vtkImageData* output = vtkImageData::SafeDownCast(
        outInfo->Get(vtkDataObject::DATA_OBJECT())
    );

    // 6. Process data
    // ... actual processing ...

    return 1;
}
```

## Type Rejection Flow

### Incompatible Type Detection

**Scenario: vtkPolyData connected to vtkImageData filter**

```
1. Pipeline Construction
   User connects: vtkSphereSource (vtkPolyData) → vtkImageThreshold (vtkImageData)
   ↓
2. FillInputPortInformation() Check
   vtkImageThreshold declares: INPUT_REQUIRED_DATA_TYPE = "vtkImageData"
   ↓
3. Pipeline Update Initiated
   pipeline->Update()
   ↓
4. RequestDataObject() Called
   - Creates output object (vtkImageData)
   - No type check here (input not yet accessed)
   ↓
5. RequestInformation() Called
   - May access input metadata
   - Type mismatch may be detected here
   ↓
6. RequestData() Called
   - Gets input: vtkPolyData
   - Attempts SafeDownCast to vtkImageData
   ↓
7. SafeDownCast() Returns nullptr
   vtkImageData* input = vtkImageData::SafeDownCast(polyData);
   // Returns nullptr (polyData is not vtkImageData)
   ↓
8. Type Mismatch Detected
   if (!input)
   {
       vtkErrorMacro("Input is not vtkImageData");
       return 0;  // Execution stops
   }
   ↓
9. Error Reported
   - Error message logged
   - Pipeline execution stops
   - Output remains empty/invalid
```

### Error Handling Example

**Complete Error Handling:**

```cpp
class vtkImageThreshold : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Declare required type
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
                     "vtkImageData");
        }
        return 1;
    }

    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        // Get input
        vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
        vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

        // Type validation
        if (!inputData)
        {
            vtkErrorMacro("No input data provided");
            return 0;
        }

        vtkImageData* input = vtkImageData::SafeDownCast(inputData);

        if (!input)
        {
            // Detailed error message
            vtkErrorMacro("Input type mismatch. "
                         << "Expected: vtkImageData, "
                         << "Got: " << inputData->GetClassName());
            return 0;
        }

        // Get output
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkImageData* output = vtkImageData::SafeDownCast(
            outInfo->Get(vtkDataObject::DATA_OBJECT())
        );

        if (!output)
        {
            vtkErrorMacro("Output is not vtkImageData");
            return 0;
        }

        // Process data
        // ... thresholding algorithm ...

        return 1;
    }
};
```

## Type Checking at Different Stages

### Stage 1: Pipeline Construction

**FillInputPortInformation() Validation:**

```cpp
// During pipeline construction, VTK can check type compatibility
bool CheckPipelineCompatibility(vtkAlgorithm* upstream,
                                vtkAlgorithm* downstream)
{
    // Get upstream output type
    vtkInformation* upOutInfo = upstream->GetOutputPortInformation(0);
    const char* upOutputType = upOutInfo->Get(vtkDataObject::DATA_TYPE_NAME());

    // Get downstream input type
    vtkInformation* downInInfo = downstream->GetInputPortInformation(0);
    const char* downInputType = downInInfo->Get(
        vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE()
    );

    // Check compatibility
    if (strcmp(upOutputType, downInputType) != 0)
    {
        // Check if upOutputType is subclass of downInputType
        vtkDataObject* testObj = vtkDataObject::NewInstance(upOutputType);
        bool compatible = testObj->IsA(downInputType);
        testObj->Delete();

        return compatible;
    }

    return true;
}
```

### Stage 2: RequestDataObject

**Output Type Validation:**

```cpp
int RequestDataObject(vtkInformation* request,
                     vtkInformationVector** inputVector,
                     vtkInformationVector* outputVector) override
{
    // Validate input exists
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* input = inInfo->Get(vtkDataObject::DATA_OBJECT());

    if (!input)
    {
        vtkErrorMacro("No input data object");
        return 0;
    }

    // Create output based on input type
    vtkInformation* outInfo = outputVector->GetInformationObject(0);
    const char* expectedType = outInfo->Get(vtkDataObject::DATA_TYPE_NAME());

    vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());

    if (!output || !output->IsA(expectedType))
    {
        // Create correct output type
        output = vtkDataObject::NewInstance(expectedType);
        outInfo->Set(vtkDataObject::DATA_OBJECT(), output);
        output->Delete();
    }

    return 1;
}
```

### Stage 3: RequestData

**Runtime Type Validation:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    // Get input with type checking
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // Runtime type validation
    vtkImageData* input = vtkImageData::SafeDownCast(inputData);

    if (!input)
    {
        // Type mismatch - reject
        vtkErrorMacro("Type mismatch: expected vtkImageData, "
                     << "got " << (inputData ? inputData->GetClassName() : "null"));
        return 0;
    }

    // Process...
    return 1;
}
```

## Advanced Type Checking Patterns

### Type Hierarchy Checking

**Check for Base Type:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // Check for base type (accepts all derived types)
    vtkDataSet* dataset = vtkDataSet::SafeDownCast(inputData);

    if (!dataset)
    {
        vtkErrorMacro("Input must be a vtkDataSet");
        return 0;
    }

    // Can now use dataset methods
    vtkIdType numCells = dataset->GetNumberOfCells();

    return 1;
}
```

### Multiple Input Ports

**Type Checking for Multiple Inputs:**

```cpp
class vtkAppendFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        // All input ports accept datasets
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
        info->Set(vtkAlgorithm::INPUT_IS_REPEATABLE(), 1);
        return 1;
    }

    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        // Check all inputs
        int numInputs = inputVector[0]->GetNumberOfInformationObjects();

        for (int i = 0; i < numInputs; i++)
        {
            vtkInformation* inInfo = inputVector[0]->GetInformationObject(i);
            vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

            vtkDataSet* dataset = vtkDataSet::SafeDownCast(inputData);

            if (!dataset)
            {
                vtkErrorMacro("Input " << i << " is not a vtkDataSet");
                return 0;
            }
        }

        // Process all inputs...
        return 1;
    }
};
```

### Conditional Type Checking

**Different Processing Based on Type:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
    vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

    // Try different types
    vtkImageData* imageData = vtkImageData::SafeDownCast(inputData);
    vtkPolyData* polyData = vtkPolyData::SafeDownCast(inputData);
    vtkUnstructuredGrid* ugrid = vtkUnstructuredGrid::SafeDownCast(inputData);

    if (imageData)
    {
        // Process image data
        return this->ProcessImageData(imageData, outputVector);
    }
    else if (polyData)
    {
        // Process polygonal data
        return this->ProcessPolyData(polyData, outputVector);
    }
    else if (ugrid)
    {
        // Process unstructured grid
        return this->ProcessUnstructuredGrid(ugrid, outputVector);
    }
    else
    {
        vtkErrorMacro("Unsupported input type: "
                     << inputData->GetClassName());
        return 0;
    }
}
```

## Complete Example: Type-Safe Filter

### Full Implementation

```cpp
class vtkGradientFilter : public vtkAlgorithm
{
public:
    static vtkGradientFilter* New();
    vtkTypeMacro(vtkGradientFilter, vtkAlgorithm);

    // Declare input port requirements
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Accept any dataset
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
                     "vtkDataSet");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
        }
        return 1;
    }

    // Declare output port type
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Output type matches input type
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkDataSet");
        }
        return 1;
    }

    // Create output data object
    int RequestDataObject(vtkInformation* request,
                         vtkInformationVector** inputVector,
                         vtkInformationVector* outputVector) override
    {
        vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
        vtkDataObject* input = inInfo->Get(vtkDataObject::DATA_OBJECT());

        if (!input)
        {
            vtkErrorMacro("No input data");
            return 0;
        }

        // Output type matches input type
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());

        if (!output || !output->IsA(input->GetClassName()))
        {
            vtkDataObject* newOutput = input->NewInstance();
            outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
            newOutput->Delete();
        }

        return 1;
    }

    // Process data with type checking
    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        // Get input with type validation
        vtkInformation* inInfo = inputVector[0]->GetInformationObject(0);
        vtkDataObject* inputData = inInfo->Get(vtkDataObject::DATA_OBJECT());

        if (!inputData)
        {
            vtkErrorMacro("No input data");
            return 0;
        }

        // Safe cast to dataset
        vtkDataSet* input = vtkDataSet::SafeDownCast(inputData);

        if (!input)
        {
            vtkErrorMacro("Input must be a vtkDataSet. "
                         << "Got: " << inputData->GetClassName());
            return 0;
        }

        // Get output
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* outputData = outInfo->Get(vtkDataObject::DATA_OBJECT());
        vtkDataSet* output = vtkDataSet::SafeDownCast(outputData);

        if (!output)
        {
            vtkErrorMacro("Output is not a vtkDataSet");
            return 0;
        }

        // Validate output type matches input type
        if (!output->IsA(input->GetClassName()))
        {
            vtkErrorMacro("Output type mismatch. "
                         << "Expected: " << input->GetClassName()
                         << ", Got: " << output->GetClassName());
            return 0;
        }

        // Process data
        output->ShallowCopy(input);

        // Compute gradients...
        // ...

        return 1;
    }
};
```

## Summary

### Type Checking Mechanisms

1. **FillInputPortInformation()**
   - Static declaration of accepted types
   - Called during pipeline construction
   - Informs pipeline about type requirements

2. **RequestDataObject()**
   - Creates output objects of correct type
   - Called before RequestData()
   - Ensures output types match expectations

3. **SafeDownCast()**
   - Runtime type validation
   - Used in RequestData()
   - Prevents invalid type assumptions

4. **RequestData()**
   - Final validation and processing
   - Uses SafeDownCast for type checking
   - Rejects incompatible types with error messages

### Type Rejection Flow

```
1. FillInputPortInformation() declares expected type
   ↓
2. Pipeline update initiated
   ↓
3. RequestDataObject() creates output
   ↓
4. RequestData() called
   ↓
5. SafeDownCast() validates input type
   ↓
6. If type mismatch:
   - SafeDownCast returns nullptr
   - Error message logged
   - RequestData returns 0
   - Pipeline execution stops
```

### Best Practices

- **Always use SafeDownCast**: Never use static_cast or C-style casts
- **Check return values**: Always check if SafeDownCast returns nullptr
- **Provide clear errors**: Include expected and actual types in error messages
- **Validate early**: Check types at the start of RequestData()
- **Match declarations**: Ensure SafeDownCast matches FillInputPortInformation()

This architecture ensures **type-safe pipeline execution**, **early error detection**, and **clear error reporting** when incompatible types are encountered.
