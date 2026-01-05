# QGIS Provenance Architecture: Exposing Derived Data Origins

## Executive Summary

QGIS provides **limited built-in provenance tracking** for derived data. While the architecture includes mechanisms for storing layer metadata and processing workflow definitions, it does **not automatically track processing history** or maintain comprehensive provenance chains. This document explains how QGIS exposes provenance through layer metadata, processing models, and temporary layer management, and analyzes the architectural choices that limit or enable users' ability to understand where data came from.

---

## 1. Provenance Mechanisms in QGIS

### 1.1 Layer Metadata (QgsLayerMetadata)

**Purpose**: Store descriptive information about layers

**What It Tracks:**
- **Title, abstract, keywords** - Descriptive information
- **Extent, CRS** - Spatial information
- **Data source** - Where data comes from
- **Custom properties** - User-defined metadata

**What It Does NOT Track:**
- **Processing history** - How data was created
- **Input layers** - What data was used to create this layer
- **Algorithm parameters** - What processing was applied
- **Execution timestamps** - When processing occurred

**Architecture:**

```python
from qgis.core import QgsLayerMetadata, QgsVectorLayer

# Layer metadata structure
class QgsLayerMetadata:
    """Standardized metadata structure for layers."""
    
    # Basic information
    title: str
    abstract: str
    keywords: List[str]
    
    # Spatial information
    extent: QgsBox3D
    crs: QgsCoordinateReferenceSystem
    
    # Data source
    identifier: str
    parentIdentifier: str
    
    # Custom properties
    customProperties: Dict[str, str]
    
    # Contact information
    contacts: List[QgsAbstractMetadataBase.Contact]
    
    # Links
    links: List[QgsAbstractMetadataBase.Link]

# Setting metadata
layer = QgsVectorLayer('/path/to/data.shp', 'Layer', 'ogr')
metadata = QgsLayerMetadata()
metadata.setTitle("My Layer")
metadata.setAbstract("Layer description")
metadata.setKeywords(["keyword1", "keyword2"])
layer.setMetadata(metadata)
```

**Storage:**
- **Project files (.qgs)**: Metadata stored in project XML
- **Layer files (.qml)**: Metadata can be saved to layer style files
- **Not in data files**: Metadata not stored in shapefiles, GeoPackages, etc.

**Limitations:**
- **Manual entry**: Users must manually enter metadata
- **No automatic tracking**: Processing doesn't automatically populate metadata
- **No history**: Metadata doesn't track how layer was created
- **No links**: No automatic links to input layers or processing steps

### 1.2 Processing Models (.model3 files)

**Purpose**: Define processing workflows as reusable models

**What It Tracks:**
- **Algorithm chain** - Sequence of algorithms
- **Parameters** - Algorithm parameters and values
- **Input/output connections** - Data flow between algorithms
- **Workflow structure** - DAG representation

**What It Does NOT Track:**
- **Execution history** - When model was executed
- **Input data versions** - Which versions of input data were used
- **Output data** - Where outputs were saved
- **Execution results** - Success/failure of executions

**Architecture:**

```python
# Processing model structure
class QgsProcessingModel:
    """Processing workflow model."""
    
    # Model parameters
    parameters: Dict[str, QgsProcessingParameterDefinition]
    
    # Algorithm chain
    algorithms: Dict[str, QgsProcessingModelAlgorithm]
    
    # Connections
    childConnections: List[QgsProcessingModelChildConnection]
    
    # Outputs
    outputs: Dict[str, QgsProcessingModelOutput]

# Model execution
model = QgsProcessingModel()
model.loadFromFile('/path/to/model.model3')

# Execute model
result = processing.runModel(
    model,
    {
        'INPUT': input_layer_id,
        'DISTANCE': 100
    }
)
```

**Model File Structure (.model3 XML):**

```xml
<Model>
  <parameters>
    <Parameter name="INPUT" type="vector"/>
    <Parameter name="DISTANCE" type="number" default="100"/>
  </parameters>
  <algorithm id="native:buffer">
    <inputs>
      <input name="INPUT" source="INPUT"/>
      <input name="DISTANCE" source="DISTANCE"/>
    </inputs>
    <outputs>
      <output name="OUTPUT" id="buffer_output"/>
    </outputs>
  </algorithm>
  <algorithm id="native:intersection">
    <inputs>
      <input name="INPUT" source="buffer_output"/>
      <input name="OVERLAY" source="OVERLAY"/>
    </inputs>
    <outputs>
      <output name="OUTPUT" id="intersection_output"/>
    </outputs>
  </algorithm>
  <outputs>
    <output name="OUTPUT" source="intersection_output"/>
  </outputs>
</Model>
```

