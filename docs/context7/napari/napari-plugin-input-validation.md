# Napari Plugin Input Validation: Active Layer Checks, Selection Models, and Runtime Guards

## Executive Summary

This document provides a comprehensive technical analysis of how napari validates plugin inputs, including active layer type checks, selection models, function annotations, runtime guards, and the complete call chain from menu action to plugin execution. It explains how napari ensures type safety and data integrity throughout the plugin execution pipeline.

**Key Insight**: Napari uses a multi-layered validation system:
- **Enablement Conditions**: Commands conditionally enabled based on selection state
- **Selection Models**: `layers.selection.active` provides active layer context
- **Function Annotations**: Type hints define expected layer types
- **Runtime Guards**: Validation checks before and during execution
- **Call Chain**: Structured flow from menu → command → validation → execution

---

## 1. Selection Models and Active Layer Context

### 1.1 Layer Selection Model

Napari maintains a selection model that tracks the active layer:

```python
# Location: napari/components/layerlist.py
class LayerList(EventedList):
    def __init__(self):
        self.selection = Selection()  # Selection model
    
    @property
    def active(self) -> Optional[Layer]:
        """Get currently active/selected layer"""
        return self.selection.active

class Selection(EventedModel):
    """Selection model for layer list"""
    active: Optional[Layer] = None
    
    @property
    def active(self) -> Optional[Layer]:
        return self._active
    
    @active.setter
    def active(self, value: Optional[Layer]):
        self._active = value
        self.events.active(value=value)  # Emit event
```

**Selection Model Properties**:
- **Active Layer**: Currently selected layer (can be None)
- **Event Emission**: Emits events when selection changes
- **Type Tracking**: Selection can be any layer type

### 1.2 Accessing Active Layer

Plugins access the active layer through the selection model:

```python
# Get active layer
active_layer = viewer.layers.selection.active

# Check if layer is selected
if active_layer is None:
    raise ValueError("No layer selected")

# Check layer type
if not isinstance(active_layer, Image):
    raise TypeError(f"Expected Image layer, got {type(active_layer)}")
```

---

## 2. Enablement Conditions

### 2.1 Command Enablement

Commands can have enablement conditions that check selection state:

```yaml
# manifest.yaml
contributions:
  commands:
    - id: my-plugin.filter
      title: Filter Image
      python_name: my_plugin.commands:filter_image
      enablement: layer.selected and isinstance(layer, Image)
      # ↑ Command only enabled when Image layer is selected
```

**Enablement Expression Evaluation**:
- **Context**: Evaluated in command context with access to viewer state
- **Variables**: `layer` refers to `viewer.layers.selection.active`
- **Functions**: Can use `isinstance()`, `hasattr()`, etc.
- **Dynamic**: Re-evaluated when selection changes

### 2.2 Enablement Expression Syntax

Enablement expressions support various checks:

```yaml
# Enablement examples:

# Only when layer is selected
enablement: layer.selected

# Only when Image layer is selected
enablement: layer.selected and isinstance(layer, Image)

# Only when Labels layer is selected
enablement: layer.selected and isinstance(layer, Labels)

# Only when layer has data
enablement: layer.selected and layer.data is not None

# Only when multiple layers exist
enablement: len(layers) > 0

# Complex condition
enablement: layer.selected and isinstance(layer, Image) and layer.data.ndim == 2
```

### 2.3 Enablement Evaluation

Enablement is evaluated dynamically:

```python
# Location: napari/_app_model/commands.py
class Command:
    def is_enabled(self, context: dict) -> bool:
        """Check if command is enabled"""
        if self.enablement is None:
            return True
        
        # Build evaluation context
        eval_context = {
            'layer': context.get('layer') or context.get('viewer').layers.selection.active,
            'layers': context.get('viewer').layers,
            'viewer': context.get('viewer'),
            'isinstance': isinstance,
            'hasattr': hasattr,
            'len': len,
            # ... other built-ins
        }
        
        # Evaluate enablement expression
        try:
            return bool(eval(self.enablement, eval_context))
        except Exception:
            return False
```

