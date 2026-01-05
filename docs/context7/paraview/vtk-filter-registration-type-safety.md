# ParaView Filter Registration: Adding New Filters with Type Safety and Pipeline Safety

## Executive Summary

ParaView allows new filters to be added through a three-layer architecture: (1) **VTK Algorithm Classes** (C++ implementation), (2) **XML Proxy Definitions** (Server Manager configuration), and (3) **Server Manager Registration** (runtime discovery and instantiation). This architecture enforces **type correctness** through port information declarations, `DataTypeDomain` constraints, and runtime validation. **Pipeline safety** is ensured through connection validation, property domain constraints, and immutable output contracts. The separation of concerns (VTK algorithm logic vs. ParaView UI/metadata) enables extensibility while maintaining type safety and pipeline integrity.

**Key Components**:
- **VTK Algorithm**: C++ class implementing `vtkAlgorithm` interface
- **XML Proxy**: Server Manager XML defining filter metadata and properties
- **Server Manager**: Runtime system for proxy creation and management
- **Type Safety**: Multi-layer validation (port info, XML domains, runtime checks)
- **Pipeline Safety**: Connection validation, property constraints, immutable outputs

**Registration Flow**:
```
1. Implement VTK Algorithm (C++)
    ↓
2. Define XML Proxy (Server Manager)
    ↓
3. Register XML (Plugin/Startup)
    ↓
4. Server Manager Creates Proxy
    ↓
5. Filter Available in ParaView UI
```

---

## 1. Layer 1: VTK Algorithm Implementation

### Base Class: vtkAlgorithm

**All filters must inherit from `vtkAlgorithm`**:

```cpp
#include <vtkAlgorithm.h>
#include <vtkDataObject.h>
#include <vtkInformation.h>
#include <vtkInformationVector.h>

class vtkMyCustomFilter : public vtkAlgorithm
{
public:
    static vtkMyCustomFilter* New();
    vtkTypeMacro(vtkMyCustomFilter, vtkAlgorithm);
    
protected:
    vtkMyCustomFilter();
    ~vtkMyCustomFilter() override;
    
private:
    // Filter parameters
    double Parameter1;
    int Parameter2;
};
```

### Required Interface Methods

**1. Port Information Methods** (Type Declaration):

```cpp
// Declare input port types
int FillInputPortInformation(int port, vtkInformation* info) override
{
    if (port == 0)
    {
        // Declare accepted input type
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
        
        // Mark as required
        info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);
        
        // Single connection only
        info->Set(vtkAlgorithm::INPUT_IS_REPEATABLE(), 0);
    }
    return 1;
}

// Declare output port types
int FillOutputPortInformation(int port, vtkInformation* info) override
{
    if (port == 0)
    {
        // Declare output type
        info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
    }
    return 1;
}
```

**2. Pipeline Execution Methods**:

```cpp
// Main computation (required)
int RequestData(vtkInformation* request,
                vtkInformationVector** inputVector,
                vtkInformationVector* outputVector) override
{
    // Get input with type validation
    vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
    if (!input)
    {
        vtkErrorMacro("Input is not vtkPolyData");
        return 0;
    }
    
    // Get output
    vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);
    
    // Execute algorithm
    // ... process input -> output ...
    
    return 1;
}

// Metadata propagation (optional but recommended)
int RequestInformation(vtkInformation* request,
                       vtkInformationVector** inputVector,
                       vtkInformationVector* outputVector) override
{
    // Propagate metadata from input to output
    // Set extents, bounds, etc.
    return 1;
}

// Extent determination (optional, for streaming)
int RequestUpdateExtent(vtkInformation* request,
                       vtkInformationVector** inputVector,
                       vtkInformationVector* outputVector) override
{
    // Determine required input extents
    return 1;
}
```

**3. Output Type Creation** (Optional, for dynamic types):

```cpp
// Create appropriate output type (optional)
int RequestDataObject(vtkInformation* request,
                     vtkInformationVector** inputVector,
                     vtkInformationVector* outputVector) override
{
    // Get declared output type
    vtkInformation* outInfo = outputVector->GetInformationObject(0);
    const char* dataType = outInfo->Get(vtkDataObject::DATA_TYPE_NAME());
    
    // Check if output already exists and is correct type
    vtkDataObject* output = outInfo->Get(vtkDataObject::DATA_OBJECT());
    if (output && output->IsA(dataType))
    {
        return 1;  // Output type is correct
    }
    
    // Create new output of correct type
    vtkDataObject* newOutput = vtkDataObjectTypes::NewDataObject(dataType);
    outInfo->Set(vtkDataObject::DATA_OBJECT(), newOutput);
    newOutput->Delete();  // Pipeline owns it
    
    return 1;
}
```

