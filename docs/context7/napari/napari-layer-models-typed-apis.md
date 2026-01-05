# Napari Layer Models and Typed Layer APIs: Data Modalities and Plugin Binding

## Executive Summary

This document provides a comprehensive technical analysis of how napari uses layer models and typed layer APIs to represent different data modalities (Image, Labels, Points, Shapes, Surface, Tracks, Vectors), and how plugin widgets and actions bind to specific layer types and data shapes/dtypes through type annotations, runtime validation, and UI integration.

**Key Insight**: Napari uses a typed layer model system:
- **Layer Models**: Each data modality has a dedicated layer class with specific APIs
- **Typed APIs**: Type annotations define layer type constraints
- **Data Constraints**: Shape and dtype constraints enforced per layer type
- **Plugin Binding**: Widgets/actions bind to layer types via type annotations
- **Runtime Validation**: Type and data constraints validated at runtime

---

## 1. Layer Models for Data Modalities

### 1.1 Layer Class Hierarchy

Napari defines a hierarchy of layer classes, each representing a different data modality:

```
Layer (Base Class)
├── Image
├── Labels
├── Points
├── Shapes
├── Surface
├── Tracks
└── Vectors
```

Each layer class inherits from the base `Layer` class and provides:
- **Type-Specific API**: Methods and properties specific to the data modality
- **Data Constraints**: Shape and dtype requirements
- **Visualization**: Rendering specific to the data type
- **Interaction**: User interaction patterns for the data type

### 1.2 Image Layer Model

**Purpose**: Represents n-dimensional image data (intensity values)

**Data Constraints**:
- **Shape**: N-dimensional array (2D, 3D, 4D, etc.)
- **Dtype**: float32, float64, uint8, uint16, uint32, int8, int16, int32
- **RGB/RGBA**: Last dimension can be 3 or 4 for color channels

**Typed API**:
```python
from napari.layers import Image

class Image(Layer):
    data: np.ndarray  # N-dimensional array
    colormap: str
    contrast_limits: Tuple[float, float]
    gamma: float
    
    def __init__(self, data: ArrayLike, **kwargs):
        # Validates shape (>= 2D, <= 5D)
        # Validates dtype (numeric types)
        # Normalizes dtype if needed
        pass
```

**Usage**:
```python
# Create image layer
image = Image(data=np.random.random((100, 100)))
# ✅ Valid: 2D float array

# Access typed API
image.contrast_limits = (0, 1)
image.colormap = 'viridis'
image.gamma = 1.0
```

### 1.3 Labels Layer Model

**Purpose**: Represents integer-labeled segmentation data

**Data Constraints**:
- **Shape**: N-dimensional array matching image dimensions
- **Dtype**: int32, uint32, int16, uint16, int8, uint8, bool
- **Values**: Each integer represents a distinct label/region

**Typed API**:
```python
from napari.layers import Labels

class Labels(Layer):
    data: np.ndarray  # Integer array
    color: Dict[int, np.ndarray]  # Color mapping per label
    num_colors: int
    
    def __init__(self, data: ArrayLike, **kwargs):
        # Validates dtype (integer or bool)
        # Validates shape
        # Converts bool to uint8 if needed
        pass
```

**Usage**:
```python
# Create labels layer
labels = Labels(data=np.random.randint(0, 10, (100, 100), dtype=np.int32))
# ✅ Valid: 2D integer array

# Access typed API
labels.color[1] = np.array([1, 0, 0, 1])  # Red for label 1
labels.num_colors = 256
```

### 1.4 Points Layer Model

**Purpose**: Represents collections of points in n-dimensional space

**Data Constraints**:
- **Shape**: (N, D) array where N is number of points, D is dimensions
- **Dtype**: float32, float64 (coordinates)
- **Minimum**: At least 2D points (D >= 2)