**Enablement Evaluation Flow**:
1. **Context Building**: Creates evaluation context with viewer state
2. **Expression Evaluation**: Evaluates enablement expression
3. **Result**: Returns boolean indicating if command is enabled
4. **UI Update**: Menu items/buttons update based on enablement

---

## 3. Function Annotations and Type Checking

### 3.1 Type Annotations in Plugin Functions

Plugin functions use type annotations to declare expected types:

```python
# my_plugin/commands.py
from napari.layers import Image, Labels
from napari.types import LayerDataTuple
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from napari.viewer import Viewer

def filter_image(
    viewer: "Viewer",
    image: Image  # ← Type annotation: expects Image layer
) -> LayerDataTuple:
    """Filter image layer"""
    # Function signature declares image must be Image type
    filtered = image.data > 0.5
    return (filtered.astype(np.uint8) * 255, {'name': 'Filtered'}, 'image')
```

**Type Annotation Benefits**:
- **Documentation**: Clear indication of expected types
- **IDE Support**: Autocomplete and type checking
- **Runtime Validation**: Can be checked at runtime
- **UI Generation**: MagicGUI uses annotations for widget creation

### 3.2 Runtime Type Checking

Type annotations can be checked at runtime:

```python
# Location: napari/_app_model/commands.py
def execute_command(command_id: str, context: dict):
    """Execute command with type checking"""
    command = registry.get(command_id)
    func = command.load_function()
    
    # Get function signature
    sig = inspect.signature(func)
    
    # Validate arguments against annotations
    for param_name, param in sig.parameters.items():
        if param_name in context:
            expected_type = param.annotation
            actual_value = context[param_name]
            
            # Skip if no annotation or Any
            if expected_type == inspect.Parameter.empty:
                continue
            if expected_type == Any:
                continue
            
            # Check type
            if not isinstance(actual_value, expected_type):
                raise TypeError(
                    f"Parameter '{param_name}' expected {expected_type.__name__}, "
                    f"got {type(actual_value).__name__}"
                )
    
    # Execute function
    return func(**context)
```

---

## 4. Runtime Guards

### 4.1 Pre-Execution Guards

Guards check conditions before execution:

```python
# Plugin function with guards
def filter_image(viewer: Viewer, image: Image) -> LayerDataTuple:
    """Filter image with runtime guards"""
    
    # Guard 1: Check layer is not None
    if image is None:
        raise ValueError("No image layer provided")
    
    # Guard 2: Check layer type
    if not isinstance(image, Image):
        raise TypeError(f"Expected Image layer, got {type(image)}")
    
    # Guard 3: Check data exists
    if image.data is None:
        raise ValueError("Image layer has no data")
    
    # Guard 4: Check data shape
    if image.data.ndim < 2:
        raise ValueError(
            f"Image must be at least 2D, got {image.data.ndim}D. "
            f"Shape: {image.data.shape}"
        )
    
    # Guard 5: Check dtype
    if not np.issubdtype(image.data.dtype, np.number):
        raise TypeError(
            f"Image data must be numeric, got {image.data.dtype}"
        )
    
    # All guards passed, proceed with execution
    filtered = image.data > 0.5
    return (filtered.astype(np.uint8) * 255, {'name': 'Filtered'}, 'image')
```

### 4.2 Guard Patterns

Common guard patterns in napari plugins:

