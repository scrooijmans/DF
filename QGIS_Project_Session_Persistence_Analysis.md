# QGIS Project/Session Persistence: Deep Architecture Analysis

## Executive Summary

QGIS implements a robust project/session persistence system centered on the **QgsProject** singleton, which orchestrates the serialization and deserialization of all project state including layers, layer tree hierarchy, styles, print layouts, map canvas state, and UI configurations. The system uses XML-based file formats (`.qgs` / `.qgz`) with a signal-driven architecture that enables clean separation between data model, persistence, and UI restoration.

---

## 1. Data Model: How QGIS Represents a "Project/Session"

### 1.1 Core Components

A QGIS project encapsulates the following state:

| Component | Class | Description |
|-----------|-------|-------------|
| **Layers** | `QgsMapLayer` (abstract), `QgsVectorLayer`, `QgsRasterLayer`, `QgsMeshLayer`, `QgsPointCloudLayer` | Data sources with geometry, attributes, and configuration |
| **Layer Tree** | `QgsLayerTree`, `QgsLayerTreeGroup`, `QgsLayerTreeLayer` | Hierarchical organization of layers and groups |
| **Styles/Symbology** | `QgsFeatureRenderer`, `QgsSymbol`, `QgsLabeling` | Visual representation rules |
| **CRS** | `QgsCoordinateReferenceSystem` | Project-wide coordinate reference system |
| **Map Canvas State** | `QgsMapSettings`, `QgsRectangle` | Current extent, scale, rotation |
| **Bookmarks** | `QgsBookmark`, `QgsBookmarkManager` | Saved spatial extents for navigation |
| **Print Layouts** | `QgsPrintLayout`, `QgsLayoutManager` | Composer layouts with map elements |
| **Map Themes** | `QgsMapThemeCollection` | Named layer visibility/style combinations |
| **Relations** | `QgsRelationManager`, `QgsRelation` | Layer relationships for joins |
| **Dock Widgets/Panels** | `QgsSettings` (application-level) | UI state and panel arrangements |

### 1.2 Ownership Hierarchy

```
QgsProject (singleton)
├── QgsMapLayerStore (owns all layers)
│   └── QgsMapLayer instances (QgsVectorLayer, QgsRasterLayer, etc.)
│       ├── Data Source (file path, database URI, web service)
│       ├── QgsCoordinateReferenceSystem (layer CRS)
│       ├── QgsFeatureRenderer (symbology)
│       ├── QgsLabeling (labeling configuration)
│       ├── QgsLayerMetadata (descriptive metadata)
│       └── Custom Properties (key-value storage)
│
├── QgsLayerTree (hierarchical structure)
│   ├── QgsLayerTreeGroup (folders/groups)
│   │   └── QgsLayerTreeLayer (layer references)
│   └── Visibility state, mutually exclusive groups
│
├── QgsLayoutManager (print layouts)
│   └── QgsPrintLayout instances
│       ├── QgsLayoutItemMap (map views)
│       ├── QgsLayoutItemLegend (legends)
│       ├── QgsLayoutItemScaleBar (scale bars)
│       └── Other layout items (labels, images, shapes)
│
├── QgsMapThemeCollection (map themes)
│   └── Named visibility/style presets
│
├── QgsRelationManager (layer relations)
│   └── QgsRelation instances
│
├── QgsBookmarkManager (spatial bookmarks)
│   └── QgsBookmark instances
│
└── Project Properties
    ├── Title, CRS, Extent
    ├── Snapping settings
    ├── Expression variables
    └── Custom project properties
```

---

## 2. File Formats: `.qgs` / `.qgz` Structure

### 2.1 Format Overview

| Format | Type | Contents |
|--------|------|----------|
| **`.qgs`** | Plain XML | Complete project state in human-readable XML |
| **`.qgz`** | ZIP archive | Compressed `.qgs` + optional `.qgd` SQLite database |

### 2.2 `.qgs` XML Structure

