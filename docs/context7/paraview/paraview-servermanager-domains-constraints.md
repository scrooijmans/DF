# ParaView ServerManager Domains & Constraints

This document explains in detail how ParaView's ServerManager proxies expose filter parameters and constraints (domains, input data type domains, array selection domains), and how those constraints map to actual VTK pipeline type requirements.

## Overview

ParaView's ServerManager uses a **domain-based constraint system** to validate filter parameters:

1. **Properties**: Expose VTK algorithm parameters
2. **Domains**: Constrain property values
3. **Type Validation**: Ensure compatibility with VTK pipeline
4. **Runtime Checking**: Validate before pipeline execution

**Key Principles:**

- **Domain-Based Validation**: Domains enforce constraints on properties
- **VTK Type Mapping**: Domains map to VTK pipeline requirements
- **Dynamic Updates**: Domains update based on input connections
- **UI Integration**: Domains control UI widget behavior
- **Error Prevention**: Invalid values rejected before VTK execution

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│         ParaView ServerManager Constraint System            │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Proxy XML Definition                                │   │
│  │  - Property definitions                              │   │
│  │  - Domain definitions                                │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  vtkSMProxy                                          │   │
│  │  - vtkSMProperty (properties)                       │   │
│  │  - vtkSMDomain (domains)                             │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Domain Validation                                   │   │
│  │  - Type checking                                     │   │
│  │  - Range validation                                  │   │
│  │  - Array validation                                  │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  VTK Pipeline                                       │   │
│  │  - FillInputPortInformation()                      │   │
│  │  - RequestData()                                    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Properties & Domains

### Property Definition

**Basic Property Structure:**

```xml
<Proxy name="MyFilter" class="vtkMyFilter">

  <!-- Property with domain -->
  <DoubleVectorProperty name="Radius"
                       command="SetRadius"
                       number_of_elements="1"
                       default_values="1.0">

    <!-- Domain constraint -->
    <DoubleRangeDomain name="range" min="0.0" max="100.0"/>

  </DoubleVectorProperty>

</Proxy>
```

### Domain Types

**Domain Hierarchy:**

```
vtkSMDomain (base class)
    ├── vtkSMDoubleRangeDomain
    ├── vtkSMIntRangeDomain
    ├── vtkSMEnumerationDomain
    ├── vtkSMStringListDomain
    ├── vtkSMFileListDomain
    ├── vtkSMProxyGroupDomain
    ├── vtkSMDataTypeDomain
    ├── vtkSMInputArrayDomain
    ├── vtkSMArraySelectionDomain
    └── vtkSMTimeStepsDomain
```

## DataTypeDomain

### Purpose

`DataTypeDomain` constrains input properties to accept specific VTK data types, mapping directly to `FillInputPortInformation()` requirements.

### XML Definition

```xml
<InputProperty name="Input"
               command="SetInputConnection"
               port_index="0">

  <!-- Data type constraint -->
  <DataTypeDomain name="input_type">
    <DataType value="vtkPolyData"/>
  </DataTypeDomain>

</InputProperty>
```

### Multiple Data Types

```xml
<DataTypeDomain name="input_type">
  <!-- Accept multiple types -->
  <DataType value="vtkPolyData"/>
  <DataType value="vtkUnstructuredGrid"/>
  <DataType value="vtkStructuredGrid"/>
</DataTypeDomain>
```

### Mapping to VTK

**VTK Algorithm Declaration:**

```cpp
class vtkMyFilter : public vtkAlgorithm
{
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            // VTK declares accepted type
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
                     "vtkPolyData");
        }
        return 1;
    }
};
```

**ServerManager Domain:**

```xml
<!-- ParaView XML enforces same constraint -->
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
</DataTypeDomain>
```

**Validation Flow:**

```
1. User connects input in ParaView UI
   ↓
2. ServerManager checks DataTypeDomain
   ↓
3. Validates input data type matches domain
   ↓
4. If valid: Connection allowed
   ↓
5. If invalid: Connection rejected (UI error)
   ↓
6. VTK pipeline: FillInputPortInformation() validates at runtime
```

### Complete Example

