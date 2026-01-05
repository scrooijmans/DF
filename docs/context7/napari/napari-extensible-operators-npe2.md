# Napari Extensible Operators: NPE2 Command Registration and Type Constraints

## Executive Summary

This document provides a comprehensive technical analysis of how napari defines extensible "operators" via its plugin system (npe2) and command registration. It explains how functions declare accepted data types (layer types like Image/Labels/Points/Shapes), how type constraints are specified, and how these constraints are surfaced in the UI through magicgui and other mechanisms.

**Key Insight**: Napari uses type annotations and npe2 to define extensible operators:
- **Type Annotations**: Functions declare accepted layer types via type hints
- **NPE2 Commands**: Commands registered in manifest with type information
- **MagicGUI Integration**: Type annotations automatically create appropriate UI widgets
- **Runtime Validation**: Type constraints enforced at runtime
- **UI Feedback**: Constraints surfaced through dropdowns, validation, and error messages

---

## 1. Operator Definition via NPE2 Commands

### 1.1 Command Registration

Operators in napari are defined as commands in the npe2 plugin manifest:

```yaml
# napari.yaml (plugin manifest)
name: my-plugin
display_name: My Plugin
version: 0.1.0
contributions:
  commands:
    - id: my-plugin.filter_image
      title: Filter Image
      python_name: my_plugin.operators:filter_image
      # Command metadata
      category: Image Processing
```

### 1.2 Function Definition with Type Annotations

Functions declare accepted layer types via type annotations:

```python
# my_plugin/operators.py
from napari.layers import Image, Labels, Points, Shapes
from napari.types import LayerDataTuple
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from napari.viewer import Viewer

def filter_image(
    viewer: "Viewer",
    image: Image,  # ← Type constraint: must be Image layer
    threshold: float = 0.5
) -> LayerDataTuple:
    """Filter image layer"""
    import numpy as np
    
    # Process image data
    filtered = image.data > threshold
    result = filtered.astype(np.uint8) * 255
    
    # Return LayerData tuple
    return (
        result,
        {'name': f'{image.name}_filtered'},
        'image'
    )
```

**Type Annotations Define Constraints**:
- `image: Image` → Function accepts only Image layers
- `labels: Labels` → Function accepts only Labels layers
- `points: Points` → Function accepts only Points layers
- `shapes: Shapes` → Function accepts only Shapes layers

### 1.3 Multiple Layer Type Support

Functions can accept multiple layer types:

```python
from napari.layers import Image, Labels
from typing import Union

def process_layer(
    viewer: "Viewer",
    layer: Union[Image, Labels],  # ← Accepts Image OR Labels
    operation: str = 'threshold'
) -> LayerDataTuple:
    """Process Image or Labels layer"""
    if isinstance(layer, Image):
        result = process_image(layer.data, operation)
    elif isinstance(layer, Labels):
        result = process_labels(layer.data, operation)
    
    return (result, {'name': f'{layer.name}_processed'}, layer.__class__.__name__.lower())
```

---

## 2. Type Constraint Declaration

### 2.1 Type Annotation Patterns

Different patterns for declaring type constraints:

#### 2.1.1 Single Layer Type

```python
def process_image(image: Image) -> LayerDataTuple:
    """Accepts only Image layers"""
    pass
```

#### 2.1.2 Union Types

```python
from typing import Union

def process_layer(layer: Union[Image, Labels]) -> LayerDataTuple:
    """Accepts Image or Labels layers"""
    pass
```

#### 2.1.3 Optional Layers

```python
from typing import Optional

def process_with_optional(
    image: Image,
    labels: Optional[Labels] = None
) -> LayerDataTuple:
    """Image required, Labels optional"""
    pass
```

#### 2.1.4 Multiple Required Layers

```python
def combine_layers(
    image: Image,
    labels: Labels
) -> LayerDataTuple:
    """Requires both Image and Labels"""
    pass
```

### 2.2 Type Checking at Runtime

Type constraints are checked at runtime:

```python
# NPE2 command execution
def execute_command(command_id: str, context: dict):
    """Execute command with type checking"""
    command = registry.get(command_id)
    func = command.load_function()
    
    # Get function signature
    sig = inspect.signature(func)
    
    # Validate arguments
    for param_name, param in sig.parameters.items():
        if param_name in context:
            expected_type = param.annotation
            actual_value = context[param_name]
            
            # Check type constraint
            if not isinstance(actual_value, expected_type):
                raise TypeError(
                    f"Parameter {param_name} expected {expected_type}, "
                    f"got {type(actual_value)}"
                )
    
    # Execute function
    return func(**context)
```