```xml
<!DOCTYPE qgis PUBLIC 'http://mrcc.com/qgis.dtd' 'SYSTEM'>
<qgis version="3.x.x" projectname="MyProject">
  
  <!-- Project Properties -->
  <title>My QGIS Project</title>
  <projectCrs>
    <spatialrefsys>
      <wkt>...</wkt>
      <proj4>+proj=longlat +datum=WGS84 +no_defs</proj4>
      <authid>EPSG:4326</authid>
    </spatialrefsys>
  </projectCrs>
  
  <!-- Layer Tree Structure -->
  <layer-tree-group>
    <customproperties/>
    <layer-tree-group name="Group 1" checked="Qt::Checked">
      <layer-tree-layer id="layer_id_123" name="My Layer" source="..." providerKey="ogr">
        <customproperties/>
      </layer-tree-layer>
    </layer-tree-group>
  </layer-tree-group>
  
  <!-- Map Canvas State -->
  <mapcanvas>
    <extent>
      <xmin>-180</xmin><ymin>-90</ymin>
      <xmax>180</xmax><ymax>90</ymax>
    </extent>
    <rotation>0</rotation>
    <destinationsrs>...</destinationsrs>
  </mapcanvas>
  
  <!-- Layer Definitions -->
  <projectlayers>
    <maplayer type="vector" id="layer_id_123">
      <datasource>/path/to/data.shp</datasource>
      <provider encoding="UTF-8">ogr</provider>
      <layername>My Layer</layername>
      <srs>...</srs>
      
      <!-- Symbology -->
      <renderer-v2 type="singleSymbol">
        <symbols>
          <symbol type="fill" name="0">
            <layer class="SimpleFill">
              <prop k="color" v="255,0,0,255"/>
              <prop k="style" v="solid"/>
            </layer>
          </symbol>
        </symbols>
      </renderer-v2>
      
      <!-- Labeling -->
      <labeling type="simple">
        <settings>...</settings>
      </labeling>
      
      <!-- Custom Properties -->
      <customproperties>
        <Option type="Map">
          <Option name="myKey" value="myValue"/>
        </Option>
      </customproperties>
    </maplayer>
  </projectlayers>
  
  <!-- Print Layouts -->
  <Layouts>
    <Layout name="Layout 1" units="mm">
      <PageCollection>
        <LayoutItem type="65638" uuid="..."><!-- Page -->
          <LayoutItemSize width="297" height="210"/>
        </LayoutItem>
      </PageCollection>
      
      <LayoutItem type="65639" uuid="..."><!-- Map Item -->
        <LayoutItemMapExtent>...</LayoutItemMapExtent>
        <LayoutItemMapLayers>...</LayoutItemMapLayers>
      </LayoutItem>
      
      <LayoutItem type="65640" uuid="..."><!-- Legend -->
        <LayoutItemLegendSettings>...</LayoutItemLegendSettings>
      </LayoutItem>
    </Layout>
  </Layouts>
  
  <!-- Map Themes -->
  <visibility-presets>
    <visibility-preset name="Theme 1">
      <layer id="layer_id_123" visible="1" style="default"/>
    </visibility-preset>
  </visibility-presets>
  
  <!-- Snapping Settings -->
  <snapping-settings>
    <individual-layer-settings>...</individual-layer-settings>
  </snapping-settings>
  
  <!-- Relations -->
  <relations>
    <relation id="rel_1" referencingLayer="..." referencedLayer="...">
      <fieldRef referencingField="..." referencedField="..."/>
    </relation>
  </relations>
  
  <!-- Bookmarks -->
  <Bookmarks>
    <Bookmark name="Bookmark 1">
      <extent>...</extent>
    </Bookmark>
  </Bookmarks>
  
</qgis>
```

### 2.3 `.qgz` Archive Contents

```
project.qgz (ZIP archive)
├── project.qgs          # Main XML project file
└── project.qgd          # SQLite database (optional)
    └── Contains:
        - Auxiliary storage
        - Annotations
        - Custom auxiliary fields
        - Plugin data
```

### 2.4 What Is Stored vs. Embedded

| Category | Storage Location | Format |
|----------|-----------------|--------|
| Project structure | `.qgs` XML | XML elements |
| Layer definitions | `.qgs` XML | XML elements |
| Symbology/styles | `.qgs` XML (inline) or `.qml` files | XML |
| Print layouts | `.qgs` XML | XML elements |
| Map themes | `.qgs` XML | XML elements |
| Auxiliary data | `.qgd` SQLite | Binary database |
| Embedded layers | `.qgz` archive | Compressed files |
| SVG symbols | External files or embedded | SVG/Base64 |
| Raster data | External files (referenced) | Various formats |

---

## 3. Architecture + Ownership: Core Classes

### 3.1 Class Responsibilities