```python
# Pattern 1: Active layer check
def process_active_layer(viewer: Viewer):
    """Process active layer with guards"""
    active = viewer.layers.selection.active
    
    # Guard: Check selection exists
    if active is None:
        raise ValueError("No layer selected")
    
    # Guard: Check layer type
    if not isinstance(active, Image):
        raise TypeError(f"Expected Image layer, got {type(active)}")
    
    # Process
    process(active)

# Pattern 2: Multiple layer check
def combine_layers(viewer: Viewer, image: Image, labels: Labels):
    """Combine layers with guards"""
    # Guard: Check both layers exist
    if image is None or labels is None:
        raise ValueError("Both image and labels layers required")
    
    # Guard: Check shapes match
    if image.data.shape != labels.data.shape:
        raise ValueError(
            f"Shapes must match. Image: {image.data.shape}, "
            f"Labels: {labels.data.shape}"
        )
    
    # Process
    combine(image, labels)

# Pattern 3: Data validation
def process_data(layer: Image):
    """Process with data validation"""
    # Guard: Check data exists
    if layer.data is None:
        raise ValueError("Layer has no data")
    
    # Guard: Check data shape
    if layer.data.ndim != 2:
        raise ValueError(f"Expected 2D data, got {layer.data.ndim}D")
    
    # Guard: Check dtype
    if not np.issubdtype(layer.data.dtype, np.floating):
        raise TypeError(f"Expected float data, got {layer.data.dtype}")
    
    # Process
    process(layer.data)
```

---

## 5. Complete Call Chain

### 5.1 Menu Action to Plugin Execution

Complete call chain from menu action to plugin execution:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: User Clicks Menu Item                          │
│  User selects "Filter Image" from menu                  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: Menu System Triggers Action                    │
│  Menu item → Action object → Command ID                 │
│  Location: napari/_qt/qt_main_window.py                │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 3: Enablement Check                                │
│  Check if command is enabled                             │
│  Location: napari/_app_model/commands.py                 │
│  - Evaluate enablement expression                        │
│  - Check layer.selected                                 │
│  - Check isinstance(layer, Image)                        │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Enabled      │ │ Disabled     │ │ Error        │
│ - Continue   │ │ - Disable UI │ │ - Show Error │
└──────┬───────┘ └──────────────┘ └──────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│  Step 4: Command Registry Lookup                         │
│  Get command by ID                                       │
│  Location: napari/_app_model/commands.py                │
│  - Look up command in registry                           │
│  - Load function from python_name                        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 5: Context Building                                │
│  Build execution context                                 │
│  Location: napari/_app_model/commands.py                 │
│  - Get active layer: viewer.layers.selection.active      │
│  - Build context dict                                    │
│  - Include viewer, layers, selection                     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 6: Type Validation                                 │
│  Validate function arguments                             │
│  Location: napari/_app_model/commands.py                 │
│  - Get function signature                                │
│  - Check type annotations                                │
│  - Validate against context                              │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Valid Types  │ │ Invalid Types│ │ Missing Args │
│ - Continue   │ │ - Raise Error│ │ - Raise Error│
└──────┬───────┘ └──────────────┘ └──────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│  Step 7: Function Execution                             │
│  Call plugin function                                    │
│  Location: my_plugin/commands.py                         │
│  - Execute function with context                        │
│  - Runtime guards execute                                │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 8: Runtime Guards                                  │
│  Plugin function guards                                  │
│  Location: my_plugin/commands.py                         │
│  - Check layer is not None                              │
│  - Check layer type                                     │
│  - Check data exists                                    │
│  - Check data shape/dtype                                │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Guards Pass  │ │ Guards Fail  │ │ Error        │
│ - Execute    │ │ - Raise Error│ │ - Show Error │
└──────┬───────┘ └──────────────┘ └──────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│  Step 9: Plugin Function Execution                      │
│  Execute plugin logic                                    │
│  Location: my_plugin/commands.py                         │
│  - Process data                                          │
│  - Return result                                         │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 10: Result Handling                                │
│  Handle function return value                            │
│  Location: napari/_app_model/commands.py                 │
│  - If LayerData tuple: create layer                     │
│  - If None: no action                                   │
│  - Emit events                                          │
└─────────────────────────────────────────────────────────┘
```

### 5.2 Code Flow Example

Complete code flow for a plugin command:

```python
# Step 1: User clicks menu
# Menu item "Filter Image" clicked