**VTK Algorithm:**

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
        }
        return 1;
    }
};
```

**ParaView Proxy XML:**

```xml
<Proxy name="Contour" class="vtkContourFilter">

  <InputProperty name="Input"
                 command="SetInputConnection"
                 port_index="0">

    <ProxyGroupDomain name="groups">
      <Group name="sources"/>
      <Group name="filters"/>
    </ProxyGroupDomain>

    <!-- Maps to FillInputPortInformation() -->
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>

  </InputProperty>

</Proxy>
```

## InputArrayDomain

### Purpose

`InputArrayDomain` ensures that input data contains required attribute arrays (point data, cell data) with specific properties (number of components, data type).

### XML Definition

```xml
<IntVectorProperty name="ScalarArray"
                   command="SetInputArrayToProcess"
                   number_of_elements="4"
                   default_values="0 0 0 0">

  <!-- Array selection domain -->
  <InputArrayDomain name="input_array"
                    number_of_components="1"
                    attribute_type="point">
    <RequiredProperties>
      <Property name="Input" function="Input"/>
    </RequiredProperties>
  </InputArrayDomain>

</IntVectorProperty>
```

### Domain Attributes

**Attribute Types:**

```xml
<!-- Point data arrays -->
<InputArrayDomain name="input_array"
                  attribute_type="point">
  <!-- ... -->
</InputArrayDomain>

<!-- Cell data arrays -->
<InputArrayDomain name="input_array"
                  attribute_type="cell">
  <!-- ... -->
</InputArrayDomain>

<!-- Field data arrays -->
<InputArrayDomain name="input_array"
                  attribute_type="field">
  <!-- ... -->
</InputArrayDomain>

<!-- Any attribute type -->
<InputArrayDomain name="input_array"
                  attribute_type="any">
  <!-- ... -->
</InputArrayDomain>
```

**Component Constraints:**

```xml
<!-- Single component (scalar) -->
<InputArrayDomain name="input_array"
                  number_of_components="1"
                  attribute_type="point">
  <!-- ... -->
</InputArrayDomain>

<!-- Three components (vector) -->
<InputArrayDomain name="input_array"
                  number_of_components="3"
                  attribute_type="point">
  <!-- ... -->
</InputArrayDomain>

<!-- Any number of components -->
<InputArrayDomain name="input_array"
                  number_of_components="any"
                  attribute_type="point">
  <!-- ... -->
</InputArrayDomain>
```

### Mapping to VTK

**VTK Array Selection:**

```cpp
class vtkGradientFilter : public vtkAlgorithm
{
    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        vtkDataSet* input = vtkDataSet::GetData(inputVector[0], 0);

        // Get selected array (from InputArrayDomain)
        vtkDataArray* array = this->GetInputArrayToProcess(0, input);

        if (!array)
        {
            vtkErrorMacro("No input array selected");
            return 0;
        }

        // Validate array properties
        if (array->GetNumberOfComponents() != 1)
        {
            vtkErrorMacro("Array must be scalar (1 component)");
            return 0;
        }

        // Process...
        return 1;
    }
};
```

**ParaView Proxy XML:**

```xml
<Proxy name="Gradient" class="vtkGradientFilter">

  <InputProperty name="Input"
                 command="SetInputConnection"
                 port_index="0">
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

  <!-- Array selection property -->
  <IntVectorProperty name="ScalarArray"
                     command="SetInputArrayToProcess"
                     number_of_elements="4"
                     default_values="0 0 0 0"
                     label="Scalar Array">

    <!-- Domain enforces array requirements -->
    <InputArrayDomain name="input_array"
                      number_of_components="1"
                      attribute_type="point">
      <RequiredProperties>
        <Property name="Input" function="Input"/>
      </RequiredProperties>
    </InputArrayDomain>

  </IntVectorProperty>