**Limitations:**
- **Workflow definition only**: Models define workflows, not execution history
- **No execution tracking**: Models don't track when/how they were executed
- **No input versioning**: Models don't track which input data versions were used
- **No output linking**: Models don't automatically link outputs to executions

### 1.3 Temporary Layer Management

**Purpose**: Manage temporary layers created during processing

**What It Tracks:**
- **Temporary layer existence** - Which layers are temporary
- **Layer lifecycle** - When layers are created/destroyed
- **Layer registration** - Which layers are registered for loading

**What It Does NOT Track:**
- **Creation method** - How temporary layer was created
- **Input dependencies** - What inputs were used to create layer
- **Algorithm used** - Which algorithm created the layer
- **Creation timestamp** - When layer was created

**Architecture:**

```python
class QgsProcessingContext:
    """Processing execution context."""
    
    def temporaryLayerStore(self):
        """Returns temporary layer store."""
        return self._temporaryStore
    
    def addLayerToLoadOnCompletion(self, layerId, details):
        """
        Schedule layer to be loaded after algorithm completion.
        
        Details include:
        - Layer name
        - CRS
        - Styling options
        - But NOT provenance information
        """
        self._layersToLoad[layerId] = details
    
    def takeResultLayer(self, layerId):
        """
        Get result layer from context.
        
        Returns layer but doesn't preserve:
        - Creation method
        - Input dependencies
        - Algorithm information
        """
        return self._temporaryStore.takeMapLayer(layerId)
```

**Temporary Layer Characteristics:**
- **Memory layers**: Created in memory, not persisted
- **Temporary files**: Created in temp directory, auto-deleted
- **No provenance**: No tracking of how temporary layers were created
- **No linking**: No links to input layers or processing steps

---

## 2. Architectural Choices That Limit Provenance

### 2.1 No Automatic Processing History Tracking

**Architectural Choice**: Processing framework doesn't automatically track execution history

**Impact:**
- **No execution log**: No record of which algorithms were executed
- **No parameter tracking**: No record of parameters used
- **No input/output linking**: No links between inputs and outputs
- **No timestamp tracking**: No record of when processing occurred

**Why This Choice Was Made:**
- **Performance**: Tracking history adds overhead
- **Storage**: History would require additional storage
- **Complexity**: History tracking adds complexity to processing framework
- **User control**: Users can manually document workflows if needed

**Example:**

```python
# Processing execution
result = processing.run('native:buffer', {
    'INPUT': input_layer_id,
    'DISTANCE': 100,
    'OUTPUT': 'memory:'
})

# Output layer created
output_layer = result['OUTPUT']

# ❌ NO automatic tracking:
# - No record that buffer algorithm was executed
# - No record of input layer used
# - No record of parameters (distance=100)
# - No record of execution timestamp
# - No link between input and output layers
```

### 2.2 Immutable Processing Outputs

**Architectural Choice**: Processing outputs are immutable (created once, never modified)

**Impact:**
- **No incremental provenance**: Can't track modifications to outputs
- **No version history**: Each execution creates new output, no versioning
- **Lost provenance**: If output is deleted, provenance is lost
- **No provenance updates**: Can't update provenance after creation

**Why This Choice Was Made:**
- **Data integrity**: Immutable outputs prevent accidental modifications
- **Simplicity**: No need to track modifications
- **Performance**: No overhead for tracking changes
- **User control**: Users control output destinations

**Example:**

```python
# Processing creates new layer
result = processing.run('native:buffer', {
    'INPUT': input_layer_id,
    'DISTANCE': 100,
    'OUTPUT': '/path/to/buffer.shp'
})

# Output is immutable
output_layer = QgsVectorLayer(result['OUTPUT'], 'Buffer', 'ogr')

# ❌ NO provenance information:
# - Output doesn't know it was created by buffer algorithm
# - Output doesn't know input layer
# - Output doesn't know parameters
# - Output doesn't know creation timestamp
```

### 2.3 Layer-Centric Architecture

**Architectural Choice**: Layers are the unit of truth, not processing steps

