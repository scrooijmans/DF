# ParaView/VTK Pipeline Filters Architecture

This document explains in detail how ParaView/VTK defines "processors" as pipeline filters (`vtkAlgorithm`, `vtkDataObject` inputs/outputs), including how filters declare accepted input types/ports and output types, and how this is represented in ParaView's proxy XML / server manager layer.

## Overview

ParaView/VTK uses a **pipeline-based architecture** where filters are defined as algorithms:

1. **vtkAlgorithm**: Base class for all pipeline filters
2. **Input/Output Ports**: Declared via port information methods
3. **vtkDataObject Types**: Ports accept specific data object types
4. **Proxy XML**: ParaView defines filters in XML for Server Manager
5. **Server Manager**: Manages VTK objects on server side

**Key Principles:**

- **Pipeline Architecture**: Filters connected in a directed acyclic graph (DAG)
- **Type Safety**: Ports declare accepted input/output types
- **Proxy Pattern**: ParaView uses proxies to abstract VTK objects
- **XML Configuration**: Filter definitions in XML for Server Manager
- **Client-Server**: Proxies bridge client UI and server VTK objects

## VTK Pipeline Architecture

### vtkAlgorithm: Base Class for Filters

**Class Hierarchy:**

```
vtkObject
    └── vtkAlgorithm
        ├── vtkSource (data sources)
        ├── vtkFilter (data filters)
        └── vtkMapper (rendering)
```

**Core Methods:**

```cpp
class vtkAlgorithm : public vtkObject
{
public:
    // Port management
    virtual int FillInputPortInformation(int port, vtkInformation* info);
    virtual int FillOutputPortInformation(int port, vtkInformation* info);

    // Pipeline execution
    virtual int RequestData(vtkInformation* request,
                          vtkInformationVector** inputVector,
                          vtkInformationVector* outputVector);

    // Input/output access
    vtkDataObject* GetInputDataObject(int port, int connection);
    vtkDataObject* GetOutputDataObject(int port);

    // Port information
    int GetNumberOfInputPorts();
    int GetNumberOfOutputPorts();
};
```

### Input/Output Ports

**Port Declaration:**

```cpp
class MyFilter : public vtkAlgorithm
{
public:
    static MyFilter* New();

    // Declare input port information
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Accept any dataset type
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
            // Or accept specific type:
            // info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");

            // Optional input
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);  // Required
        }
        return 1;
    }

    // Declare output port information
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Output is same type as input
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }
};
```

### Port Type Constraints

**Accepted Data Types:**

```cpp
// Accept any dataset
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");

// Accept specific dataset types
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");      // Polylines/polygons
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkUnstructuredGrid");  // Unstructured
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkStructuredGrid");    // Structured
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkImageData");         // Image/volume
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkRectilinearGrid");   // Rectilinear

// Accept multiple types (comma-separated)
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
          "vtkPolyData;vtkUnstructuredGrid");

// Accept any data object
info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataObject");
```

**Port Properties:**

```cpp
// Required vs optional
info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);  // Required
info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 1);  // Optional

// Multiple connections allowed
info->Set(vtkAlgorithm::INPUT_IS_REPEATABLE(), 0);  // Single connection
info->Set(vtkAlgorithm::INPUT_IS_REPEATABLE(), 1);  // Multiple connections

// Port name (for documentation)
info->Set(vtkAlgorithm::INPUT_PORT_NAME(), "Input");
```

### Complete Filter Example

```cpp
class vtkBufferFilter : public vtkAlgorithm
{
public:
    static vtkBufferFilter* New();
    vtkTypeMacro(vtkBufferFilter, vtkAlgorithm);

    // Input port information
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Accept polygonal data
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
            info->Set(vtkAlgorithm::INPUT_PORT_NAME(), "Input PolyData");
        }
        return 1;
    }

    // Output port information
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // Output is polygonal data
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }

    // Pipeline execution
    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        // Get input
        vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
        if (!input)
        {
            return 0;
        }

        // Get output
        vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);

        // Process data
        // ... buffer algorithm ...

        return 1;
    }

    // Algorithm properties
    vtkSetMacro(BufferDistance, double);
    vtkGetMacro(BufferDistance, double);

private:
    double BufferDistance = 1.0;
};
```