### Type Safety in VTK Layer

**1. Port Information Declarations**:
- `FillInputPortInformation()` declares accepted types
- `FillOutputPortInformation()` declares output types
- Used by pipeline system for connection validation

**2. Runtime Type Checks**:
- `GetData()` validates types automatically
- `SafeDownCast()` for manual type checking
- `IsA()` for type hierarchy checks

**3. Immutable Output Contract**:
- Outputs are immutable once cached
- Filters must create new outputs, not modify existing
- Ensures pipeline safety

### Complete VTK Filter Example

```cpp
// vtkMyCustomFilter.h
class vtkMyCustomFilter : public vtkAlgorithm
{
public:
    static vtkMyCustomFilter* New();
    vtkTypeMacro(vtkMyCustomFilter, vtkAlgorithm);
    
    // Parameter setters/getters
    vtkSetMacro(Parameter1, double);
    vtkGetMacro(Parameter1, double);
    
    vtkSetMacro(Parameter2, int);
    vtkGetMacro(Parameter2, int);
    
protected:
    vtkMyCustomFilter();
    ~vtkMyCustomFilter() override;
    
    // Port information
    int FillInputPortInformation(int port, vtkInformation* info) override;
    int FillOutputPortInformation(int port, vtkInformation* info) override;
    
    // Pipeline execution
    int RequestData(vtkInformation* request,
                   vtkInformationVector** inputVector,
                   vtkInformationVector* outputVector) override;
    
private:
    double Parameter1;
    int Parameter2;
};

// vtkMyCustomFilter.cxx
vtkStandardNewMacro(vtkMyCustomFilter);

vtkMyCustomFilter::vtkMyCustomFilter()
{
    this->Parameter1 = 1.0;
    this->Parameter2 = 10;
    this->SetNumberOfInputPorts(1);
    this->SetNumberOfOutputPorts(1);
}

int vtkMyCustomFilter::FillInputPortInformation(int port, vtkInformation* info)
{
    if (port == 0)
    {
        info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
        return 1;
    }
    return 0;
}

int vtkMyCustomFilter::FillOutputPortInformation(int port, vtkInformation* info)
{
    if (port == 0)
    {
        info->Set(vtkDataObject::DATA_TYPE_NAME(), "vtkPolyData");
        return 1;
    }
    return 0;
}

int vtkMyCustomFilter::RequestData(
    vtkInformation* request,
    vtkInformationVector** inputVector,
    vtkInformationVector* outputVector)
{
    // Get input with type validation
    vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
    if (!input)
    {
        vtkErrorMacro("Input is not vtkPolyData");
        return 0;
    }
    
    // Get output
    vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);
    
    // Process data using parameters
    // ... algorithm implementation ...
    
    return 1;
}
```

---

## 2. Layer 2: XML Proxy Definition

### Server Manager XML Structure

**XML defines filter metadata and property mappings**:

```xml
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <SourceProxy name="MyCustomFilter"
                 class="vtkMyCustomFilter"
                 label="My Custom Filter">
      
      <!-- Documentation -->
      <Documentation
         long_help="This filter performs custom processing on polygonal data."
         short_help="Custom filter for polygonal data">
      </Documentation>
      
      <!-- Input port definition -->
      <InputProperty name="Input"
                     command="SetInputConnection"
                     port_index="0"
                     optional="0">
        <!-- Which proxy groups can connect -->
        <ProxyGroupDomain name="groups">
          <Group name="sources"/>
          <Group name="filters"/>
        </ProxyGroupDomain>
        
        <!-- Type constraint: only vtkPolyData -->
        <DataTypeDomain name="input_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
        
        <Documentation>
          Input polygonal data to process.
        </Documentation>
      </InputProperty>
      
      <!-- Filter parameters -->
      <DoubleVectorProperty name="Parameter1"
                           command="SetParameter1"
                           number_of_elements="1"
                           default_values="1.0"
                           label="Parameter 1">
        <!-- Value domain: valid range -->
        <DoubleRangeDomain name="range" min="0.0" max="100.0"/>
        <Documentation>
          First parameter controlling filter behavior.
        </Documentation>
      </DoubleVectorProperty>
      
      <IntVectorProperty name="Parameter2"
                        command="SetParameter2"
                        number_of_elements="1"
                        default_values="10"
                        label="Parameter 2">
        <IntRangeDomain name="range" min="1" max="100"/>
        <Documentation>
          Second parameter controlling filter behavior.
        </Documentation>
      </IntVectorProperty>
      
      <!-- Output port definition -->
      <OutputProperty name="Output"
                     port_index="0"
                     command="GetOutputPort">
        <DataTypeDomain name="output_type">
          <DataType value="vtkPolyData"/>
        </DataTypeDomain>
      </OutputProperty>
      
      <!-- UI hints -->
      <Hints>
        <ShowInMenu category="Custom Filters"/>
      </Hints>
      
    </SourceProxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

### XML Elements Explained

**1. SourceProxy**:
- `name`: Unique identifier for proxy (used in `NewProxy()`)
- `class`: VTK class name (must match C++ class)
- `label`: Display name in UI

**2. InputProperty**:
- `name`: Property name (typically "Input")
- `command`: VTK method to call (`SetInputConnection`)
- `port_index`: Input port number
- `optional`: Whether input is required (0) or optional (1)

**3. DataTypeDomain**:
- **Type Safety Constraint**: Restricts accepted input types
- Prevents incompatible connections in UI
- Must match `FillInputPortInformation()` declaration

**4. Property Definitions**:
- `DoubleVectorProperty`: Double parameters
- `IntVectorProperty`: Integer parameters
- `StringVectorProperty`: String parameters
- `command`: VTK setter method name
- `default_values`: Default parameter values

**5. Domain Constraints**:
- `DoubleRangeDomain`: Valid range for double parameters
- `IntRangeDomain`: Valid range for integer parameters
- `EnumerationDomain`: Valid enumeration values
- **Pipeline Safety**: Prevents invalid parameter values

**6. OutputProperty**:
- `port_index`: Output port number
- `command`: VTK getter method (`GetOutputPort`)
- `DataTypeDomain`: Declares output type

### Type Safety in XML Layer

**1. DataTypeDomain Constraints**:
```xml
<!-- Input type constraint -->
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
</DataTypeDomain>

<!-- Multiple types allowed -->
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
  <DataType value="vtkUnstructuredGrid"/>
</DataTypeDomain>

<!-- Base type (accepts all datasets) -->
<DataTypeDomain name="input_type">
  <DataType value="vtkDataSet"/>
</DataTypeDomain>
```

**2. Property Domain Constraints**:
```xml
<!-- Range constraint -->
<DoubleRangeDomain name="range" min="0.0" max="100.0"/>

<!-- Enumeration constraint -->
<EnumerationDomain name="enum">
  <Entry value="0" text="Option 1"/>
  <Entry value="1" text="Option 2"/>
</EnumerationDomain>
```

**3. ProxyGroupDomain**:
```xml
<!-- Only sources and filters can connect -->
<ProxyGroupDomain name="groups">
  <Group name="sources"/>
  <Group name="filters"/>
</ProxyGroupDomain>
```

---

## 3. Layer 3: Server Manager Registration

### XML Registration

**Loading XML Configuration**:

```cpp
// In plugin initialization or startup
#include <vtkSMProxyManager.h>

