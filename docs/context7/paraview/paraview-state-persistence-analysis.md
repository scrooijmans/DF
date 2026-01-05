# ParaView Session/State Persistence System: Deep Architecture Analysis

## Executive Summary

ParaView's session/state persistence system enables complete capture and restoration of visualization workflows through XML-based state files (`.pvsm`). The system serializes the entire Server Manager state—including pipelines, views, representations, layouts, color maps, and camera settings—into a structured XML format that can be reloaded to exactly reproduce a visualization session. The architecture uses a proxy-based client-server model where proxies own their state, and state is propagated through the Server Manager layer for serialization/deserialization.

**Key Architectural Principles**:

- **Proxy Ownership**: Each proxy (source, filter, view, representation) owns its state
- **Server Manager Centralization**: `vtkSMProxyManager` coordinates all state operations
- **XML Serialization**: Human-readable XML format for state files
- **Versioning Support**: State files include version information for backward compatibility
- **Extensibility**: Custom plugins/filters/views hook into state system via proxy registration

---

## 1. Data Model: How ParaView Represents a "Session/State"

### Core State Components

A ParaView session/state encompasses the complete visualization environment:

#### 1.1 Pipeline State

- **Sources**: Data generators (file readers, procedural sources like Sphere, Wavelet)
- **Filters**: Data transformers (Contour, Shrink, Clip, etc.)
- **Pipeline Connections**: Input-output relationships between sources and filters
- **Filter Parameters**: All parameter values for each filter/source

**Representation**:

```cpp
// Pipeline state structure
struct PipelineState {
    std::vector<SourceProxy> sources;      // All data sources
    std::vector<FilterProxy> filters;       // All filters
    std::map<int, int> connections;         // Filter ID -> Input Source/Filter ID
    std::map<int, ParameterMap> parameters; // Proxy ID -> Parameter values
};
```

#### 1.2 View State

- **View Types**: RenderView (3D), ChartView (2D), SpreadSheetView (table)
- **View Properties**: Camera position, background color, orientation
- **View Layout**: Position and size within the application window
- **Multi-View Arrangements**: Split panes, viewport configurations

**Representation**:

```cpp
// View state structure
struct ViewState {
    std::string viewType;              // "RenderView", "ChartView", etc.
    CameraState camera;                 // Camera position, focal point, up vector
    ViewportGeometry viewport;          // Position and size in layout
    std::vector<int> representationIds; // Representations displayed in this view
    ViewProperties properties;          // Background color, orientation, etc.
};
```

#### 1.3 Representation State

- **Representation Type**: Geometry, Surface, Wireframe, Points, etc.
- **Input Connection**: Which pipeline output is visualized
- **Display Properties**: Color mapping, opacity, visibility
- **Color Map**: Scalar field selection, color transfer function

**Representation**:

```cpp
// Representation state structure
struct RepresentationState {
    std::string representationType;    // "GeometryRepresentation", etc.
    int inputProxyId;                   // ID of source/filter being visualized
    std::string colorArrayName;         // Scalar field for coloring
    ColorTransferFunction colorMap;    // Color mapping function
    double opacity;                      // Opacity value
    bool visibility;                     // Visible/hidden state
};
```

#### 1.4 Layout State

- **Layout Structure**: Tree of view containers (split panes)
- **Viewport Assignments**: Which views occupy which viewports
- **Split Configuration**: Horizontal/vertical splits, split ratios

**Representation**:

```cpp
// Layout state structure
struct LayoutState {
    LayoutNode* rootNode;              // Root of layout tree
    std::map<int, ViewportGeometry> viewportMap; // View ID -> Viewport geometry
};

// Layout tree node
struct LayoutNode {
    enum Type { Split, View };
    Type type;
    union {
        struct {
            bool isHorizontal;          // Horizontal or vertical split
            double splitRatio;           // Split position (0.0 to 1.0)
            LayoutNode* leftChild;
            LayoutNode* rightChild;
        } split;
        struct {
            int viewId;                  // View ID if this is a view node
        } view;
    };
};
```

#### 1.5 Color Map State

- **Transfer Functions**: Scalar-to-color mappings
- **Lookup Tables**: Precomputed color tables
- **Scalar Bar Configuration**: Color legend properties

**Representation**:

```cpp
// Color map state structure
struct ColorMapState {
    std::string scalarArrayName;        // Scalar field name
    ColorTransferFunction transferFunction; // RGB points, interpolation
    ScalarBarProperties scalarBar;       // Legend configuration
};
```

### Complete State Model

```cpp
// Complete session state
struct ParaViewSessionState {
    std::string version;                 // ParaView version (e.g., "5.11.0")
    PipelineState pipeline;              // All sources, filters, connections
    std::vector<ViewState> views;        // All views
    std::vector<RepresentationState> representations; // All representations
    LayoutState layout;                  // View layout structure
    std::vector<ColorMapState> colorMaps; // All color maps
    std::map<std::string, std::string> metadata; // Additional metadata
};
```

---

## 2. File Formats: `.pvsm` Structure and Serialization

### 2.1 `.pvsm` File Format

ParaView State Files (`.pvsm`) are XML-based files that serialize the complete Server Manager state.

#### XML Structure

```xml
<?xml version="1.0"?>
<ServerManagerState version="5.11.0">
  <!-- Proxy Collection: All proxies (sources, filters, views, representations) -->
  <ProxyCollection>
    <!-- Source Proxy -->
    <Item id="0" name="Wavelet1" group="sources" type="Wavelet">
      <Property name="WholeExtent" value="0 20 0 20 0 20"/>
      <Property name="Maximum" value="255"/>
      <Property name="XFrequency" value="60"/>
      <Property name="YFrequency" value="30"/>
      <Property name="ZFrequency" value="40"/>
    </Item>

    <!-- Filter Proxy -->
    <Item id="1" name="Contour1" group="filters" type="Contour">
      <Property name="Input" value="0"/>  <!-- References source id="0" -->
      <Property name="ContourValues" value="127.5"/>
      <Property name="ComputeScalars" value="1"/>
      <Property name="ComputeNormals" value="1"/>
    </Item>

    <!-- Representation Proxy -->
    <Item id="2" name="Contour1Representation" group="representations"
          type="GeometryRepresentation">
      <Property name="Input" value="1"/>  <!-- References filter id="1" -->
      <Property name="Representation" value="Surface"/>
      <Property name="ColorArrayName" value="RTData"/>
      <Property name="Opacity" value="1.0"/>
      <Property name="Visibility" value="1"/>
    </Item>

    <!-- View Proxy -->
    <Item id="3" name="RenderView1" group="views" type="RenderView">
      <Property name="Representations" value="2"/>  <!-- References representation id="2" -->
      <Property name="CameraPosition" value="0 0 100"/>
      <Property name="CameraFocalPoint" value="10 10 10"/>
      <Property name="CameraViewUp" value="0 1 0"/>
      <Property name="CameraViewAngle" value="30"/>
      <Property name="Background" value="0.32 0.34 0.43"/>
      <Property name="ViewSize" value="800 600"/>
    </Item>

    <!-- Chart View Proxy -->
    <Item id="4" name="ChartView1" group="views" type="XYChartView">
      <Property name="Title" value="Data Plot"/>
      <Property name="XAxisTitle" value="Time"/>
      <Property name="YAxisTitle" value="Value"/>
      <Property name="ShowLegend" value="1"/>
    </Item>
  </ProxyCollection>

  <!-- View Collection: Layout and viewport configuration -->
  <ViewCollection>
    <View id="3" type="RenderView" x="0" y="0" width="0.5" height="1.0"/>
    <View id="4" type="XYChartView" x="0.5" y="0" width="0.5" height="1.0"/>
  </ViewCollection>

  <!-- Layout Collection: Multi-view layout structure -->
  <LayoutCollection>
    <Layout name="MainLayout">
      <Split direction="horizontal" ratio="0.5">
        <View id="3"/>
        <View id="4"/>
      </Split>
    </Layout>
  </LayoutCollection>
</ServerManagerState>
```

### 2.2 What Gets Serialized vs Reconstructed

#### Serialized (Stored in `.pvsm`):

1. **Proxy Definitions**: All proxy types, names, groups
2. **Property Values**: All non-default property values
3. **Proxy Connections**: Input-output relationships (via proxy IDs)
4. **View Configurations**: Camera, background, viewport geometry
5. **Representation Settings**: Type, color mapping, visibility
6. **Layout Structure**: Split panes, viewport assignments
7. **Color Map Definitions**: Transfer functions, lookup tables
8. **Metadata**: Version, timestamps, user information

#### Reconstructed (Not Stored, Recreated on Load):

