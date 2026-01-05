# ParaView Pipeline Provenance and Reproducibility: Pipeline Browser, Filter Chain, and State Files

## Executive Summary

ParaView exposes complete **pipeline provenance** to users through three primary mechanisms: (1) **Pipeline Browser** (visual tree of sources and filters), (2) **Filter Chain** (sequential processing steps), and (3) **Parameter Visibility** (all filter parameters accessible in Properties panel). This architectural choice makes the entire data processing workflow transparent and reproducible. ParaView captures this provenance in **state files** (`.pvsm`) that record pipeline topology, filter parameters, and view settings, enabling complete reproducibility by reloading the same state with the same input data.

**Key Provenance Mechanisms**:
- **Pipeline Browser**: Visual tree showing complete pipeline structure
- **Filter Chain**: Sequential list of all processing steps
- **Parameter Exposure**: All filter parameters visible and editable
- **State Files**: XML-based capture of complete pipeline state
- **Python Scripts**: Recordable/replayable pipeline definitions

**Reproducibility Features**:
- Complete pipeline topology captured
- All filter parameters recorded
- Input data file paths stored
- View settings preserved
- Python script generation for programmatic reproduction

---

## 1. Pipeline Browser: Visual Provenance

### What is the Pipeline Browser?

The **Pipeline Browser** is ParaView's primary interface for displaying the complete pipeline structure as a hierarchical tree. It shows all sources, filters, and their connections, making the data processing workflow fully visible to users.

**Visual Structure**:

```
Pipeline Browser
├── Sources
│   ├── Sphere1
│   ├── Cone1
│   └── Wavelet1
├── Filters
│   ├── Contour1 (Input: Sphere1)
│   ├── Shrink1 (Input: Contour1)
│   └── Clip1 (Input: Wavelet1)
└── Representations
    ├── Contour1 → GeometryRepresentation
    └── Clip1 → GeometryRepresentation
```

### Pipeline Browser Implementation

**Server Manager Architecture**:

```cpp
// Pipeline Browser displays Server Manager proxy tree
class pqPipelineBrowser : public QTreeWidget
{
public:
    // Update pipeline browser from proxy manager
    void updateFromProxyManager()
    {
        // Get all proxies
        vtkSMProxyManager* proxyManager = 
            vtkSMProxyManager::GetProxyManager();
        
        vtkSMProxyIterator* iter = vtkSMProxyIterator::New();
        iter->SetModeToActiveOnly();
        
        // Iterate through all proxies
        for (iter->Begin(); !iter->IsAtEnd(); iter->Next())
        {
            vtkSMProxy* proxy = iter->GetProxy();
            
            // Add to tree based on proxy type
            if (this->IsSource(proxy))
            {
                this->addSourceItem(proxy);
            }
            else if (this->IsFilter(proxy))
            {
                this->addFilterItem(proxy);
            }
        }
        
        iter->Delete();
    }
    
    // Add source to tree
    void addSourceItem(vtkSMProxy* source)
    {
        QTreeWidgetItem* item = new QTreeWidgetItem();
        item->setText(0, source->GetXMLLabel());
        item->setData(0, Qt::UserRole, QVariant::fromValue(source));
        
        // Add to sources group
        this->SourcesGroup->addChild(item);
    }
    
    // Add filter to tree
    void addFilterItem(vtkSMProxy* filter)
    {
        QTreeWidgetItem* item = new QTreeWidgetItem();
        item->setText(0, filter->GetXMLLabel());
        item->setData(0, Qt::UserRole, QVariant::fromValue(filter));
        
        // Get input proxy
        vtkSMProxy* input = this->getInputProxy(filter);
        if (input)
        {
            // Find input item in tree
            QTreeWidgetItem* inputItem = this->findItemForProxy(input);
            if (inputItem)
            {
                // Add as child of input
                inputItem->addChild(item);
            }
        }
    }
};
```

### Pipeline Browser Features

**1. Hierarchical Display**:
- Sources at top level
- Filters nested under their inputs
- Representations shown under filters
- Clear parent-child relationships

**2. Visual Indicators**:
- Icons for different proxy types (source, filter, representation)
- Visibility indicators (eye icon)
- Selection highlighting
- Active/inactive state

**3. Interactive Operations**:
- Select proxy (updates Properties panel)
- Delete proxy (removes from pipeline)
- Rename proxy (custom names)
- Show/Hide (toggle visibility)

