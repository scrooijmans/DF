# ParaView UI Filter Validation & Prevention

This document explains in detail how ParaView prevents invalid filter application in the UI (e.g., disabling filters based on active source type), and where the checks live (client-side vs server-side).

## Overview

ParaView uses a **dual-layer validation system** to prevent invalid filter application:

1. **Client-Side**: UI-level checks that disable incompatible filters
2. **Server-Side**: Domain validation that enforces constraints
3. **Integration**: Client queries server domains to determine filter availability

**Key Principles:**

- **Proactive Prevention**: Filters disabled before user can select them
- **Domain-Driven**: UI state driven by ServerManager domains
- **Real-Time Updates**: Filter availability updates when selection changes
- **User Feedback**: Clear visual indicators and tooltips explain why filters are disabled
- **Fail-Safe**: Server-side validation as backup if client checks are bypassed

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│         ParaView Filter Validation System                   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Client-Side (UI Layer)                              │   │
│  │  - pqProxySelectionModel                            │   │
│  │  - Filter menu enabling/disabling                    │   │
│  │  - Domain querying                                  │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Server-Side (ServerManager)                         │   │
│  │  - vtkSMProxy (proxy definitions)                    │   │
│  │  - vtkSMDomain (domain validation)                   │   │
│  │  - DataTypeDomain, InputArrayDomain, etc.            │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  VTK Pipeline                                        │   │
│  │  - FillInputPortInformation()                       │   │
│  │  - RequestData() validation                         │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Client-Side Mechanisms

### Filter Menu Management

**Dynamic Filter Enabling/Disabling:**

The ParaView filter menu dynamically enables or disables filter options based on the active source's compatibility.

**Implementation Classes:**

```cpp
// pqProxySelectionModel: Manages active selection
class pqProxySelectionModel
{
public:
    // Get currently selected proxy
    vtkSMProxy* getCurrentProxy();

    // Check if filter can be applied to current selection
    bool canApplyFilter(vtkSMProxy* filterProxy);
};
```

**Filter Menu Update Flow:**

```
1. User selects source in pipeline browser
   ↓
2. pqProxySelectionModel updates current selection
   ↓
3. Filter menu queries each filter's domains
   ↓
4. For each filter:
   a. Check DataTypeDomain against active source
   b. Check InputArrayDomain if applicable
   c. Check ProxyGroupDomain
   ↓
5. Enable/disable filter menu item
   ↓
6. Update tooltip with reason if disabled
```

### pqProxy and Domain Queries

**Client-Side Proxy:**

```cpp
// pqProxy: Client-side representation of server proxy
class pqProxy
{
public:
    // Get server-side proxy
    vtkSMProxy* getProxy();

    // Check if property is enabled (domain validation)
    bool isPropertyEnabled(vtkSMProperty* property);

    // Get domain information
    vtkSMDomain* getDomain(vtkSMProperty* property, const char* domainName);
};
```

**Domain Querying:**

```cpp
// Check if filter can be applied to active source
bool CanApplyFilter(vtkSMProxy* filterProxy, vtkSMProxy* activeSource)
{
    // Get input property
    vtkSMProperty* inputProp = filterProxy->GetProperty("Input");
    if (!inputProp)
    {
        return false;
    }

    // Check DataTypeDomain
    vtkSMDataTypeDomain* dataTypeDomain =
        vtkSMDataTypeDomain::SafeDownCast(
            inputProp->FindDomain("vtkSMDataTypeDomain")
        );

    if (dataTypeDomain)
    {
        // Get active source output type
        vtkDataObject* outputData = activeSource->GetOutputDataObject(0);
        const char* outputType = outputData->GetClassName();

        // Check if output type is in domain
        if (!dataTypeDomain->IsInDomain(outputType))
        {
            return false;  // Filter disabled
        }
    }

    // Check ProxyGroupDomain
    vtkSMProxyGroupDomain* groupDomain =
        vtkSMProxyGroupDomain::SafeDownCast(
            inputProp->FindDomain("vtkSMProxyGroupDomain")
        );

    if (groupDomain)
    {
        // Check if active source is in allowed group
        const char* sourceGroup = activeSource->GetXMLGroup();
        if (!groupDomain->IsInDomain(sourceGroup))
        {
            return false;  // Filter disabled
        }
    }

    return true;  // Filter enabled
}
```