**Typed API**:
```python
from napari.layers import Points

class Points(Layer):
    data: np.ndarray  # (N, D) array
    size: float
    face_color: str
    edge_color: str
    
    def __init__(self, data: ArrayLike, **kwargs):
        # Validates shape (must be 2D: N×D)
        # Validates dtype (float)
        # Validates minimum dimensions (D >= 2)
        pass
    
    def add(self, point: ArrayLike):
        """Add point to layer"""
        pass
    
    def remove(self, indices: ArrayLike):
        """Remove points from layer"""
        pass
```

**Usage**:
```python
# Create points layer
points = Points(data=np.array([[0, 0], [10, 10], [20, 20]], dtype=np.float32))
# ✅ Valid: 2D array of 2D points

# Access typed API
points.add([30, 30])
points.size = 5.0
points.face_color = 'red'
```

### 1.5 Shapes Layer Model

**Purpose**: Represents geometric shapes (polygons, rectangles, ellipses, lines)

**Data Constraints**:
- **Shape**: List of (N, D) arrays, each defining vertices of a shape
- **Dtype**: float32, float64 (vertex coordinates)
- **Structure**: Each shape can have different number of vertices

**Typed API**:
```python
from napari.layers import Shapes

class Shapes(Layer):
    data: List[np.ndarray]  # List of (N, D) arrays
    shape_type: List[str]  # 'polygon', 'rectangle', 'ellipse', 'line', 'path'
    face_color: str
    edge_color: str
    
    def __init__(self, data: List[ArrayLike], **kwargs):
        # Validates list structure
        # Validates each array is (N, D)
        # Validates dtype (float)
        pass
    
    def add_ellipse(self, center: ArrayLike, radii: ArrayLike):
        """Add ellipse shape"""
        pass
    
    def add_polygon(self, vertices: ArrayLike):
        """Add polygon shape"""
        pass
```

**Usage**:
```python
# Create shapes layer
polygon = np.array([[0, 0], [10, 0], [10, 10], [0, 10]], dtype=np.float32)
shapes = Shapes(data=[polygon])
# ✅ Valid: List of vertex arrays

# Access typed API
shapes.add_ellipse(center=[50, 50], radii=[20, 10])
shapes.face_color = 'blue'
```

### 1.6 Surface Layer Model

**Purpose**: Represents 3D surface meshes

**Data Constraints**:
- **Shape**: Tuple of (vertices, faces, values)
  - vertices: (V, D) array
  - faces: (F, 3) array of indices
  - values: (V,) array
- **Dtype**: float for vertices/values, int for faces

**Typed API**:
```python
from napari.layers import Surface

class Surface(Layer):
    data: Tuple[np.ndarray, np.ndarray, np.ndarray]  # (vertices, faces, values)
    colormap: str
    
    def __init__(self, data: Tuple[ArrayLike, ArrayLike, ArrayLike], **kwargs):
        # Validates tuple structure
        # Validates array shapes
        # Validates dtypes
        pass
```

### 1.7 Tracks Layer Model

**Purpose**: Represents time-series tracking data

**Data Constraints**:
- **Shape**: (N, D+1) array where D+1 includes track ID and coordinates
- **Dtype**: float for coordinates, int for track IDs

**Typed API**:
```python
from napari.layers import Tracks

class Tracks(Layer):
    data: np.ndarray  # (N, D+1) array
    graph: Dict[int, List[int]]  # Track graph
    
    def __init__(self, data: ArrayLike, **kwargs):
        # Validates shape (must be 2D: N×(D+1))
        # Validates structure
        pass
```

### 1.8 Vectors Layer Model

**Purpose**: Represents vector fields

**Data Constraints**:
- **Shape**: (N, 2, D) array where each vector has start and end points
- **Dtype**: float32, float64

**Typed API**:
```python
from napari.layers import Vectors

class Vectors(Layer):
    data: np.ndarray  # (N, 2, D) array
    edge_color: str
    length: float
    
    def __init__(self, data: ArrayLike, **kwargs):
        # Validates shape (must be 3D: N×2×D)
        # Validates dtype (float)
        pass
```

---

## 2. Typed Layer APIs

### 2.1 Type Annotations