1. **VTK Algorithm Instances**: Created from proxy definitions
2. **Data Objects**: Data must be loaded separately (file paths stored, not data)
3. **Render Windows**: Recreated from view definitions
4. **UI Widgets**: Recreated from layout definitions
5. **Pipeline Execution**: Pipeline runs on load to generate data

### 2.3 Serialization Process

```cpp
// High-level serialization flow
void vtkSMProxyManager::SaveState(vtkPVXMLElement* root)
{
    // 1. Create ProxyCollection element
    vtkPVXMLElement* collection = vtkPVXMLElement::New();
    collection->SetName("ProxyCollection");

    // 2. Iterate through all proxies
    vtkSMProxyIterator* iter = vtkSMProxyIterator::New();
    iter->SetModeToActiveOnly();

    int proxyId = 0;
    for (iter->Begin(); !iter->IsAtEnd(); iter->Next())
    {
        vtkSMProxy* proxy = iter->GetProxy();

        // 3. Serialize each proxy
        vtkPVXMLElement* item = this->SerializeProxy(proxy, proxyId);
        collection->AddNestedElement(item);
        proxyId++;
    }

    root->AddNestedElement(collection);

    // 4. Serialize view collection
    vtkPVXMLElement* viewCollection = this->SerializeViewCollection();
    root->AddNestedElement(viewCollection);

    // 5. Serialize layout collection
    vtkPVXMLElement* layoutCollection = this->SerializeLayoutCollection();
    root->AddNestedElement(layoutCollection);

    iter->Delete();
}

// Serialize individual proxy
vtkPVXMLElement* vtkSMProxyManager::SerializeProxy(
    vtkSMProxy* proxy, int id)
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

        // Skip properties marked as DONT_SAVE_STATE
        if (prop->GetInformation()->Has(vtkSMProperty::DONT_SAVE_STATE()))
        {
            continue;
        }

        // Serialize property
        vtkPVXMLElement* propElem = this->SerializeProperty(proxy, prop);
        if (propElem)
        {
            item->AddNestedElement(propElem);
        }
    }

    propIter->Delete();
    return item;
}

// Serialize property (handles different property types)
vtkPVXMLElement* vtkSMProxyManager::SerializeProperty(
    vtkSMProxy* proxy, vtkSMProperty* prop)
{
    vtkPVXMLElement* propElem = vtkPVXMLElement::New();
    propElem->SetName("Property");
    propElem->AddAttribute("name", prop->GetXMLName());

    // Handle different property types
    if (vtkSMInputProperty* inputProp =
        vtkSMInputProperty::SafeDownCast(prop))
    {
        // Input property: serialize proxy ID reference
        if (inputProp->GetNumberOfProxies() > 0)
        {
            vtkSMProxy* inputProxy = inputProp->GetProxy(0);
            int inputId = this->GetProxyId(inputProxy);
            propElem->AddAttribute("value", inputId);
        }
    }
    else if (vtkSMIntVectorProperty* intProp =
             vtkSMIntVectorProperty::SafeDownCast(prop))
    {
        // Integer vector property
        std::stringstream ss;
        for (unsigned int i = 0; i < intProp->GetNumberOfElements(); i++)
        {
            if (i > 0) ss << " ";
            ss << intProp->GetElement(i);
        }
        propElem->AddAttribute("value", ss.str().c_str());
    }
    else if (vtkSMDoubleVectorProperty* doubleProp =
             vtkSMDoubleVectorProperty::SafeDownCast(prop))
    {
        // Double vector property
        std::stringstream ss;
        for (unsigned int i = 0; i < doubleProp->GetNumberOfElements(); i++)
        {
            if (i > 0) ss << " ";
            ss << doubleProp->GetElement(i);
        }
        propElem->AddAttribute("value", ss.str().c_str());
    }
    else if (vtkSMStringVectorProperty* stringProp =
             vtkSMStringVectorProperty::SafeDownCast(prop))
    {
        // String vector property
        std::stringstream ss;
        for (unsigned int i = 0; i < stringProp->GetNumberOfElements(); i++)
        {
            if (i > 0) ss << " ";
            ss << stringProp->GetElement(i);
        }
        propElem->AddAttribute("value", ss.str().c_str());
    }
    // ... handle other property types

    return propElem;
}
```

---

## 3. Architecture + Ownership: State Management Subsystems

### 3.1 Component Ownership Model

#### Server Manager Layer (Core State Owner)

- **`vtkSMProxyManager`**: Central registry and coordinator

  - Owns all proxy instances
  - Manages proxy lifecycle (create, destroy, register)
  - Coordinates state serialization/deserialization
  - Maintains proxy ID mappings

- **`vtkSMProxy`**: Individual proxy state owner
  - Each proxy owns its properties and configuration
  - Proxies wrap VTK algorithm instances
  - Properties map to VTK algorithm methods/parameters
  - State serializable via `SaveState()` / `LoadState()`

#### View Layer (View State Owner)