**Impact:**
- **No processing step objects**: No objects representing processing steps
- **No DAG representation**: No explicit DAG of processing steps
- **No step metadata**: No metadata attached to processing steps
- **No step linking**: No links between processing steps

**Why This Choice Was Made:**
- **Simplicity**: Layer-centric architecture is simpler
- **Performance**: No overhead for processing step objects
- **Focus**: Focus on data, not processing
- **Flexibility**: Users can organize data as needed

**Example:**

```python
# Processing creates layer
result = processing.run('native:buffer', {...})
output_layer = result['OUTPUT']

# Layer is the unit of truth
# ❌ NO processing step object:
# - No QgsProcessingStep object
# - No step metadata
# - No step linking
# - No step history
```

### 2.4 No Provenance Database

**Architectural Choice**: No built-in provenance database

**Impact:**
- **No centralized tracking**: No central place to track provenance
- **No querying**: Can't query provenance information
- **No cross-layer provenance**: Can't track provenance across layers
- **No historical tracking**: Can't track provenance over time

**Why This Choice Was Made:**
- **Simplicity**: No need for database infrastructure
- **Performance**: No database overhead
- **Flexibility**: Users can use external tools if needed
- **Focus**: Focus on GIS functionality, not provenance

### 2.5 Temporary Layer Lifecycle

**Architectural Choice**: Temporary layers are ephemeral

**Impact:**
- **Lost provenance**: Temporary layers deleted lose provenance
- **No persistence**: Temporary layers not persisted
- **No tracking**: No tracking of temporary layer creation
- **No linking**: No links to temporary layers after deletion

**Why This Choice Was Made:**
- **Performance**: Temporary layers are for performance
- **Storage**: Don't want to persist temporary data
- **Simplicity**: No need to track temporary layers
- **User control**: Users can save temporary layers if needed

---

## 3. Architectural Choices That Enable Provenance

### 3.1 Layer Metadata System

**Architectural Choice**: Layers have metadata storage

**Enables:**
- **Manual provenance entry**: Users can manually enter provenance
- **Custom properties**: Users can store custom provenance information
- **Metadata persistence**: Metadata persists in project files
- **Metadata sharing**: Metadata can be shared with layers

**Example:**

```python
# Manual provenance entry
layer = QgsVectorLayer('/path/to/output.shp', 'Buffer', 'ogr')
metadata = layer.metadata()

# Set custom provenance properties
metadata.setCustomProperty('processing_algorithm', 'native:buffer')
metadata.setCustomProperty('input_layer', input_layer_id)
metadata.setCustomProperty('parameters', '{"DISTANCE": 100}')
metadata.setCustomProperty('execution_date', '2024-01-01')

layer.setMetadata(metadata)
```

**Limitations:**
- **Manual**: Requires manual entry
- **No automatic tracking**: Not automatically populated
- **No validation**: No validation of provenance information
- **No linking**: No automatic links to input layers

### 3.2 Processing Model System

**Architectural Choice**: Processing models define workflows

**Enables:**
- **Workflow documentation**: Models document workflows
- **Reproducibility**: Models can be re-executed
- **Sharing**: Models can be shared
- **Workflow visualization**: Models show workflow structure

**Example:**

```python
# Create processing model
model = QgsProcessingModel()

# Add algorithm
buffer_alg = model.addAlgorithm('native:buffer')
buffer_alg.setParameterValue('INPUT', 'INPUT')
buffer_alg.setParameterValue('DISTANCE', 100)

# Save model
model.saveToFile('/path/to/model.model3')

# Model documents workflow
# But doesn't track execution history
```

**Limitations:**
- **Workflow only**: Models define workflows, not execution history
- **No execution tracking**: Models don't track executions
- **No input versioning**: Models don't track input versions
- **No output linking**: Models don't link to outputs

### 3.3 Layer ID System

**Architectural Choice**: Layers have unique IDs

**Enables:**
- **Layer referencing**: Can reference layers by ID
- **Stable references**: IDs are stable across sessions
- **Cross-layer linking**: Can link layers using IDs
- **Project persistence**: IDs persist in project files

**Example:**

```python
# Layer IDs enable referencing
input_layer = QgsVectorLayer('/path/to/input.shp', 'Input', 'ogr')
input_layer.setId('input_layer_001')

# Can reference by ID
project = QgsProject.instance()
project.addMapLayer(input_layer)

# Reference in metadata
output_layer = QgsVectorLayer('/path/to/output.shp', 'Output', 'ogr')
metadata = output_layer.metadata()
metadata.setCustomProperty('input_layer_id', 'input_layer_001')
output_layer.setMetadata(metadata)
```