```
┌─────────────────────────────────────────────────────────────────────┐
│                        QgsProject (Singleton)                        │
│  Responsibilities:                                                   │
│  - Central project state container                                   │
│  - Coordinates save/load operations                                  │
│  - Manages layer store, layout manager, theme collection             │
│  - Emits signals for project state changes                           │
│  - Handles project file I/O (.qgs/.qgz)                             │
│  - Manages project-level properties (CRS, title, etc.)              │
└────────────────────────────┬────────────────────────────────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ QgsMapLayerStore│ │ QgsLayerTree    │ │ QgsLayoutManager│
│                 │ │                 │ │                 │
│ - Owns layers   │ │ - Tree structure│ │ - Print layouts │
│ - Layer lookup  │ │ - Groups/layers │ │ - Layout I/O    │
│ - Layer signals │ │ - Visibility    │ │ - Layout signals│
└────────┬────────┘ └────────┬────────┘ └────────┬────────┘
         │                   │                   │
         ▼                   ▼                   ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ QgsMapLayer     │ │QgsLayerTreeNode │ │ QgsPrintLayout  │
│ (abstract)      │ │ (abstract)      │ │                 │
│                 │ │                 │ │ - Layout items  │
│ - Data source   │ │ - QgsLayerTree- │ │ - Page setup    │
│ - CRS           │ │   Group         │ │ - Export        │
│ - Renderer      │ │ - QgsLayerTree- │ │                 │
│ - Labeling      │ │   Layer         │ │                 │
│ - Metadata      │ │                 │ │                 │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

### 3.2 Key Classes and Their Roles

| Class | Role | Key Methods |
|-------|------|-------------|
| **`QgsProject`** | Project singleton, coordinates all state | `read()`, `write()`, `addMapLayer()`, `layerTreeRoot()` |
| **`QgsMapLayerStore`** | Layer ownership and registry | `addMapLayer()`, `removeMapLayer()`, `mapLayer()` |
| **`QgsLayerTree`** | Root of layer tree hierarchy | `findLayer()`, `findLayers()`, `insertChildNode()` |
| **`QgsLayerTreeGroup`** | Folder/group in layer tree | `addLayer()`, `removeChildNode()`, `setItemVisibilityChecked()` |
| **`QgsLayerTreeLayer`** | Layer reference in tree | `layer()`, `setItemVisibilityChecked()` |
| **`QgsLayoutManager`** | Manages all print layouts | `addLayout()`, `removeLayout()`, `layoutByName()` |
| **`QgsPrintLayout`** | Individual print layout | `addLayoutItem()`, `exportToPdf()`, `exportToImage()` |
| **`QgsMapThemeCollection`** | Map themes/presets | `insert()`, `mapThemeState()`, `applyTheme()` |
| **`QgsSettings`** | Application settings (UI state) | `value()`, `setValue()` |

### 3.3 Bridge Classes for Synchronization

```
QgsLayerTreeRegistryBridge
├── Connects: QgsProject ↔ QgsLayerTree
├── Listens: project.layersAdded, project.layersRemoved
└── Updates: layer tree nodes

QgsLayerTreeMapCanvasBridge
├── Connects: QgsLayerTree ↔ QgsMapCanvas
├── Listens: tree.addedChildren, tree.visibilityChanged
└── Updates: canvas layer set
```

---

## 4. Call Stack: Save Project and Open Project

### 4.1 Save Project Call Stack

```
User Action: File → Save Project (or Ctrl+S)
    │
    ▼
QgisApp::saveProject()
    │
    ▼
QgsProject::write(fileName)
    │
    ├─► Determine format (.qgs or .qgz)
    │
    ├─► QgsProject::writeProject(doc)
    │   │
    │   ├─► Write project properties
    │   │   └─► <title>, <projectCrs>, <projectExtent>
    │   │
    │   ├─► QgsLayerTree::writeXml(doc)
    │   │   └─► Serialize layer tree structure
    │   │       └─► QgsLayerTreeGroup::writeXml() (recursive)
    │   │           └─► QgsLayerTreeLayer::writeXml()
    │   │
    │   ├─► Write all layers: <projectlayers>
    │   │   └─► For each QgsMapLayer:
    │   │       └─► QgsMapLayer::writeLayerXml(doc, context)
    │   │           ├─► Write data source, provider
    │   │           ├─► QgsMapLayer::writeSymbology(doc, context)
    │   │           │   └─► QgsFeatureRenderer::save(doc, context)
    │   │           ├─► Write labeling configuration
    │   │           ├─► Write custom properties
    │   │           └─► Write layer-specific settings
    │   │
    │   ├─► QgsLayoutManager::writeXml(doc)
    │   │   └─► For each QgsPrintLayout:
    │   │       └─► QgsPrintLayout::writeXml(doc, context)
    │   │           └─► Write all layout items
    │   │
    │   ├─► QgsMapThemeCollection::writeXml(doc)
    │   │   └─► Write visibility presets
    │   │
    │   ├─► QgsRelationManager::writeXml(doc)
    │   │   └─► Write layer relations
    │   │
    │   ├─► QgsBookmarkManager::writeXml(doc)
    │   │   └─► Write spatial bookmarks
    │   │
    │   ├─► Write snapping settings
    │   │
    │   └─► Write project variables
    │
    ├─► If .qgz format:
    │   ├─► Create ZIP archive
    │   ├─► Add .qgs file
    │   └─► Add .qgd SQLite database (if exists)
    │
    ├─► QgsProject::setDirty(false)
    │
    └─► Emit: QgsProject::projectSaved()