**4. Connection Visualization**:
- Tree structure shows connections
- Parent-child relationship = input-output connection
- Multiple children = one output to multiple filters

### Provenance Information Exposed

**What Users Can See**:

1. **Complete Pipeline Structure**:
   - All sources and filters
   - Connection relationships
   - Processing order

2. **Filter Names and Types**:
   - Custom names (e.g., "Contour1", "Shrink1")
   - Filter types (e.g., "Contour", "Shrink")
   - Source types (e.g., "Sphere", "Wavelet")

3. **Pipeline Topology**:
   - Which filters connect to which sources
   - Filter chains (sequential processing)
   - Branching (one source to multiple filters)

4. **Representation Mapping**:
   - Which filters are visualized
   - Representation types (Geometry, Surface, etc.)
   - View assignments

---

## 2. Filter Chain: Sequential Provenance

### What is the Filter Chain?

The **Filter Chain** represents the sequential processing steps applied to data. It's the linear path from source through filters to final output, showing the complete transformation history.

**Example Filter Chain**:

```
Source: Wavelet
    ↓
Filter: Contour (Value: 0.5)
    ↓
Filter: Shrink (Factor: 0.8)
    ↓
Filter: Clip (Plane: X=0)
    ↓
Output: Clip1
```

### Filter Chain Representation

**In Pipeline Browser**:

The pipeline browser shows the filter chain as a tree path:

```
Wavelet1
└── Contour1
    └── Shrink1
        └── Clip1
```

**In Properties Panel**:

When a filter is selected, the Properties panel shows:
- **Input**: The upstream filter/source
- **Output**: Available for downstream filters
- **Parameters**: All filter-specific parameters

### Filter Chain Metadata

**Chain Information Captured**:

```cpp
// Filter chain metadata
struct FilterChainInfo
{
    std::vector<vtkSMProxy*> Filters;  // Sequential filters
    vtkSMProxy* Source;                 // Root source
    std::vector<std::string> FilterTypes;  // Filter class names
    std::vector<std::map<std::string, Variant>> Parameters;  // Filter parameters
};
```

**Extracting Filter Chain**:

```cpp
// Get filter chain for a proxy
FilterChainInfo GetFilterChain(vtkSMProxy* proxy)
{
    FilterChainInfo chain;
    
    // Walk upstream to source
    vtkSMProxy* current = proxy;
    while (current)
    {
        chain.Filters.insert(chain.Filters.begin(), current);
        
        // Get input
        vtkSMInputProperty* inputProp = 
            vtkSMInputProperty::SafeDownCast(
                current->GetProperty("Input"));
        
        if (inputProp && inputProp->GetNumberOfProxies() > 0)
        {
            current = inputProp->GetProxy(0);
        }
        else
        {
            // Reached source
            chain.Source = current;
            break;
        }
    }
    
    // Extract parameters for each filter
    for (auto& filter : chain.Filters)
    {
        std::map<std::string, Variant> params;
        
        // Get all properties
        vtkSMPropertyIterator* propIter = filter->NewPropertyIterator();
        for (propIter->Begin(); !propIter->IsAtEnd(); propIter->Next())
        {
            vtkSMProperty* prop = propIter->GetProperty();
            std::string name = prop->GetXMLName();
            Variant value = prop->GetVariantValue();
            params[name] = value;
        }
        
        chain.Parameters.push_back(params);
    }
    
    return chain;
}
```

### Filter Chain Display

**User-Visible Information**:

1. **Processing Order**:
   - Filters listed in execution order
   - Clear sequence from source to output

2. **Filter Types**:
   - Each filter's class name (e.g., "vtkContourFilter")
   - Display name (e.g., "Contour")

3. **Parameters**:
   - All parameters for each filter
   - Current values
   - Default values

4. **Input-Output Relationships**:
   - Which filter feeds into which
   - Data flow direction

---

## 3. Parameter Exposure: Complete Transparency

### Properties Panel

The **Properties Panel** exposes all filter parameters, making the complete processing configuration visible and editable.

**Parameter Display**:

```cpp
// Properties panel shows all proxy properties
class pqPropertiesPanel : public QWidget
{
    void updateForProxy(vtkSMProxy* proxy)
    {
        // Clear existing widgets
        this->clear();
        
        // Get all properties
        vtkSMPropertyIterator* propIter = proxy->NewPropertyIterator();
        
        for (propIter->Begin(); !propIter->IsAtEnd(); propIter->Next())
        {
            vtkSMProperty* prop = propIter->GetProperty();
            
            // Skip hidden properties
            if (prop->GetInformation()->Has(vtkSMProperty::HIDE_FROM_GUI()))
            {
                continue;
            }
            
            // Create widget for property
            pqPropertyWidget* widget = this->createWidgetForProperty(prop);
            this->addWidget(widget);
        }
        
        propIter->Delete();
    }
};
```

### Parameter Categories

**Property Types Exposed**:

1. **Input Properties**:
   - Input connections
   - Input port selection
   - Multiple input handling

2. **Algorithm Parameters**:
   - Filter-specific parameters
   - Threshold values
   - Transformation parameters
   - Processing options

3. **Display Properties**:
   - Color mapping
   - Opacity
   - Representation type
   - Visibility

4. **Information Properties**:
   - Data statistics
   - Bounds
   - Number of points/cells

### Parameter Provenance

**What Gets Captured**:

```cpp
// Parameter provenance structure
struct ParameterProvenance
{
    std::string PropertyName;      // Property name
    std::string PropertyType;     // Property type (Double, Int, String, etc.)
    Variant Value;                 // Current value
    Variant DefaultValue;          // Default value
    std::string Documentation;    // Help text
    std::map<std::string, Variant> DomainConstraints;  // Valid ranges, etc.
};
```

**Extracting Parameters**:

```cpp
// Get all parameters for a proxy
std::vector<ParameterProvenance> GetParameters(vtkSMProxy* proxy)
{
    std::vector<ParameterProvenance> params;
    
    vtkSMPropertyIterator* propIter = proxy->NewPropertyIterator();
    
    for (propIter->Begin(); !propIter->IsAtEnd(); propIter->Next())
    {
        vtkSMProperty* prop = propIter->GetProperty();
        
        ParameterProvenance param;
        param.PropertyName = prop->GetXMLName();
        param.PropertyType = prop->GetClassName();
        param.Value = prop->GetVariantValue();
        param.DefaultValue = prop->GetDefaultValue();
        param.Documentation = prop->GetDocumentation();
        
        // Get domain constraints
        vtkSMDomainIterator* domainIter = prop->NewDomainIterator();
        for (domainIter->Begin(); !domainIter->IsAtEnd(); domainIter->Next())
        {
            vtkSMDomain* domain = domainIter->GetDomain();
            // Extract constraints (ranges, lists, etc.)
            param.DomainConstraints[domain->GetClassName()] = 
                domain->GetVariantValue();
        }
        domainIter->Delete();
        
        params.push_back(param);
    }
    
    propIter->Delete();
    return params;
}
```

---

## 4. State Files: Complete Provenance Capture

### What are State Files?

**State files** (`.pvsm`) are XML files that capture the complete ParaView session state, including:
- All pipeline sources and filters
- All filter parameters
- Pipeline connections
- View settings
- Representation configurations

### State File Structure

**XML Format**:

```xml
<ServerManagerState version="5.8.0">
  <ProxyCollection>
    <!-- Sources -->
    <Item id="0" name="Wavelet1" group="sources" type="Wavelet">
      <Property name="WholeExtent" value="0 20 0 20 0 20"/>
      <Property name="Maximum" value="255"/>
      <Property name="XFrequency" value="60"/>
    </Item>
    
    <!-- Filters -->
    <Item id="1" name="Contour1" group="filters" type="Contour">
      <Property name="Input" value="0"/>  <!-- References source id="0" -->
      <Property name="ContourValues" value="127.5"/>
      <Property name="ComputeScalars" value="1"/>
    </Item>
    
    <Item id="2" name="Shrink1" group="filters" type="Shrink">
      <Property name="Input" value="1"/>  <!-- References filter id="1" -->
      <Property name="ShrinkFactor" value="0.8"/>
    </Item>
    
    <!-- Representations -->
    <Item id="3" name="Contour1Representation" group="representations" 
          type="GeometryRepresentation">
      <Property name="Input" value="2"/>  <!-- References filter id="2" -->
      <Property name="Representation" value="Surface"/>
      <Property name="ColorArrayName" value="RTData"/>
    </Item>
    
    <!-- Views -->
    <Item id="4" name="RenderView1" group="views" type="RenderView">
      <Property name="Representations" value="3"/>  <!-- References representation -->
      <Property name="CameraPosition" value="0 0 100"/>
      <Property name="CameraFocalPoint" value="10 10 10"/>
    </Item>
  </ProxyCollection>
</ServerManagerState>
```