**Limitations:**
- **Manual linking**: Requires manual linking
- **No automatic tracking**: Not automatically tracked
- **No validation**: No validation of links
- **No bidirectional links**: Links are one-way

### 3.4 Project File System

**Architectural Choice**: Project files store layer information

**Enables:**
- **Layer persistence**: Layers persist in project files
- **Layer metadata**: Metadata persists in project files
- **Layer references**: Layer references persist
- **Project state**: Project state can be saved/restored

**Example:**

```python
# Project file stores layer information
project = QgsProject.instance()
project.addMapLayer(layer)

# Save project
project.write('/path/to/project.qgs')

# Project file contains:
# - Layer IDs
# - Layer sources
# - Layer metadata
# - Layer styling
# But NOT processing history
```

**Limitations:**
- **No processing history**: Project files don't store processing history
- **No execution tracking**: No execution information
- **No workflow tracking**: No workflow execution information
- **No provenance chain**: No provenance chain information

### 3.5 Custom Properties System

**Architectural Choice**: Layers support custom properties

**Enables:**
- **User-defined provenance**: Users can define custom provenance
- **Flexible storage**: Can store any provenance information
- **Persistence**: Custom properties persist in project files
- **Extensibility**: Can extend provenance as needed

**Example:**

```python
# Custom properties for provenance
layer = QgsVectorLayer('/path/to/output.shp', 'Output', 'ogr')

# Store provenance in custom properties
layer.setCustomProperty('provenance_algorithm', 'native:buffer')
layer.setCustomProperty('provenance_input', input_layer_id)
layer.setCustomProperty('provenance_parameters', json.dumps({
    'DISTANCE': 100,
    'SEGMENTS': 5
}))
layer.setCustomProperty('provenance_timestamp', datetime.now().isoformat())

# Retrieve provenance
algorithm = layer.customProperty('provenance_algorithm')
input_id = layer.customProperty('provenance_input')
parameters = json.loads(layer.customProperty('provenance_parameters'))
```

**Limitations:**
- **Manual entry**: Requires manual entry
- **No validation**: No validation of provenance
- **No automatic tracking**: Not automatically tracked
- **No standardization**: No standard provenance format

---

## 4. Provenance Exposure Mechanisms

### 4.1 Layer Properties Dialog

**How Provenance is Exposed:**
- **Metadata tab**: Shows layer metadata
- **Custom properties**: Shows custom properties
- **Source information**: Shows data source
- **No processing history**: No processing history shown

**User Experience:**
- **Manual entry**: Users must manually enter provenance
- **Limited visibility**: Provenance not prominently displayed
- **No linking**: No links to input layers or processing steps
- **No history**: No execution history shown

**Example:**

```python
# Layer properties dialog shows:
# - Metadata tab: Title, abstract, keywords
# - Source tab: Data source path
# - Custom properties: User-defined properties
# - But NOT:
#   - Processing history
#   - Input layers
#   - Algorithm parameters
#   - Execution timestamps
```

### 4.2 Processing Toolbox

**How Provenance is Exposed:**
- **Algorithm information**: Shows algorithm name and description
- **Parameter values**: Shows parameter values during execution
- **No execution history**: No history of past executions
- **No output linking**: No links to outputs

**User Experience:**
- **Current execution only**: Only shows current execution
- **No history**: No history of past executions
- **No linking**: No links to outputs
- **No tracking**: No tracking of executions

### 4.3 Processing Modeler

**How Provenance is Exposed:**
- **Workflow visualization**: Shows workflow structure
- **Algorithm chain**: Shows algorithm sequence
- **Parameter values**: Shows parameter values
- **No execution history**: No execution history

**User Experience:**
- **Workflow definition**: Shows how workflow is defined
- **No execution tracking**: Doesn't track executions
- **No output linking**: Doesn't link to outputs
- **No versioning**: Doesn't track workflow versions

### 4.4 Python Console

**How Provenance is Exposed:**
- **Script execution**: Users can write scripts that track provenance
- **Manual tracking**: Users can manually track provenance
- **No automatic tracking**: No automatic provenance tracking
- **No integration**: Script provenance not integrated with UI

