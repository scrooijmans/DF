# Napari NPE2 Plugin and Command Extensibility Model

## Executive Summary

This document provides a comprehensive technical analysis of napari's plugin and command extensibility model, focusing on npe2 (napari plugin engine 2). It explains how new operators are safely integrated into the viewer and data model through a well-defined plugin architecture, command-based action system, and rigorous validation mechanisms.

**Key Insight**: Napari's npe2 provides a standardized, safe extensibility model:
- **Manifest-Based Configuration**: Plugins declare capabilities via `manifest.yaml`
- **Contribution Points**: Standardized interfaces for extending functionality
- **Command System**: Decoupled command-based actions with unique IDs
- **Type Safety**: Runtime validation ensures data integrity
- **Isolation**: Plugins operate in separate namespaces to prevent conflicts

---

## 1. NPE2 Architecture Overview

### 1.1 What is NPE2?

**NPE2 (Napari Plugin Engine 2)** is napari's modern plugin system that provides:
- Standardized plugin discovery and registration
- Declarative plugin configuration via manifest files
- Type-safe contribution points
- Safe plugin execution and isolation
- Command-based extensibility

### 1.2 Key Components

```
┌─────────────────────────────────────────────────────────┐
│              NPE2 Plugin System                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Plugin Discovery                                  │  │
│  │  - Entry points                                    │  │
│  │  - Manifest parsing                                │  │
│  │  - Validation                                       │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Contribution Points                              │  │
│  │  - Commands                                        │  │
│  │  - Readers/Writers                                 │  │
│  │  - Sample Data                                     │  │
│  │  - Widgets                                         │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Command Registry                                 │  │
│  │  - Command registration                           │  │
│  │  - Command execution                             │  │
│  │  - UI integration                                 │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Safety Mechanisms                                │  │
│  │  - Type validation                                │  │
│  │  - Isolation                                      │  │
│  │  - Error handling                                 │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 1.3 Plugin Lifecycle

1. **Discovery**: NPE2 discovers plugins via entry points
2. **Loading**: Plugin manifest is parsed and validated
3. **Registration**: Contributions are registered with napari
4. **Integration**: Commands/widgets integrated into UI
5. **Execution**: Plugin code executed when invoked
6. **Cleanup**: Resources cleaned up when plugin disabled

---

## 2. Plugin Manifest and Configuration

### 2.1 Manifest Structure

Plugins declare their capabilities via `manifest.yaml`:

```yaml
# napari.yaml (plugin manifest)
name: my-plugin
display_name: My Plugin
version: 0.1.0
contributions:
  commands:
    - id: my-plugin.filter
      title: Apply Filter
      python_name: my_plugin.commands:apply_filter
  readers:
    - command: my-plugin.read
      filename_patterns: ["*.tif", "*.tiff"]
      accepts_directories: false
  writers:
    - command: my-plugin.write
      filename_patterns: ["*.tif"]
      layer_types: ["image"]
  sample_data:
    - key: sample_image
      display_name: Sample Image
      uri: my_plugin.sample_data:sample_image
  widgets:
    - command: my-plugin.widget
      display_name: My Widget
```

### 2.2 Manifest Validation

NPE2 validates manifests before registration:

```python
# Validation checks:
# 1. Required fields present
# 2. Valid contribution types
# 3. Command IDs are unique
# 4. Python names are importable
# 5. File patterns are valid
# 6. Layer types are recognized
```

**Validation Errors Prevent Registration**:
- Invalid manifest structure
- Missing required fields
- Duplicate command IDs
- Unimportable Python names
- Invalid contribution types

### 2.3 Entry Points

Plugins are discovered via Python entry points:

```python
# setup.py or pyproject.toml
[project.entry-points."napari.manifest"]
my-plugin = "my_plugin:napari.yaml"