# Step 2: Menu system
# Location: napari/_qt/qt_main_window.py
def on_menu_action_triggered(self, action_id: str):
    """Handle menu action"""
    # Get command ID from action
    command_id = self._action_to_command[action_id]
    
    # Execute command
    self._app.execute_command(command_id)

# Step 3: Enablement check
# Location: napari/_app_model/commands.py
def execute_command(self, command_id: str):
    """Execute command with enablement check"""
    command = self.registry.get(command_id)
    
    # Check enablement
    context = self._build_context()
    if not command.is_enabled(context):
        return  # Command disabled, do nothing
    
    # Continue execution
    self._execute_command_internal(command, context)

# Step 4: Command registry lookup
def _execute_command_internal(self, command: Command, context: dict):
    """Execute command"""
    # Load function
    func = command.load_function()
    # func = my_plugin.commands.filter_image
    
    # Build execution context
    context['viewer'] = self.viewer
    context['layer'] = self.viewer.layers.selection.active
    
    # Validate types
    self._validate_types(func, context)
    
    # Execute
    result = func(**context)
    
    # Handle result
    self._handle_result(result)

# Step 5: Type validation
def _validate_types(self, func: Callable, context: dict):
    """Validate function argument types"""
    sig = inspect.signature(func)
    
    for param_name, param in sig.parameters.items():
        if param_name in context:
            expected_type = param.annotation
            actual_value = context[param_name]
            
            if expected_type != inspect.Parameter.empty:
                if not isinstance(actual_value, expected_type):
                    raise TypeError(
                        f"Parameter '{param_name}' expected {expected_type.__name__}, "
                        f"got {type(actual_value).__name__}"
                    )

# Step 6: Plugin function execution
# Location: my_plugin/commands.py
def filter_image(viewer: Viewer, image: Image) -> LayerDataTuple:
    """Plugin function with guards"""
    # Runtime guard 1: Check layer exists
    if image is None:
        raise ValueError("No image layer provided")
    
    # Runtime guard 2: Check layer type
    if not isinstance(image, Image):
        raise TypeError(f"Expected Image layer, got {type(image)}")
    
    # Runtime guard 3: Check data
    if image.data is None:
        raise ValueError("Image layer has no data")
    
    # Process
    filtered = image.data > 0.5
    result = filtered.astype(np.uint8) * 255
    
    return (result, {'name': 'Filtered'}, 'image')

# Step 7: Result handling
def _handle_result(self, result):
    """Handle command result"""
    if result is None:
        return
    
    if isinstance(result, tuple) and len(result) == 3:
        # LayerData tuple
        data, metadata, layer_type = result
        layer = self.viewer._add_layer_from_data(data, metadata, layer_type)
        return layer
```

---

## 6. Validation Layers

### 6.1 Multi-Layer Validation

Validation happens at multiple layers:

```
┌─────────────────────────────────────────────────────────┐
│  Layer 1: Enablement Check                               │
│  - Check enablement expression                           │
│  - Check layer.selected                                 │
│  - Check layer type in expression                        │
│  Location: Command.is_enabled()                          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Layer 2: Type Annotation Validation                    │
│  - Check function signature                              │
│  - Validate type annotations                            │
│  - Check isinstance() matches annotations                │
│  Location: execute_command() type checking               │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Layer 3: Runtime Guards                                 │
│  - Check layer is not None                              │
│  - Check layer type                                     │
│  - Check data exists                                    │
│  - Check data shape/dtype                                │
│  Location: Plugin function guards                        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Layer 4: Layer Model Validation                        │
│  - Layer constructor validation                         │
│  - Pydantic validators                                  │
│  - Data shape/dtype validation                           │
│  Location: Layer.__init__()                             │
└─────────────────────────────────────────────────────────┘
```

### 6.2 Validation Order

Validation occurs in specific order:

1. **Enablement Check** (Before menu/button enabled)
   - Prevents invalid commands from being accessible
   - Updates UI based on selection state

2. **Type Annotation Check** (Before execution)
   - Validates function arguments match annotations
   - Catches type mismatches early

3. **Runtime Guards** (During execution)
   - Plugin-specific validation
   - Data shape/dtype checks
   - Business logic validation

4. **Layer Model Validation** (If creating layers)
   - Layer constructor validation
   - Pydantic validators
   - Final data validation

---

## 7. Selection Model Integration

### 7.1 Accessing Selection in Plugins

Plugins access selection through viewer:

```python
# Method 1: Direct access
active_layer = viewer.layers.selection.active

