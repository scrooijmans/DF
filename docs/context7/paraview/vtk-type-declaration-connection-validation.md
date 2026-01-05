# VTK Type Declaration and Connection Validation: From Port Information to UI Prevention

## Executive Summary

VTK filters declare accepted input/output types through a multi-layered type system that includes **port information declarations**, **RequestDataObject** for output type creation, and **runtime type checks** in `RequestData()`. ParaView builds on this foundation by using **Server Manager XML** with `DataTypeDomain` constraints to prevent incompatible pipeline connections at the UI level, providing immediate user feedback and preventing invalid pipeline construction before execution.

**Key Mechanisms**:
- **FillInputPortInformation()**: Declares accepted input types at compile time
- **FillOutputPortInformation()**: Declares output types
- **RequestDataObject()**: Creates appropriate output data object type at runtime
- **Runtime Checks**: `RequestData()` validates types with `SafeDownCast()` and `GetData()`
- **DataTypeDomain**: ParaView XML constraints prevent incompatible UI connections
- **Server Manager Validation**: Validates connections before VTK pipeline execution

---

## 1. VTK Port Information System

### FillInputPortInformation(): Declaring Input Types

**Purpose**: Declare what data types an input port accepts.

```cpp
class vtkMyFilter : public vtkAlgorithm
{
public:
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Declare accepted input type
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
            
            // Optional: Mark as required or optional
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);  // Required
            
            // Optional: Allow multiple connections
            info->Set(vtkAlgorithm::INPUT_IS_REPEATABLE(), 0);  // Single connection
            
            // Optional: Port name for documentation
            info->Set(vtkAlgorithm::INPUT_PORT_NAME(), "Input PolyData");
        }
        return 1;
    }
};
```

**Type Declaration Options**:

```cpp
// Accept specific type
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");

// Accept any dataset (more flexible)
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");

// Accept multiple types (comma-separated)
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), 
          "vtkPolyData;vtkUnstructuredGrid");

// Accept any data object (most flexible)
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataObject");
```

**Common Data Types**:

```cpp
"vtkDataObject"          // Base class (accepts anything)
"vtkDataSet"             // Any dataset type
"vtkPolyData"             // Polygonal data (polylines, polygons)
"vtkUnstructuredGrid"     // Unstructured grid
"vtkStructuredGrid"       // Structured grid
"vtkImageData"            // Regular grid (voxels)
"vtkRectilinearGrid"      // Rectilinear grid
"vtkTable"                // Tabular data
"vtkGraph"                // Graph data
```

### FillOutputPortInformation(): Declaring Output Types

**Purpose**: Declare what data type an output port produces.

```cpp
class vtkMyFilter : public vtkAlgorithm
{
public:
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Declare output type
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }
};
```

**Output Type Scenarios**:

```cpp
// Output is same type as input
int FillOutputPortInformation(int port, vtkInformation* info) override
{
    // Get input type from input port
    vtkInformation* inputInfo = this->GetInputPortInformation(0);
    const char* inputType = inputInfo->Get(
        vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE());
    
    // Output matches input
    info->Set(vtkDataObject::DATA_TYPE_NAME(), inputType);
    return 1;
}

// Output is always specific type (regardless of input)
info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");

// Output type depends on algorithm logic
// (determined in RequestDataObject())
```

### Multiple Input Ports

**Example: Filter with Multiple Inputs**:

```cpp
class vtkMergeFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // First input: required, any dataset
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
        }
        else if (port == 1)
        {
            // Second input: optional, any dataset
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 1);
        }
        return 1;
    }
    
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Output is same as first input
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkDataSet");
        }
        return 1;
    }
};
```

---

## 2. RequestDataObject(): Creating Output Types

### Purpose and Role

**RequestDataObject()** is called during pipeline execution to ensure the correct output data object type exists before `RequestData()` is called. It creates the appropriate output data object based on input types and algorithm requirements.

**Execution Order**:

```
1. RequestInformation()     - Metadata propagation
2. RequestUpdateExtent()    - Extent determination
3. RequestDataObject()       - Create output data object type
4. RequestData()             - Actual computation
```

### Default Implementation

**Base Class Behavior**:

```cpp
// In vtkAlgorithm (default implementation)
int vtkAlgorithm::RequestDataObject(
    vtkInformation* request,
    vtkInformationVector** inputVector,
    vtkInformationVector* outputVector)
{
    // Get output port information
    vtkInformation* outInfo = outputVector->GetInformationObject(0);
    
    // Get declared output type
    const char* dataType = outInfo->Get(vtkDataObject::DATA_TYPE_NAME());
    
    // Get existing output (if any)
    vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());
    
    // Check if output type matches
    if (output && output->IsA(dataType))
    {
        return 1;  // Output type is correct
    }
    
    // Create new output of correct type
    vtkDataObject* newOutput = vtkDataObject::SafeDownCast(
        vtkDataObjectTypes::NewDataObject(dataType));
    
    if (!newOutput)
    {
        vtkErrorMacro("Cannot create data object of type: " << dataType);
        return 0;
    }
    
    // Set in output port
    outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
    newOutput->Delete();  // Pipeline owns it
    
    return 1;
}
```

### Custom RequestDataObject() Implementation

**When to Override**: When output type depends on input types or algorithm state.

```cpp
class vtkContourFilter : public vtkAlgorithm
{
    int RequestDataObject(
        vtkInformation* request,
        vtkInformationVector** inputVector,
        vtkInformationVector* outputVector) override
    {
        // Get input
        vtkDataObject* input = inputVector[0]->GetInformationObject(0)
            ->Get(vtkDataObject::DATA_OBJECT());
        
        if (!input)
        {
            return 0;
        }
        
        // Contour always produces vtkPolyData (regardless of input type)
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());
        
        // Check if output is already vtkPolyData
        if (output && output->IsA("vtkPolyData"))
        {
            return 1;  // Correct type
        }
        
        // Create vtkPolyData output
        vtkPolyData* newOutput = vtkPolyData::New();
        outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
        newOutput->Delete();
        
        return 1;
    }
};
```

**Example: Output Type Matches Input**:

```cpp
class vtkPassThroughFilter : public vtkAlgorithm
{
    int RequestDataObject(
        vtkInformation* request,
        vtkInformationVector** inputVector,
        vtkInformationVector* outputVector) override
    {
        // Get input
        vtkDataObject* input = inputVector[0]->GetInformationObject(0)
            ->Get(vtkDataObject::DATA_OBJECT());
        
        if (!input)
        {
            return 0;
        }
        
        // Output should match input type
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());
        
        // Check if output type matches input
        if (output && output->GetClassName() == input->GetClassName())
        {
            return 1;  // Types match
        }
        
        // Create output of same type as input
        vtkDataObject* newOutput = input->NewInstance();
        outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
        newOutput->Delete();
        
        return 1;
    }
};
```

---

## 3. Runtime Type Checks in RequestData()

### Type Validation Strategies

**Strategy 1: Using GetData() (Recommended)**:

```cpp
int vtkMyFilter::RequestData(
    vtkInformation* request,
    vtkInformationVector** inputVector,
    vtkInformationVector* outputVector) override
{
    // GetData() automatically validates type
    vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
    
    if (!input)
    {
        vtkErrorMacro("Input is not vtkPolyData");
        return 0;  // Type mismatch
    }
    
    // Get output
    vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);
    
    // Process...
    return 1;
}
```

**How GetData() Works**:

```cpp
// vtkPolyData::GetData() implementation (simplified)
vtkPolyData* vtkPolyData::GetData(
    vtkInformationVector* v, int i)
{
    if (!v || i < 0 || i >= v->GetNumberOfInformationObjects())
    {
        return nullptr;
    }
    
    vtkInformation* info = v->GetInformationObject(i);
    vtkDataObject* data = info->Get(vtkDataObject::DATA_OBJECT());
    
    // Safe downcast to vtkPolyData
    return vtkPolyData::SafeDownCast(data);
}
```