Layer classes use type annotations to define their APIs:

```python
from napari.layers import Image, Labels, Points, Shapes
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from napari.viewer import Viewer

# Type annotations define layer type
def process_image(image: Image) -> LayerDataTuple:
    """Function accepts only Image layers"""
    pass

def process_labels(labels: Labels) -> LayerDataTuple:
    """Function accepts only Labels layers"""
    pass

def process_points(points: Points) -> LayerDataTuple:
    """Function accepts only Points layers"""
    pass
```

### 2.2 Type Checking

Runtime type checking validates layer types:

```python
def validate_layer_type(layer: Layer, expected_type: Type[Layer]) -> bool:
    """Check if layer matches expected type"""
    return isinstance(layer, expected_type)

# Usage
image = viewer.layers[0]
if validate_layer_type(image, Image):
    # Process as image
    process_image(image)
else:
    raise TypeError(f"Expected Image layer, got {type(image)}")
```

### 2.3 Union Types

Functions can accept multiple layer types:

```python
from typing import Union

def process_layer(layer: Union[Image, Labels]) -> LayerDataTuple:
    """Process Image or Labels layer"""
    if isinstance(layer, Image):
        return process_image_layer(layer)
    elif isinstance(layer, Labels):
        return process_labels_layer(layer)
    else:
        raise TypeError(f"Expected Image or Labels, got {type(layer)}")
```

---

## 3. Plugin Widget Binding to Layer Types

### 3.1 Type Annotation Binding

Plugin widgets bind to layer types via type annotations:

```python
from magicgui import magicgui
from napari.layers import Image, Labels

@magicgui
def filter_widget(
    image: Image,  # ← Binds to Image layer type
    threshold: float = 0.5
):
    """Widget bound to Image layer type"""
    filtered = image.data > threshold
    return filtered.astype(np.uint8) * 255
```

**What Happens**:
- **Widget Creation**: MagicGUI creates layer dropdown widget
- **Type Filtering**: Dropdown shows only Image layers
- **Validation**: Validates selected layer is Image type
- **Error Handling**: Shows error if wrong type selected

### 3.2 Explicit Widget Configuration

Widgets can be explicitly configured:

```python
from magicgui import magicgui

@magicgui(
    image={'widget_type': 'Layer', 'layer_type': Image},  # ← Explicit binding
    labels={'widget_type': 'Layer', 'layer_type': Labels, 'nullable': True}
)
def process_widget(
    image: Image,
    labels: Labels = None
):
    """Widget with explicit layer type binding"""
    pass
```

**Configuration Options**:
- `widget_type`: 'Layer' for layer selection
- `layer_type`: Specific layer class (Image, Labels, etc.)
- `nullable`: Whether None is allowed

### 3.3 Multiple Layer Type Binding

Widgets can bind to multiple layer types:

```python
from typing import Union

@magicgui
def process_widget(
    layer: Union[Image, Labels]  # ← Binds to Image OR Labels
):
    """Widget bound to multiple layer types"""
    if isinstance(layer, Image):
        # Handle Image
        pass
    elif isinstance(layer, Labels):
        # Handle Labels
        pass
```

**UI Behavior**:
- Dropdown shows both Image and Labels layers
- User can select either type
- Function handles both types

---

## 4. Plugin Action Binding to Layer Types

### 4.1 Command Type Constraints

Commands bind to layer types via function signatures:

```python
# my_plugin/commands.py
from napari.layers import Image, Labels
from napari.types import LayerDataTuple

def filter_image(
    viewer: Viewer,
    image: Image  # ← Type constraint
) -> LayerDataTuple:
    """Command bound to Image layer type"""
    filtered = image.data > 0.5
    return (filtered.astype(np.uint8) * 255, {'name': 'Filtered'}, 'image')
```

**Manifest Registration**:
```yaml
# napari.yaml
contributions:
  commands:
    - id: my-plugin.filter
      title: Filter Image
      python_name: my_plugin.commands:filter_image
      enablement: layer.selected and isinstance(layer, Image)
      # ↑ Command only enabled when Image layer selected
```