</Proxy>
```

### Dynamic Array Updates

**Domain Updates Based on Input:**

```cpp
// When input changes:
// 1. InputArrayDomain queries input dataset
// 2. Discovers available arrays
// 3. Filters arrays based on domain constraints
// 4. Updates UI dropdown with valid arrays
```

**Example: Gradient Filter**

```xml
<Proxy name="Gradient" class="vtkGradientFilter">

  <!-- Input array selection -->
  <IntVectorProperty name="ScalarArray"
                     command="SetInputArrayToProcess"
                     number_of_elements="4"
                     default_values="0 0 0 0">

    <InputArrayDomain name="input_array"
                      number_of_components="1"
                      attribute_type="point">
      <RequiredProperties>
        <Property name="Input" function="Input"/>
      </RequiredProperties>
    </InputArrayDomain>

  </IntVectorProperty>

  <!-- Output array selection -->
  <IntVectorProperty name="ResultArray"
                     command="SetResultArrayName"
                     number_of_components="1"
                     default_values="Gradient">
    <!-- No domain - free text input -->
  </IntVectorProperty>

</Proxy>
```

## ArraySelectionDomain

### Purpose

`ArraySelectionDomain` allows selection of multiple arrays from input data, typically for operations that process multiple arrays simultaneously.

### XML Definition

```xml
<StringVectorProperty name="CellArrays"
                     command="SetCellArrayStatus"
                     number_of_elements="0"
                     repeat_command="1"
                     label="Cell Arrays">

  <!-- Array selection domain -->
  <ArraySelectionDomain name="array_list"
                         attribute_type="cell">
    <RequiredProperties>
      <Property name="Input" function="Input"/>
    </RequiredProperties>
  </ArraySelectionDomain>

</StringVectorProperty>
```

### Usage Example

**VTK Algorithm:**

```cpp
class vtkPassArrays : public vtkAlgorithm
{
    // Enable/disable arrays
    void SetCellArrayStatus(const char* name, int status);
    int GetCellArrayStatus(const char* name);

    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        vtkDataSet* input = vtkDataSet::GetData(inputVector[0], 0);
        vtkDataSet* output = vtkDataSet::GetData(outputVector, 0);

        // Copy selected arrays
        vtkCellData* cellData = input->GetCellData();
        for (int i = 0; i < cellData->GetNumberOfArrays(); i++)
        {
            const char* name = cellData->GetArrayName(i);
            if (this->GetCellArrayStatus(name))
            {
                output->GetCellData()->AddArray(cellData->GetArray(i));
            }
        }

        return 1;
    }
};
```

**ParaView Proxy XML:**

```xml
<Proxy name="PassArrays" class="vtkPassArrays">

  <InputProperty name="Input"
                 command="SetInputConnection"
                 port_index="0">
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

  <!-- Cell array selection -->
  <StringVectorProperty name="CellArrays"
                       command="SetCellArrayStatus"
                       number_of_elements="0"
                       repeat_command="1"
                       label="Cell Arrays">

    <ArraySelectionDomain name="array_list"
                         attribute_type="cell">
      <RequiredProperties>
        <Property name="Input" function="Input"/>
      </RequiredProperties>
    </ArraySelectionDomain>

  </StringVectorProperty>

  <!-- Point array selection -->
  <StringVectorProperty name="PointArrays"
                       command="SetPointArrayStatus"
                       number_of_elements="0"
                       repeat_command="1"
                       label="Point Arrays">

    <ArraySelectionDomain name="array_list"
                         attribute_type="point">
      <RequiredProperties>
        <Property name="Input" function="Input"/>
      </RequiredProperties>
    </ArraySelectionDomain>

  </StringVectorProperty>

</Proxy>
```

## Range Domains

### DoubleRangeDomain

**Purpose**: Constrain floating-point properties to valid ranges.

**XML Definition:**

```xml
<DoubleVectorProperty name="Radius"
                     command="SetRadius"
                     number_of_elements="1"
                     default_values="1.0">

  <!-- Range constraint -->
  <DoubleRangeDomain name="range" min="0.0" max="100.0"/>

</DoubleVectorProperty>
```

**VTK Mapping:**

```cpp
class vtkSphereSource : public vtkSource
{
    vtkSetClampMacro(Radius, double, 0.0, VTK_DOUBLE_MAX);
    vtkGetMacro(Radius, double);

private:
    double Radius = 1.0;
};
```

### IntRangeDomain

**Purpose**: Constrain integer properties to valid ranges.

**XML Definition:**

```xml
<IntVectorProperty name="Resolution"
                  command="SetResolution"
                  number_of_elements="1"
                  default_values="10">

  <!-- Integer range constraint -->
  <IntRangeDomain name="range" min="3" max="100"/>