**Strategy 2: Using SafeDownCast()**:

```cpp
int vtkMyFilter::RequestData(...) override
{
    // Get input as base type
    vtkDataObject* inputData = this->GetInputDataObject(0, 0);
    
    if (!inputData)
    {
        vtkErrorMacro("No input data");
        return 0;
    }
    
    // Cast to specific type
    vtkPolyData* input = vtkPolyData::SafeDownCast(inputData);
    
    if (!input)
    {
        vtkErrorMacro("Input must be vtkPolyData, got: " 
                     << inputData->GetClassName());
        return 0;
    }
    
    // Process...
    return 1;
}
```

**Strategy 3: Type Checking with Error Messages**:

```cpp
int vtkMyFilter::RequestData(...) override
{
    vtkDataObject* inputData = this->GetInputDataObject(0, 0);
    
    // Check for specific type
    if (!inputData->IsA("vtkPolyData"))
    {
        vtkErrorMacro("Expected vtkPolyData, got: " 
                     << inputData->GetClassName());
        return 0;
    }
    
    vtkPolyData* input = static_cast<vtkPolyData*>(inputData);
    
    // Process...
    return 1;
}
```

### Multiple Input Type Validation

```cpp
int vtkMergeFilter::RequestData(...) override
{
    // Validate first input
    vtkDataSet* input1 = vtkDataSet::GetData(inputVector[0], 0);
    if (!input1)
    {
        vtkErrorMacro("First input is not a vtkDataSet");
        return 0;
    }
    
    // Validate second input (optional)
    vtkDataSet* input2 = nullptr;
    if (inputVector[1]->GetNumberOfInformationObjects() > 0)
    {
        input2 = vtkDataSet::GetData(inputVector[1], 0);
        if (!input2)
        {
            vtkErrorMacro("Second input is not a vtkDataSet");
            return 0;
        }
    }
    
    // Process...
    return 1;
}
```

### Type Hierarchy Validation

**Accepting Base Types**:

```cpp
// Filter accepts vtkDataSet (base class)
int FillInputPortInformation(int port, vtkInformation* info) override
{
    info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
    return 1;
}

// In RequestData(), handle specific subtypes
int RequestData(...) override
{
    vtkDataSet* input = vtkDataSet::GetData(inputVector[0], 0);
    
    // Handle different dataset types
    if (vtkPolyData::SafeDownCast(input))
    {
        // Handle polygonal data
    }
    else if (vtkUnstructuredGrid::SafeDownCast(input))
    {
        // Handle unstructured grid
    }
    else if (vtkImageData::SafeDownCast(input))
    {
        // Handle image data
    }
    else
    {
        vtkErrorMacro("Unsupported dataset type");
        return 0;
    }
    
    return 1;
}
```

---

## 4. ParaView UI Connection Prevention

### DataTypeDomain in Server Manager XML

**Purpose**: Declare type constraints in XML that ParaView UI uses to prevent incompatible connections.

**Basic DataTypeDomain**:

```xml
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <Proxy name="ContourFilter" class="vtkContourFilter">
      
      <!-- Input property with type constraint -->
      <InputProperty name="Input"
                     command="SetInputConnection"
                     port_index="0">
        <!-- Proxy group domain: which proxy types can connect -->
        <ProxyGroupDomain name="groups">
          <Group name="sources"/>
          <Group name="filters"/>
        </ProxyGroupDomain>
        
        <!-- Data type domain: which data types are accepted -->
        <DataTypeDomain name="input_type">
          <DataType value="vtkDataSet"/>
        </DataTypeDomain>
      </InputProperty>
      
      <!-- Output port with type declaration -->
      <OutputPort name="Output" index="0">
        <DataTypeDomain name="output_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
      </OutputPort>
      
    </Proxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

**Multiple Accepted Types**:

```xml
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
  <DataType value="vtkUnstructuredGrid"/>