```

### 4.2 Open Project Call Stack

```
User Action: File → Open Project
    │
    ▼
QgisApp::openProject(fileName)
    │
    ▼
QgsProject::read(fileName)
    │
    ├─► If .qgz format:
    │   └─► Extract .qgs and .qgd from ZIP
    │
    ├─► QgsProject::readProject(doc)
    │   │
    │   ├─► Read project properties
    │   │   └─► <title>, <projectCrs>, <projectExtent>
    │   │
    │   ├─► Read layers: <projectlayers>
    │   │   └─► For each <maplayer>:
    │   │       ├─► Determine layer type (vector, raster, etc.)
    │   │       ├─► Create appropriate QgsMapLayer subclass
    │   │       └─► QgsMapLayer::readLayerXml(node, context)
    │   │           ├─► Read data source, provider
    │   │           ├─► QgsMapLayer::readSymbology(node, context)
    │   │           │   └─► QgsFeatureRenderer::load(node, context)
    │   │           ├─► Read labeling configuration
    │   │           ├─► Read custom properties
    │   │           └─► Read layer-specific settings
    │   │
    │   ├─► QgsLayerTree::readXml(node)
    │   │   └─► Reconstruct layer tree structure
    │   │       └─► QgsLayerTreeGroup::readXml() (recursive)
    │   │           └─► QgsLayerTreeLayer::readXml()
    │   │
    │   ├─► QgsLayoutManager::readXml(node, doc)
    │   │   └─► For each <Layout>:
    │   │       └─► QgsPrintLayout::readXml(node, doc, context)
    │   │           └─► Reconstruct all layout items
    │   │
    │   ├─► QgsMapThemeCollection::readXml(node)
    │   │   └─► Read visibility presets
    │   │
    │   ├─► QgsRelationManager::readXml(node)
    │   │   └─► Read layer relations
    │   │
    │   ├─► QgsBookmarkManager::readXml(node)
    │   │   └─► Read spatial bookmarks
    │   │
    │   ├─► Read snapping settings
    │   │
    │   └─► Read project variables
    │
    ├─► Resolve layer references
    │   └─► QgsMapLayer::resolveReferences(project)
    │
    ├─► Restore map canvas state
    │   └─► QgsMapCanvas::setExtent(), setRotation()
    │
    ├─► Emit: QgsProject::readProject(doc)
    │
    └─► Emit: QgsProject::projectRead()
```

---

## 5. Layout + View Restoration

### 5.1 Print Layout Persistence

**Save Flow:**
```python
QgsLayoutManager::writeXml(doc)
    └─► For each layout in mLayouts:
        └─► QgsPrintLayout::writeXml(doc, context)
            ├─► Write page collection
            │   └─► Page sizes, orientations
            ├─► Write layout items:
            │   ├─► QgsLayoutItemMap::writeXml()
            │   │   ├─► Map extent, scale, layers
            │   │   └─► Map theme reference
            │   ├─► QgsLayoutItemLegend::writeXml()
            │   │   └─► Legend model, styles
            │   ├─► QgsLayoutItemScaleBar::writeXml()
            │   │   └─► Scale bar style, units
            │   ├─► QgsLayoutItemLabel::writeXml()
            │   │   └─► Text content, font
            │   └─► Other items (picture, shape, etc.)
            └─► Write layout guides, grids
```

**Load Flow:**
```python
QgsLayoutManager::readXml(node, doc)
    └─► For each <Layout> element:
        ├─► Create new QgsPrintLayout
        └─► QgsPrintLayout::readXml(node, doc, context)
            ├─► Read page collection
            ├─► Read and create layout items:
            │   ├─► QgsLayoutItemMap::readXml()
            │   │   └─► Restore extent, layers, theme
            │   ├─► QgsLayoutItemLegend::readXml()
            │   │   └─► Rebuild legend model
            │   └─► Other items...
            └─► Restore guides, grids
```

### 5.2 Map Theme Persistence

**Data Structure:**
```python
QgsMapThemeCollection:
    themes: Dict[str, MapThemeRecord]
        └─► MapThemeRecord:
            ├─► layerRecords: List[MapThemeLayerRecord]
            │   └─► layerId, visible, style, expandedLegendItems
            └─► checkedGroupNodes: List[str]