# Or in setup.py:
setup(
    entry_points={
        "napari.manifest": [
            "my-plugin = my_plugin:napari.yaml"
        ]
    }
)
```

**Discovery Process**:
1. NPE2 scans installed packages for entry points
2. Loads manifest files
3. Validates manifests
4. Registers valid plugins

---

## 3. Contribution Points

### 3.1 Commands

Commands are callable actions identified by unique string IDs.

#### 3.1.1 Command Definition

```python
# my_plugin/commands.py
from napari.types import LayerDataTuple
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from napari.viewer import Viewer

def apply_filter(
    viewer: "Viewer",
    layer: "Image",
    threshold: float = 0.5
) -> LayerDataTuple:
    """Apply filter to image layer"""
    import numpy as np
    
    # Process data
    filtered = layer.data > threshold
    result = filtered.astype(np.uint8) * 255
    
    # Return LayerData tuple
    return (result, {'name': f'{layer.name}_filtered'}, 'image')
```

#### 3.1.2 Command Registration

```yaml
# manifest.yaml
contributions:
  commands:
    - id: my-plugin.filter
      title: Apply Filter
      python_name: my_plugin.commands:apply_filter
      # Optional: enablement condition
      enablement: layer.selected
```

#### 3.1.3 Command Execution

```python
# Command execution flow:
# 1. User invokes command (menu, button, keybinding)
# 2. NPE2 looks up command by ID
# 3. Validates command is enabled
# 4. Resolves function from python_name
# 5. Calls function with context
# 6. Handles return value (LayerData, etc.)
# 7. Integrates result into viewer
```

### 3.2 Readers and Writers

Plugins can add file format support.

#### 3.2.1 Reader Contribution

```python
# my_plugin/readers.py
from napari.types import LayerDataTuple, ReaderFunction
from typing import List

def read_tif(path: str) -> List[LayerDataTuple]:
    """Read TIFF file"""
    from PIL import Image
    import numpy as np
    
    img = Image.open(path)
    data = np.array(img)
    
    return [(data, {'name': path.stem}, 'image')]
```

```yaml
# manifest.yaml
contributions:
  readers:
    - command: my-plugin.read_tif
      filename_patterns: ["*.tif", "*.tiff"]
      accepts_directories: false
```

#### 3.2.2 Writer Contribution

```python
# my_plugin/writers.py
from napari.types import WriterFunction
from napari.layers import Image

def write_tif(
    path: str,
    layer_data: tuple,
    *,
    layer_type: str
) -> List[str]:
    """Write image layer to TIFF"""
    from PIL import Image
    import numpy as np
    
    data, metadata, _ = layer_data
    img = Image.fromarray(data)
    img.save(path)
    
    return [path]
```

```yaml
# manifest.yaml
contributions:
  writers:
    - command: my-plugin.write_tif
      filename_patterns: ["*.tif"]
      layer_types: ["image"]
```

### 3.3 Sample Data

Plugins can provide sample datasets.

```python
# my_plugin/sample_data.py
from napari.types import LayerDataTuple

def sample_image() -> LayerDataTuple:
    """Provide sample image data"""
    import numpy as np
    
    data = np.random.random((100, 100))
    return (data, {'name': 'Sample Image'}, 'image')
```

```yaml
# manifest.yaml
contributions:
  sample_data:
    - key: sample_image
      display_name: Sample Image
      uri: my_plugin.sample_data:sample_image
```

### 3.4 Widgets

Plugins can add dockable widgets.

```python
# my_plugin/widgets.py
from magicgui import magicgui
from napari.layers import Image

@magicgui
def filter_widget(
    layer: Image,
    threshold: float = 0.5
):
    """Filter widget"""
    import numpy as np
    
    filtered = layer.data > threshold
    result = filtered.astype(np.uint8) * 255
    layer.data = result
```

```yaml
# manifest.yaml
contributions:
  widgets:
    - command: my-plugin.filter_widget
      display_name: Filter Widget