### UI Widget Updates

**Property Widget Enabling/Disabling:**

```cpp
// pqPropertyWidget: Base class for property widgets
class pqPropertyWidget
{
public:
    // Update widget state based on domain
    virtual void updateWidget(bool showing_advanced_properties);

    // Check if property is enabled
    bool isPropertyEnabled();

protected:
    // Called when domain changes
    virtual void domainChanged();
};
```

**Domain Change Handling:**

```cpp
void pqPropertyWidget::domainChanged()
{
    vtkSMProperty* property = this->getProperty();

    // Check all domains
    bool enabled = true;

    for (int i = 0; i < property->GetNumberOfDomains(); i++)
    {
        vtkSMDomain* domain = property->GetDomain(i);

        // Query domain if property value is valid
        if (!domain->IsInDomain(property))
        {
            enabled = false;
            break;
        }
    }

    // Update widget state
    this->setEnabled(enabled);

    // Update tooltip with reason if disabled
    if (!enabled)
    {
        QString reason = this->getDomainFailureReason();
        this->setToolTip(reason);
    }
}
```

### Filter Menu Implementation

**Menu Item State Management:**

```cpp
// pqFilterMenuReaction: Handles filter menu actions
class pqFilterMenuReaction : public pqReaction
{
public:
    // Update menu item state
    void updateEnableState() override
    {
        // Get active source
        pqProxySelectionModel* selModel =
            pqApplicationCore::instance()->getSelectionModel();
        vtkSMProxy* activeSource = selModel->getCurrentProxy();

        if (!activeSource)
        {
            this->parentAction()->setEnabled(false);
            return;
        }

        // Get filter proxy
        vtkSMProxy* filterProxy = this->getFilterProxy();

        // Check if filter can be applied
        bool canApply = this->canApplyFilter(filterProxy, activeSource);

        // Update menu item
        this->parentAction()->setEnabled(canApply);

        // Update tooltip
        if (!canApply)
        {
            QString reason = this->getDisableReason(filterProxy, activeSource);
            this->parentAction()->setToolTip(reason);
        }
    }

private:
    bool canApplyFilter(vtkSMProxy* filterProxy, vtkSMProxy* activeSource)
    {
        // Get input property
        vtkSMProperty* inputProp = filterProxy->GetProperty("Input");
        if (!inputProp)
        {
            return false;
        }

        // Query domains
        return this->checkDomains(inputProp, activeSource);
    }
};
```

## Server-Side Mechanisms

### Domain Validation

**Domain Checking:**

```cpp
// vtkSMDomain: Base class for domains
class vtkSMDomain
{
public:
    // Check if value is in domain
    virtual bool IsInDomain(vtkSMProperty* property);

    // Update domain based on dependencies
    virtual void Update(vtkSMProperty* property);
};
```

**DataTypeDomain Validation:**

```cpp
// vtkSMDataTypeDomain: Validates data types
class vtkSMDataTypeDomain : public vtkSMDomain
{
    bool IsInDomain(vtkSMProperty* property) override
    {
        // Get input proxy from property
        vtkSMProxy* inputProxy = this->GetInputProxy(property);
        if (!inputProxy)
        {
            return false;
        }

        // Get output data type
        vtkDataObject* outputData = inputProxy->GetOutputDataObject(0);
        if (!outputData)
        {
            return false;
        }

        const char* dataType = outputData->GetClassName();

        // Check if data type is in domain
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

### Proxy Property Validation

**Property State Checking:**

```cpp
// vtkSMProperty: Property with domains
class vtkSMProperty
{
public:
    // Check if property is enabled (all domains valid)
    bool IsEnabled();