# Method 2: Through layers property
active_layer = viewer.layers.selection.active

# Method 3: Check if selected
if viewer.layers.selection.active is not None:
    layer = viewer.layers.selection.active
    # Process layer
```

### 7.2 Selection Change Events

Plugins can listen to selection changes:

```python
@viewer.layers.selection.events.active.connect
def on_selection_changed(event):
    """React to selection changes"""
    new_layer = event.value
    
    if new_layer is None:
        # No layer selected
        disable_plugin_ui()
    elif isinstance(new_layer, Image):
        # Image layer selected
        enable_plugin_ui()
        update_ui_for_image(new_layer)
    else:
        # Other layer type selected
        disable_plugin_ui()
```

### 7.3 Selection in Enablement Expressions

Enablement expressions use selection:

```yaml
# manifest.yaml
contributions:
  commands:
    - id: my-plugin.filter
      enablement: layer.selected and isinstance(layer, Image)
      # ↑ layer refers to viewer.layers.selection.active
```

**Enablement Context**:
```python
# Enablement evaluation context
{
    'layer': viewer.layers.selection.active,  # Active layer
    'layers': viewer.layers,                   # All layers
    'viewer': viewer,                          # Viewer instance
    'isinstance': isinstance,                  # Type checking
    'hasattr': hasattr,                       # Attribute checking
    'len': len,                                # Length function
    # ... other built-ins
}
```

---

## 8. Error Handling

### 8.1 Validation Error Types

Different validation errors are raised:

```python
# TypeError: Wrong layer type
if not isinstance(layer, Image):
    raise TypeError(
        f"Expected Image layer, got {type(layer).__name__}"
    )

# ValueError: Missing selection
if layer is None:
    raise ValueError("No layer selected")

# ValueError: Invalid data shape
if layer.data.ndim != 2:
    raise ValueError(
        f"Expected 2D data, got {layer.data.ndim}D. "
        f"Shape: {layer.data.shape}"
    )

# TypeError: Invalid dtype
if not np.issubdtype(layer.data.dtype, np.floating):
    raise TypeError(
        f"Expected float data, got {layer.data.dtype}"
    )
```

### 8.2 Error Display

Errors are displayed to users:

```python
# Location: napari/_qt/qt_main_window.py
def execute_command(self, command_id: str):
    """Execute command with error handling"""
    try:
        result = self._app.execute_command(command_id)
        return result
    except TypeError as e:
        # Type error
        self._show_error("Type Error", str(e))
    except ValueError as e:
        # Value error
        self._show_error("Value Error", str(e))
    except Exception as e:
        # Unexpected error
        self._show_error("Error", str(e))
        logger.error(f"Command execution failed: {e}", exc_info=True)
```

---

## 9. Complete Example

### 9.1 Plugin Command with Full Validation

```python
# my_plugin/commands.py
from napari.layers import Image
from napari.types import LayerDataTuple
from typing import TYPE_CHECKING
import numpy as np

if TYPE_CHECKING:
    from napari.viewer import Viewer