## ParaView Proxy XML

### Proxy Definition Structure

**Basic Proxy XML:**

```xml
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <Proxy name="BufferFilter"
           class="vtkBufferFilter"
           label="Buffer">

      <!-- Input port definition -->
      <InputProperty name="Input"
                     command="SetInputConnection">
        <ProxyGroupDomain name="groups">
          <Group name="sources"/>
          <Group name="filters"/>
        </ProxyGroupDomain>
        <DataTypeDomain name="input_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
        <InputArrayDomain name="input_array" number_of_components="1">
          <RequiredProperties>
            <Property name="Input" function="Input"/>
          </RequiredProperties>
        </InputArrayDomain>
      </InputProperty>

      <!-- Output port definition -->
      <OutputPort name="Output"
                 index="0"
                 id="port0">
        <DataTypeDomain name="output_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
      </OutputPort>

      <!-- Algorithm properties -->
      <DoubleVectorProperty name="BufferDistance"
                           command="SetBufferDistance"
                           number_of_elements="1"
                           default_values="1.0">
        <DoubleRangeDomain name="range" min="0.0" max="1000.0"/>
      </DoubleVectorProperty>

    </Proxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

### Input Port Definition

**Input Port XML:**

```xml
<InputProperty name="Input"
               command="SetInputConnection"
               port_index="0"
               optional="0">

  <!-- Allowed proxy groups -->
  <ProxyGroupDomain name="groups">
    <Group name="sources"/>      <!-- Data sources -->
    <Group name="filters"/>       <!-- Other filters -->
  </ProxyGroupDomain>

  <!-- Allowed data types -->
  <DataTypeDomain name="input_type">
    <DataType value="vtkPolyData"/>
    <!-- Or multiple types: -->
    <!-- <DataType value="vtkPolyData"/> -->
    <!-- <DataType value="vtkUnstructuredGrid"/> -->
  </DataTypeDomain>

  <!-- Optional: array requirements -->
  <InputArrayDomain name="input_array"
                    number_of_components="1"
                    attribute_type="point">
    <RequiredProperties>
      <Property name="Input" function="Input"/>
    </RequiredProperties>
  </InputArrayDomain>

</InputProperty>
```

**Multiple Input Ports:**

```xml
<Proxy name="MergeFilter" class="vtkAppendFilter">

  <!-- First input -->
  <InputProperty name="Input1"
                 command="SetInputConnection"
                 port_index="0"
                 repeatable="1">
    <ProxyGroupDomain name="groups">
      <Group name="sources"/>
      <Group name="filters"/>
    </ProxyGroupDomain>
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

  <!-- Second input (optional) -->
  <InputProperty name="Input2"
                 command="AddInputConnection"
                 port_index="0"
                 repeatable="1"
                 optional="1">
    <ProxyGroupDomain name="groups">
      <Group name="sources"/>
      <Group name="filters"/>
    </ProxyGroupDomain>
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

</Proxy>
```

### Output Port Definition

**Output Port XML:**

```xml
<OutputPort name="Output"
           index="0"
           id="port0">

  <!-- Output data type -->
  <DataTypeDomain name="output_type">
    <DataType value="vtkPolyData"/>
  </DataTypeDomain>

  <!-- Optional: output array information -->
  <ArraySelectionDomain name="array_list">
    <RequiredProperties>
      <Property name="Input" function="Input"/>
    </RequiredProperties>
  </ArraySelectionDomain>

</OutputPort>
```

**Multiple Output Ports:**

```xml
<Proxy name="SplitFilter" class="vtkSplitFilter">

  <!-- First output -->
  <OutputPort name="Output1"
              index="0"
              id="port0">
    <DataTypeDomain name="output_type">
      <DataType value="vtkPolyData"/>
    </DataTypeDomain>
  </OutputPort>

  <!-- Second output -->
  <OutputPort name="Output2"
              index="1"
              id="port1">
    <DataTypeDomain name="output_type">
      <DataType value="vtkPolyData"/>
    </DataTypeDomain>
  </OutputPort>