---

## 3. UI Integration via MagicGUI

### 3.1 Automatic Widget Generation

MagicGUI automatically creates UI widgets from type annotations:

```python
from magicgui import magicgui
from napari.layers import Image, Labels

@magicgui
def filter_operator(
    image: Image,  # ← Creates layer dropdown widget
    threshold: float = 0.5  # ← Creates float slider
):
    """Filter operator with UI"""
    filtered = image.data > threshold
    return filtered.astype(np.uint8) * 255
```

**What MagicGUI Does**:
- **Layer Type Annotations**: Creates dropdown widgets showing only matching layers
- **Type Filtering**: Filters layers by type (Image, Labels, etc.)
- **Validation**: Disables "Run" button if constraints not met
- **Error Display**: Shows error messages for invalid selections

### 3.2 Widget Type Configuration

Widgets can be configured explicitly:

```python
from magicgui import magicgui

@magicgui(
    image={'widget_type': 'Layer', 'layer_type': Image},  # ← Explicit config
    threshold={'widget_type': 'FloatSlider', 'min': 0, 'max': 1}
)
def filter_operator(image: Image, threshold: float = 0.5):
    """Filter operator with explicit widget config"""
    pass
```

**Widget Configuration Options**:
- `widget_type`: Type of widget ('Layer', 'FloatSlider', etc.)
- `layer_type`: Specific layer type for layer widgets
- `min`/`max`: Range constraints for numeric widgets
- `nullable`: Whether None is allowed

### 3.3 Layer Dropdown Widget

Layer type annotations create filtered dropdowns:

```python
@magicgui
def process_image(image: Image):
    """Image processing operator"""
    pass

# UI automatically:
# - Creates dropdown showing only Image layers
# - Filters out Labels, Points, Shapes layers
# - Updates when layers are added/removed
# - Validates selection before enabling "Run"
```

**Dropdown Behavior**:
- **Filtered by Type**: Only shows layers matching the type annotation
- **Dynamic Updates**: Updates when layers are added/removed
- **Selection Validation**: Validates selection before execution
- **Error Feedback**: Shows error if invalid layer selected

---

## 4. Constraint Surfacing in UI

### 4.1 Dropdown Filtering

Layer type constraints are surfaced through filtered dropdowns:

```python
# Function definition
@magicgui
def threshold_image(image: Image, threshold: float = 0.5):
    """Threshold image layer"""
    pass

# UI behavior:
# 1. Dropdown shows only Image layers
# 2. Labels, Points, Shapes layers are hidden
# 3. If no Image layers exist, dropdown is empty
# 4. "Run" button is disabled if no valid layer selected
```

**Visual Representation**:
```
┌─────────────────────────────────────┐
│ Filter Image                        │
├─────────────────────────────────────┤
│ Image: [Dropdown ▼]                 │
│   - My Image                        │
│   - Another Image                   │
│   (Labels/Points/Shapes hidden)     │
│                                     │
│ Threshold: [0.5] ━━━━━━━━━━━━━━━  │
│                                     │
│ [Run] (enabled if Image selected)   │
└─────────────────────────────────────┘
```

### 4.2 Validation Feedback

Constraints are validated and feedback is provided:

```python
@magicgui(auto_call=True)  # ← Validate on every change
def process_image(image: Image):
    """Process image with real-time validation"""
    if image is None:
        return "No image selected"
    
    if image.data.ndim != 2:
        return f"Image must be 2D, got {image.data.ndim}D"
    
    return "Ready to process"
```

**Validation Feedback**:
- **Real-Time Validation**: Validates as user selects layers
- **Error Messages**: Shows specific error messages
- **Button State**: Disables "Run" button if validation fails
- **Tooltips**: Provides helpful tooltips explaining constraints

### 4.3 Enablement Conditions

Commands can have enablement conditions:

```yaml
# manifest.yaml
contributions:
  commands:
    - id: my-plugin.filter
      title: Filter Image
      python_name: my_plugin.operators:filter_image
      enablement: layer.selected and isinstance(layer, Image)
      # ↑ Command only enabled when Image layer is selected
```

**Enablement Expressions**:
- **Context-Based**: Evaluated in command context
- **Type Checking**: Can check layer types
- **State-Dependent**: Can depend on viewer state
- **Dynamic**: Updates as state changes

### 4.4 Error Messages

Type constraint violations show clear error messages:

```python
# User selects Labels layer for Image-requiring function
# Error displayed:
"Parameter 'image' expected Image layer, got Labels layer"

# User selects no layer
# Error displayed:
"Parameter 'image' is required but no Image layer is selected"
```

---

## 5. Complete Operator Example

### 5.1 Operator Definition

```python
# my_plugin/operators.py
from napari.layers import Image, Labels
from napari.types import LayerDataTuple
from typing import TYPE_CHECKING, Optional
import numpy as np

if TYPE_CHECKING:
    from napari.viewer import Viewer

def segment_image(
    viewer: "Viewer",
    image: Image,  # ← Type constraint: Image required
    labels: Optional[Labels] = None,  # ← Optional Labels
    threshold: float = 0.5
) -> LayerDataTuple:
    """Segment image with optional existing labels"""
    
    # Process image
    if labels is None:
        # Create new labels
        segmented = image.data > threshold
        result = segmented.astype(np.int32)
    else:
        # Update existing labels
        mask = image.data > threshold
        result = labels.data.copy()
        result[mask] = np.max(result) + 1
    
    return (
        result,
        {'name': f'{image.name}_segmented'},
        'labels'
    )
```

### 5.2 Manifest Registration

```yaml
# napari.yaml
name: my-plugin
display_name: My Plugin
version: 0.1.0
contributions:
  commands:
    - id: my-plugin.segment
      title: Segment Image
      python_name: my_plugin.operators:segment_image
      category: Segmentation
```

### 5.3 UI Widget Creation

```python
# Widget automatically created from type annotations:
# - image: Image → Dropdown showing only Image layers
# - labels: Optional[Labels] → Dropdown showing Labels layers (nullable)
# - threshold: float → Float slider (0.0 to 1.0)

# UI appearance:
┌─────────────────────────────────────┐
│ Segment Image                       │
├─────────────────────────────────────┤
│ Image: [My Image ▼]                 │
│   - My Image                        │
│   - Another Image                   │
│                                     │
│ Labels: [None ▼]                    │
│   - None                            │
│   - Existing Labels                 │
│                                     │
│ Threshold: [0.5] ━━━━━━━━━━━━━━━  │
│                                     │
│ [Run]                               │
└─────────────────────────────────────┘
```

### 5.4 Runtime Validation

```python
# When user clicks "Run":
# 1. Validate image is Image layer
if not isinstance(image, Image):
    raise TypeError("image must be Image layer")

# 2. Validate labels is Labels or None
if labels is not None and not isinstance(labels, Labels):
    raise TypeError("labels must be Labels layer or None")

# 3. Validate threshold range
if not 0.0 <= threshold <= 1.0:
    raise ValueError("threshold must be between 0 and 1")

# 4. Execute function
result = segment_image(viewer, image, labels, threshold)
```

---

## 6. Advanced Type Constraints

### 6.1 Multiple Layer Types

Functions can accept multiple layer types with different handling:

```python
from napari.layers import Image, Labels, Points
from typing import Union

def process_multiple_types(
    layer: Union[Image, Labels, Points]
) -> LayerDataTuple:
    """Process different layer types"""
    if isinstance(layer, Image):
        result = process_image(layer.data)
        layer_type = 'image'
    elif isinstance(layer, Labels):
        result = process_labels(layer.data)
        layer_type = 'labels'
    elif isinstance(layer, Points):
        result = process_points(layer.data)
        layer_type = 'points'
    
    return (result, {'name': f'{layer.name}_processed'}, layer_type)
```

**UI Behavior**:
- Dropdown shows all matching layer types (Image, Labels, Points)
- User can select any of these types
- Function handles each type appropriately

### 6.2 Layer Type Constraints with Conditions

Functions can have conditional type constraints:

```python
from napari.layers import Image, Labels
from typing import Union

def combine_layers(
    image: Image,
    labels: Labels,
    operation: str = 'overlay'
) -> LayerDataTuple:
    """Combine Image and Labels layers"""
    # Both types required
    # UI shows two separate dropdowns
    # Each filtered to its specific type
    pass
```

**UI Behavior**:
- Two separate dropdowns
- First dropdown: Only Image layers
- Second dropdown: Only Labels layers
- Both must be selected for "Run" to be enabled

### 6.3 Generic Layer Constraints

Functions can accept any layer type:

```python
from napari.layers import Layer

def process_any_layer(layer: Layer) -> LayerDataTuple:
    """Process any layer type"""
    # Accepts Image, Labels, Points, Shapes, etc.
    # UI shows dropdown with all layer types
    pass
```

**UI Behavior**:
- Dropdown shows all layer types
- No type filtering
- Function must handle all types