def filter_image(
    viewer: "Viewer",
    image: Image  # ← Type annotation
) -> LayerDataTuple:
    """Filter image with complete validation"""
    
    # Runtime guard 1: Check layer exists
    if image is None:
        raise ValueError("No image layer provided")
    
    # Runtime guard 2: Check layer type (redundant but explicit)
    if not isinstance(image, Image):
        raise TypeError(f"Expected Image layer, got {type(image)}")
    
    # Runtime guard 3: Check data exists
    if image.data is None:
        raise ValueError("Image layer has no data")
    
    # Runtime guard 4: Check data shape
    if image.data.ndim < 2:
        raise ValueError(
            f"Image must be at least 2D, got {image.data.ndim}D. "
            f"Shape: {image.data.shape}"
        )
    
    # Runtime guard 5: Check dtype
    if not np.issubdtype(image.data.dtype, np.number):
        raise TypeError(
            f"Image data must be numeric, got {image.data.dtype}"
        )
    
    # All guards passed, process
    filtered = image.data > 0.5
    result = filtered.astype(np.uint8) * 255
    
    return (result, {'name': f'{image.name}_filtered'}, 'image')
```

### 9.2 Manifest with Enablement

```yaml
# napari.yaml
name: my-plugin
display_name: My Plugin
version: 0.1.0
contributions:
  commands:
    - id: my-plugin.filter
      title: Filter Image
      python_name: my_plugin.commands:filter_image
      enablement: layer.selected and isinstance(layer, Image)
      # ↑ Only enabled when Image layer is selected
```

### 9.3 Complete Call Chain

```
User clicks "Filter Image" menu item
    ↓
Menu system triggers action
    ↓
Command registry looks up command
    ↓
Enablement check:
  - Evaluate: layer.selected and isinstance(layer, Image)
  - Get: viewer.layers.selection.active
  - Check: isinstance(active_layer, Image)
  - Result: True (if Image selected)
    ↓
Build execution context:
  - viewer: Viewer instance
  - image: viewer.layers.selection.active
    ↓
Type validation:
  - Get function signature
  - Check: isinstance(image, Image)
  - Result: True
    ↓
Execute function:
  - Call: filter_image(viewer, image)
    ↓
Runtime guards:
  - Guard 1: image is not None ✓
  - Guard 2: isinstance(image, Image) ✓
  - Guard 3: image.data is not None ✓
  - Guard 4: image.data.ndim >= 2 ✓
  - Guard 5: numeric dtype ✓
    ↓
Process data:
  - filtered = image.data > 0.5
  - result = filtered.astype(np.uint8) * 255
    ↓
Return result:
  - (result, {'name': 'Filtered'}, 'image')
    ↓
Handle result:
  - Create new Image layer
  - Add to viewer
  - Emit events
  - Update UI
```

---

## 10. Summary

Napari validates plugin inputs through a comprehensive multi-layered system:

### Validation Layers

1. **Enablement Conditions**: Commands conditionally enabled based on selection
2. **Type Annotations**: Function signatures declare expected types
3. **Runtime Guards**: Plugin functions validate inputs before processing
4. **Layer Model Validation**: Final validation when creating layers

### Selection Model

- **Active Layer**: `viewer.layers.selection.active` provides context
- **Selection Events**: Plugins can listen to selection changes
- **Enablement Integration**: Enablement expressions use selection state

### Call Chain

1. **Menu Action**: User clicks menu item
2. **Enablement Check**: Validate command is enabled
3. **Command Lookup**: Get command from registry
4. **Context Building**: Build execution context with selection
5. **Type Validation**: Check function argument types
6. **Function Execution**: Call plugin function
7. **Runtime Guards**: Plugin validates inputs
8. **Result Handling**: Process return value

### Error Handling

- **TypeError**: Wrong layer type or dtype
- **ValueError**: Missing selection or invalid shape
- **Clear Messages**: Specific error messages with context
- **User Feedback**: Errors displayed in UI

This system ensures type safety and data integrity throughout the plugin execution pipeline, preventing errors and providing clear feedback to users.