</Proxy>
```

### Data Type Domains

**DataTypeDomain:**

```xml
<!-- Single type -->
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
</DataTypeDomain>

<!-- Multiple types -->
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
  <DataType value="vtkUnstructuredGrid"/>
  <DataType value="vtkStructuredGrid"/>
</DataTypeDomain>

<!-- Any dataset -->
<DataTypeDomain name="input_type">
  <DataType value="vtkDataSet"/>
</DataTypeDomain>

<!-- Any data object -->
<DataTypeDomain name="input_type">
  <DataType value="vtkDataObject"/>
</DataTypeDomain>
```

## Server Manager Layer

### Proxy Architecture

**Three-Layer Architecture:**

```
┌─────────────────────────────────────────────────────────────┐
│              ParaView Architecture                          │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Client Layer (pq*)                                  │   │
│  │  - Qt UI components                                  │   │
│  │  - pqProxy (client-side proxy)                       │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Communication Layer (vtkClientServerStream)         │   │
│  │  - RPC serialization                                  │   │
│  │  - Network transmission                               │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Server Layer (vtkSM*)                                │   │
│  │  - vtkSMProxy (server-side proxy)                    │   │
│  │  - vtkPV* (server-side VTK objects)                  │   │
│  │  - vtkAlgorithm (pipeline filters)                  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Proxy Creation from XML

**XML to Proxy Flow:**

```cpp
// 1. Load XML configuration
vtkSMProxyManager::GetProxyManager()->LoadConfigurationXML(xmlString);

// 2. Create proxy from XML definition
vtkSMProxy* proxy = vtkSMProxyManager::GetProxyManager()->NewProxy(
    "filters",           // Proxy group
    "BufferFilter"       // Proxy name
);

// 3. Proxy internally creates VTK algorithm
// XML definition → vtkAlgorithm instance

// 4. Set input connection
proxy->SetInputConnection(0, inputProxy, 0);

// 5. Update pipeline
proxy->UpdatePipeline();
```

### Proxy Property Mapping

**Property XML to VTK Method:**

```xml
<DoubleVectorProperty name="BufferDistance"
                     command="SetBufferDistance"
                     number_of_elements="1"
                     default_values="1.0">
```

**Maps to:**

```cpp
// Server Manager calls:
vtkAlgorithm* algorithm = proxy->GetClientSideObject();
algorithm->SetBufferDistance(1.0);
```

### Input Connection Management

**Connection XML:**

```xml
<InputProperty name="Input"
               command="SetInputConnection"
               port_index="0">
```

**Server Manager Connection:**

```cpp
class vtkSMProxy
{
public:
    // Set input connection
    void SetInputConnection(int outputPort,
                           vtkSMProxy* input,
                           int inputPort);

    // Get input proxy
    vtkSMProxy* GetInputProxy(int port, int connection);

    // Get input data object
    vtkDataObject* GetInputDataObject(int port, int connection);
};
```

## Complete Example: Custom Filter

### VTK Algorithm Implementation

```cpp
// vtkMyCustomFilter.h
class vtkMyCustomFilter : public vtkAlgorithm
{
public:
    static vtkMyCustomFilter* New();
    vtkTypeMacro(vtkMyCustomFilter, vtkAlgorithm);

    // Input port: accepts vtkPolyData
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
            info->Set(vtkAlgorithm::INPUT_PORT_NAME(), "Input");
        }
        return 1;
    }

    // Output port: produces vtkPolyData
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }

    // Pipeline execution
    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        // Get input
        vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
        if (!input)
        {
            vtkErrorMacro("No input data");
            return 0;
        }

        // Get output
        vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);

        // Process data
        output->ShallowCopy(input);
        // ... custom processing ...

        return 1;
    }

    // Properties
    vtkSetMacro(Parameter, double);
    vtkGetMacro(Parameter, double);

private:
    double Parameter = 1.0;
};
```

### ParaView Proxy XML