### State File Generation

**Saving State**:

```cpp
// Save current state to file
void SaveStateFile(const std::string& filename)
{
    vtkSMProxyManager* proxyManager = 
        vtkSMProxyManager::GetProxyManager();
    
    // Create state XML
    vtkPVXMLElement* root = vtkPVXMLElement::New();
    root->SetName("ServerManagerState");
    root->AddAttribute("version", "5.8.0");
    
    // Create proxy collection
    vtkPVXMLElement* collection = vtkPVXMLElement::New();
    collection->SetName("ProxyCollection");
    
    // Serialize all proxies
    vtkSMProxyIterator* iter = vtkSMProxyIterator::New();
    iter->SetModeToActiveOnly();
    
    int id = 0;
    for (iter->Begin(); !iter->IsAtEnd(); iter->Next())
    {
        vtkSMProxy* proxy = iter->GetProxy();
        
        // Serialize proxy
        vtkPVXMLElement* item = SerializeProxy(proxy, id);
        collection->AddNestedElement(item);
        id++;
    }
    
    root->AddNestedElement(collection);
    
    // Write to file
    root->SaveFile(filename.c_str());
    
    root->Delete();
    iter->Delete();
}
```

**Serializing Proxy**:

```cpp
// Serialize a proxy to XML
vtkPVXMLElement* SerializeProxy(vtkSMProxy* proxy, int id)
{
    vtkPVXMLElement* item = vtkPVXMLElement::New();
    item->SetName("Item");
    item->AddAttribute("id", id);
    item->AddAttribute("name", proxy->GetXMLLabel());
    item->AddAttribute("group", proxy->GetXMLGroup());
    item->AddAttribute("type", proxy->GetXMLName());
    
    // Serialize all properties
    vtkSMPropertyIterator* propIter = proxy->NewPropertyIterator();
    for (propIter->Begin(); !propIter->IsAtEnd(); propIter->Next())
    {
        vtkSMProperty* prop = propIter->GetProperty();
        
        // Skip properties that shouldn't be saved
        if (prop->GetInformation()->Has(vtkSMProperty::DONT_SAVE_STATE()))
        {
            continue;
        }
        
        // Serialize property
        vtkPVXMLElement* propElem = SerializeProperty(prop);
        item->AddNestedElement(propElem);
    }
    
    propIter->Delete();
    return item;
}
```

### State File Loading

**Loading State**:

```cpp
// Load state from file
void LoadStateFile(const std::string& filename)
{
    // Parse XML
    vtkPVXMLParser* parser = vtkPVXMLParser::New();
    parser->SetFileName(filename.c_str());
    parser->Parse();
    
    vtkPVXMLElement* root = parser->GetRootElement();
    
    // Get proxy collection
    vtkPVXMLElement* collection = root->FindNestedElementByName("ProxyCollection");
    
    // Deserialize proxies
    int numItems = collection->GetNumberOfNestedElements();
    for (int i = 0; i < numItems; i++)
    {
        vtkPVXMLElement* item = collection->GetNestedElement(i);
        DeserializeProxy(item);
    }
    
    parser->Delete();
}
```

---

## 5. Reproducibility Mechanisms

### Complete Provenance Capture

**What Gets Captured for Reproducibility**:

1. **Pipeline Topology**:
   - All sources and filters
   - Connection relationships
   - Processing order

2. **Filter Parameters**:
   - All parameter values
   - Default values
   - Parameter types

3. **Input Data References**:
   - File paths (for file readers)
   - Source parameters (for generated data)
   - Time step information

4. **View Settings**:
   - Camera positions
   - View types
   - Representation settings

5. **Display Properties**:
   - Color maps
   - Opacity settings
   - Visibility states

### Reproducibility Workflow

**Step 1: Create Pipeline**:

```
User creates pipeline:
1. Load data file: "data.vtk"
2. Apply Contour filter (Value: 0.5)
3. Apply Shrink filter (Factor: 0.8)
4. Visualize in 3D view
```