**User Experience:**
- **Manual effort**: Requires manual effort to track provenance
- **No UI integration**: Provenance not shown in UI
- **No persistence**: Script provenance not persisted
- **No sharing**: Script provenance not easily shared

---

## 5. Architectural Limitations Analysis

### 5.1 Why Processing History Isn't Tracked

**Design Decisions:**

1. **Performance**: Tracking history adds overhead
   - Every algorithm execution would need to log information
   - History storage would consume memory/disk
   - History queries would add latency

2. **Storage**: History requires storage
   - Processing history would need to be stored somewhere
   - Project files would grow larger
   - Database would add complexity

3. **Complexity**: History tracking adds complexity
   - Need to design history data structures
   - Need to implement history storage
   - Need to implement history queries
   - Need to handle history cleanup

4. **User Control**: Users can document workflows manually
   - Processing models provide workflow documentation
   - Python scripts provide reproducibility
   - Users can add metadata manually

**Trade-offs:**
- **Simplicity vs. Provenance**: Simpler architecture without provenance
- **Performance vs. Tracking**: Better performance without tracking overhead
- **Flexibility vs. Structure**: More flexibility without structured provenance

### 5.2 Why Temporary Layers Don't Track Provenance

**Design Decisions:**

1. **Ephemeral nature**: Temporary layers are meant to be temporary
   - No need to track provenance for temporary data
   - Temporary layers are deleted, so provenance would be lost
   - Focus on performance, not provenance

2. **Performance**: Tracking provenance adds overhead
   - Every temporary layer creation would need to log information
   - Provenance storage would consume memory
   - Provenance queries would add latency

3. **Simplicity**: Simpler without provenance tracking
   - No need to design provenance data structures
   - No need to implement provenance storage
   - No need to handle provenance cleanup

**Trade-offs:**
- **Performance vs. Provenance**: Better performance without provenance overhead
- **Simplicity vs. Tracking**: Simpler architecture without tracking
- **Temporary vs. Persistent**: Temporary data doesn't need provenance

### 5.3 Why Layer Metadata Doesn't Auto-Populate

**Design Decisions:**

1. **Separation of concerns**: Metadata is descriptive, not procedural
   - Metadata describes data, not how it was created
   - Processing is separate from metadata
   - Users control metadata entry

2. **Flexibility**: Users can define metadata as needed
   - No forced metadata structure
   - Users can add custom properties
   - Users can define provenance format

3. **Performance**: Auto-population adds overhead
   - Every processing execution would need to update metadata
   - Metadata updates would add latency
   - Metadata storage would consume resources

**Trade-offs:**
- **Flexibility vs. Automation**: More flexibility without automation
- **Performance vs. Tracking**: Better performance without auto-population
- **User Control vs. Automation**: More user control without automation

---

## 6. What Users Can Do to Track Provenance

### 6.1 Manual Metadata Entry

**Strategy**: Manually enter provenance in layer metadata

```python
# After processing
result = processing.run('native:buffer', {
    'INPUT': input_layer_id,
    'DISTANCE': 100,
    'OUTPUT': '/path/to/buffer.shp'
})

# Load output layer
output_layer = QgsVectorLayer(result['OUTPUT'], 'Buffer', 'ogr')

# Manually add provenance
metadata = output_layer.metadata()
metadata.setTitle("Buffer of Input Layer")
metadata.setAbstract("Created by buffer algorithm with distance 100")
metadata.setCustomProperty('processing_algorithm', 'native:buffer')
metadata.setCustomProperty('input_layer_id', input_layer_id)
metadata.setCustomProperty('parameters', json.dumps({'DISTANCE': 100}))
metadata.setCustomProperty('execution_date', datetime.now().isoformat())
output_layer.setMetadata(metadata)
```

**Limitations:**
- **Manual effort**: Requires manual entry
- **Easy to forget**: Easy to forget to add provenance
- **No validation**: No validation of provenance
- **No linking**: No automatic links to inputs

### 6.2 Processing Models

**Strategy**: Use processing models to document workflows

```python
# Create model
model = QgsProcessingModel()

# Add algorithms
buffer_alg = model.addAlgorithm('native:buffer')
buffer_alg.setParameterValue('INPUT', 'INPUT')
buffer_alg.setParameterValue('DISTANCE', 100)

intersection_alg = model.addAlgorithm('native:intersection')
intersection_alg.setParameterValue('INPUT', buffer_alg.output('OUTPUT'))
intersection_alg.setParameterValue('OVERLAY', 'OVERLAY')

# Save model
model.saveToFile('/path/to/workflow.model3')

# Model documents workflow
# But doesn't track execution history
```