### 4.2 Enablement Conditions

Commands can have type-based enablement:

```python
# Enablement expression checks layer type
enablement: "layer.selected and isinstance(layer, Image)"
# ↑ Command only enabled when Image layer is selected
```

**Enablement Evaluation**:
- Evaluated in command context
- Checks layer type at runtime
- Updates dynamically as selection changes

---

## 5. Binding to Data Shapes and Dtypes

### 5.1 Shape Validation in Plugins

Plugins can validate data shapes:

```python
from napari.layers import Image
import numpy as np

@magicgui
def process_2d_image(image: Image):
    """Widget that requires 2D image"""
    # Validate shape
    if image.data.ndim != 2:
        raise ValueError(
            f"Image must be 2D, got {image.data.ndim}D. "
            f"Shape: {image.data.shape}"
        )
    
    # Process 2D image
    result = process_2d(image.data)
    return result
```

**Shape Validation**:
- **Runtime Checking**: Validates shape when function called
- **Error Messages**: Provides specific error messages
- **UI Feedback**: Shows errors in widget UI

### 5.2 Dtype Validation in Plugins

Plugins can validate data dtypes:

```python
from napari.layers import Image
import numpy as np

@magicgui
def process_float_image(image: Image):
    """Widget that requires float image"""
    # Validate dtype
    if not np.issubdtype(image.data.dtype, np.floating):
        raise TypeError(
            f"Image must be float dtype, got {image.data.dtype}"
        )
    
    # Process float image
    result = process_float(image.data)
    return result
```

**Dtype Validation**:
- **Type Checking**: Validates dtype matches requirements
- **Error Messages**: Clear error messages for mismatches
- **UI Feedback**: Shows errors in widget UI

### 5.3 Combined Shape and Dtype Validation

Plugins can validate both shape and dtype:

```python
@magicgui(auto_call=True)  # ← Validate on every change
def process_image(image: Image):
    """Widget with combined validation"""
    if image is None:
        return "No image selected"
    
    # Shape validation
    if image.data.ndim != 2:
        return f"Image must be 2D, got {image.data.ndim}D"
    
    # Dtype validation
    if not np.issubdtype(image.data.dtype, np.floating):
        return f"Image must be float, got {image.data.dtype}"
    
    # Size validation
    if image.data.shape[0] < 100 or image.data.shape[1] < 100:
        return "Image must be at least 100x100 pixels"
    
    # All validations passed
    return "Ready to process"
```

**Real-Time Validation**:
- **Auto-Call**: Validates on every change
- **Status Messages**: Shows validation status
- **Button State**: Disables "Run" if validation fails

---

## 6. UI Integration and Constraint Surfacing

### 6.1 Layer Type Dropdowns

Layer type constraints surface as filtered dropdowns:

```python
@magicgui
def process_image(image: Image):
    """Widget with Image layer constraint"""
    pass

# UI automatically:
# - Creates dropdown showing only Image layers
# - Filters out Labels, Points, Shapes layers
# - Updates when layers added/removed
# - Validates selection before enabling "Run"
```

**Visual Representation**:
```
┌─────────────────────────────────────┐
│ Process Image                       │
├─────────────────────────────────────┤
│ Image: [My Image ▼]                 │
│   - My Image                        │
│   - Another Image                   │
│   (Labels/Points/Shapes hidden)     │
│                                     │
│ [Run] (enabled if Image selected)   │
└─────────────────────────────────────┘
```

### 6.2 Shape/Dtype Validation Feedback

Shape and dtype constraints surface through validation:

```python
@magicgui(auto_call=True)
def process_2d_float_image(image: Image):
    """Widget with shape and dtype constraints"""
    if image is None:
        return "No image selected"
    
    if image.data.ndim != 2:
        return f"❌ Image must be 2D, got {image.data.ndim}D"
    
    if not np.issubdtype(image.data.dtype, np.floating):
        return f"❌ Image must be float, got {image.data.dtype}"
    
    return "✅ Ready to process"
```