void RegisterMyFilter()
{
    vtkSMProxyManager* proxyManager = 
        vtkSMProxyManager::GetProxyManager();
    
    // Load XML from file
    proxyManager->LoadConfigurationXMLFromFile("MyCustomFilter.xml");
    
    // Or load from string
    const char* xmlConfig = R"(
        <ServerManagerConfiguration>
          <ProxyGroup name="filters">
            <SourceProxy name="MyCustomFilter" class="vtkMyCustomFilter">
              <!-- ... -->
            </SourceProxy>
          </ProxyGroup>
        </ServerManagerConfiguration>
    )";
    proxyManager->LoadConfigurationXML(xmlConfig);
}
```

### Proxy Creation

**Server Manager Creates Proxy from XML**:

```cpp
// In vtkSMProxyManager
vtkSMProxy* vtkSMProxyManager::NewProxy(
    const char* group,      // "filters"
    const char* name)       // "MyCustomFilter"
{
    // 1. Find XML definition
    vtkPVXMLElement* xmlDefinition = this->FindProxyDefinition(group, name);
    if (!xmlDefinition)
    {
        vtkErrorMacro("Proxy definition not found: " << group << "/" << name);
        return nullptr;
    }
    
    // 2. Parse XML
    std::string className = xmlDefinition->GetAttribute("class");
    
    // 3. Create VTK algorithm instance
    vtkAlgorithm* algorithm = vtkAlgorithm::SafeDownCast(
        vtkObjectFactory::CreateInstance(className.c_str()));
    
    if (!algorithm)
    {
        vtkErrorMacro("Cannot create VTK class: " << className);
        return nullptr;
    }
    
    // 4. Create proxy wrapper
    vtkSMProxy* proxy = vtkSMProxy::New();
    proxy->SetVTKClassName(className);
    proxy->SetXMLName(name);
    proxy->SetXMLGroup(group);
    
    // 5. Configure proxy from XML
    proxy->ReadXMLAttributes(xmlDefinition);
    
    // 6. Set client-side object (VTK algorithm)
    proxy->SetClientSideObject(algorithm);
    algorithm->Delete();  // Proxy owns it
    
    // 7. Initialize properties from XML
    proxy->SetupProperties();
    
    return proxy;
}
```

### Property Mapping

**XML Properties Map to VTK Methods**:

```cpp
// In vtkSMProxy::SetupProperties()
void vtkSMProxy::SetupProperties()
{
    // Parse property definitions from XML
    vtkPVXMLElement* xmlDef = this->GetXMLDefinition();
    
    int numProps = xmlDef->GetNumberOfNestedElements();
    for (int i = 0; i < numProps; i++)
    {
        vtkPVXMLElement* propElem = xmlDef->GetNestedElement(i);
        
        if (strcmp(propElem->GetName(), "DoubleVectorProperty") == 0)
        {
            // Create property
            vtkSMDoubleVectorProperty* prop = vtkSMDoubleVectorProperty::New();
            
            // Get property name
            const char* propName = propElem->GetAttribute("name");
            prop->SetXMLName(propName);
            
            // Get command (VTK method name)
            const char* command = propElem->GetAttribute("command");
            prop->SetCommand(command);
            
            // Get default values
            const char* defaults = propElem->GetAttribute("default_values");
            // Parse and set defaults...
            
            // Add property to proxy
            this->AddProperty(propName, prop);
            prop->Delete();
        }
        // ... handle other property types ...
    }
}
```

### Property Update Flow

**When Property Changes, Update VTK Algorithm**:

```cpp
// In vtkSMProxy
void vtkSMProxy::UpdateVTKObjects()
{
    vtkAlgorithm* algorithm = vtkAlgorithm::SafeDownCast(
        this->GetClientSideObject());
    
    if (!algorithm)
    {
        return;
    }
    
    // Update all properties
    vtkSMPropertyIterator* propIter = this->NewPropertyIterator();
    for (propIter->Begin(); !propIter->IsAtEnd(); propIter->Next())
    {
        vtkSMProperty* prop = propIter->GetProperty();
        
        // Skip input properties (handled separately)
        if (vtkSMInputProperty::SafeDownCast(prop))
        {
            continue;
        }
        
        // Get command (VTK method name)
        const char* command = prop->GetCommand();
        if (!command)
        {
            continue;
        }
        
        // Get property value
        vtkVariant value = prop->GetVariantValue();
        
        // Call VTK method via reflection
        vtkClientServerStream stream;
        stream << vtkClientServerStream::Invoke
               << algorithm
               << command
               << value
               << vtkClientServerStream::End;
        
        // Execute stream (calls algorithm->SetParameter1(value))
        algorithm->ProcessStream(stream);
    }
    
    propIter->Delete();
}
```

---

## 4. Architectural Constraints for Type Correctness

### Constraint Layer 1: VTK Port Information

**Compile-Time Type Declaration**:

```cpp
// Filter declares accepted types
int FillInputPortInformation(int port, vtkInformation* info) override
{
    info->Set(vtkAlgorithm::INPUT_REQUIRED_DATA_TYPE(), "vtkPolyData");
    return 1;
}
```

**Enforcement**:
- Pipeline system checks port information before connections
- `SetInputConnection()` validates types
- Runtime checks in `RequestData()`

### Constraint Layer 2: XML DataTypeDomain

**UI-Level Type Prevention**:

```xml
<DataTypeDomain name="input_type">
  <DataType value="vtkPolyData"/>