```

**XML Format:**
```xml
<visibility-presets>
  <visibility-preset name="Theme 1">
    <layer id="layer_123" visible="1" style="mystyle"/>
    <expanded-legend-items>
      <item id="legend_item_1"/>
    </expanded-legend-items>
    <checked-group-nodes>
      <group id="group_1"/>
    </checked-group-nodes>
  </visibility-preset>
</visibility-presets>
```

### 5.3 Layer Style Persistence

**Inline in Project:**
```xml
<maplayer type="vector" id="layer_123">
  <renderer-v2 type="categorizedSymbol" attr="category">
    <categories>
      <category symbol="0" value="A" label="Category A"/>
      <category symbol="1" value="B" label="Category B"/>
    </categories>
    <symbols>
      <symbol type="fill" name="0">
        <layer class="SimpleFill">
          <prop k="color" v="255,0,0,255"/>
        </layer>
      </symbol>
    </symbols>
  </renderer-v2>
</maplayer>
```

**External Style Files (`.qml`):**
```python
# Save style to external file
layer.saveNamedStyle('/path/to/style.qml')

# Load style from external file
layer.loadNamedStyle('/path/to/style.qml')
```

### 5.4 Panel/Dock Arrangement Persistence

**Note:** Panel arrangements are stored in **application settings**, not project files.

```python
# QgsSettings stores UI state
settings = QgsSettings()

# Panel visibility
settings.setValue('UI/panels/layerTreeVisible', True)
settings.setValue('UI/panels/browserVisible', False)

# Dock positions (Qt's QMainWindow state)
settings.setValue('UI/windowState', mainWindow.saveState())
settings.setValue('UI/geometry', mainWindow.saveGeometry())
```

---

## 6. Chart/Plot Integration

### 6.1 Built-in Charting Support

QGIS has **limited built-in charting** capabilities:

| Feature | Location | Persistence |
|---------|----------|-------------|
| **Diagrams** | Layer properties → Diagrams | Saved in layer XML |
| **Histogram** | Raster layer properties | Saved in layer XML |
| **Print Layout Charts** | Layout items | Saved in layout XML |

**Diagram Persistence (Layer-level):**
```xml
<maplayer type="vector">
  <DiagramRenderer type="LinearlyInterpolated" diagramType="Pie">
    <DiagramCategory>
      <prop k="penColor" v="#000000"/>
      <prop k="penWidth" v="0"/>
    </DiagramCategory>
  </DiagramRenderer>
</maplayer>
```

### 6.2 Plugin-Based Plotting (DataPlotly)

**DataPlotly Plugin** provides interactive charts:

```python
# DataPlotly stores plot configurations in project custom properties
project = QgsProject.instance()
project.writeEntry('DataPlotly', 'plots', json.dumps(plot_configs))

# On load
plot_configs = json.loads(project.readEntry('DataPlotly', 'plots')[0])
```

**Persistence Mechanism:**
- Plots stored as JSON in project custom properties
- Plot configurations include:
  - Chart type (scatter, bar, histogram, etc.)
  - Data source (layer ID, field names)
  - Visual properties (colors, titles)
  - Layout settings

### 6.3 Plot Persistence Limitations

| Aspect | Status |
|--------|--------|
| Built-in diagrams | ✅ Saved in project |
| DataPlotly plots | ✅ Saved via custom properties |
| Interactive chart state | ❌ Not persisted |
| Chart zoom/pan state | ❌ Not persisted |
| External plotting tools | ❌ Plugin-dependent |

---

## 7. Extensibility: Plugin Contributions

### 7.1 Plugin Project Integration Points

Plugins can contribute to project saving/loading through:

```python
# 1. Custom Properties (per-layer)
layer.setCustomProperty('myplugin/setting', 'value')
value = layer.customProperty('myplugin/setting')

# 2. Project Custom Properties
project.writeEntry('MyPlugin', 'key', 'value')
value = project.readEntry('MyPlugin', 'key')[0]

# 3. Project Read/Write Signals
project.readProject.connect(myPlugin.onProjectRead)
project.projectSaved.connect(myPlugin.onProjectSaved)