</IntVectorProperty>
```

**VTK Mapping:**

```cpp
class vtkSphereSource : public vtkSource
{
    vtkSetClampMacro(Resolution, int, 3, VTK_INT_MAX);
    vtkGetMacro(Resolution, int);

private:
    int Resolution = 10;
};
```

## EnumerationDomain

### Purpose

`EnumerationDomain` restricts property values to a predefined set of options.

### XML Definition

```xml
<IntVectorProperty name="ExtractionMode"
                  command="SetExtractionMode"
                  number_of_elements="1"
                  default_values="0"
                  label="Extraction Mode">

  <!-- Enumeration domain -->
  <EnumerationDomain name="enum">
    <Entry value="0" text="Point"/>
    <Entry value="1" text="Cell"/>
    <Entry value="2" text="Vertex"/>
    <Entry value="3" text="Edge"/>
  </EnumerationDomain>

</IntVectorProperty>
```

### VTK Mapping

```cpp
class vtkExtractSelection : public vtkAlgorithm
{
    enum ExtractionMode
    {
        POINT = 0,
        CELL = 1,
        VERTEX = 2,
        EDGE = 3
    };

    vtkSetMacro(ExtractionMode, int);
    vtkGetMacro(ExtractionMode, int);

private:
    int ExtractionMode = POINT;
};
```

## ProxyGroupDomain

### Purpose

`ProxyGroupDomain` restricts input connections to proxies from specific groups (sources, filters, etc.).

### XML Definition

```xml
<InputProperty name="Input"
               command="SetInputConnection"
               port_index="0">

  <!-- Proxy group constraint -->
  <ProxyGroupDomain name="groups">
    <Group name="sources"/>      <!-- Allow sources -->
    <Group name="filters"/>      <!-- Allow filters -->
    <!-- Exclude: <Group name="readers"/> -->
  </ProxyGroupDomain>

  <DataTypeDomain name="input_type">
    <DataType value="vtkDataSet"/>
  </DataTypeDomain>

</InputProperty>
```

### Usage

**Restrict to Sources Only:**

```xml
<InputProperty name="Input"
               command="SetInputConnection"
               port_index="0">

  <ProxyGroupDomain name="groups">
    <Group name="sources"/>      <!-- Only sources allowed -->
  </ProxyGroupDomain>

</InputProperty>
```

**Allow Filters Only:**

```xml
<InputProperty name="Input"
               command="SetInputConnection"
               port_index="0">

  <ProxyGroupDomain name="groups">
    <Group name="filters"/>      <!-- Only filters allowed -->
  </ProxyGroupDomain>

</InputProperty>
```

## Domain Dependencies

### RequiredProperties

**Purpose**: Domains can depend on other properties to determine their constraints.

**Example: Array Domain Depends on Input:**

```xml
<Proxy name="Gradient" class="vtkGradientFilter">

  <!-- Input property -->
  <InputProperty name="Input"
                 command="SetInputConnection"
                 port_index="0">
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

  <!-- Array selection depends on Input -->
  <IntVectorProperty name="ScalarArray"
                     command="SetInputArrayToProcess"
                     number_of_elements="4"
                     default_values="0 0 0 0">

    <InputArrayDomain name="input_array"
                      number_of_components="1"
                      attribute_type="point">
      <!-- Domain queries Input property -->
      <RequiredProperties>
        <Property name="Input" function="Input"/>
      </RequiredProperties>
    </InputArrayDomain>

  </IntVectorProperty>

</Proxy>
```

### Dynamic Domain Updates

**Update Flow:**

```
1. User connects Input property
   ↓
2. InputArrayDomain detects Input change
   ↓
3. Domain queries Input dataset
   ↓
4. Discovers available arrays
   ↓
5. Filters arrays based on domain constraints
   ↓
6. Updates UI dropdown with valid arrays
   ↓
7. User selects array
   ↓
8. Property value set
   ↓