```

---

## 4. Command System Architecture

### 4.1 Command Registry

NPE2 maintains a central command registry:

```python
# Conceptual command registry
class CommandRegistry:
    def __init__(self):
        self._commands = {}  # id -> Command
    
    def register(self, command_id: str, command: Command):
        """Register a command"""
        if command_id in self._commands:
            raise ValueError(f"Command {command_id} already registered")
        self._commands[command_id] = command
    
    def get(self, command_id: str) -> Command:
        """Get command by ID"""
        return self._commands[command_id]
    
    def execute(self, command_id: str, context: dict):
        """Execute command with context"""
        command = self.get(command_id)
        if not command.is_enabled(context):
            raise RuntimeError(f"Command {command_id} is not enabled")
        return command.execute(context)
```

### 4.2 Command Structure

```python
class Command:
    def __init__(
        self,
        id: str,
        title: str,
        python_name: str,
        enablement: str = None,
        toggled: str = None
    ):
        self.id = id
        self.title = title
        self.python_name = python_name
        self.enablement = enablement
        self.toggled = toggled
        self._function = None
    
    def load(self):
        """Load function from python_name"""
        module_path, func_name = self.python_name.split(':')
        module = importlib.import_module(module_path)
        self._function = getattr(module, func_name)
    
    def is_enabled(self, context: dict) -> bool:
        """Check if command is enabled"""
        if self.enablement is None:
            return True
        return eval(self.enablement, context)
    
    def execute(self, context: dict):
        """Execute command"""
        if self._function is None:
            self.load()
        return self._function(**context)
```

### 4.3 Command Naming Conventions

Commands follow specific naming conventions:

```python
# Built-in napari commands
"napari.window.file.open_files_dialog"
"napari.layers.add_image"
"napari.viewer.reset_view"

# Plugin commands
"my-plugin.filter"
"my-plugin.process"
"my-plugin.analyze"

# Naming rules:
# - Use dots to separate words
# - Start with 'napari' for built-in commands
# - Start with plugin name for plugin commands
# - Use lowercase with underscores
# - Be descriptive and specific
```

### 4.4 Command Enablement and Toggle States

Commands can be conditionally enabled:

```yaml
# manifest.yaml
contributions:
  commands:
    - id: my-plugin.filter
      title: Apply Filter
      python_name: my_plugin.commands:apply_filter
      # Enable only when image layer is selected
      enablement: layer.selected and isinstance(layer, Image)
      # Toggle state (for checkable commands)
      toggled: layer.visible
```

**Enablement Expressions**:
- Evaluated in command context
- Access to viewer, layers, selection
- Boolean result determines if command is enabled

---

## 5. Safe Integration Mechanisms

### 5.1 Type Safety

NPE2 enforces type safety through multiple mechanisms:

#### 5.1.1 Type Hints

```python
from napari.types import LayerDataTuple
from napari.layers import Image
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from napari.viewer import Viewer

def process_image(
    viewer: "Viewer",
    layer: Image,
    threshold: float = 0.5
) -> LayerDataTuple:
    """Type hints ensure correct usage"""
    # Type checker validates types
    # Runtime can validate types
    pass
```

#### 5.1.2 Runtime Validation

```python
# NPE2 validates:
# 1. Function signature matches expected types
# 2. Arguments are correct types
# 3. Return values match expected types
# 4. LayerData tuples are valid

def validate_command(command: Command, context: dict):
    """Validate command before execution"""
    # Check argument types
    sig = inspect.signature(command._function)
    for param_name, param in sig.parameters.items():
        if param_name in context:
            expected_type = param.annotation
            actual_value = context[param_name]
            if not isinstance(actual_value, expected_type):
                raise TypeError(
                    f"Parameter {param_name} expected {expected_type}, "
                    f"got {type(actual_value)}"
                )