</DataTypeDomain>
```

**Any Dataset Type**:

```xml
<DataTypeDomain name="input_type">
  <DataType value="vtkDataSet"/>
</DataTypeDomain>
```

### Server Manager Validation

**Connection Validation Flow**:

```cpp
// In vtkSMProxy (Server Manager)
bool vtkSMProxy::CanAcceptInput(
    vtkSMProxy* inputProxy,
    int inputPort,
    int outputPort)
{
    // Get input property
    vtkSMInputProperty* inputProp = vtkSMInputProperty::SafeDownCast(
        this->GetProperty("Input"));
    
    if (!inputProp)
    {
        return false;
    }
    
    // Get output proxy's output type
    vtkSMOutputPort* outputPortObj = inputProxy->GetOutputPort(outputPort);
    const char* outputType = outputPortObj->GetDataClassName();
    
    // Check DataTypeDomain constraints
    vtkSMDomainIterator* domainIter = inputProp->NewDomainIterator();
    domainIter->Begin();
    
    while (!domainIter->IsAtEnd())
    {
        vtkSMDomain* domain = domainIter->GetDomain();
        
        // Check DataTypeDomain
        vtkSMDataTypeDomain* dataTypeDomain = 
            vtkSMDataTypeDomain::SafeDownCast(domain);
        
        if (dataTypeDomain)
        {
            // Check if output type is accepted
            if (!dataTypeDomain->IsInDomain(outputType))
            {
                domainIter->Delete();
                return false;  // Type mismatch
            }
        }
        
        domainIter->Next();
    }
    
    domainIter->Delete();
    return true;  // Type is compatible
}
```

### UI-Level Prevention

**ParaView UI Connection Validation**:

```cpp
// In pqPipelineBrowser (ParaView UI)
bool pqPipelineBrowser::canConnect(
    pqPipelineSource* source,
    pqPipelineSource* destination)
{
    // Get source output type
    vtkSMOutputPort* outputPort = source->getOutputPort(0);
    const char* outputType = outputPort->GetDataClassName();
    
    // Get destination input property
    vtkSMProxy* destProxy = destination->getProxy();
    vtkSMInputProperty* inputProp = vtkSMInputProperty::SafeDownCast(
        destProxy->GetProperty("Input"));
    
    if (!inputProp)
    {
        return false;
    }
    
    // Check DataTypeDomain
    vtkSMDomainIterator* domainIter = inputProp->NewDomainIterator();
    domainIter->Begin();
    
    bool canConnect = false;
    
    while (!domainIter->IsAtEnd())
    {
        vtkSMDomain* domain = domainIter->GetDomain();
        vtkSMDataTypeDomain* dataTypeDomain = 
            vtkSMDataTypeDomain::SafeDownCast(domain);
        
        if (dataTypeDomain && dataTypeDomain->IsInDomain(outputType))
        {
            canConnect = true;
            break;
        }
        
        domainIter->Next();
    }
    
    domainIter->Delete();
    return canConnect;
}
```

**Visual Feedback in UI**:

```cpp
// In pqPipelineBrowserTreeWidget
void pqPipelineBrowserTreeWidget::updateConnectionValidity()
{
    // For each potential connection
    for (auto& connection : potentialConnections)
    {
        bool valid = canConnect(connection.source, connection.destination);
        
        // Visual feedback
        if (valid)
        {
            // Show green connection indicator
            connection.uiElement->setStyleSheet("color: green;");
        }
        else
        {
            // Show red connection indicator (disabled)
            connection.uiElement->setStyleSheet("color: red;");
            connection.uiElement->setEnabled(false);
        }
    }
}
```

### Type Hierarchy in DataTypeDomain

**Inheritance-Aware Validation**:

```cpp
// In vtkSMDataTypeDomain
bool vtkSMDataTypeDomain::IsInDomain(const char* dataType)
{
    // Check exact match
    for (int i = 0; i < this->GetNumberOfTypes(); i++)
    {
        const char* acceptedType = this->GetType(i);
        
        // Exact match
        if (strcmp(dataType, acceptedType) == 0)
        {
            return true;
        }
        
        // Check inheritance: is dataType a subclass of acceptedType?
        if (vtkDataObjectTypes::TypeExists(dataType) &&
            vtkDataObjectTypes::TypeExists(acceptedType))
        {
            // Check if dataType inherits from acceptedType
            vtkDataObject* testObj = vtkDataObjectTypes::NewDataObject(dataType);
            if (testObj && testObj->IsA(acceptedType))
            {
                testObj->Delete();
                return true;
            }
            if (testObj)
            {
                testObj->Delete();
            }
        }
    }
    
    return false;
}
```

**Example: Inheritance Validation**:

```xml
<!-- Filter accepts vtkDataSet -->
<DataTypeDomain name="input_type">
  <DataType value="vtkDataSet"/>