    // Get domain
    vtkSMDomain* FindDomain(const char* domainName);

    // Get all domains
    vtkSMDomain* GetDomain(int index);
    int GetNumberOfDomains();
};
```

**Property Enable State:**

```cpp
bool vtkSMProperty::IsEnabled()
{
    // Check all domains
    for (int i = 0; i < this->GetNumberOfDomains(); i++)
    {
        vtkSMDomain* domain = this->GetDomain(i);

        // Skip non-constraining domains
        if (!domain->IsInDomain(this))
        {
            return false;  // Property disabled
        }
    }

    return true;  // Property enabled
}
```

## Integration: Client-Server Communication

### Domain Query Flow

**Client Queries Server Domains:**

```
1. User selects source in UI
   ↓
2. Client (pqProxySelectionModel) detects selection change
   ↓
3. Client queries server for filter availability
   ↓
4. For each filter:
   a. Client gets filter proxy (vtkSMProxy)
   b. Client gets input property
   c. Client queries domains on server
   d. Server checks domains against active source
   e. Server returns domain validity
   ↓
5. Client updates UI (enables/disables filter menu item)
   ↓
6. User sees updated filter menu
```

### Real-Time Updates

**Selection Change Handling:**

```cpp
// pqProxySelectionModel: Manages selection
class pqProxySelectionModel : public QObject
{
    Q_OBJECT

public slots:
    void onSelectionChanged()
    {
        // Get current selection
        vtkSMProxy* currentProxy = this->getCurrentProxy();

        // Update all filter menu items
        this->updateFilterMenus(currentProxy);
    }

private:
    void updateFilterMenus(vtkSMProxy* activeSource)
    {
        // Get all filter proxies
        vtkSMProxyManager* proxyManager =
            vtkSMProxyManager::GetProxyManager();

        vtkSMProxyIterator* iter = vtkSMProxyIterator::New();
        iter->SetModeToOneGroup();
        iter->Begin("filters");

        while (!iter->IsAtEnd())
        {
            vtkSMProxy* filterProxy = iter->GetProxy();

            // Check if filter can be applied
            bool canApply = this->canApplyFilter(filterProxy, activeSource);

            // Update menu item (via signal/slot)
            emit filterAvailabilityChanged(filterProxy, canApply);

            iter->Next();
        }

        iter->Delete();
    }
};
```

## Complete Example: Filter Disabling

### Scenario: Image Threshold Filter

**Filter Definition (XML):**

```xml
<Proxy name="Threshold" class="vtkImageThreshold">

  <InputProperty name="Input"
                 command="SetInputConnection"
                 port_index="0">

    <!-- Data type constraint -->
    <DataTypeDomain name="input_type">
      <DataType value="vtkImageData"/>
    </DataTypeDomain>

    <!-- Proxy group constraint -->
    <ProxyGroupDomain name="groups">
      <Group name="sources"/>
      <Group name="filters"/>
    </ProxyGroupDomain>

  </InputProperty>