```

### 5.2 Isolation

Plugins operate in isolated namespaces:

#### 5.2.1 Namespace Isolation

```python
# Each plugin has its own namespace
# Plugin commands: "my-plugin.*"
# Built-in commands: "napari.*"
# No conflicts between plugins

# Command IDs are unique
# Plugin A: "plugin-a.filter"
# Plugin B: "plugin-b.filter"
# No conflict!
```

#### 5.2.2 Import Isolation

```python
# Plugins import their own dependencies
# No shared state between plugins
# Each plugin loads independently

# Plugin A
import numpy as np
# Uses its own numpy

# Plugin B
import numpy as np
# Uses its own numpy
# No interference
```

### 5.3 Error Handling

NPE2 provides robust error handling:

```python
# Command execution with error handling
def execute_command_safely(command_id: str, context: dict):
    """Execute command with error handling"""
    try:
        command = registry.get(command_id)
        if not command.is_enabled(context):
            return None  # Command not enabled, not an error
        
        result = command.execute(context)
        return result
    
    except ImportError as e:
        # Plugin dependency missing
        logger.error(f"Plugin dependency missing: {e}")
        show_error_to_user("Plugin dependency missing")
        return None
    
    except TypeError as e:
        # Type mismatch
        logger.error(f"Type error in command: {e}")
        show_error_to_user("Invalid data types")
        return None
    
    except Exception as e:
        # Unexpected error
        logger.error(f"Error executing command: {e}", exc_info=True)
        show_error_to_user("Command execution failed")
        return None
```

### 5.4 Validation Best Practices

NPE2 enforces best practices:

#### 5.4.1 Avoid Lambda Functions

```python
# ❌ Bad: Lambda function
command = Command(
    id="my-plugin.filter",
    python_name="lambda x: x * 2"  # Memory leak risk
)

# ✅ Good: Named function
def apply_filter(data):
    return data * 2

command = Command(
    id="my-plugin.filter",
    python_name="my_plugin.commands:apply_filter"
)
```

#### 5.4.2 Use qtpy for Qt

```python
# ❌ Bad: Direct Qt import
from PyQt5.QtWidgets import QWidget

# ✅ Good: qtpy abstraction
from qtpy.QtWidgets import QWidget
# Works with any Qt backend
```

#### 5.4.3 Minimize C Dependencies

```python
# ❌ Bad: Heavy C dependencies
# Requires compilation, platform-specific

# ✅ Good: Pure Python or pre-compiled wheels
# Easy installation, cross-platform
```

---

## 6. Integration with Viewer and Data Model

### 6.1 Viewer Integration

Commands integrate with viewer through context:

```python
# Command receives viewer in context
def my_command(viewer: Viewer, layer: Image):
    """Command has access to viewer"""
    # Can access all viewer state
    all_layers = viewer.layers
    active_layer = viewer.layers.selection.active
    dims = viewer.dims
    
    # Can modify viewer
    new_layer = viewer.add_image(processed_data)
    viewer.layers.remove(layer)
```

### 6.2 Data Model Integration

Commands interact with data model via LayerData tuples:

```python
# Command returns LayerData tuple
def process_layer(layer: Image) -> LayerDataTuple:
    """Process and return new layer"""
    processed = process(layer.data)
    
    # LayerData tuple format:
    # (data, metadata_dict, layer_type_string)
    return (
        processed,                    # Data array
        {'name': 'Processed'},        # Metadata
        'image'                       # Layer type
    )

# NPE2 automatically:
# 1. Creates new layer from tuple
# 2. Adds to viewer
# 3. Applies metadata
# 4. Sets layer type
```

### 6.3 UI Integration

Commands can be integrated into UI:

```python
# Commands can be:
# 1. Menu items
menu.add_command("my-plugin.filter")

# 2. Toolbar buttons
toolbar.add_command("my-plugin.filter")

# 3. Keybindings
keybindings.register("Ctrl+F", "my-plugin.filter")