**Limitations:**
- **Workflow only**: Models define workflows, not execution history
- **No execution tracking**: Models don't track executions
- **No input versioning**: Models don't track input versions
- **No output linking**: Models don't link to outputs

### 6.3 Python Scripts

**Strategy**: Write Python scripts that track provenance

```python
import processing
import json
from datetime import datetime

# Track provenance manually
provenance_log = []

# Step 1: Buffer
result1 = processing.run('native:buffer', {
    'INPUT': 'input.shp',
    'DISTANCE': 100,
    'OUTPUT': 'buffer.shp'
})

provenance_log.append({
    'algorithm': 'native:buffer',
    'input': 'input.shp',
    'output': result1['OUTPUT'],
    'parameters': {'DISTANCE': 100},
    'timestamp': datetime.now().isoformat()
})

# Step 2: Intersection
result2 = processing.run('native:intersection', {
    'INPUT': result1['OUTPUT'],
    'OVERLAY': 'overlay.shp',
    'OUTPUT': 'intersection.shp'
})

provenance_log.append({
    'algorithm': 'native:intersection',
    'input': result1['OUTPUT'],
    'overlay': 'overlay.shp',
    'output': result2['OUTPUT'],
    'parameters': {},
    'timestamp': datetime.now().isoformat()
})

# Save provenance log
with open('provenance.json', 'w') as f:
    json.dump(provenance_log, f, indent=2)
```

**Limitations:**
- **Manual effort**: Requires manual tracking
- **No UI integration**: Provenance not shown in UI
- **No persistence**: Provenance not persisted in project
- **No sharing**: Provenance not easily shared

### 6.4 Custom Plugins

**Strategy**: Use plugins that add provenance tracking

**Available Plugins:**
- **Processing History**: Some plugins track processing history
- **Provenance Tracker**: Some plugins provide provenance tracking
- **Workflow Manager**: Some plugins manage workflows

**Limitations:**
- **Not core**: Not part of core QGIS
- **Vary in quality**: Plugins vary in implementation
- **Not standardized**: No standard provenance format
- **Maintenance**: Plugins may not be maintained

---

## 7. Architectural Enablers and Limitations Summary

### 7.1 What Enables Provenance Understanding

**Architectural Enablers:**

1. **Layer Metadata System**
   - ✅ Stores descriptive information
   - ✅ Supports custom properties
   - ✅ Persists in project files
   - ❌ Requires manual entry
   - ❌ No automatic tracking

2. **Processing Models**
   - ✅ Document workflows
   - ✅ Can be re-executed
   - ✅ Show workflow structure
   - ❌ No execution history
   - ❌ No input versioning

3. **Layer ID System**
   - ✅ Enables layer referencing
   - ✅ Stable across sessions
   - ✅ Persists in project files
   - ❌ No automatic linking
   - ❌ No validation

4. **Custom Properties**
   - ✅ Flexible storage
   - ✅ User-defined format
   - ✅ Persists in project files
   - ❌ No standardization
   - ❌ No validation

### 7.2 What Limits Provenance Understanding

**Architectural Limitations:**

1. **No Automatic Processing History**
   - ❌ No execution log
   - ❌ No parameter tracking
   - ❌ No input/output linking
   - ❌ No timestamp tracking

2. **Immutable Processing Outputs**
   - ❌ No incremental provenance
   - ❌ No version history
   - ❌ Lost provenance on deletion
   - ❌ No provenance updates

3. **Layer-Centric Architecture**
   - ❌ No processing step objects
   - ❌ No DAG representation
   - ❌ No step metadata
   - ❌ No step linking

4. **No Provenance Database**
   - ❌ No centralized tracking
   - ❌ No querying
   - ❌ No cross-layer provenance
   - ❌ No historical tracking

5. **Temporary Layer Lifecycle**
   - ❌ Lost provenance on deletion
   - ❌ No persistence
   - ❌ No tracking
   - ❌ No linking

---

## 8. Impact on User Understanding

### 8.1 What Users Can Understand

**From Layer Metadata:**
- **What the layer is**: Title, abstract, keywords
- **Where data comes from**: Data source path
- **Spatial information**: Extent, CRS
- **Custom information**: Custom properties (if manually entered)