</Proxy>
```

### Client-Side Check

**Filter Menu Update:**

```cpp
// When user selects vtkPolyData source
void UpdateThresholdFilterMenu(vtkSMProxy* activeSource)
{
    // Get Threshold filter proxy
    vtkSMProxy* thresholdProxy =
        vtkSMProxyManager::GetProxyManager()->NewProxy("filters", "Threshold");

    // Get input property
    vtkSMProperty* inputProp = thresholdProxy->GetProperty("Input");

    // Check DataTypeDomain
    vtkSMDataTypeDomain* dataTypeDomain =
        vtkSMDataTypeDomain::SafeDownCast(
            inputProp->FindDomain("vtkSMDataTypeDomain")
        );

    if (dataTypeDomain)
    {
        // Get active source output type
        vtkDataObject* outputData = activeSource->GetOutputDataObject(0);
        const char* outputType = outputData->GetClassName();

        // Check compatibility
        bool compatible = dataTypeDomain->IsInDomain(outputType);

        // Update menu item
        if (!compatible)
        {
            // Disable filter menu item
            thresholdMenuAction->setEnabled(false);
            thresholdMenuAction->setToolTip(
                "Threshold filter requires vtkImageData input. "
                "Current source produces " + QString(outputType)
            );
        }
        else
        {
            // Enable filter menu item
            thresholdMenuAction->setEnabled(true);
            thresholdMenuAction->setToolTip("");
        }
    }
}
```

### Server-Side Validation

**Domain Validation:**

```cpp
// Server-side domain check
bool vtkSMDataTypeDomain::IsInDomain(vtkSMProperty* property)
{
    // Get input proxy
    vtkSMProxy* inputProxy = this->GetInputProxy(property);
    if (!inputProxy)
    {
        return false;
    }

    // Get output data
    vtkDataObject* outputData = inputProxy->GetOutputDataObject(0);
    if (!outputData)
    {
        return false;
    }

    const char* dataType = outputData->GetClassName();

    // Check against domain types
    // Domain expects: "vtkImageData"
    // Actual type: "vtkPolyData"
    // Result: false (incompatible)

    for (int i = 0; i < this->GetNumberOfDataTypes(); i++)
    {
        if (strcmp(dataType, this->GetDataType(i)) == 0)
        {
            return true;
        }
    }

    return false;  // Type mismatch
}
```

## InputArrayDomain Validation

### Array Requirement Checking

**Filter with Array Requirement:**

```xml
<Proxy name="Gradient" class="vtkGradientFilter">

  <InputProperty name="Input"
                 command="SetInputConnection"
                 port_index="0">
    <DataTypeDomain name="input_type">
      <DataType value="vtkDataSet"/>
    </DataTypeDomain>
  </InputProperty>

  <IntVectorProperty name="Scalars"
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