```xml
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <Proxy name="MyCustomFilter"
           class="vtkMyCustomFilter"
           label="My Custom Filter">

      <!-- Documentation -->
      <Documentation>
        <ShortHelp>Custom filter for processing polygonal data</ShortHelp>
        <LongHelp>
          This filter processes vtkPolyData input and produces
          vtkPolyData output with custom processing applied.
        </LongHelp>
      </Documentation>

      <!-- Input port -->
      <InputProperty name="Input"
                     command="SetInputConnection"
                     port_index="0"
                     optional="0">
        <ProxyGroupDomain name="groups">
          <Group name="sources"/>
          <Group name="filters"/>
        </ProxyGroupDomain>
        <DataTypeDomain name="input_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
      </InputProperty>

      <!-- Output port -->
      <OutputPort name="Output"
                 index="0"
                 id="port0">
        <DataTypeDomain name="output_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
      </OutputPort>

      <!-- Filter properties -->
      <DoubleVectorProperty name="Parameter"
                           command="SetParameter"
                           number_of_elements="1"
                           default_values="1.0"
                           label="Parameter">
        <DoubleRangeDomain name="range" min="0.0" max="100.0"/>
        <Documentation>
          Parameter controlling filter behavior.
        </Documentation>
      </DoubleVectorProperty>

    </Proxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

## Port Type Checking

### Runtime Type Validation

**Type Checking in RequestData:**

```cpp
int RequestData(vtkInformation* request,
               vtkInformationVector** inputVector,
               vtkInformationVector* outputVector) override
{
    // Get input with type checking
    vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
    if (!input)
    {
        vtkErrorMacro("Input is not vtkPolyData");
        return 0;
    }

    // Alternative: Get as base type and check
    vtkDataObject* inputData = GetInputDataObject(0, 0);
    vtkPolyData* polyData = vtkPolyData::SafeDownCast(inputData);
    if (!polyData)
    {
        vtkErrorMacro("Input must be vtkPolyData");
        return 0;
    }

    // Process...
    return 1;
}
```

### Port Information Query

**Querying Port Information:**

```cpp
// Get input port information
vtkInformation* inputInfo = this->GetInputPortInformation(0);
const char* requiredType = inputInfo->Get(
    vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE()
);

// Get output port information
vtkInformation* outputInfo = this->GetOutputPortInformation(0);
const char* outputType = outputInfo->Get(
    vtkDataObject::DATA_TYPE_NAME()
);

// Check if input is optional
int isOptional = inputInfo->Get(vtkAlgorithm::INPUT_IS_OPTIONAL());
```

## Advanced Port Features

### Multiple Input Ports

```cpp
class vtkMergeFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // First input: required
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
        }
        else if (port == 1)
        {
            // Second input: optional
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
            info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 1);
        }
        return 1;
    }
};
```

**XML for Multiple Inputs:**

```xml
<Proxy name="MergeFilter" class="vtkMergeFilter">

  <!-- First input (required) -->
  <InputProperty name="Input1"
                 command="SetInputConnection"
                 port_index="0"
                 optional="0">
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

  <!-- Second input (optional) -->
  <InputProperty name="Input2"
                 command="SetInputConnection"
                 port_index="1"
                 optional="1">
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

</Proxy>
```

### Multiple Output Ports

```cpp
class vtkSplitFilter : public vtkAlgorithm
{
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        else if (port == 1)
        {
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }
};
```

**XML for Multiple Outputs:**

```xml
<Proxy name="SplitFilter" class="vtkSplitFilter">

  <!-- First output -->
  <OutputPort name="Output1"
             index="0"
             id="port0">
    <DataTypeDomain name="output_type">
      <DataType value="vtkPolyData"/>
    </DataTypeDomain>
  </OutputPort>

  <!-- Second output -->
  <OutputPort name="Output2"
             index="1"
             id="port1">
    <DataTypeDomain name="output_type">
      <DataType value="vtkPolyData"/>
    </DataTypeDomain>
  </OutputPort>