</DataTypeDomain>
```

**Enforcement**:
- Server Manager validates connections before setting
- UI prevents incompatible connections
- `CanAcceptInput()` checks DataTypeDomain

### Constraint Layer 3: Server Manager Validation

**Connection Validation**:

```cpp
// In vtkSMProxy
bool vtkSMProxy::CanAcceptInput(
    vtkSMProxy* inputProxy,
    int inputPort,
    int outputPort)
{
    // Get input property
    vtkSMInputProperty* inputProp = 
        vtkSMInputProperty::SafeDownCast(this->GetProperty("Input"));
    
    if (!inputProp)
    {
        return false;
    }
    
    // Get output proxy's output type
    vtkSMOutputPort* outputPortObj = inputProxy->GetOutputPort(outputPort);
    const char* outputType = outputPortObj->GetDataClassName();
    
    // Check DataTypeDomain
    vtkSMDomainIterator* domainIter = inputProp->NewDomainIterator();
    domainIter->Begin();
    
    while (!domainIter->IsAtEnd())
    {
        vtkSMDomain* domain = domainIter->GetDomain();
        vtkSMDataTypeDomain* dataTypeDomain = 
            vtkSMDataTypeDomain::SafeDownCast(domain);
        
        if (dataTypeDomain && !dataTypeDomain->IsInDomain(outputType))
        {
            domainIter->Delete();
            return false;  // Type mismatch
        }
        
        domainIter->Next();
    }
    
    domainIter->Delete();
    return true;  // Type is compatible
}
```

### Constraint Layer 4: Runtime Type Checks

**Execution-Time Validation**:

```cpp
// In RequestData()
int RequestData(...) override
{
    // Type-safe retrieval with validation
    vtkPolyData* input = vtkPolyData::GetData(inputVector[0], 0);
    if (!input)
    {
        vtkErrorMacro("Input is not vtkPolyData");
        return 0;  // Fail if type mismatch
    }
    
    // Process...
    return 1;
}
```

### Type Safety Summary

**Multi-Layer Validation**:

1. **VTK Port Information**: Declares types at algorithm level
2. **XML DataTypeDomain**: Declares types at UI/metadata level
3. **Server Manager Validation**: Validates before connections
4. **Runtime Checks**: Validates during execution

**Type Mismatch Prevention**:
- UI prevents incompatible connections
- Server Manager rejects invalid connections
- Runtime fails gracefully with error messages

---

## 5. Architectural Constraints for Pipeline Safety

### Constraint 1: Immutable Output Contract

**Outputs Cannot Be Modified After Creation**:

```cpp
// CORRECT: Create new output
int RequestData(...) override
{
    vtkPolyData* output = vtkPolyData::GetData(outputVector, 0);
    
    // Create new data
    vtkNew<vtkPoints> newPoints;
    // ... populate newPoints ...
    output->SetPoints(newPoints);
    
    return 1;  // Output is now immutable
}

// WRONG: Modifying cached output
void SomeFunction(vtkPolyData* output)
{
    output->GetPoints()->SetPoint(0, newPoint);  // BREAKS IMMUTABILITY
}
```

**Enforcement**:
- Convention and documentation
- Pipeline architecture (outputs are cached)
- Reference counting ensures safe sharing

### Constraint 2: Property Domain Constraints

**Parameter Values Must Be Valid**:

```xml
<!-- Range constraint -->
<DoubleRangeDomain name="range" min="0.0" max="100.0"/>

<!-- Enumeration constraint -->
<EnumerationDomain name="enum">
  <Entry value="0" text="Option 1"/>
  <Entry value="1" text="Option 2"/>