---

## 7. UI Widget Types

### 7.1 Layer Widget

Layer type annotations create layer widgets:

```python
@magicgui
def my_operator(layer: Image):
    """Operator with layer widget"""
    pass

# Widget properties:
# - Type: Dropdown
# - Options: Filtered by layer type
# - Updates: When layers added/removed
# - Validation: Type checking
```

### 7.2 Numeric Widgets

Numeric types create appropriate widgets:

```python
@magicgui
def my_operator(
    threshold: float = 0.5,  # → FloatSlider
    iterations: int = 10,    # → IntSlider
    sigma: float = 1.0       # → FloatSpinBox
):
    """Operator with numeric parameters"""
    pass
```

### 7.3 Boolean Widgets

Boolean types create checkboxes:

```python
@magicgui
def my_operator(
    invert: bool = False,  # → Checkbox
    normalize: bool = True  # → Checkbox
):
    """Operator with boolean parameters"""
    pass
```

### 7.4 String Widgets

String types create text inputs:

```python
@magicgui
def my_operator(
    name: str = "Result",  # → LineEdit
    method: str = "gaussian"  # → ComboBox (if choices provided)
):
    """Operator with string parameters"""
    pass
```

---

## 8. Constraint Validation Flow

### 8.1 UI Validation

Validation happens at multiple levels:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: User Selects Layer                             │
│  User selects layer from dropdown                       │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: Type Checking                                  │
│  Check if selected layer matches type annotation        │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Valid        │ │ Invalid     │ │ None        │
│ - Enable Run │ │ - Show Error│ │ - Disable Run│
│ - Clear Error│ │ - Disable Run│ │ - Show Hint │
└──────────────┘ └──────────────┘ └──────────────┘
```

### 8.2 Runtime Validation

Runtime validation before execution:

```python
def execute_operator(func, context: dict):
    """Execute operator with validation"""
    
    # Step 1: Get function signature
    sig = inspect.signature(func)
    
    # Step 2: Validate each parameter
    for param_name, param in sig.parameters.items():
        if param_name in context:
            value = context[param_name]
            expected_type = param.annotation
            
            # Check type
            if not isinstance(value, expected_type):
                raise TypeError(
                    f"Parameter {param_name} expected {expected_type}, "
                    f"got {type(value)}"
                )
    
    # Step 3: Execute if all valid
    return func(**context)
```

---

## 9. Plugin Integration

### 9.1 Widget Contribution

Plugins can contribute widgets with type constraints:

```yaml
# manifest.yaml
contributions:
  widgets:
    - command: my-plugin.filter_widget
      display_name: Filter Widget
```

```python
# my_plugin/widgets.py
from magicgui import magicgui
from napari.layers import Image

@magicgui
def filter_widget(
    image: Image,  # ← Type constraint
    threshold: float = 0.5
):
    """Filter widget with type constraints"""
    filtered = image.data > threshold
    return filtered.astype(np.uint8) * 255
```

### 9.2 Command Contribution

Commands can be contributed with type information:

```yaml
# manifest.yaml
contributions:
  commands:
    - id: my-plugin.filter
      title: Filter Image
      python_name: my_plugin.operators:filter_image
      # Type information inferred from function signature
```

---

## 10. Summary

Napari defines extensible operators through npe2 and type annotations:

### Key Mechanisms

1. **Type Annotations**: Functions declare accepted layer types via type hints
2. **NPE2 Commands**: Commands registered in manifest with function references
3. **MagicGUI Integration**: Type annotations automatically create UI widgets
4. **Runtime Validation**: Type constraints enforced before execution
5. **UI Feedback**: Constraints surfaced through dropdowns, validation, and errors

### Type Constraint Declaration

- **Single Type**: `image: Image` → Only Image layers
- **Union Types**: `layer: Union[Image, Labels]` → Image or Labels
- **Optional**: `labels: Optional[Labels]` → Labels or None
- **Multiple**: `image: Image, labels: Labels` → Both required

### UI Surfacing

- **Filtered Dropdowns**: Show only matching layer types
- **Validation Feedback**: Real-time error messages
- **Button State**: Enable/disable based on constraints
- **Tooltips**: Helpful hints about constraints

### Benefits

- **Type Safety**: Prevents incorrect layer type usage
- **User-Friendly**: Clear UI feedback about constraints
- **Extensible**: Easy to add new operators
- **Consistent**: Standardized approach across plugins

This system enables developers to create type-safe, user-friendly operators that integrate seamlessly with napari's UI and validation systems.