**Step 2: Save State**:

```
User saves state file: "pipeline.pvsm"
State file contains:
- File path: "data.vtk"
- Contour filter: Value=0.5
- Shrink filter: Factor=0.8
- View settings
```

**Step 3: Reproduce**:

```
User loads state file: "pipeline.pvsm"
ParaView:
1. Loads "data.vtk"
2. Creates Contour filter with Value=0.5
3. Creates Shrink filter with Factor=0.8
4. Applies same view settings
Result: Identical visualization
```

### Python Script Generation

**Trace Recording**:

ParaView can record user actions as Python scripts:

```python
# Generated Python script
from paraview.simple import *

# Create source
wavelet1 = Wavelet(WholeExtent=[0, 20, 0, 20, 0, 20],
                   Maximum=255,
                   XFrequency=60)

# Apply filters
contour1 = Contour(Input=wavelet1,
                   ContourValues=[127.5],
                   ComputeScalars=1)

shrink1 = Shrink(Input=contour1,
                 ShrinkFactor=0.8)

# Create representation
contour1Display = Show(shrink1, renderView1)

# Set representation properties
contour1Display.Representation = 'Surface'
contour1Display.ColorArrayName = ['POINTS', 'RTData']

# Render
Render()
```

**Script Execution**:

```python
# Reproduce by running script
exec(open('pipeline.py').read())
# Produces identical pipeline
```

### Reproducibility Guarantees

**What State Files Guarantee**:

1. **Pipeline Structure**:
   - Same sources and filters
   - Same connections
   - Same processing order

2. **Parameter Values**:
   - All parameters set to same values
   - Defaults preserved

3. **View Configuration**:
   - Same camera position
   - Same representation settings
   - Same color maps

**What State Files Don't Guarantee**:

1. **Input Data**:
   - State files reference file paths, not data content
   - Data files must exist and be accessible
   - Data content must be unchanged

2. **VTK Version**:
   - Different VTK versions may produce slightly different results
   - Algorithm implementations may change

3. **System Differences**:
   - Rendering may differ on different systems
   - Performance may vary

---

## 6. Provenance Architecture Benefits

### Transparency

**Complete Visibility**:
- Users can see entire pipeline
- All parameters exposed
- No hidden processing steps

**Educational Value**:
- Users learn data processing workflows
- Understand filter effects
- See parameter relationships

### Reproducibility

**State File Reproducibility**:
- Complete pipeline captured
- Can reload exact state
- Share pipelines with others

**Python Script Reproducibility**:
- Programmatic reproduction
- Version control friendly
- Automated pipeline execution

### Debugging

**Pipeline Inspection**:
- See what filters were applied
- Check parameter values
- Understand data transformations

**Error Diagnosis**:
- Identify problematic filters
- Check parameter ranges
- Verify connections

### Collaboration

**Pipeline Sharing**:
- Share state files
- Reproduce others' work
- Build on existing pipelines

**Documentation**:
- State files document workflows
- Python scripts serve as documentation
- Provenance provides audit trail

---

## 7. Summary: Provenance and Reproducibility

### Key Mechanisms

1. **Pipeline Browser**:
   - Visual tree of complete pipeline
   - Shows all sources, filters, connections
   - Hierarchical structure

2. **Filter Chain**:
   - Sequential processing steps
   - Processing order visible
   - Transformation history

3. **Parameter Exposure**:
   - All parameters visible in Properties panel
   - Complete configuration transparency
   - Editable values

4. **State Files**:
   - XML-based complete state capture
   - Pipeline topology and parameters
   - View settings

5. **Python Scripts**:
   - Recordable user actions
   - Programmatic reproduction
   - Version control friendly

### Reproducibility Features

- **Complete Capture**: All pipeline information saved
- **Reloadable**: State files can reload exact state
- **Shareable**: State files can be shared
- **Documented**: Provenance provides audit trail
- **Programmatic**: Python scripts enable automation

### Architectural Benefits

- **Transparency**: Complete visibility into processing
- **Reproducibility**: Can reproduce exact results
- **Collaboration**: Easy to share and reproduce work
- **Debugging**: Easy to inspect and diagnose
- **Education**: Learn from existing pipelines

This architectural choice makes ParaView pipelines **fully transparent and reproducible**, supporting scientific workflows where reproducibility is essential.