# 4. Context menus
context_menu.add_command("my-plugin.filter")
```

### 6.4 Widget Integration

Widgets integrate as dockable panels:

```python
# Widget contribution
@magicgui
def my_widget(layer: Image):
    """Dockable widget"""
    # Widget appears in napari UI
    # Can interact with layers
    pass

# Automatically:
# - Added to Window menu
# - Dockable in viewer
# - Persists across sessions
```

---

## 7. Plugin Discovery and Loading

### 7.1 Discovery Process

```python
# 1. NPE2 scans entry points
def discover_plugins():
    """Discover all installed plugins"""
    plugins = []
    
    # Get entry points
    entry_points = pkg_resources.iter_entry_points('napari.manifest')
    
    for entry_point in entry_points:
        try:
            # Load manifest
            manifest_path = entry_point.load()
            manifest = parse_manifest(manifest_path)
            
            # Validate manifest
            if validate_manifest(manifest):
                plugins.append(manifest)
        except Exception as e:
            logger.warning(f"Failed to load plugin {entry_point.name}: {e}")
    
    return plugins
```

### 7.2 Loading Process

```python
# 2. Load and register plugin
def load_plugin(manifest: dict):
    """Load plugin and register contributions"""
    plugin_name = manifest['name']
    
    # Register commands
    for command_spec in manifest.get('contributions', {}).get('commands', []):
        command = create_command(command_spec)
        registry.register(command.id, command)
    
    # Register readers
    for reader_spec in manifest.get('contributions', {}).get('readers', []):
        reader = create_reader(reader_spec)
        reader_registry.register(reader)
    
    # Register writers
    for writer_spec in manifest.get('contributions', {}).get('writers', []):
        writer = create_writer(writer_spec)
        writer_registry.register(writer)
    
    # Register widgets
    for widget_spec in manifest.get('contributions', {}).get('widgets', []):
        widget = create_widget(widget_spec)
        widget_registry.register(widget)
```

### 7.3 Lazy Loading

Commands are loaded lazily:

```python
# Commands are not loaded until needed
class Command:
    def __init__(self, python_name: str):
        self.python_name = python_name
        self._function = None  # Not loaded yet
    
    def load(self):
        """Load function when needed"""
        if self._function is None:
            module_path, func_name = self.python_name.split(':')
            module = importlib.import_module(module_path)
            self._function = getattr(module, func_name)
    
    def execute(self, context: dict):
        """Load and execute"""
        if self._function is None:
            self.load()
        return self._function(**context)
```

**Benefits**:
- Faster startup (don't load all plugins)
- Only load what's used
- Handle import errors gracefully

---

## 8. Safety Mechanisms

### 8.1 Validation Layers

Multiple validation layers ensure safety:

```python
# 1. Manifest validation
validate_manifest(manifest)
# - Structure validation
# - Required fields
# - Type validation

# 2. Command validation
validate_command(command)
# - Function exists
# - Signature valid
# - Types match

# 3. Runtime validation
validate_execution(context)
# - Arguments valid
# - Types correct
# - State valid

# 4. Result validation
validate_result(result)
# - Return type correct
# - LayerData valid
# - Data structure valid
```

### 8.2 Error Recovery

NPE2 provides error recovery:

```python
# Plugin errors don't crash napari
def execute_with_recovery(command_id: str, context: dict):
    """Execute with error recovery"""
    try:
        return execute_command(command_id, context)
    except PluginError as e:
        # Plugin-specific error
        logger.error(f"Plugin error: {e}")
        show_error_to_user(f"Plugin error: {e.message}")
        return None
    except Exception as e:
        # Unexpected error
        logger.error(f"Unexpected error: {e}", exc_info=True)
        show_error_to_user("An unexpected error occurred")
        # Napari continues running
        return None
```

### 8.3 Plugin Isolation

Plugins are isolated from each other:

```python
# Plugin A error doesn't affect Plugin B
try:
    plugin_a_command.execute()
except Exception:
    # Plugin A fails
    pass