# 4. Layer Read/Write Signals
layer.readCustomSymbology.connect(myPlugin.onReadSymbology)
layer.writeCustomSymbology.connect(myPlugin.onWriteSymbology)
```

### 7.2 Plugin Architecture for Persistence

```python
class MyPlugin:
    def __init__(self, iface):
        self.iface = iface
        
        # Connect to project signals
        QgsProject.instance().readProject.connect(self.onProjectRead)
        QgsProject.instance().writeProject.connect(self.onProjectWrite)
    
    def onProjectRead(self, doc):
        """Called when project is loaded."""
        # Read plugin data from project
        project = QgsProject.instance()
        data = project.readEntry('MyPlugin', 'data')[0]
        if data:
            self.restoreState(json.loads(data))
    
    def onProjectWrite(self, doc):
        """Called when project is saved."""
        # Write plugin data to project
        project = QgsProject.instance()
        project.writeEntry('MyPlugin', 'data', json.dumps(self.getState()))
    
    def getState(self):
        """Serialize plugin state."""
        return {'setting1': self.setting1, 'setting2': self.setting2}
    
    def restoreState(self, state):
        """Restore plugin state."""
        self.setting1 = state.get('setting1')
        self.setting2 = state.get('setting2')
```

### 7.3 Compatibility and Versioning

```xml
<!-- Project file includes version information -->
<qgis version="3.28.0" projectname="MyProject">
  <!-- Plugins can check version for compatibility -->
  <customproperties>
    <Option type="Map">
      <Option name="MyPlugin/version" value="2.0"/>
      <Option name="MyPlugin/data" value="{...}"/>
    </Option>
  </customproperties>
</qgis>
```

**Version Handling:**
```python
def onProjectRead(self, doc):
    project = QgsProject.instance()
    plugin_version = project.readEntry('MyPlugin', 'version')[0]
    
    if plugin_version < '2.0':
        # Migrate old data format
        self.migrateFromV1(project)
    else:
        # Load current format
        self.loadCurrentFormat(project)
```

---

## 8. Key Source Folders/Files

### 8.1 Core Source Locations

```
QGIS Source Tree
├── src/core/
│   ├── project/
│   │   ├── qgsproject.cpp              # QgsProject implementation
│   │   ├── qgsprojectproperty.cpp      # Project property handling
│   │   └── qgsprojectversion.cpp       # Version handling
│   │
│   ├── layertree/
│   │   ├── qgslayertree.cpp            # Layer tree root
│   │   ├── qgslayertreegroup.cpp       # Layer groups
│   │   ├── qgslayertreelayer.cpp       # Layer nodes
│   │   └── qgslayertreemodel.cpp       # Qt model for tree
│   │
│   ├── layout/
│   │   ├── qgslayoutmanager.cpp        # Layout manager
│   │   ├── qgsprintlayout.cpp          # Print layout
│   │   ├── qgslayoutitem.cpp           # Base layout item
│   │   ├── qgslayoutitemmap.cpp        # Map item
│   │   ├── qgslayoutitemlegend.cpp     # Legend item
│   │   └── qgslayoutitemscalebar.cpp   # Scale bar item
│   │
│   ├── qgsmaplayer.cpp                 # Base layer class
│   ├── qgsvectorlayer.cpp              # Vector layer
│   ├── qgsrasterlayer.cpp              # Raster layer
│   │
│   ├── symbology/
│   │   ├── qgsrenderer.cpp             # Base renderer
│   │   ├── qgssinglesymbolrenderer.cpp
│   │   ├── qgscategorizedsymbolrenderer.cpp
│   │   └── qgsgraduatedsymbolrenderer.cpp
│   │
│   └── qgsmapthemecollection.cpp       # Map themes
│
├── src/gui/
│   ├── qgsmapcanvas.cpp                # Map canvas widget
│   ├── qgslayertreeview.cpp            # Layer tree view
│   └── layout/
│       └── qgslayoutdesignerinterface.cpp
│
└── src/app/
    ├── qgisapp.cpp                     # Main application
    └── qgsprojectproperties.cpp        # Project properties dialog