</Proxy>
```

### Client-Side Array Check

**Array Availability Check:**

```cpp
bool CanApplyGradientFilter(vtkSMProxy* filterProxy, vtkSMProxy* activeSource)
{
    // First check data type
    if (!CanApplyFilter(filterProxy, activeSource))
    {
        return false;
    }

    // Get scalars property
    vtkSMProperty* scalarsProp = filterProxy->GetProperty("Scalars");

    // Check InputArrayDomain
    vtkSMInputArrayDomain* arrayDomain =
        vtkSMInputArrayDomain::SafeDownCast(
            scalarsProp->FindDomain("vtkSMInputArrayDomain")
        );

    if (arrayDomain)
    {
        // Get input dataset
        vtkDataObject* outputData = activeSource->GetOutputDataObject(0);
        vtkDataSet* dataset = vtkDataSet::SafeDownCast(outputData);

        if (!dataset)
        {
            return false;
        }

        // Check if compatible arrays exist
        vtkAbstractArray* array = this->FindCompatibleArray(
            dataset,
            arrayDomain->GetAttributeType(),
            arrayDomain->GetNumberOfComponents()
        );

        if (!array)
        {
            return false;  // No compatible arrays
        }
    }

    return true;
}
```

## Status Bar Messages

### Tooltip Generation

**Disable Reason Messages:**

```cpp
QString GetDisableReason(vtkSMProxy* filterProxy, vtkSMProxy* activeSource)
{
    QStringList reasons;

    // Check DataTypeDomain
    vtkSMProperty* inputProp = filterProxy->GetProperty("Input");
    vtkSMDataTypeDomain* dataTypeDomain =
        vtkSMDataTypeDomain::SafeDownCast(
            inputProp->FindDomain("vtkSMDataTypeDomain")
        );

    if (dataTypeDomain)
    {
        vtkDataObject* outputData = activeSource->GetOutputDataObject(0);
        const char* outputType = outputData->GetClassName();

        if (!dataTypeDomain->IsInDomain(outputType))
        {
            QString expectedTypes;
            for (int i = 0; i < dataTypeDomain->GetNumberOfDataTypes(); i++)
            {
                if (i > 0) expectedTypes += ", ";
                expectedTypes += dataTypeDomain->GetDataType(i);
            }

            reasons << QString("Filter requires %1 input, but source produces %2")
                       .arg(expectedTypes)
                       .arg(outputType);
        }
    }

    // Check InputArrayDomain
    vtkSMProperty* arrayProp = filterProxy->GetProperty("Scalars");
    vtkSMInputArrayDomain* arrayDomain =
        vtkSMInputArrayDomain::SafeDownCast(
            arrayProp->FindDomain("vtkSMInputArrayDomain")
        );

    if (arrayDomain)
    {
        vtkDataObject* outputData = activeSource->GetOutputDataObject(0);
        vtkDataSet* dataset = vtkDataSet::SafeDownCast(outputData);

        if (dataset)
        {
            vtkAbstractArray* array = this->FindCompatibleArray(
                dataset,
                arrayDomain->GetAttributeType(),
                arrayDomain->GetNumberOfComponents()
            );

            if (!array)
            {
                reasons << QString("Filter requires %1-component %2 data array")
                           .arg(arrayDomain->GetNumberOfComponents())
                           .arg(this->AttributeTypeToString(
                               arrayDomain->GetAttributeType()));
            }
        }
    }

    return reasons.join("; ");
}
```

## Pipeline Browser Integration

### Selection-Based Filtering

**Pipeline Browser Selection:**

```cpp
// pqPipelineBrowser: Manages pipeline display
class pqPipelineBrowser : public QTreeWidget
{
    void selectionChanged()
    {
        // Get selected item
        QTreeWidgetItem* selectedItem = this->currentItem();

        if (selectedItem)
        {
            // Get proxy from item
            vtkSMProxy* selectedProxy =
                this->getProxyFromItem(selectedItem);

            // Update filter menu availability
            pqProxySelectionModel* selModel =
                pqApplicationCore::instance()->getSelectionModel();
            selModel->setCurrentProxy(selectedProxy);

            // This triggers filter menu update
        }
    }
};
```

## Summary

### Client-Side Checks

1. **Filter Menu Management**
   - Dynamic enabling/disabling based on active source
   - Real-time updates when selection changes
   - Tooltip messages explaining why filters are disabled

2. **Domain Querying**
   - Client queries server domains
   - Checks DataTypeDomain, InputArrayDomain, ProxyGroupDomain
   - Updates UI state based on domain validity

3. **Property Widget Updates**
   - Property widgets check domains
   - Enable/disable based on domain validity
   - Update tooltips with failure reasons

### Server-Side Checks

1. **Domain Validation**
   - DataTypeDomain validates input data types
   - InputArrayDomain validates array availability
   - ProxyGroupDomain validates proxy groups

2. **Property State**
   - Properties check all domains
   - IsEnabled() returns false if any domain invalid
   - Used by client to determine UI state

3. **Pipeline Execution**
   - Final validation during pipeline execution
   - VTK pipeline validates types (FillInputPortInformation)
   - Server rejects invalid filter applications

### Integration Flow

```
1. User selects source
   ↓
2. Client detects selection change
   ↓
3. Client queries server domains for each filter
   ↓
4. Server validates domains against active source
   ↓
5. Server returns domain validity
   ↓
6. Client updates filter menu (enables/disables items)
   ↓
7. User sees updated menu with tooltips
```

### Key Classes

- **Client-Side**: `pqProxySelectionModel`, `pqProxy`, `pqPropertyWidget`, `pqFilterMenuReaction`
- **Server-Side**: `vtkSMProxy`, `vtkSMProperty`, `vtkSMDomain`, `vtkSMDataTypeDomain`, `vtkSMInputArrayDomain`

This architecture ensures **proactive prevention** of invalid filter applications, **clear user feedback**, and **robust validation** at both client and server levels.