9. VTK algorithm receives array selection
```

## Complete Example: Contour Filter

### VTK Algorithm

```cpp
class vtkContourFilter : public vtkAlgorithm
{
    // Input port: accepts any dataset
    int FillInputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(),
                     "vtkDataSet");
        }
        return 1;
    }

    // Output port: produces polygonal data
    int FillOutputPortInformation(int port, vtkInformation* info) override
    {
        if (port == 0)
        {
            info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        }
        return 1;
    }

    // Process data
    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override
    {
        vtkDataSet* input = vtkDataSet::GetData(inputVector[0], 0);
        vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);

        // Get selected array
        vtkDataArray* scalars = this->GetInputArrayToProcess(0, input);
        if (!scalars)
        {
            vtkErrorMacro("No scalar array selected");
            return 0;
        }

        // Validate array
        if (scalars->GetNumberOfComponents() != 1)
        {
            vtkErrorMacro("Array must be scalar");
            return 0;
        }

        // Generate contours...
        return 1;
    }

    // Properties
    vtkSetMacro(ContourValues, vtkDataArray*);
    vtkGetMacro(ContourValues, vtkDataArray*);

private:
    vtkDataArray* ContourValues = nullptr;
};
```

### ParaView Proxy XML

```xml
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <Proxy name="Contour" class="vtkContourFilter" label="Contour">

      <!-- Documentation -->
      <Documentation>
        <ShortHelp>Generate isosurfaces/isolines</ShortHelp>
        <LongHelp>
          The Contour filter generates isosurfaces or isolines
          from scalar data in the input dataset.
        </LongHelp>
      </Documentation>

      <!-- Input property -->
      <InputProperty name="Input"
                     command="SetInputConnection"
                     port_index="0"
                     optional="0">

        <!-- Proxy group constraint -->
        <ProxyGroupDomain name="groups">
          <Group name="sources"/>
          <Group name="filters"/>
        </ProxyGroupDomain>

        <!-- Data type constraint (maps to FillInputPortInformation) -->
        <DataTypeDomain name="input_type">
          <DataType value="vtkDataSet"/>
        </DataTypeDomain>

      </InputProperty>

      <!-- Scalar array selection -->
      <IntVectorProperty name="Scalars"
                        command="SetInputArrayToProcess"
                        number_of_elements="4"
                        default_values="0 0 0 0"
                        label="Scalars">

        <!-- Array domain (maps to GetInputArrayToProcess validation) -->
        <InputArrayDomain name="input_array"
                         number_of_components="1"
                         attribute_type="point">
          <RequiredProperties>
            <Property name="Input" function="Input"/>
          </RequiredProperties>
        </InputArrayDomain>

      </IntVectorProperty>

      <!-- Contour values -->
      <DoubleVectorProperty name="ContourValues"
                           command="SetValue"
                           number_of_elements="1"
                           repeat_command="1"
                           default_values="0.0"
                           label="Contour Values">

        <!-- Range constraint -->
        <DoubleRangeDomain name="range">
          <!-- No explicit min/max - uses data range -->
        </DoubleRangeDomain>

        <!-- Array domain to get data range -->
        <InputArrayDomain name="input_array"
                         number_of_components="1"
                         attribute_type="point">
          <RequiredProperties>
            <Property name="Input" function="Input"/>
            <Property name="Scalars" function="Input"/>
          </RequiredProperties>
        </InputArrayDomain>

      </DoubleVectorProperty>

      <!-- Output port -->
      <OutputPort name="Output"
                 index="0"
                 id="port0">
        <DataTypeDomain name="output_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
      </OutputPort>

    </Proxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

## Domain Validation Flow

### Validation Sequence

```
1. User sets property value in UI
   ↓
2. vtkSMProperty::SetElement() called
   ↓
3. Property validates value against all domains
   ↓
4. For each domain:
   a. Domain::IsInDomain() called
   b. Domain checks constraints
   c. Returns valid/invalid
   ↓
5. If all domains valid: Value accepted
   ↓
6. If any domain invalid: Value rejected, error shown
   ↓
7. Valid value sent to VTK algorithm
   ↓
8. VTK algorithm validates at runtime (FillInputPortInformation, RequestData)
```

### Domain Implementation

**Base Domain Class:**

```cpp
class vtkSMDomain : public vtkSMObject
{
public:
    // Check if value is in domain
    virtual bool IsInDomain(vtkSMProperty* property);

    // Update domain based on dependencies
    virtual void Update(vtkSMProperty* property);

    // Get domain information
    virtual void GetDomainInfo(vtkSMProperty* property,
                              vtkSMDomainInformation* info);
};
```