**From Processing Models:**
- **How workflows are defined**: Algorithm chain, parameters
- **Workflow structure**: DAG representation
- **Reproducibility**: Can re-execute workflows

**From Custom Properties:**
- **User-defined provenance**: Any provenance information users add
- **Flexible format**: Users can define their own format

### 8.2 What Users Cannot Understand

**Missing Information:**
- **Processing history**: Which algorithms were executed
- **Execution timestamps**: When processing occurred
- **Input/output links**: Which inputs created which outputs
- **Parameter values**: What parameters were used
- **Execution results**: Success/failure of executions
- **Data lineage**: Complete data lineage chain
- **Version history**: How data evolved over time

**Impact:**
- **Hard to reproduce**: Difficult to reproduce exact processing
- **Hard to debug**: Difficult to debug processing issues
- **Hard to audit**: Difficult to audit data processing
- **Hard to share**: Difficult to share processing information
- **Hard to understand**: Difficult to understand data origins

### 8.3 Workarounds and Best Practices

**Best Practices:**

1. **Document workflows in processing models**
   - Save workflows as .model3 files
   - Document parameters and connections
   - Share models with others

2. **Add provenance to layer metadata**
   - Manually enter provenance information
   - Use custom properties for structured provenance
   - Document input layers and parameters

3. **Use Python scripts for reproducibility**
   - Write scripts that document processing
   - Save scripts with outputs
   - Version control scripts

4. **Use project notes**
   - Document processing steps in project notes
   - Describe workflows and parameters
   - Note input/output relationships

---

## 9. Comparison with Other Systems

### 9.1 Systems with Better Provenance

**ParaView:**
- **Pipeline representation**: Explicit pipeline DAG
- **Execution tracking**: Tracks pipeline execution
- **Data lineage**: Can trace data lineage
- **Version history**: Tracks data versions

**KNIME:**
- **Workflow execution**: Tracks workflow execution
- **Execution logs**: Detailed execution logs
- **Data lineage**: Can trace data lineage
- **Version control**: Workflow version control

**ImageJ2:**
- **Command logging**: Logs executed commands
- **Parameter tracking**: Tracks command parameters
- **Execution history**: Maintains execution history

### 9.2 Why QGIS Differs

**QGIS Design Philosophy:**
- **Focus on GIS functionality**: Focus on geospatial operations
- **User control**: Users control their workflows
- **Simplicity**: Keep architecture simple
- **Performance**: Prioritize performance over tracking

**Trade-offs:**
- **Simplicity vs. Provenance**: Simpler without provenance
- **Performance vs. Tracking**: Better performance without tracking
- **User Control vs. Automation**: More user control without automation

---

## 10. Conclusion

QGIS provides **limited built-in provenance tracking** through:

1. **Layer Metadata**: Manual entry of descriptive information
2. **Processing Models**: Workflow definitions (not execution history)
3. **Custom Properties**: User-defined provenance storage
4. **Temporary Layer Management**: Basic lifecycle management

**Architectural choices that limit provenance:**
- **No automatic processing history**: No execution log or parameter tracking
- **Immutable processing outputs**: No incremental provenance or versioning
- **Layer-centric architecture**: No processing step objects or DAG representation
- **No provenance database**: No centralized tracking or querying
- **Temporary layer lifecycle**: Lost provenance on deletion

**Architectural choices that enable provenance:**
- **Layer metadata system**: Stores descriptive information and custom properties
- **Processing model system**: Documents workflows and enables reproducibility
- **Layer ID system**: Enables layer referencing and linking
- **Project file system**: Persists layer information and metadata
- **Custom properties system**: Flexible storage for user-defined provenance

**Impact on users:**
- **Can understand**: What layers are, where data comes from (if documented), workflow definitions
- **Cannot understand**: Processing history, execution timestamps, input/output links, data lineage, version history

**Workarounds:**
- Manual metadata entry
- Processing models for workflow documentation
- Python scripts for reproducibility
- Custom plugins for provenance tracking

The architectural choices reflect a **focus on GIS functionality and user control** rather than automatic provenance tracking, requiring users to **manually document workflows** if they need provenance information. This design prioritizes **simplicity and performance** over comprehensive provenance tracking, making QGIS suitable for users who can manually document their workflows but limiting automatic provenance understanding.