</DataTypeDomain>
```

**This accepts**:
- `vtkPolyData` (inherits from `vtkDataSet`)
- `vtkUnstructuredGrid` (inherits from `vtkDataSet`)
- `vtkImageData` (inherits from `vtkDataSet`)
- `vtkDataSet` itself

**But rejects**:
- `vtkTable` (does not inherit from `vtkDataSet`)
- `vtkGraph` (does not inherit from `vtkDataSet`)

---

## 5. Complete Type Validation Flow

### End-to-End Validation

**Step 1: XML Declaration** (ParaView):

```xml
<InputProperty name="Input">
  <DataTypeDomain name="input_type">
    <DataType value="vtkPolyData"/>
  </DataTypeDomain>
</InputProperty>
```

**Step 2: UI Connection Attempt** (ParaView UI):

```cpp
// User tries to connect source to filter
if (!canConnect(source, filter))
{
    // Show error: "Incompatible types"
    // Prevent connection
    return;
}
```

**Step 3: Server Manager Validation** (ParaView Server):

```cpp
// When connection is set
vtkSMProxy::SetInputConnection(inputPort, inputProxy, outputPort)
{
    // Validate types
    if (!CanAcceptInput(inputProxy, inputPort, outputPort))
    {
        vtkErrorMacro("Incompatible input type");
        return;
    }
    
    // Set connection
    // ...
}
```

**Step 4: VTK Port Information** (VTK):

```cpp
// Filter declares accepted types
int FillInputPortInformation(int port, vtkInformation* info) override
{
    info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
    return 1;
}
```

**Step 5: RequestDataObject** (VTK):

```cpp
// Creates correct output type
int RequestDataObject(...) override
{
    // Create vtkPolyData output
    // ...
}
```

**Step 6: Runtime Check** (VTK):

```cpp
// Validates types at execution time
int RequestData(...) override
{
    vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
    if (!input)
    {
        return 0;  // Type mismatch
    }
    // Process...
}
```

### Validation Layers Summary

| Layer | Mechanism | Purpose | When |
|-------|-----------|---------|------|
| **XML** | `DataTypeDomain` | Declare type constraints | Filter definition |
| **UI** | `canConnect()` | Prevent invalid connections | User interaction |
| **Server Manager** | `CanAcceptInput()` | Validate before setting | Connection setup |
| **Port Information** | `FillInputPortInformation()` | Declare accepted types | Filter definition |
| **RequestDataObject** | `RequestDataObject()` | Create output type | Pipeline execution |
| **Runtime** | `GetData()`, `SafeDownCast()` | Validate at execution | `RequestData()` |

---

## 6. Practical Examples

### Example 1: Strict Type Filter

**Filter that only accepts vtkPolyData**:

```cpp
// VTK Algorithm
class vtkPolyDataFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
        return 1;
    }
    
    int RequestData(...) override
    {
        vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
        if (!input)
        {
            vtkErrorMacro("Input must be vtkPolyData");
            return 0;
        }
        // Process...
    }
};
```

**ParaView XML**:

```xml
<InputProperty name="Input">
  <DataTypeDomain name="input_type">
    <DataType value="vtkPolyData"/>
  </DataTypeDomain>