# Plugin B still works
plugin_b_command.execute()  # ✅ Works fine
```

---

## 9. Practical Examples

### 9.1 Complete Plugin Example

```python
# my_plugin/__init__.py
# Empty, just marks package

# my_plugin/commands.py
from napari.types import LayerDataTuple
from napari.layers import Image
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from napari.viewer import Viewer

def threshold_image(
    viewer: "Viewer",
    layer: Image,
    threshold: float = 0.5
) -> LayerDataTuple:
    """Threshold image layer"""
    import numpy as np
    
    thresholded = layer.data > threshold
    result = thresholded.astype(np.uint8) * 255
    
    return (
        result,
        {'name': f'{layer.name}_thresholded'},
        'image'
    )

# napari.yaml (manifest)
name: my-plugin
display_name: My Plugin
version: 0.1.0
contributions:
  commands:
    - id: my-plugin.threshold
      title: Threshold Image
      python_name: my_plugin.commands:threshold_image
      enablement: layer.selected and isinstance(layer, Image)

# setup.py
from setuptools import setup

setup(
    name="my-plugin",
    version="0.1.0",
    entry_points={
        "napari.manifest": [
            "my-plugin = my_plugin:napari.yaml"
        ]
    }
)
```

### 9.2 Reader Plugin Example

```python
# my_plugin/readers.py
from napari.types import LayerDataTuple, ReaderFunction
from typing import List
from pathlib import Path

def read_custom_format(path: str) -> List[LayerDataTuple]:
    """Read custom file format"""
    import numpy as np
    
    # Read file
    data = np.loadtxt(path)
    
    return [(data, {'name': Path(path).stem}, 'image')]

# manifest.yaml
contributions:
  readers:
    - command: my-plugin.read_custom
      filename_patterns: ["*.custom"]
      accepts_directories: false
```

### 9.3 Widget Plugin Example

```python
# my_plugin/widgets.py
from magicgui import magicgui
from napari.layers import Image
import numpy as np

@magicgui(
    layer={'widget_type': 'Layer', 'layer_type': Image},
    threshold={'widget_type': 'FloatSlider', 'min': 0, 'max': 1}
)
def threshold_widget(layer: Image, threshold: float = 0.5):
    """Threshold widget"""
    thresholded = layer.data > threshold
    layer.data = thresholded.astype(np.uint8) * 255

# manifest.yaml
contributions:
  widgets:
    - command: my-plugin.threshold_widget
      display_name: Threshold Widget
```

---

## 10. Summary

Napari's npe2 plugin and command extensibility model provides a robust, safe framework for extending napari:

### Key Components

1. **Manifest-Based Configuration**: Plugins declare capabilities via `manifest.yaml`
2. **Contribution Points**: Standardized interfaces (commands, readers, writers, widgets)
3. **Command System**: Decoupled command-based actions with unique IDs
4. **Type Safety**: Runtime validation ensures data integrity
5. **Isolation**: Plugins operate in separate namespaces

### Safety Mechanisms

- **Validation**: Multiple validation layers (manifest, command, runtime, result)
- **Error Handling**: Robust error recovery prevents crashes
- **Isolation**: Plugin errors don't affect other plugins or napari core
- **Best Practices**: Guidelines prevent common issues (lambda functions, Qt imports)

### Integration Points

- **Viewer**: Commands receive viewer context
- **Data Model**: LayerData tuples integrate with data model
- **UI**: Commands integrate into menus, toolbars, keybindings
- **Widgets**: Dockable widgets extend UI

### Benefits

- **Modularity**: Plugins don't modify core code
- **Safety**: Validation and isolation prevent issues
- **Extensibility**: Easy to add new functionality
- **Maintainability**: Standardized structure and practices
- **Discoverability**: Plugins are discoverable and installable

This architecture enables developers to safely and effectively extend napari's functionality while maintaining stability and compatibility.