**DataTypeDomain Implementation:**

```cpp
class vtkSMDataTypeDomain : public vtkSMDomain
{
    bool IsInDomain(vtkSMProperty* property) override
    {
        // Get input proxy
        vtkSMProxy* inputProxy = GetInputProxy(property);
        if (!inputProxy)
        {
            return false;
        }

        // Get output data type
        vtkDataObject* data = inputProxy->GetOutputDataObject(0);
        if (!data)
        {
            return false;
        }

        // Check if data type matches domain
        const char* dataType = data->GetClassName();
        for (int i = 0; i < this->GetNumberOfDataTypes(); i++)
        {
            if (strcmp(dataType, this->GetDataType(i)) == 0)
            {
                return true;
            }
        }

        return false;
    }
};
```

**InputArrayDomain Implementation:**

```cpp
class vtkSMInputArrayDomain : public vtkSMDomain
{
    bool IsInDomain(vtkSMProperty* property) override
    {
        // Get input proxy
        vtkSMProxy* inputProxy = GetInputProxy(property);
        if (!inputProxy)
        {
            return false;
        }

        // Get input dataset
        vtkDataObject* data = inputProxy->GetOutputDataObject(0);
        vtkDataSet* dataset = vtkDataSet::SafeDownCast(data);
        if (!dataset)
        {
            return false;
        }

        // Get attribute data based on attribute_type
        vtkAbstractArray* array = GetArrayFromDataset(dataset);
        if (!array)
        {
            return false;
        }

        // Check component count
        if (this->GetNumberOfComponents() != -1)
        {
            if (array->GetNumberOfComponents() !=
                this->GetNumberOfComponents())
            {
                return false;
            }
        }

        return true;
    }

    void Update(vtkSMProperty* property) override
    {
        // Update list of available arrays
        // Called when input changes
        this->UpdateArrayList(property);
    }
};
```

## Mapping Summary

### Domain to VTK Mapping

| ServerManager Domain   | VTK Pipeline Requirement     | Validation Point                    |
| ---------------------- | ---------------------------- | ----------------------------------- |
| `DataTypeDomain`       | `FillInputPortInformation()` | Input port type declaration         |
| `InputArrayDomain`     | `GetInputArrayToProcess()`   | Array selection in `RequestData()`  |
| `ArraySelectionDomain` | Array status methods         | Array processing in `RequestData()` |
| `DoubleRangeDomain`    | Property setter validation   | Property setter (if implemented)    |
| `IntRangeDomain`       | Property setter validation   | Property setter (if implemented)    |
| `EnumerationDomain`    | Enum/constant values         | Property setter validation          |
| `ProxyGroupDomain`     | Input connection validation  | Proxy connection validation         |

### Validation Layers

```
Layer 1: ServerManager Domain Validation
  - Validates before value is set
  - Prevents invalid UI interactions
  - Provides immediate feedback

Layer 2: VTK Algorithm Validation
  - FillInputPortInformation(): Type checking
  - RequestData(): Runtime validation
  - Provides detailed error messages

Layer 3: Pipeline Execution
  - Actual data processing
  - Final validation during execution
```

## Summary

### Key Concepts

1. **Properties**: Expose VTK algorithm parameters
2. **Domains**: Constrain property values
3. **Type Safety**: Domains ensure VTK compatibility
4. **Dynamic Updates**: Domains update based on dependencies
5. **Validation**: Multi-layer validation (UI → VTK → Execution)

### Domain Types

- **DataTypeDomain**: Input data type constraints
- **InputArrayDomain**: Array selection constraints
- **ArraySelectionDomain**: Multiple array selection
- **Range Domains**: Numeric value constraints
- **EnumerationDomain**: Predefined value sets
- **ProxyGroupDomain**: Input connection constraints

### Mapping to VTK

- **DataTypeDomain** → `FillInputPortInformation()`
- **InputArrayDomain** → `GetInputArrayToProcess()`
- **ArraySelectionDomain** → Array status methods
- **Range/Enumeration Domains** → Property setters

This architecture ensures **type-safe pipeline construction**, **user-friendly validation**, and **seamless integration** between ParaView UI and VTK pipeline execution.