```

### 8.2 Most Important Classes/Functions

| Class | Key Methods | Purpose |
|-------|-------------|---------|
| `QgsProject` | `read()`, `write()`, `writeProject()`, `readProject()` | Project I/O |
| `QgsMapLayer` | `writeLayerXml()`, `readLayerXml()`, `writeSymbology()`, `readSymbology()` | Layer serialization |
| `QgsLayerTree` | `writeXml()`, `readXml()` | Tree structure I/O |
| `QgsLayoutManager` | `writeXml()`, `readXml()`, `addLayout()` | Layout management |
| `QgsPrintLayout` | `writeXml()`, `readXml()` | Layout serialization |
| `QgsFeatureRenderer` | `save()`, `load()` | Symbology I/O |
| `QgsMapThemeCollection` | `writeXml()`, `readXml()`, `insert()`, `applyTheme()` | Theme management |

---

## 9. Architecture Diagram (Components + Responsibilities)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              QGIS APPLICATION                                │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                         UI LAYER (Qt Widgets)                         │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │  Map     │  │  Layer   │  │  Layout  │  │  Panels/ │            │   │
│  │  │  Canvas  │  │  Tree    │  │  Designer│  │  Docks   │            │   │
│  │  │  View    │  │  View    │  │  View    │  │          │            │   │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘            │   │
│  └───────┼─────────────┼─────────────┼─────────────┼────────────────────┘   │
│          │             │             │             │                         │
│          │   BRIDGES   │             │             │                         │
│          ▼             ▼             │             │                         │
│  ┌───────────────────────────┐       │             │                         │
│  │ QgsLayerTreeMapCanvasBridge│       │             │                         │
│  │ QgsLayerTreeRegistryBridge │       │             │                         │
│  └───────────────┬───────────┘       │             │                         │
│                  │                   │             │                         │
│  ┌───────────────┴───────────────────┴─────────────┴───────────────────┐   │
│  │                         DATA MODEL LAYER                              │   │
│  │                                                                        │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐  │   │
│  │  │                      QgsProject (Singleton)                      │  │   │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │  │   │
│  │  │  │QgsMapLayer- │  │ QgsLayer-   │  │ QgsLayout-  │              │  │   │
│  │  │  │   Store     │  │   Tree      │  │   Manager   │              │  │   │
│  │  │  │             │  │             │  │             │              │  │   │
│  │  │  │ ┌─────────┐ │  │ ┌─────────┐ │  │ ┌─────────┐ │              │  │   │
│  │  │  │ │QgsVector│ │  │ │QgsLayer │ │  │ │QgsPrint │ │              │  │   │
│  │  │  │ │  Layer  │ │  │ │TreeGroup│ │  │ │ Layout  │ │              │  │   │
│  │  │  │ ├─────────┤ │  │ ├─────────┤ │  │ ├─────────┤ │              │  │   │
│  │  │  │ │QgsRaster│ │  │ │QgsLayer │ │  │ │ Layout  │ │              │  │   │
│  │  │  │ │  Layer  │ │  │ │TreeLayer│ │  │ │  Items  │ │              │  │   │
│  │  │  │ └─────────┘ │  │ └─────────┘ │  │ └─────────┘ │              │  │   │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘              │  │   │
│  │  │                                                                   │  │   │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │  │   │
│  │  │  │ QgsMapTheme │  │ QgsRelation │  │ QgsBookmark │              │  │   │
│  │  │  │ Collection  │  │   Manager   │  │   Manager   │              │  │   │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘              │  │   │
│  │  └─────────────────────────────────────────────────────────────────┘  │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                          │
│                                    ▼                                          │
│  ┌────────────────────────────────────────────────────────────────────────┐   │
│  │                        PERSISTENCE LAYER                               │   │
│  │                                                                        │   │
│  │  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐    │   │
│  │  │  XML Serializer  │  │  ZIP Handler     │  │  SQLite Handler  │    │   │
│  │  │  (.qgs files)    │  │  (.qgz archives) │  │  (.qgd database) │    │   │
│  │  └──────────────────┘  └──────────────────┘  └──────────────────┘    │   │
│  │                                                                        │   │
│  │  ┌──────────────────────────────────────────────────────────────────┐ │   │
│  │  │                    External Data Sources                          │ │   │
│  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │ │   │
│  │  │  │Shapefiles│  │GeoPackage│  │ PostGIS  │  │WMS/WFS   │        │ │   │
│  │  │  │ (.shp)   │  │ (.gpkg)  │  │          │  │ Services │        │ │   │
│  │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │ │   │
│  │  └──────────────────────────────────────────────────────────────────┘ │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 10. Mapping QGIS Concepts to "Tracks/Subcharts/Curves" Style UIs

For applications like well log viewers, seismic displays, or time-series charts:

| QGIS Concept | Track/Chart UI Equivalent | Persistence Strategy |
|--------------|---------------------------|----------------------|
| **QgsProject** | Session/Document | Single file containing all state |
| **QgsMapLayer** | Track/Data Series | Individual data channel with display properties |
| **QgsLayerTreeGroup** | Track Group/Panel | Hierarchical organization of related tracks |
| **QgsLayerTreeLayer** | Track Reference | Link to data with visibility state |
| **QgsFeatureRenderer** | Track Style/Curve Style | Visual properties (color, line style, fill) |
| **QgsPrintLayout** | Chart Layout/Report | Arrangement of multiple views for export |
| **QgsLayoutItemMap** | Chart View/Plot Area | Display area showing data |
| **QgsMapTheme** | View Preset/Display Configuration | Named combination of visible tracks and styles |
| **QgsBookmark** | Zoom Preset/Time Window | Saved view extent (depth range, time range) |
| **Layer Custom Properties** | Track Metadata/Annotations | Key-value storage for custom data |
| **Project Custom Properties** | Session Settings | Application-specific configuration |

### Session Save Pattern (Track/Chart UI)

```python
class ChartSession:
    """Analogous to QgsProject for track/chart applications."""
    
    def save(self, filepath):
        """Save session state."""
        doc = QDomDocument()
        root = doc.createElement('session')
        
        # Save track definitions (like layers)
        tracks_elem = doc.createElement('tracks')
        for track in self.tracks:
            track_elem = track.writeXml(doc)
            tracks_elem.appendChild(track_elem)
        root.appendChild(tracks_elem)
        
        # Save track tree (like layer tree)
        tree_elem = self.trackTree.writeXml(doc)
        root.appendChild(tree_elem)
        
        # Save layouts (like print layouts)
        layouts_elem = doc.createElement('layouts')
        for layout in self.layouts:
            layout_elem = layout.writeXml(doc)
            layouts_elem.appendChild(layout_elem)
        root.appendChild(layouts_elem)
        
        # Save view presets (like map themes)
        presets_elem = self.viewPresets.writeXml(doc)
        root.appendChild(presets_elem)
        
        # Write to file
        with open(filepath, 'w') as f:
            f.write(doc.toString())
    
    def load(self, filepath):
        """Load session state."""
        doc = QDomDocument()
        with open(filepath, 'r') as f:
            doc.setContent(f.read())
        
        root = doc.documentElement()
        
        # Load tracks
        tracks_elem = root.firstChildElement('tracks')
        # ... restore tracks
        
        # Load track tree
        tree_elem = root.firstChildElement('trackTree')
        self.trackTree.readXml(tree_elem)
        
        # Load layouts
        layouts_elem = root.firstChildElement('layouts')
        # ... restore layouts
        
        # Load view presets
        presets_elem = root.firstChildElement('viewPresets')
        self.viewPresets.readXml(presets_elem)