- **`vtkSMRenderView`**: 3D render view state

  - Owns camera state (position, focal point, view up)
  - Owns viewport geometry
  - Owns representation list (what's displayed)
  - Owns background color, orientation

- **`vtkSMChartView`**: 2D chart view state

  - Owns chart type (XY, Bar, etc.)
  - Owns axis configurations
  - Owns legend settings
  - Owns plot series configurations

- **`pqView`**: Client-side view wrapper
  - Wraps `vtkSMView` proxy
  - Manages UI representation
  - Handles view updates and rendering

#### Layout Layer (Layout State Owner)

- **`pqMultiViewWidget`**: Multi-view layout manager

  - Owns layout tree structure
  - Manages split pane configurations
  - Assigns views to viewports
  - Handles layout serialization

- **`pqViewFrame`**: Individual view frame
  - Owns viewport geometry within layout
  - Manages view frame UI
  - Handles view frame resizing

#### Representation Layer (Representation State Owner)

- **`vtkSMRepresentationProxy`**: Representation state
  - Owns representation type (Geometry, Surface, etc.)
  - Owns input connection (which pipeline output)
  - Owns display properties (color, opacity, visibility)
  - Owns color map association

### 3.2 State Propagation: Client ↔ Server

#### Save State Flow (Client → Server → File)

```
User Action: "Save State"
    │
    ▼
pqSaveStateReaction::saveState()
    │
    ▼
pqApplicationCore::saveState(filename)
    │
    ▼
vtkSMProxyManager::SaveState(rootElement)
    │
    ├─> Iterate all proxies
    │   ├─> vtkSMProxy::SaveState(element)  [Each proxy serializes itself]
    │   └─> Collect proxy states
    │
    ├─> Serialize ViewCollection
    │   └─> pqMultiViewWidget::saveState()
    │
    ├─> Serialize LayoutCollection
    │   └─> pqMultiViewWidget::saveLayoutState()
    │
    └─> Write XML to file
```

#### Load State Flow (File → Server → Client)

```
User Action: "Load State"
    │
    ▼
pqLoadStateReaction::loadState()
    │
    ▼
pqApplicationCore::loadState(filename)
    │
    ▼
vtkPVXMLParser::Parse(filename)
    │
    ▼
vtkSMProxyManager::LoadState(rootElement)
    │
    ├─> Parse ProxyCollection
    │   ├─> Create proxies from XML
    │   ├─> vtkSMProxyManager::NewProxy(group, type)
    │   ├─> vtkSMProxy::LoadState(element)  [Each proxy deserializes itself]
    │   └─> Restore proxy properties
    │
    ├─> Parse ViewCollection
    │   ├─> Create view proxies
    │   └─> Restore view configurations
    │
    ├─> Parse LayoutCollection
    │   ├─> pqMultiViewWidget::loadLayoutState()
    │   └─> Reconstruct layout tree
    │
    └─> Emit stateLoaded() signal
        │
        ▼
    pqApplicationCore::stateLoaded()
        │
        ├─> Update Pipeline Browser
        ├─> Update Properties Panel
        ├─> Update Views
        └─> Trigger pipeline execution
```

### 3.3 State Synchronization

#### Property Synchronization

- **Client → Server**: Property changes in UI update proxy properties, which update VTK algorithms
- **Server → Client**: Pipeline execution results update proxy output ports, which update UI

#### View Synchronization

- **Client → Server**: View interactions (camera movement, zoom) update view proxy properties
- **Server → Client**: View proxy updates trigger render window updates

#### Layout Synchronization

- **Client → Server**: Layout changes (split, resize) update layout state
- **Server → Client**: Layout state changes trigger UI layout reconstruction

---

## 4. Call Stack: Save State and Load State

### 4.1 Save State Call Stack

#### High-Level Flow

```
1. User Action
   └─> File Menu → "Save State"
       │
       ▼
2. UI Layer
   pqSaveStateReaction::saveState()
       │
       ├─> QFileDialog::getSaveFileName()  [Get filename from user]
       │
       └─> pqApplicationCore::saveState(filename)
           │
           ▼
3. Application Core
   pqApplicationCore::saveState(const QString& filename)
       │
       ├─> Create vtkPVXMLElement root
       │   └─> root->SetName("ServerManagerState")
       │   └─> root->AddAttribute("version", ParaViewVersion)
       │
       └─> vtkSMProxyManager::SaveState(root)
           │
           ▼
4. Server Manager
   vtkSMProxyManager::SaveState(vtkPVXMLElement* root)
       │
       ├─> Create ProxyCollection element
       │
       ├─> Iterate all proxies
       │   │
       │   ├─> vtkSMProxyIterator::Begin() / Next()
       │   │
       │   └─> For each proxy:
       │       │
       │       └─> vtkSMProxy::SaveState(element, id)
       │           │
       │           ├─> Serialize proxy metadata
       │           │   ├─> id, name, group, type
       │           │   │
       │           ├─> Iterate all properties
       │           │   │
       │           │   └─> vtkSMProperty::SaveState(propElement)
       │           │       │
       │           │       ├─> vtkSMIntVectorProperty::SaveState()
       │           │       ├─> vtkSMDoubleVectorProperty::SaveState()
       │           │       ├─> vtkSMStringVectorProperty::SaveState()
       │           │       ├─> vtkSMInputProperty::SaveState()
       │           │       │   └─> Resolve input proxy ID
       │           │       └─> ... (other property types)
       │           │
       │           └─> Add property elements to proxy element
       │
       ├─> Serialize ViewCollection
       │   │
       │   └─> pqMultiViewWidget::saveState(viewCollection)
       │       │
       │       ├─> Iterate all views
       │       │   │
       │       │   └─> For each view:
       │       │       ├─> vtkSMView::SaveState(viewElement)
       │       │       │   ├─> Serialize camera state
       │       │       │   ├─> Serialize viewport geometry
       │       │       │   └─> Serialize representation IDs
       │       │       │
       │       │       └─> pqView::saveState()  [Client-side view state]
       │       │
       │       └─> Add view elements to ViewCollection
       │
       ├─> Serialize LayoutCollection
       │   │
       │   └─> pqMultiViewWidget::saveLayoutState(layoutCollection)
       │       │
       │       ├─> Serialize layout tree
       │       │   ├─> Recursively serialize split nodes
       │       │   └─> Serialize view assignments
       │       │
       │       └─> Add layout element to LayoutCollection
       │
       └─> Write XML to file
           │
           └─> root->SaveFile(filename)
```

#### Detailed Method Signatures

```cpp
// UI Layer
class pqSaveStateReaction : public pqReaction
{
public:
    static void saveState();
    // Shows file dialog, calls pqApplicationCore::saveState()
};

// Application Core
class pqApplicationCore
{
public:
    void saveState(const QString& filename);
    // Coordinates state saving, creates XML root
};

// Server Manager
class vtkSMProxyManager
{
public:
    void SaveState(vtkPVXMLElement* root);
    // Serializes all proxies, views, layouts
};

class vtkSMProxy
{
public:
    void SaveState(vtkPVXMLElement* element, int id);
    // Serializes proxy properties
};

class vtkSMProperty
{
public:
    virtual void SaveState(vtkPVXMLElement* element);
    // Base class for property serialization
};

// View Layer
class pqMultiViewWidget
{
public:
    void saveState(vtkPVXMLElement* viewCollection);
    // Serializes all views
    void saveLayoutState(vtkPVXMLElement* layoutCollection);
    // Serializes layout structure
};
```

### 4.2 Load State Call Stack

#### High-Level Flow

```
1. User Action
   └─> File Menu → "Load State"
       │
       ▼
2. UI Layer
   pqLoadStateReaction::loadState()
       │
       ├─> QFileDialog::getOpenFileName()  [Get filename from user]
       │
       └─> pqApplicationCore::loadState(filename)
           │
           ▼
3. Application Core
   pqApplicationCore::loadState(const QString& filename)
       │
       ├─> Create vtkPVXMLParser
       │
       └─> vtkPVXMLParser::Parse(filename)
           │
           └─> vtkSMProxyManager::LoadState(rootElement)
               │
               ▼
4. Server Manager
   vtkSMProxyManager::LoadState(vtkPVXMLElement* root)
       │
       ├─> Check version compatibility
       │   └─> root->GetAttribute("version")
       │
       ├─> Parse ProxyCollection
       │   │
       │   ├─> root->FindNestedElementByName("ProxyCollection")
       │   │
       │   └─> For each Item element:
       │       │
       │       ├─> Extract proxy metadata
       │       │   ├─> id, name, group, type
       │       │
       │       ├─> Create proxy
       │       │   └─> vtkSMProxyManager::NewProxy(group, type)
       │       │       │
       │       │       └─> vtkSMProxyManager::GetProxyDefinitionManager()
       │       │           └─> Load XML definition for proxy type
       │       │
       │       ├─> Register proxy with ID
       │       │   └─> proxyManager->RegisterProxy(id, proxy)
       │       │
       │       └─> Load proxy state
       │           └─> vtkSMProxy::LoadState(itemElement)
       │               │
       │               ├─> Parse property elements
       │               │   │
       │               │   └─> For each Property element:
       │               │       │
       │               │       ├─> Get property by name
       │               │       │   └─> proxy->GetProperty(propName)
       │               │       │
       │               │       └─> vtkSMProperty::LoadState(propElement)
       │               │           │
       │               │           ├─> vtkSMIntVectorProperty::LoadState()
       │               │           ├─> vtkSMDoubleVectorProperty::LoadState()
       │               │           ├─> vtkSMStringVectorProperty::LoadState()
       │               │           ├─> vtkSMInputProperty::LoadState()
       │               │           │   └─> Resolve input proxy ID
       │               │           │       └─> proxyManager->GetProxy(id)
       │               │           └─> ... (other property types)
       │               │
       │               └─> Set property values
       │
       ├─> Parse ViewCollection
       │   │
       │   ├─> root->FindNestedElementByName("ViewCollection")
       │   │
       │   └─> For each View element:
       │       │
       │       ├─> Get view proxy by ID
       │       │   └─> proxyManager->GetProxy(viewId)
       │       │
       │       └─> Restore view configuration
       │           └─> vtkSMView::LoadState(viewElement)
       │               ├─> Restore camera state
       │               ├─> Restore viewport geometry
       │               └─> Restore representation assignments
       │
       ├─> Parse LayoutCollection
       │   │
       │   ├─> root->FindNestedElementByName("LayoutCollection")
       │   │
       │   └─> pqMultiViewWidget::loadLayoutState(layoutElement)
       │       │
       │       ├─> Parse layout tree
       │       │   ├─> Recursively parse split nodes
       │       │   └─> Parse view assignments
       │       │
       │       └─> Reconstruct layout
       │           ├─> Create split panes
       │           └─> Assign views to viewports
       │
       └─> Emit stateLoaded() signal
           │
           ▼
5. Application Core (Signal Handler)
   pqApplicationCore::onStateLoaded()
       │
       ├─> Update Pipeline Browser
       │   └─> pqPipelineBrowser::update()
       │
       ├─> Update Properties Panel
       │   └─> pqPropertiesPanel::update()
       │
       ├─> Update Views
       │   └─> pqViewManager::updateViews()
       │
       └─> Trigger pipeline execution
           └─> vtkSMProxy::UpdatePipeline()
               │
               └─> Execute VTK pipeline to generate data
```

#### Detailed Method Signatures

```cpp
// UI Layer
class pqLoadStateReaction : public pqReaction
{
public:
    static void loadState();
    // Shows file dialog, calls pqApplicationCore::loadState()
};

// Application Core
class pqApplicationCore
{
public:
    void loadState(const QString& filename);
    // Coordinates state loading, parses XML
    void onStateLoaded();  // Signal handler for post-load updates
};

// Server Manager
class vtkSMProxyManager
{
public:
    void LoadState(vtkPVXMLElement* root);
    // Deserializes all proxies, views, layouts
    vtkSMProxy* NewProxy(const char* group, const char* type);
    // Creates new proxy from XML definition
    void RegisterProxy(int id, vtkSMProxy* proxy);
    // Registers proxy with ID mapping
    vtkSMProxy* GetProxy(int id);
    // Gets proxy by ID
};

class vtkSMProxy
{
public:
    void LoadState(vtkPVXMLElement* element);
    // Deserializes proxy properties
};

class vtkSMProperty
{
public:
    virtual void LoadState(vtkPVXMLElement* element);
    // Base class for property deserialization
};

// View Layer
class pqMultiViewWidget
{
public:
    void loadLayoutState(vtkPVXMLElement* layoutElement);
    // Deserializes and reconstructs layout
};
```

---

## 5. View/Layout Specifics: Multi-View Layouts and Chart Views

### 5.1 Multi-View Layout Capture

#### Layout Tree Structure

ParaView supports complex multi-view layouts with split panes:

```
Layout Example:
┌─────────────────────────────────────┐
│         Main Layout                 │
│  ┌──────────┬──────────┐            │
│  │          │          │            │
│  │  View 1  │  View 2  │            │
│  │ (Render) │ (Chart)  │            │
│  │          │          │            │
│  └──────────┴──────────┘            │
│  ┌──────────────────────┐          │
│  │                      │          │
│  │      View 3          │          │
│  │     (Render)         │          │
│  │                      │          │
│  └──────────────────────┘          │
└─────────────────────────────────────┘
```

#### Layout Serialization

```cpp
// Layout tree serialization
void pqMultiViewWidget::saveLayoutState(vtkPVXMLElement* layoutCollection)
{
    vtkPVXMLElement* layoutElem = vtkPVXMLElement::New();
    layoutElem->SetName("Layout");
    layoutElem->AddAttribute("name", "MainLayout");

    // Serialize layout tree recursively
    this->serializeLayoutNode(this->rootLayoutNode, layoutElem);

    layoutCollection->AddNestedElement(layoutElem);
}

void pqMultiViewWidget::serializeLayoutNode(
    LayoutNode* node, vtkPVXMLElement* parent)
{
    if (node->type == LayoutNode::Split)
    {
        // Serialize split node
        vtkPVXMLElement* splitElem = vtkPVXMLElement::New();
        splitElem->SetName("Split");
        splitElem->AddAttribute("direction",
            node->split.isHorizontal ? "horizontal" : "vertical");
        splitElem->AddAttribute("ratio", node->split.splitRatio);

        // Recursively serialize children
        this->serializeLayoutNode(node->split.leftChild, splitElem);
        this->serializeLayoutNode(node->split.rightChild, splitElem);

        parent->AddNestedElement(splitElem);
    }
    else if (node->type == LayoutNode::View)
    {
        // Serialize view node
        vtkPVXMLElement* viewElem = vtkPVXMLElement::New();
        viewElem->SetName("View");
        viewElem->AddAttribute("id", node->view.viewId);

        parent->AddNestedElement(viewElem);
    }
}
```

#### Layout XML Structure

```xml
<LayoutCollection>
  <Layout name="MainLayout">
    <Split direction="horizontal" ratio="0.5">
      <View id="3"/>  <!-- RenderView1 -->
      <View id="4"/>  <!-- ChartView1 -->
    </Split>
    <Split direction="vertical" ratio="0.3">
      <View id="5"/>  <!-- RenderView2 -->
    </Split>
  </Layout>
</LayoutCollection>
```

### 5.2 Render View Capture and Restoration

#### Render View State

```cpp
// Render view state structure
struct RenderViewState {
    // Camera state
    double cameraPosition[3];
    double cameraFocalPoint[3];
    double cameraViewUp[3];
    double cameraViewAngle;

    // Viewport geometry
    int viewportX, viewportY;
    int viewportWidth, viewportHeight;

    // View properties
    double background[3];
    bool useGradientBackground;
    double gradientBackground[2][3];

    // Representation IDs
    std::vector<int> representationIds;

    // Interaction settings
    bool interactorStyle;  // Trackball, etc.
};
```

#### Render View Serialization

```cpp
void vtkSMRenderView::SaveState(vtkPVXMLElement* element)
{
    // Save camera state
    vtkCamera* camera = this->GetActiveCamera();

    vtkPVXMLElement* cameraElem = vtkPVXMLElement::New();
    cameraElem->SetName("Camera");

    double pos[3], focal[3], up[3];
    camera->GetPosition(pos);
    camera->GetFocalPoint(focal);
    camera->GetViewUp(up);

    cameraElem->AddAttribute("Position",
        this->VectorToString(pos, 3).c_str());
    cameraElem->AddAttribute("FocalPoint",
        this->VectorToString(focal, 3).c_str());
    cameraElem->AddAttribute("ViewUp",
        this->VectorToString(up, 3).c_str());
    cameraElem->AddAttribute("ViewAngle", camera->GetViewAngle());

    element->AddNestedElement(cameraElem);

    // Save viewport geometry
    int* viewSize = this->GetViewSize();
    vtkPVXMLElement* viewportElem = vtkPVXMLElement::New();
    viewportElem->SetName("Viewport");
    viewportElem->AddAttribute("x", 0);
    viewportElem->AddAttribute("y", 0);
    viewportElem->AddAttribute("width", viewSize[0]);
    viewportElem->AddAttribute("height", viewSize[1]);
    element->AddNestedElement(viewportElem);

    // Save background
    double* bg = this->GetBackground();
    element->AddAttribute("Background",
        this->VectorToString(bg, 3).c_str());

    // Save representation IDs
    vtkPVXMLElement* repsElem = vtkPVXMLElement::New();
    repsElem->SetName("Representations");
    std::stringstream ss;
    for (size_t i = 0; i < this->RepresentationIds.size(); i++)
    {
        if (i > 0) ss << " ";
        ss << this->RepresentationIds[i];
    }
    repsElem->AddAttribute("value", ss.str().c_str());
    element->AddNestedElement(repsElem);
}
```

#### Render View Restoration

```cpp
void vtkSMRenderView::LoadState(vtkPVXMLElement* element)
{
    // Restore camera state
    vtkPVXMLElement* cameraElem =
        element->FindNestedElementByName("Camera");
    if (cameraElem)
    {
        vtkCamera* camera = this->GetActiveCamera();

        double pos[3], focal[3], up[3];
        this->StringToVector(
            cameraElem->GetAttribute("Position"), pos, 3);
        this->StringToVector(
            cameraElem->GetAttribute("FocalPoint"), focal, 3);
        this->StringToVector(
            cameraElem->GetAttribute("ViewUp"), up, 3);

        camera->SetPosition(pos);
        camera->SetFocalPoint(focal);
        camera->SetViewUp(up);
        camera->SetViewAngle(
            atof(cameraElem->GetAttribute("ViewAngle")));
    }

    // Restore viewport geometry
    vtkPVXMLElement* viewportElem =
        element->FindNestedElementByName("Viewport");
    if (viewportElem)
    {
        int x = atoi(viewportElem->GetAttribute("x"));
        int y = atoi(viewportElem->GetAttribute("y"));
        int w = atoi(viewportElem->GetAttribute("width"));
        int h = atoi(viewportElem->GetAttribute("height"));
        this->SetViewport(x, y, w, h);
    }

    // Restore background
    const char* bgStr = element->GetAttribute("Background");
    if (bgStr)
    {
        double bg[3];
        this->StringToVector(bgStr, bg, 3);
        this->SetBackground(bg);
    }

    // Restore representations
    vtkPVXMLElement* repsElem =
        element->FindNestedElementByName("Representations");
    if (repsElem)
    {
        std::string repsStr = repsElem->GetAttribute("value");
        std::vector<int> repIds = this->ParseIntVector(repsStr);
        this->RepresentationIds = repIds;
    }

    // Trigger render update
    this->StillRender();
}
```

### 5.3 Chart View Capture and Restoration

#### Chart View State

```cpp
// Chart view state structure
struct ChartViewState {
    std::string chartType;  // "XYChartView", "BarChartView", etc.

    // Axis configurations
    std::string xAxisTitle;
    std::string yAxisTitle;
    bool xAxisLogScale;
    bool yAxisLogScale;
    double xAxisRange[2];
    double yAxisRange[2];

    // Chart properties
    std::string title;
    bool showLegend;
    std::string legendLocation;  // "top", "bottom", "left", "right"

    // Plot series
    std::vector<PlotSeriesState> plotSeries;
};

struct PlotSeriesState {
    int inputProxyId;        // Which pipeline output
    std::string xArrayName;  // X-axis array
    std::string yArrayName;  // Y-axis array
    int lineStyle;           // Solid, dashed, etc.
    int markerStyle;         // Circle, square, etc.
    double lineWidth;
    double markerSize;
    double color[3];
};
```

#### Chart View Serialization

```cpp
void vtkSMChartView::SaveState(vtkPVXMLElement* element)
{
    // Save chart type
    element->AddAttribute("chartType", this->GetChartType());

    // Save title
    element->AddAttribute("title", this->GetTitle());

    // Save axis configurations
    vtkPVXMLElement* xAxisElem = vtkPVXMLElement::New();
    xAxisElem->SetName("XAxis");
    xAxisElem->AddAttribute("title", this->GetXAxisTitle());
    xAxisElem->AddAttribute("logScale",
        this->GetXAxisLogScale() ? "1" : "0");
    double xRange[2];
    this->GetXAxisRange(xRange);
    xAxisElem->AddAttribute("range",
        this->VectorToString(xRange, 2).c_str());
    element->AddNestedElement(xAxisElem);

    vtkPVXMLElement* yAxisElem = vtkPVXMLElement::New();
    yAxisElem->SetName("YAxis");
    yAxisElem->AddAttribute("title", this->GetYAxisTitle());
    yAxisElem->AddAttribute("logScale",
        this->GetYAxisLogScale() ? "1" : "0");
    double yRange[2];
    this->GetYAxisRange(yRange);
    yAxisElem->AddAttribute("range",
        this->VectorToString(yRange, 2).c_str());
    element->AddNestedElement(yAxisElem);

    // Save legend
    vtkPVXMLElement* legendElem = vtkPVXMLElement::New();
    legendElem->SetName("Legend");
    legendElem->AddAttribute("show",
        this->GetShowLegend() ? "1" : "0");
    legendElem->AddAttribute("location",
        this->GetLegendLocation());
    element->AddNestedElement(legendElem);

    // Save plot series
    vtkPVXMLElement* seriesCollection = vtkPVXMLElement::New();
    seriesCollection->SetName("PlotSeriesCollection");

    for (size_t i = 0; i < this->PlotSeries.size(); i++)
    {
        PlotSeriesState& series = this->PlotSeries[i];

        vtkPVXMLElement* seriesElem = vtkPVXMLElement::New();
        seriesElem->SetName("PlotSeries");
        seriesElem->AddAttribute("inputId", series.inputProxyId);
        seriesElem->AddAttribute("xArray", series.xArrayName);
        seriesElem->AddAttribute("yArray", series.yArrayName);
        seriesElem->AddAttribute("lineStyle", series.lineStyle);
        seriesElem->AddAttribute("markerStyle", series.markerStyle);
        seriesElem->AddAttribute("lineWidth", series.lineWidth);
        seriesElem->AddAttribute("markerSize", series.markerSize);
        seriesElem->AddAttribute("color",
            this->VectorToString(series.color, 3).c_str());

        seriesCollection->AddNestedElement(seriesElem);
    }

    element->AddNestedElement(seriesCollection);
}
```

#### Chart View Restoration

```cpp
void vtkSMChartView::LoadState(vtkPVXMLElement* element)
{
    // Restore chart type
    const char* chartType = element->GetAttribute("chartType");
    if (chartType)
    {
        this->SetChartType(chartType);
    }

    // Restore title
    const char* title = element->GetAttribute("title");
    if (title)
    {
        this->SetTitle(title);
    }

    // Restore X axis
    vtkPVXMLElement* xAxisElem =
        element->FindNestedElementByName("XAxis");
    if (xAxisElem)
    {
        this->SetXAxisTitle(xAxisElem->GetAttribute("title"));
        this->SetXAxisLogScale(
            atoi(xAxisElem->GetAttribute("logScale")) != 0);
        double xRange[2];
        this->StringToVector(
            xAxisElem->GetAttribute("range"), xRange, 2);
        this->SetXAxisRange(xRange);
    }

    // Restore Y axis
    vtkPVXMLElement* yAxisElem =
        element->FindNestedElementByName("YAxis");
    if (yAxisElem)
    {
        this->SetYAxisTitle(yAxisElem->GetAttribute("title"));
        this->SetYAxisLogScale(
            atoi(yAxisElem->GetAttribute("logScale")) != 0);
        double yRange[2];
        this->StringToVector(
            yAxisElem->GetAttribute("range"), yRange, 2);
        this->SetYAxisRange(yRange);
    }

    // Restore legend
    vtkPVXMLElement* legendElem =
        element->FindNestedElementByName("Legend");
    if (legendElem)
    {
        this->SetShowLegend(
            atoi(legendElem->GetAttribute("show")) != 0);
        this->SetLegendLocation(
            legendElem->GetAttribute("location"));
    }

    // Restore plot series
    vtkPVXMLElement* seriesCollection =
        element->FindNestedElementByName("PlotSeriesCollection");
    if (seriesCollection)
    {
        this->PlotSeries.clear();

        int numSeries = seriesCollection->GetNumberOfNestedElements();
        for (int i = 0; i < numSeries; i++)
        {
            vtkPVXMLElement* seriesElem =
                seriesCollection->GetNestedElement(i);

            PlotSeriesState series;
            series.inputProxyId =
                atoi(seriesElem->GetAttribute("inputId"));
            series.xArrayName = seriesElem->GetAttribute("xArray");
            series.yArrayName = seriesElem->GetAttribute("yArray");
            series.lineStyle =
                atoi(seriesElem->GetAttribute("lineStyle"));
            series.markerStyle =
                atoi(seriesElem->GetAttribute("markerStyle"));
            series.lineWidth =
                atof(seriesElem->GetAttribute("lineWidth"));
            series.markerSize =
                atof(seriesElem->GetAttribute("markerSize"));
            this->StringToVector(
                seriesElem->GetAttribute("color"),
                series.color, 3);

            this->PlotSeries.push_back(series);
        }
    }

    // Trigger chart update
    this->Update();
}
```

### 5.4 Viewport and Split Pane Management

#### Viewport Assignment

```cpp
// Viewport assignment during layout restoration
void pqMultiViewWidget::assignViewToViewport(
    int viewId, const ViewportGeometry& viewport)
{
    // Get view proxy
    vtkSMProxy* viewProxy =
        this->proxyManager->GetProxy(viewId);

    // Create view frame
    pqViewFrame* frame = new pqViewFrame();
    frame->setViewProxy(viewProxy);

    // Set viewport geometry
    frame->setGeometry(
        viewport.x, viewport.y,
        viewport.width, viewport.height);

    // Add to layout
    this->addViewFrame(frame);
}
```

---

## 6. Extensibility: Custom Plugins and Versioning

### 6.1 Custom Plugin Integration

#### Registering Custom Filters/Views

Custom plugins integrate into state saving/loading through Server Manager XML registration:

```xml
<!-- CustomFilter.xml -->
<ServerManagerConfiguration>
  <ProxyGroup name="filters">
    <SourceProxy name="MyCustomFilter"
                 class="vtkMyCustomFilter"
                 label="My Custom Filter">

      <!-- Input property -->
      <InputProperty name="Input" command="SetInputConnection">
        <ProxyGroupDomain name="groups">
          <Group name="sources"/>
          <Group name="filters"/>
        </ProxyGroupDomain>
      </InputProperty>

      <!-- Custom parameters -->
      <DoubleVectorProperty
          name="CustomParameter"
          command="SetCustomParameter"
          number_of_elements="1"
          default_values="1.0">
      </DoubleVectorProperty>

      <!-- State saving hints -->
      <Hints>
        <StateSaveState/>  <!-- Include in state files -->
      </Hints>

    </SourceProxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

#### Plugin State Serialization

When a custom filter is registered, its state is automatically included in state files:

```cpp
// Custom filter state is serialized automatically
// No special code needed - uses standard proxy serialization

// When state is saved:
// 1. vtkSMProxyManager iterates all proxies (including custom ones)
// 2. Custom proxy's SaveState() is called
// 3. All properties (including custom ones) are serialized
// 4. Custom proxy appears in .pvsm file like any other proxy

// When state is loaded:
// 1. XML parser encounters custom proxy definition
// 2. vtkSMProxyManager::NewProxy("filters", "MyCustomFilter") is called
// 3. Custom proxy is created from XML definition
// 4. Custom properties are restored
```

#### Custom View Integration

```xml
<!-- CustomView.xml -->
<ServerManagerConfiguration>
  <ProxyGroup name="views">
    <Proxy name="MyCustomView"
           class="vtkSMCustomView"
           base_proxygroup="views"
           base_proxyname="RenderView">

      <!-- Custom view properties -->
      <DoubleVectorProperty
          name="CustomViewSetting"
          command="SetCustomViewSetting"
          number_of_elements="1"
          default_values="0.5">
      </DoubleVectorProperty>

    </Proxy>
  </ProxyGroup>
</ServerManagerConfiguration>
```

### 6.2 Versioning and Backward Compatibility

#### Version Information in State Files

```xml
<ServerManagerState version="5.11.0">
  <!-- State file content -->
</ServerManagerState>
```

#### Version Checking During Load

```cpp
void vtkSMProxyManager::LoadState(vtkPVXMLElement* root)
{
    // Get version from state file
    const char* stateVersion = root->GetAttribute("version");

    // Get current ParaView version
    const char* currentVersion = ParaViewVersion::GetVersion();

    // Compare versions
    if (this->isVersionCompatible(stateVersion, currentVersion))
    {
        // Load state normally
        this->loadStateInternal(root);
    }
    else
    {
        // Apply version migration
        vtkPVXMLElement* migratedRoot =
            this->migrateStateFile(root, stateVersion, currentVersion);
        this->loadStateInternal(migratedRoot);
    }
}

bool vtkSMProxyManager::isVersionCompatible(
    const char* stateVersion, const char* currentVersion)
{
    // Parse version strings (e.g., "5.11.0")
    int stateMajor, stateMinor, statePatch;
    int currentMajor, currentMinor, currentPatch;

    this->parseVersion(stateVersion, stateMajor, stateMinor, statePatch);
    this->parseVersion(currentVersion, currentMajor, currentMinor, currentPatch);

    // Compatible if major version matches
    return (stateMajor == currentMajor);
}
```

#### State File Migration

```cpp
vtkPVXMLElement* vtkSMProxyManager::migrateStateFile(
    vtkPVXMLElement* root,
    const char* fromVersion,
    const char* toVersion)
{
    vtkPVXMLElement* migrated = root->DeepCopy();

    // Apply migration rules based on version differences
    if (this->needsMigration(fromVersion, toVersion))
    {
        // Example: Rename deprecated property names
        this->renameProperties(migrated,
            "OldPropertyName", "NewPropertyName");

        // Example: Convert property values
        this->convertPropertyValues(migrated,
            "PropertyName", oldFormat, newFormat);

        // Example: Add default values for new properties
        this->addDefaultProperties(migrated);
    }

    return migrated;
}
```

#### Deprecation Handling

```cpp
// In proxy XML definition
<DoubleVectorProperty
    name="OldParameter"
    command="SetParameter"
    deprecated="1"
    deprecated_in="5.11.0"
    replacement="NewParameter">
  <!-- Property definition -->
</DoubleVectorProperty>

// During state loading
void vtkSMProxy::LoadState(vtkPVXMLElement* element)
{
    // Check for deprecated properties
    vtkPVXMLElement* propElem =
        element->FindNestedElementByName("Property");

    if (propElem)
    {
        const char* propName = propElem->GetAttribute("name");

        // Check if property is deprecated
        if (this->isPropertyDeprecated(propName))
        {
            // Migrate to new property name
            const char* newName =
                this->getPropertyReplacement(propName);

            // Load value into new property
            vtkSMProperty* newProp = this->GetProperty(newName);
            if (newProp)
            {
                newProp->LoadState(propElem);
            }
        }
        else
        {
            // Load normally
            vtkSMProperty* prop = this->GetProperty(propName);
            if (prop)
            {
                prop->LoadState(propElem);
            }
        }
    }
}
```

### 6.3 Extensibility Points

#### Hooks for Custom State Saving

```cpp
// Custom proxy can override SaveState() for special handling
class vtkSMCustomProxy : public vtkSMProxy
{
public:
    void SaveState(vtkPVXMLElement* element, int id) override
    {
        // Call base class to save standard properties
        this->Superclass::SaveState(element, id);

        // Add custom state
        vtkPVXMLElement* customElem = vtkPVXMLElement::New();
        customElem->SetName("CustomState");
        customElem->AddAttribute("customData", this->CustomData);
        element->AddNestedElement(customElem);
    }

    void LoadState(vtkPVXMLElement* element) override
    {
        // Call base class to load standard properties
        this->Superclass::LoadState(element);

        // Load custom state
        vtkPVXMLElement* customElem =
            element->FindNestedElementByName("CustomState");
        if (customElem)
        {
            this->CustomData = customElem->GetAttribute("customData");
        }
    }
};
```

#### Property Exclusion from State

```cpp
// Mark property as DONT_SAVE_STATE
vtkSMProperty* prop = proxy->GetProperty("TemporaryProperty");
prop->GetInformation()->Set(vtkSMProperty::DONT_SAVE_STATE(), 1);

// Property will be skipped during SaveState()
```

---

## 7. Key Source Files and Classes

### 7.1 Core State Management

#### Server Manager Layer

**Files**:

- `ParaView/ServerManager/Core/vtkSMProxyManager.cxx`
- `ParaView/ServerManager/Core/vtkSMProxyManager.h`

  - Central proxy registry and state coordinator
  - `SaveState()` / `LoadState()` methods

- `ParaView/ServerManager/Core/vtkSMProxy.cxx`
- `ParaView/ServerManager/Core/vtkSMProxy.h`

  - Individual proxy state management
  - `SaveState()` / `LoadState()` methods

- `ParaView/ServerManager/Core/vtkSMProperty.cxx`
- `ParaView/ServerManager/Core/vtkSMProperty.h`
  - Base class for all properties
  - Property serialization/deserialization

**Key Classes**:

```cpp
class vtkSMProxyManager {
    // State management
    void SaveState(vtkPVXMLElement* root);
    void LoadState(vtkPVXMLElement* root);

    // Proxy management
    vtkSMProxy* NewProxy(const char* group, const char* type);
    void RegisterProxy(int id, vtkSMProxy* proxy);
    vtkSMProxy* GetProxy(int id);
};

class vtkSMProxy {
    // State serialization
    void SaveState(vtkPVXMLElement* element, int id);
    void LoadState(vtkPVXMLElement* element);

    // Property access
    vtkSMProperty* GetProperty(const char* name);
};
```

#### XML Serialization

**Files**:

- `ParaView/ServerManager/Core/vtkPVXMLElement.cxx`
- `ParaView/ServerManager/Core/vtkPVXMLElement.h`

  - XML element representation
  - `SaveFile()` / `LoadFile()` methods

- `ParaView/ServerManager/Core/vtkPVXMLParser.cxx`
- `ParaView/ServerManager/Core/vtkPVXMLParser.h`
  - XML parsing for state files

**Key Classes**:

```cpp
class vtkPVXMLElement {
    void SaveFile(const char* filename);
    void AddNestedElement(vtkPVXMLElement* elem);
    vtkPVXMLElement* FindNestedElementByName(const char* name);
    const char* GetAttribute(const char* name);
    void AddAttribute(const char* name, const char* value);
};

class vtkPVXMLParser {
    void Parse(const char* filename);
    vtkPVXMLElement* GetRootElement();
};
```

### 7.2 Application Layer

#### UI Actions

**Files**:

- `ParaView/Qt/ApplicationComponents/pqSaveStateReaction.cxx`
- `ParaView/Qt/ApplicationComponents/pqSaveStateReaction.h`

  - "Save State" menu action handler

- `ParaView/Qt/ApplicationComponents/pqLoadStateReaction.cxx`
- `ParaView/Qt/ApplicationComponents/pqLoadStateReaction.h`
  - "Load State" menu action handler

**Key Classes**:

```cpp
class pqSaveStateReaction : public pqReaction {
    static void saveState();
};

class pqLoadStateReaction : public pqReaction {
    static void loadState();
};
```

#### Application Core

**Files**:

- `ParaView/Qt/ApplicationComponents/pqApplicationCore.cxx`
- `ParaView/Qt/ApplicationComponents/pqApplicationCore.h`
  - Central application state coordinator
  - Coordinates state save/load operations

**Key Classes**:

```cpp
class pqApplicationCore {
    void saveState(const QString& filename);
    void loadState(const QString& filename);

signals:
    void stateLoaded();  // Emitted after state load
};
```

### 7.3 View Layer

#### View Proxies

**Files**:

- `ParaView/ServerManager/Rendering/vtkSMRenderViewProxy.cxx`
- `ParaView/ServerManager/Rendering/vtkSMRenderViewProxy.h`

  - 3D render view state management

- `ParaView/ServerManager/Charting/vtkSMChartViewProxy.cxx`
- `ParaView/ServerManager/Charting/vtkSMChartViewProxy.h`
  - 2D chart view state management

**Key Classes**:

```cpp
class vtkSMRenderViewProxy : public vtkSMViewProxy {
    void SaveState(vtkPVXMLElement* element);
    void LoadState(vtkPVXMLElement* element);

    // Camera management
    vtkCamera* GetActiveCamera();
    void SetCameraPosition(double pos[3]);
    void SetCameraFocalPoint(double focal[3]);
};

class vtkSMChartViewProxy : public vtkSMViewProxy {
    void SaveState(vtkPVXMLElement* element);
    void LoadState(vtkPVXMLElement* element);

    // Chart configuration
    void SetXAxisTitle(const char* title);
    void SetYAxisTitle(const char* title);
    void SetShowLegend(bool show);
};
```

#### Client-Side Views

**Files**:

- `ParaView/Qt/Components/pqRenderView.cxx`
- `ParaView/Qt/Components/pqRenderView.h`

  - Client-side 3D render view wrapper

- `ParaView/Qt/Components/pqChartView.cxx`
- `ParaView/Qt/Components/pqChartView.h`
  - Client-side chart view wrapper

**Key Classes**:

```cpp
class pqRenderView : public pqView {
    void saveState(vtkPVXMLElement* element);
    void loadState(vtkPVXMLElement* element);
};

class pqChartView : public pqView {
    void saveState(vtkPVXMLElement* element);
    void loadState(vtkPVXMLElement* element);
};
```

### 7.4 Layout Management

**Files**:

- `ParaView/Qt/Components/pqMultiViewWidget.cxx`
- `ParaView/Qt/Components/pqMultiViewWidget.h`
  - Multi-view layout manager
  - Layout tree serialization/deserialization

**Key Classes**:

```cpp
class pqMultiViewWidget {
    void saveState(vtkPVXMLElement* viewCollection);
    void saveLayoutState(vtkPVXMLElement* layoutCollection);
    void loadLayoutState(vtkPVXMLElement* layoutElement);

    // Layout tree management
    LayoutNode* rootLayoutNode;
    void serializeLayoutNode(LayoutNode* node, vtkPVXMLElement* parent);
    LayoutNode* deserializeLayoutNode(vtkPVXMLElement* element);
};
```

### 7.5 Representation Management

**Files**:

- `ParaView/ServerManager/Rendering/vtkSMRepresentationProxy.cxx`
- `ParaView/ServerManager/Rendering/vtkSMRepresentationProxy.h`
  - Representation state management

**Key Classes**:

```cpp
class vtkSMRepresentationProxy : public vtkSMProxy {
    void SaveState(vtkPVXMLElement* element, int id);
    void LoadState(vtkPVXMLElement* element);

    // Representation configuration
    void SetRepresentationType(const char* type);
    void SetColorArrayName(const char* name);
    void SetOpacity(double opacity);
    void SetVisibility(bool visible);
};
```

---

## 8. Architecture Diagram (Text Description)

### 8.1 Component Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    USER INTERFACE LAYER                         │
│  ┌──────────────────┐  ┌──────────────────┐                  │
│  │ pqSaveState       │  │ pqLoadState      │                  │
│  │ Reaction          │  │ Reaction         │                  │
│  │ (File Menu)       │  │ (File Menu)      │                  │
│  └────────┬─────────┘  └────────┬─────────┘                  │
│           │                      │                              │
│           └──────────┬───────────┘                              │
│                      │                                          │
│                      ▼                                          │
│           ┌──────────────────────┐                             │
│           │ pqApplicationCore    │                             │
│           │ - saveState()         │                             │
│           │ - loadState()         │                             │
│           │ - stateLoaded() signal│                             │
│           └──────────┬───────────┘                             │
└──────────────────────┼──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                 SERVER MANAGER LAYER                            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │           vtkSMProxyManager                              │  │
│  │  - Central proxy registry                                │  │
│  │  - SaveState() / LoadState()                             │  │
│  │  - Proxy ID mapping                                       │  │
│  │                                                           │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │  │
│  │  │ vtkSMProxy   │  │ vtkSMProxy   │  │ vtkSMProxy   │  │  │
│  │  │ (Source)     │  │ (Filter)     │  │ (View)       │  │  │
│  │  │              │  │              │  │              │  │  │
│  │  │ SaveState()  │  │ SaveState()  │  │ SaveState()  │  │  │
│  │  │ LoadState()  │  │ LoadState()  │  │ LoadState()  │  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │  │
│  │                                                           │  │
│  │  ┌──────────────┐  ┌──────────────┐                      │  │
│  │  │ vtkSMProperty │  │ vtkSMProperty│                      │  │
│  │  │ (Properties)  │  │ (Properties) │                      │  │
│  │  │               │  │              │                      │  │
│  │  │ SaveState()   │  │ SaveState()  │                      │  │
│  │  │ LoadState()   │  │ LoadState()  │                      │  │
│  │  └──────────────┘  └──────────────┘                      │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    XML SERIALIZATION LAYER                       │
│  ┌──────────────────┐  ┌──────────────────┐                  │
│  │ vtkPVXMLElement  │  │ vtkPVXMLParser   │                  │
│  │ - SaveFile()     │  │ - Parse()        │                  │
│  │ - AddNested()    │  │ - GetRoot()      │                  │
│  └──────────────────┘  └──────────────────┘                  │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                      STATE FILE (.pvsm)                          │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ <ServerManagerState version="5.11.0">                    │  │
│  │   <ProxyCollection>                                       │  │
│  │     <Item id="0" name="..." group="..." type="...">      │  │
│  │       <Property name="..." value="..."/>                 │  │
│  │     </Item>                                               │  │
│  │   </ProxyCollection>                                       │  │
│  │   <ViewCollection>...</ViewCollection>                   │  │
│  │   <LayoutCollection>...</LayoutCollection>                │  │
│  │ </ServerManagerState>                                     │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 View and Layout Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    VIEW MANAGEMENT LAYER                         │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              pqMultiViewWidget                            │  │
│  │  - Layout tree management                                 │  │
│  │  - Viewport assignment                                    │  │
│  │  - saveLayoutState() / loadLayoutState()                   │  │
│  │                                                           │  │
│  │  ┌────────────────────────────────────────────────────┐  │  │
│  │  │              Layout Tree                           │  │  │
│  │  │                                                    │  │  │
│  │  │              Split (horizontal, 0.5)              │  │  │
│  │  │              /              \                     │  │  │
│  │  │         View (id=3)      View (id=4)              │  │  │
│  │  │      (RenderView)      (ChartView)                 │  │  │
│  │  └────────────────────────────────────────────────────┘  │  │
│  │                                                           │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │  │
│  │  │ pqRenderView │  │ pqChartView │  │ pqViewFrame │  │  │
│  │  │              │  │             │  │             │  │  │
│  │  │ saveState()  │  │ saveState() │  │ setGeometry│  │  │
│  │  │ loadState()  │  │ loadState() │  │             │  │  │
│  │  └──────┬───────┘  └──────┬──────┘  └──────┬──────┘  │  │
│  │         │                  │                 │          │  │
│  │         └──────────────────┼─────────────────┘          │  │
│  │                            │                            │  │
│  │         ┌───────────────────▼───────────────────┐      │  │
│  │         │    vtkSMViewProxy (Server-side)       │      │  │
│  │         │    - SaveState() / LoadState()        │      │  │
│  │         │    - Camera state                     │      │  │
│  │         │    - Viewport geometry               │      │  │
│  │         └──────────────────────────────────────┘      │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 8.3 State Flow Architecture

#### Save State Flow

```
User Action (Save State)
    │
    ▼
pqSaveStateReaction::saveState()
    │
    ▼
pqApplicationCore::saveState(filename)
    │
    ├─> Create XML root element
    │
    └─> vtkSMProxyManager::SaveState(root)
        │
        ├─> Serialize ProxyCollection
        │   │
        │   └─> For each proxy:
        │       │
        │       └─> vtkSMProxy::SaveState(element, id)
        │           │
        │           └─> For each property:
        │               │
        │               └─> vtkSMProperty::SaveState(propElement)
        │
        ├─> Serialize ViewCollection
        │   │
        │   └─> pqMultiViewWidget::saveState(viewCollection)
        │       │
        │       └─> For each view:
        │           │
        │           └─> vtkSMViewProxy::SaveState(viewElement)
        │
        └─> Serialize LayoutCollection
            │
            └─> pqMultiViewWidget::saveLayoutState(layoutCollection)
                │
                └─> Recursively serialize layout tree
        │
        └─> Write XML to file
            │
            └─> root->SaveFile(filename)
```

#### Load State Flow

```
User Action (Load State)
    │
    ▼
pqLoadStateReaction::loadState()
    │
    ▼
pqApplicationCore::loadState(filename)
    │
    ├─> Parse XML file
    │   │
    │   └─> vtkPVXMLParser::Parse(filename)
    │
    └─> vtkSMProxyManager::LoadState(rootElement)
        │
        ├─> Check version compatibility
        │
        ├─> Parse ProxyCollection
        │   │
        │   └─> For each Item element:
        │       │
        │       ├─> Create proxy
        │       │   └─> vtkSMProxyManager::NewProxy(group, type)
        │       │
        │       ├─> Register proxy with ID
        │       │   └─> proxyManager->RegisterProxy(id, proxy)
        │       │
        │       └─> Load proxy state
        │           └─> vtkSMProxy::LoadState(itemElement)
        │               │
        │               └─> For each Property element:
        │                   │
        │                   └─> vtkSMProperty::LoadState(propElement)
        │
        ├─> Parse ViewCollection
        │   │
        │   └─> For each View element:
        │       │
        │       ├─> Get view proxy by ID
        │       │   └─> proxyManager->GetProxy(viewId)
        │       │
        │       └─> Restore view configuration
        │           └─> vtkSMViewProxy::LoadState(viewElement)
        │
        └─> Parse LayoutCollection
            │
            └─> pqMultiViewWidget::loadLayoutState(layoutElement)
                │
                └─> Reconstruct layout tree
                    │
                    ├─> Create split panes
                    └─> Assign views to viewports
        │
        └─> Emit stateLoaded() signal
            │
            ▼
        pqApplicationCore::onStateLoaded()
            │
            ├─> Update Pipeline Browser
            ├─> Update Properties Panel
            ├─> Update Views
            └─> Trigger pipeline execution
```

---

## 9. Mapping ParaView Concepts to "Tracks/Subcharts/Curves" Style UIs

### 9.1 Conceptual Mapping

#### ParaView → Tracks/Subcharts/Curves Model

| ParaView Concept   | Tracks/Subcharts/Curves Equivalent | Description                                            |
| ------------------ | ---------------------------------- | ------------------------------------------------------ |
| **Pipeline**       | **Track**                          | A sequence of data processing steps (source → filters) |
| **Source**         | **Track Source**                   | Data input (file reader, procedural source)            |
| **Filter**         | **Track Processor**                | Data transformation step in the track                  |
| **View**           | **Subchart**                       | A visualization panel displaying data                  |
| **Representation** | **Curve/Plot**                     | Visual representation of data within a subchart        |
| **Layout**         | **Subchart Arrangement**           | How subcharts are arranged (split panes)               |
| **Color Map**      | **Curve Styling**                  | Color, line style, marker style for curves             |
| **Camera**         | **Subchart Viewport**              | Zoom, pan, view angle within a subchart                |

### 9.2 Detailed Mapping

#### Pipeline as Track

```
ParaView Pipeline:
Wavelet1 (Source)
    │
    └─> Contour1 (Filter)
            │
            └─> Shrink1 (Filter)

Tracks/Subcharts/Curves Equivalent:
Track: "Data Processing"
    Source: Wavelet1
    Processor: Contour1
    Processor: Shrink1
```

#### View as Subchart

```
ParaView View:
RenderView1
    - Camera position
    - Viewport geometry
    - Background color
    - Representations: [Contour1Representation]

Tracks/Subcharts/Curves Equivalent:
Subchart: "3D Visualization"
    Viewport: (x, y, width, height)
    Camera: (position, focal, up)
    Curves: [Contour1Curve]
```

#### Representation as Curve

```
ParaView Representation:
Contour1Representation
    - Input: Shrink1 (filter output)
    - Type: Surface
    - Color: RTData (scalar field)
    - Opacity: 1.0
    - Visibility: true

Tracks/Subcharts/Curves Equivalent:
Curve: "Contour1"
    Data Source: Shrink1 output
    Style: Surface
    Color: Mapped to RTData field
    Opacity: 1.0
    Visible: true
```

#### Layout as Subchart Arrangement

```
ParaView Layout:
Split (horizontal, 0.5)
    ├─> RenderView1 (left)
    └─> ChartView1 (right)

Tracks/Subcharts/Curves Equivalent:
Subchart Arrangement:
    Split (horizontal, 0.5)
        ├─> Subchart: "3D View" (left)
        └─> Subchart: "Chart View" (right)
```

### 9.3 State Persistence Mapping

#### ParaView State File Structure

```xml
<ServerManagerState>
  <ProxyCollection>
    <!-- Track definition -->
    <Item id="0" name="Wavelet1" group="sources" type="Wavelet">
      <!-- Track source parameters -->
    </Item>
    <Item id="1" name="Contour1" group="filters" type="Contour">
      <!-- Track processor parameters -->
      <Property name="Input" value="0"/>  <!-- Track connection -->
    </Item>

    <!-- Curve definition -->
    <Item id="2" name="Contour1Representation" group="representations">
      <!-- Curve styling -->
      <Property name="Input" value="1"/>  <!-- Curve data source -->
      <Property name="ColorArrayName" value="RTData"/>  <!-- Curve color -->
    </Item>

    <!-- Subchart definition -->
    <Item id="3" name="RenderView1" group="views" type="RenderView">
      <!-- Subchart viewport and camera -->
      <Property name="Representations" value="2"/>  <!-- Curves in subchart -->
    </Item>
  </ProxyCollection>

  <!-- Subchart arrangement -->
  <LayoutCollection>
    <Layout name="MainLayout">
      <Split direction="horizontal" ratio="0.5">
        <View id="3"/>  <!-- Subchart assignment -->
      </Split>
    </Layout>
  </LayoutCollection>
</ServerManagerState>
```

#### Equivalent Tracks/Subcharts/Curves State Structure

```json
{
  "version": "1.0",
  "tracks": [
    {
      "id": 0,
      "name": "Data Processing",
      "source": {
        "type": "Wavelet",
        "parameters": {
          /* ... */
        }
      },
      "processors": [
        {
          "id": 1,
          "type": "Contour",
          "input": 0,
          "parameters": {
            /* ... */
          }
        }
      ]
    }
  ],
  "subcharts": [
    {
      "id": 3,
      "type": "RenderView",
      "viewport": {
        "x": 0,
        "y": 0,
        "width": 0.5,
        "height": 1.0
      },
      "camera": {
        /* ... */
      },
      "curves": [
        {
          "id": 2,
          "dataSource": 1,
          "style": "Surface",
          "color": "RTData",
          "opacity": 1.0,
          "visible": true
        }
      ]
    }
  ],
  "layout": {
    "type": "split",
    "direction": "horizontal",
    "ratio": 0.5,
    "children": [{ "type": "subchart", "id": 3 }]
  }
}
```

### 9.4 Implementation Implications

#### For Tracks/Subcharts/Curves UI Implementation

1. **Track State**: Serialize track structure (source, processors, connections)
2. **Subchart State**: Serialize subchart viewport, camera, and curve assignments
3. **Curve State**: Serialize curve styling (color, line style, visibility)
4. **Layout State**: Serialize subchart arrangement (split panes, viewport geometry)

#### Key Design Patterns to Adopt

1. **Proxy Pattern**: Each track/subchart/curve is a "proxy" that owns its state
2. **Central Registry**: Central manager (like `vtkSMProxyManager`) coordinates all state
3. **XML/JSON Serialization**: Human-readable format for state files
4. **Versioning**: Include version information for backward compatibility
5. **Extensibility**: Allow custom track processors/subcharts/curves to hook into state system

---

## 10. Summary

### 10.1 Key Architectural Insights

1. **Proxy-Based State Ownership**: Each component (source, filter, view, representation) owns its state through a proxy, enabling clean separation and serialization.

2. **Centralized State Management**: `vtkSMProxyManager` coordinates all state operations, providing a single point of control for save/load operations.

3. **XML Serialization**: Human-readable XML format enables manual editing, debugging, and version control of state files.

4. **Versioning Support**: State files include version information, enabling backward compatibility and migration strategies.

5. **Extensibility**: Custom plugins/filters/views automatically integrate into state system through Server Manager XML registration.

### 10.2 Critical Implementation Details

1. **State File Structure**: `.pvsm` files contain `ProxyCollection`, `ViewCollection`, and `LayoutCollection` sections.

2. **Proxy ID Mapping**: Proxies are assigned IDs during save, and IDs are used to restore connections during load.

3. **Property Serialization**: Properties are serialized by type (Int, Double, String, Input), with special handling for input properties (proxy ID references).

4. **Layout Tree Serialization**: Multi-view layouts are serialized as trees of split nodes and view nodes.

5. **View State Capture**: Views capture camera state, viewport geometry, and representation assignments.

### 10.3 Design Patterns Worth Adopting

1. **Proxy Pattern**: Wrap components with proxies that own state and handle serialization.
2. **Central Registry**: Single manager coordinates all state operations.
3. **Versioned Serialization**: Include version information and support migration.
4. **Extensible Registration**: Allow custom components to register and participate in state system.
5. **Separation of Concerns**: Clear separation between UI, state management, and serialization layers.

---

## References

- ParaView Documentation: https://www.paraview.org/documentation/
- ParaView Source Code: https://gitlab.kitware.com/paraview/paraview
- VTK Documentation: https://vtk.org/documentation/
- ParaView Developer Guide: https://www.paraview.org/Wiki/ParaView