**UI Feedback**:
- **Status Display**: Shows validation status
- **Error Messages**: Specific error messages
- **Button State**: Disables "Run" if validation fails
- **Tooltips**: Helpful tooltips explaining constraints

### 6.3 Multi-Layer Widgets

Widgets can bind to multiple layers with different constraints:

```python
@magicgui
def combine_layers(
    image: Image,  # ← 2D or 3D, any numeric dtype
    labels: Labels,  # ← Integer dtype, matching shape
    threshold: float = 0.5
):
    """Widget with multiple layer constraints"""
    # Validate shapes match
    if image.data.shape != labels.data.shape:
        raise ValueError(
            f"Image and Labels must have same shape. "
            f"Image: {image.data.shape}, Labels: {labels.data.shape}"
        )
    
    # Process
    result = combine(image.data, labels.data, threshold)
    return result
```

**UI Behavior**:
- Two separate dropdowns
- Each filtered to its layer type
- Shape validation when both selected
- Error messages for mismatches

---

## 7. Runtime Type and Data Validation

### 7.1 Type Checking Flow

Complete type checking flow:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: User Selects Layer                             │
│  User selects layer from dropdown                       │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: Type Validation                                │
│  Check isinstance(selected_layer, expected_type)       │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Valid Type   │ │ Invalid Type │ │ None         │
│ - Enable Run │ │ - Show Error │ │ - Disable Run│
│ - Clear Error│ │ - Disable Run│ │ - Show Hint  │
└──────────────┘ └──────────────┘ └──────────────┘
```

### 7.2 Data Validation Flow

Complete data validation flow:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: Function Called                                │
│  process_image(image, threshold)                        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: Type Validation                                │
│  Check isinstance(image, Image)                         │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 3: Shape Validation                               │
│  Check image.data.ndim == 2                             │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 4: Dtype Validation                               │
│  Check np.issubdtype(image.data.dtype, np.floating)    │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 5: Execute Function                               │
│  All validations passed, execute processing             │
└─────────────────────────────────────────────────────────┘
```

### 7.3 Validation Error Handling

Validation errors provide clear feedback:

```python
def validate_layer_for_processing(layer: Layer, expected_type: Type[Layer]):
    """Validate layer for processing"""
    # Type check
    if not isinstance(layer, expected_type):
        raise TypeError(
            f"Expected {expected_type.__name__} layer, "
            f"got {type(layer).__name__}"
        )
    
    # Shape check (if needed)
    if expected_type == Image:
        if layer.data.ndim < 2:
            raise ValueError(
                f"Image must be at least 2D, got {layer.data.ndim}D. "
                f"Shape: {layer.data.shape}"
            )
    
    # Dtype check (if needed)
    if expected_type == Labels:
        if not np.issubdtype(layer.data.dtype, np.integer):
            raise TypeError(
                f"Labels must be integer dtype, got {layer.data.dtype}"
            )
    
    return True
```

---

## 8. Complete Examples

### 8.1 Image Processing Widget

```python
from magicgui import magicgui
from napari.layers import Image
import numpy as np

@magicgui(
    image={'widget_type': 'Layer', 'layer_type': Image},
    threshold={'widget_type': 'FloatSlider', 'min': 0, 'max': 1}
)
def threshold_image(image: Image, threshold: float = 0.5):
    """Threshold image widget"""
    # Type constraint: image must be Image
    # Shape constraint: validated by Image layer
    # Dtype constraint: validated by Image layer
    
    # Process
    thresholded = image.data > threshold
    result = thresholded.astype(np.uint8) * 255
    
    # Return new layer
    return (result, {'name': f'{image.name}_thresholded'}, 'image')
```

**Binding**:
- **Layer Type**: Binds to Image layers only
- **Shape**: Inherits from Image layer validation
- **Dtype**: Inherits from Image layer validation

### 8.2 Labels Processing Widget