</Proxy>
```

### Repeatable Input Ports

**Multiple Connections:**

```cpp
class vtkAppendFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        // Accept multiple connections on port 0
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkDataSet");
        info->Set(vtkAlgorithm::INPUT_IS_REPEATABLE(), 1);
        return 1;
    }
};
```

**XML for Repeatable Inputs:**

```xml
<InputProperty name="Input"
               command="AddInputConnection"
               port_index="0"
               repeatable="1">
  <ProxyGroupDomain name="groups">
    <Group name="sources"/>
    <Group name="filters"/>
  </ProxyGroupDomain>
  <DataTypeDomain name="input_type">
    <DataType value="vtkDataSet"/>
  </DataTypeDomain>
</InputProperty>
```

## Server Manager Proxy Creation

### Proxy Registration

**XML Loading:**

```cpp
// Load proxy XML configuration
vtkSMProxyManager* proxyManager = vtkSMProxyManager::GetProxyManager();

// Load from file
proxyManager->LoadConfigurationXMLFromFile("MyFilters.xml");

// Or load from string
const char* xmlConfig = "...";
proxyManager->LoadConfigurationXML(xmlConfig);
```

### Proxy Instantiation

**Creating Proxy:**

```cpp
// Create proxy instance
vtkSMProxy* proxy = proxyManager->NewProxy(
    "filters",           // Proxy group (from XML)
    "MyCustomFilter"     // Proxy name (from XML)
);

// Proxy internally:
// 1. Parses XML definition
// 2. Creates vtkAlgorithm instance
// 3. Sets up property mappings
// 4. Configures input/output ports
```

### Pipeline Connection

**Connecting Filters:**

```cpp
// Create source
vtkSMProxy* source = proxyManager->NewProxy("sources", "SphereSource");

// Create filter
vtkSMProxy* filter = proxyManager->NewProxy("filters", "MyCustomFilter");

// Connect: source output → filter input
filter->SetInputConnection(0, source, 0);

// Update pipeline
filter->UpdatePipeline();

// Get output
vtkDataObject* output = filter->GetOutputDataObject(0);
```

## Type System Summary

### vtkDataObject Hierarchy

```
vtkDataObject (abstract base)
    ├── vtkDataSet (abstract)
    │   ├── vtkPolyData (polylines, polygons)
    │   ├── vtkUnstructuredGrid (arbitrary cells)
    │   ├── vtkStructuredGrid (topologically regular)
    │   ├── vtkImageData (regular grid)
    │   └── vtkRectilinearGrid (axis-aligned)
    ├── vtkGraph (networks)
    ├── vtkTable (tabular data)
    └── vtkMolecule (molecular data)
```

### Port Type Declarations

| Declaration Method            | Purpose                   | Example                                                |
| ----------------------------- | ------------------------- | ------------------------------------------------------ |
| `FillInputPortInformation()`  | Declare input port types  | `info->Set(INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData")` |
| `FillOutputPortInformation()` | Declare output port types | `info->Set(DATA_TYPE_NAME(), "vtkPolyData")`           |
| `<DataTypeDomain>` in XML     | XML type constraint       | `<DataType value="vtkPolyData"/>`                      |
| `GetInputDataObject()`        | Runtime type retrieval    | `vtkPolyData::SafeDownCast(input)`                     |

## Summary

### Filter Definition Components

1. **VTK Algorithm (`vtkAlgorithm`)**
   - Base class for all filters
   - Declares input/output ports
   - Implements `RequestData()` for processing

2. **Port Information**
   - `FillInputPortInformation()`: Declares accepted input types
   - `FillOutputPortInformation()`: Declares output types
   - Type constraints via `vtkInformation`

3. **ParaView Proxy XML**
   - Defines filter in Server Manager
   - Maps properties to VTK methods
   - Declares input/output ports
   - Specifies data type domains

4. **Server Manager**
   - Creates VTK objects from XML
   - Manages proxy instances
   - Handles client-server communication

### Key Mechanisms

- **Type Safety**: Ports enforce data type constraints
- **Pipeline Execution**: `RequestData()` processes inputs to outputs
- **Proxy Pattern**: XML proxies abstract VTK objects
- **Client-Server**: Proxies bridge UI and VTK pipeline
- **Automatic Validation**: Server Manager validates port connections

This architecture ensures **type-safe pipeline connections**, **flexible filter definitions**, and **seamless integration** between ParaView UI and VTK pipeline execution.