</InputProperty>
```

**Result**: UI prevents connecting non-PolyData sources.

### Example 2: Flexible Type Filter

**Filter that accepts any dataset**:

```cpp
// VTK Algorithm
class vtkDataSetFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
        return 1;
    }
    
    int RequestData(...) override
    {
        vtkDataSet* input = vtkDataSet::GetData(inputVector[0], 0);
        if (!input)
        {
            return 0;
        }
        
        // Handle different dataset types
        if (vtkPolyData::SafeDownCast(input))
        {
            // Handle polygonal data
        }
        else if (vtkImageData::SafeDownCast(input))
        {
            // Handle image data
        }
        // ...
    }
};
```

**ParaView XML**:

```xml
<InputProperty name="Input">
  <DataTypeDomain name="input_type">
    <DataType value="vtkDataSet"/>
  </DataTypeDomain>
</InputProperty>
```

**Result**: UI allows connecting any dataset type.

### Example 3: Type-Converting Filter

**Filter that converts input type to output type**:

```cpp
// VTK Algorithm
class vtkToPolyDataFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        // Accept any dataset
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
        return 1;
    }
    
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        // Always output vtkPolyData
        info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        return 1;
    }
    
    int RequestDataObject(...) override
    {
        // Always create vtkPolyData output
        vtkInformation* outInfo = outputVector->GetInformationObject(0);
        vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());
        
        if (!output || !output->IsA("vtkPolyData"))
        {
            vtkPolyData* newOutput = vtkPolyData::New();
            outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
            newOutput->Delete();
        }
        
        return 1;
    }
    
    int RequestData(...) override
    {
        vtkDataSet* input = vtkDataSet::GetData(inputVector[0], 0);
        vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);
        
        // Convert input to PolyData
        // ...
    }
};
```

**ParaView XML**:

```xml
<InputProperty name="Input">
  <DataTypeDomain name="input_type">
    <DataType value="vtkDataSet"/>
  </DataTypeDomain>
</InputProperty>

<OutputPort name="Output" index="0">
  <DataTypeDomain name="output_type">
    <DataType value="vtkPolyData"/>
  </DataTypeDomain>
</OutputPort>
```

**Result**: UI allows any dataset input, but downstream filters see PolyData output.

---

## 7. Summary: Type Declaration and Validation

### Key Mechanisms

1. **FillInputPortInformation()**:
   - Declares accepted input types
   - Used by VTK pipeline system
   - Basis for type validation

2. **FillOutputPortInformation()**:
   - Declares output types
   - Used by downstream filters
   - Basis for connection validation

3. **RequestDataObject()**:
   - Creates appropriate output type
   - Called before `RequestData()`
   - Ensures correct output type exists

4. **Runtime Checks**:
   - `GetData()`: Type-safe data retrieval
   - `SafeDownCast()`: Type validation
   - `IsA()`: Type checking

5. **DataTypeDomain**:
   - XML type constraints
   - UI connection prevention
   - Server Manager validation

### Validation Flow

```
XML Definition (DataTypeDomain)
    ↓
UI Connection Attempt
    ↓
Server Manager Validation (CanAcceptInput)
    ↓
VTK Port Information (FillInputPortInformation)
    ↓
RequestDataObject (Create Output Type)
    ↓
RequestData (Runtime Type Check)
```

### Best Practices

1. **Declare Types Explicitly**: Use `FillInputPortInformation()` to declare accepted types
2. **Use GetData()**: Prefer `GetData()` over manual casting for type safety
3. **Validate at Multiple Layers**: XML, UI, Server Manager, and runtime
4. **Provide Clear Error Messages**: Help users understand type mismatches
5. **Use Type Hierarchies**: Accept base types when appropriate (e.g., `vtkDataSet`)
6. **Override RequestDataObject()**: When output type depends on input or algorithm state

This multi-layered type system ensures **type-safe pipeline construction** from UI interaction through VTK pipeline execution, preventing incompatible connections and providing clear feedback to users.