</EnumerationDomain>
```

**Enforcement**:
- UI widgets enforce domains (sliders, dropdowns)
- Server Manager validates before setting
- Invalid values rejected with error messages

### Constraint 3: Connection Validation

**Only Valid Connections Allowed**:

```cpp
// Connection validation
bool CanConnect(vtkSMProxy* source, vtkSMProxy* destination)
{
    // Check type compatibility
    if (!destination->CanAcceptInput(source, 0, 0))
    {
        return false;  // Type mismatch
    }
    
    // Check for cycles (pipeline must be DAG)
    if (WouldCreateCycle(source, destination))
    {
        return false;  // Cycle detected
    }
    
    return true;  // Connection is valid
}
```

**Enforcement**:
- UI prevents invalid connections
- Server Manager validates before setting
- Pipeline execution checks for cycles

### Constraint 4: Required Inputs

**Required Inputs Must Be Connected**:

```cpp
// In FillInputPortInformation()
info->Set(vtkAlgorithm::INPUT_IS_OPTIONAL(), 0);  // Required

// In XML
<InputProperty name="Input" optional="0">
```

**Enforcement**:
- Pipeline execution fails if required input missing
- UI shows warnings for unconnected required inputs
- Server Manager validates before execution

### Constraint 5: Property Command Validation

**Property Commands Must Match VTK Methods**:

```xml
<!-- Command must be valid VTK method -->
<DoubleVectorProperty name="Parameter1"
                     command="SetParameter1">
```

**Enforcement**:
- Server Manager validates method exists via reflection
- Runtime error if method not found
- Compile-time check if VTK class is available

---

## 6. Complete Registration Example

### Step-by-Step: Adding a New Filter

**Step 1: Implement VTK Algorithm**:

```cpp
// vtkMyFilter.h
class vtkMyFilter : public vtkAlgorithm
{
    // ... (as shown in Section 1)
};

// vtkMyFilter.cxx
// ... (implementation as shown in Section 1)
```

**Step 2: Create XML Proxy Definition**:

```xml
<!-- MyFilter.xml -->
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <SourceProxy name="MyFilter" class="vtkMyFilter">
      <!-- ... (as shown in Section 2) -->
    </SourceProxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

**Step 3: Register XML**:

```cpp
// In plugin or startup code
void InitializePlugin()
{
    vtkSMProxyManager* pm = vtkSMProxyManager::GetProxyManager();
    pm->LoadConfigurationXMLFromFile("MyFilter.xml");
}
```

**Step 4: Use in ParaView**:

```cpp
// Filter is now available
vtkSMProxy* filter = proxyManager->NewProxy("filters", "MyFilter");

// Set input
filter->SetInputConnection(0, sourceProxy, 0);

// Set parameters
filter->SetProperty("Parameter1", 2.0);
filter->SetProperty("Parameter2", 20);

// Update pipeline
filter->UpdatePipeline();
```

---

## 7. Summary: Filter Registration Architecture

### Three-Layer Architecture

1. **VTK Algorithm Layer** (C++):
   - Implements `vtkAlgorithm` interface
   - Declares port types
   - Implements processing logic
   - Enforces type safety at algorithm level

2. **XML Proxy Layer** (Metadata):
   - Defines filter metadata
   - Maps properties to VTK methods
   - Declares type constraints
   - Defines UI behavior

3. **Server Manager Layer** (Runtime):
   - Creates proxies from XML
   - Manages proxy lifecycle
   - Validates connections
   - Coordinates pipeline execution

### Type Correctness Guarantees

- **Port Information**: VTK algorithms declare types
- **DataTypeDomain**: XML constrains accepted types
- **Server Manager Validation**: Connections validated before setting
- **Runtime Checks**: Types validated during execution

### Pipeline Safety Guarantees

- **Immutable Outputs**: Outputs cannot be modified after creation
- **Property Domains**: Parameter values constrained to valid ranges
- **Connection Validation**: Only valid connections allowed
- **Required Inputs**: Required inputs must be connected
- **Command Validation**: Property commands must match VTK methods

### Benefits

- **Extensibility**: Easy to add new filters
- **Type Safety**: Multi-layer type validation
- **Pipeline Safety**: Constraints prevent invalid pipelines
- **Separation of Concerns**: Algorithm logic separate from UI/metadata
- **Reproducibility**: Complete pipeline state captured

This architecture enables **safe, extensible filter registration** while maintaining **type correctness** and **pipeline safety** through multiple validation layers.