```python
from magicgui import magicgui
from napari.layers import Image, Labels
import numpy as np

@magicgui(
    image={'widget_type': 'Layer', 'layer_type': Image},
    labels={'widget_type': 'Layer', 'layer_type': Labels, 'nullable': True}
)
def segment_image(image: Image, labels: Labels = None):
    """Segment image with optional labels"""
    # Type constraints:
    # - image: Image (required)
    # - labels: Labels (optional)
    
    # Shape validation
    if labels is not None:
        if image.data.shape != labels.data.shape:
            raise ValueError(
                f"Image and Labels must have same shape. "
                f"Image: {image.data.shape}, Labels: {labels.data.shape}"
            )
    
    # Process
    if labels is None:
        # Create new labels
        segmented = image.data > 0.5
        result = segmented.astype(np.int32)
    else:
        # Update existing labels
        mask = image.data > 0.5
        result = labels.data.copy()
        result[mask] = np.max(result) + 1
    
    return (result, {'name': f'{image.name}_segmented'}, 'labels')
```

**Binding**:
- **Layer Types**: Binds to Image (required) and Labels (optional)
- **Shape Constraint**: Validates shapes match when both provided
- **Dtype Constraint**: Inherits from layer type validations

### 8.3 Points Processing Widget

```python
from magicgui import magicgui
from napari.layers import Points, Image
import numpy as np

@magicgui(
    points={'widget_type': 'Layer', 'layer_type': Points},
    image={'widget_type': 'Layer', 'layer_type': Image, 'nullable': True}
)
def filter_points(points: Points, image: Image = None):
    """Filter points based on image intensity"""
    # Type constraints:
    # - points: Points (required)
    # - image: Image (optional)
    
    # Shape validation
    if image is not None:
        # Points must be in image dimensions
        if points.data.shape[1] != image.data.ndim:
            raise ValueError(
                f"Points dimensions ({points.data.shape[1]}) must match "
                f"image dimensions ({image.data.ndim})"
            )
    
    # Dtype validation
    # Points data is already validated as float by Points layer
    # Image data is already validated by Image layer
    
    # Process
    if image is not None:
        # Filter points based on image intensity
        filtered = []
        for point in points.data:
            coords = tuple(int(c) for c in point)
            if image.data[coords] > 0.5:
                filtered.append(point)
        result = np.array(filtered)
    else:
        result = points.data
    
    return (result, {'name': f'{points.name}_filtered'}, 'points')
```

**Binding**:
- **Layer Types**: Binds to Points (required) and Image (optional)
- **Shape Constraint**: Validates point dimensions match image dimensions
- **Dtype Constraint**: Inherits from layer type validations

---

## 9. Summary

Napari uses layer models and typed APIs to represent different data modalities:

### Layer Models

1. **Image**: N-dimensional arrays, numeric dtypes
2. **Labels**: Integer arrays for segmentation
3. **Points**: (N, D) coordinate arrays, float dtypes
4. **Shapes**: List of vertex arrays, float dtypes
5. **Surface**: Tuple of (vertices, faces, values)
6. **Tracks**: (N, D+1) arrays for trajectories
7. **Vectors**: (N, 2, D) arrays for vector fields

### Typed APIs

- **Type Annotations**: Functions declare accepted layer types
- **Type Checking**: Runtime validation of layer types
- **Union Types**: Support for multiple layer types

### Plugin Binding

- **Widget Binding**: Type annotations create filtered dropdowns
- **Action Binding**: Commands bind via function signatures
- **Enablement**: Type-based enablement conditions

### Data Constraints

- **Shape Validation**: Runtime checking of array shapes
- **Dtype Validation**: Runtime checking of data types
- **Combined Validation**: Both shape and dtype constraints

### UI Integration

- **Filtered Dropdowns**: Show only matching layer types
- **Validation Feedback**: Real-time error messages
- **Button State**: Enable/disable based on constraints
- **Tooltips**: Helpful hints about constraints

This system enables type-safe, user-friendly plugins that integrate seamlessly with napari's layer models and validation systems.