```

---

## 11. Signal Flow for Project Operations

### 11.1 Save Project Signal Flow

```
User triggers save
    │
    ▼
QgsProject::write()
    │
    ├─► beforeSaveProject() signal
    │   └─► Plugins prepare data for saving
    │
    ├─► writeProject(doc) signal
    │   └─► Plugins write custom data to XML
    │
    ├─► Write XML to file
    │
    ├─► setDirty(false)
    │
    └─► projectSaved() signal
        └─► UI updates (title bar, etc.)
```

### 11.2 Load Project Signal Flow

```
User opens project file
    │
    ▼
QgsProject::read()
    │
    ├─► Clear existing project
    │   └─► removeAll() signal
    │
    ├─► Parse XML file
    │
    ├─► Load layers
    │   └─► layersAdded() signal for each layer
    │       └─► Bridges update layer tree
    │           └─► Canvas updates
    │
    ├─► Load layouts
    │   └─► layoutAdded() signal for each layout
    │
    ├─► readProject(doc) signal
    │   └─► Plugins read custom data from XML
    │
    ├─► resolveReferences()
    │   └─► Layers resolve cross-references
    │
    └─► projectRead() signal
        └─► UI fully restored
```

---

## Summary

QGIS implements a comprehensive project persistence system with:

1. **Centralized State Management**: `QgsProject` singleton coordinates all project state
2. **XML-Based Serialization**: Human-readable `.qgs` files with optional `.qgz` compression
3. **Hierarchical Layer Organization**: `QgsLayerTree` manages layer groups and visibility
4. **Decoupled Architecture**: Bridge classes synchronize data model with UI
5. **Extensible Plugin System**: Custom properties and signals enable plugin integration
6. **Comprehensive Layout Support**: `QgsLayoutManager` handles print layouts with full item serialization
7. **Theme/Preset System**: `QgsMapThemeCollection` saves named visibility/style combinations

The architecture demonstrates best practices for session persistence in complex data visualization applications, with clear separation between data model, persistence layer, and UI components.

---

## References

- QGIS Documentation: https://docs.qgis.org/
- QGIS Python API: https://qgis.org/pyqgis/master/
- QGIS Source Code: https://github.com/qgis/QGIS
- QGIS File Formats: https://docs.qgis.org/latest/en/docs/user_manual/appendices/qgis_file_formats.html

