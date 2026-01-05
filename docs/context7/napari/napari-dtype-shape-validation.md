# Napari Array Dtype/Shape Validation and Normalization

## Executive Summary

This document provides a comprehensive technical analysis of how napari handles array dtype and shape constraints for image-like layers, including where validation occurs, how invalid inputs are rejected or normalized, and where conversions happen in the viewer, layer constructor, and plugin layers.

**Key Insight**: Napari uses a multi-layered validation and normalization system:
- **Viewer Level**: Initial acceptance and basic validation
- **Layer Constructor**: Comprehensive validation and normalization
- **Pydantic Validators**: Type checking and coercion
- **Runtime Validation**: Shape and dtype constraints enforced
- **Conversion Points**: Normalization happens at layer construction

---

## 1. Validation Layers

### 1.1 Three-Tier Validation System

Napari implements validation at three levels:

```
┌─────────────────────────────────────────────────────────┐
│  Tier 1: Viewer Level                                    │
│  viewer.add_image(data)                                  │
│  - Accepts array-like objects                            │
│  - Basic type checking                                   │
│  - Deferred conversion                                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Tier 2: Layer Constructor                              │
│  Image(data, ...)                                       │
│  - Comprehensive validation                             │
│  - Shape checking                                        │
│  - Dtype validation                                      │
│  - Normalization/conversion                              │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Tier 3: Pydantic Validators                            │
│  EventedModel validators                                 │
│  - Type coercion                                         │
│  - Constraint validation                                 │
│  - Final normalization                                   │
└─────────────────────────────────────────────────────────┘
```

### 1.2 Validation Flow

Complete validation flow from user input to layer creation:

```
User Input → Viewer → Layer Constructor → Pydantic → Layer Object
     │          │            │                │            │
     │          │            │                │            │
     ▼          ▼            ▼                ▼            ▼
  Array    Basic      Comprehensive    Type        Validated
  Object   Check      Validation       Coercion    Layer
```

---

## 2. Viewer Level Validation

### 2.1 viewer.add_image() Entry Point

The viewer provides the entry point for adding image data:

```python
# Location: napari/viewer.py
class Viewer:
    def add_image(
        self,
        data: ArrayLike,
        *,
        rgb: bool = None,
        colormap: str = None,
        contrast_limits: Tuple[float, float] = None,
        **kwargs
    ) -> Image:
        """Add image layer to viewer"""
        # Step 1: Accept array-like objects
        # - NumPy arrays
        # - Dask arrays
        # - Xarray arrays
        # - Zarr arrays
        # - Any object convertible with np.asarray()
        
        # Step 2: Basic type checking
        if not isinstance(data, (np.ndarray, list, tuple)):
            # Try to convert
            try:
                data = np.asarray(data)
            except (TypeError, ValueError) as e:
                raise TypeError(
                    f"Cannot convert {type(data)} to array: {e}"
                )
        
        # Step 3: Defer detailed validation to layer constructor
        layer = Image(
            data=data,
            rgb=rgb,
            colormap=colormap,
            contrast_limits=contrast_limits,
            **kwargs
        )
        
        # Step 4: Add to viewer
        self.layers.append(layer)
        return layer
```

**Viewer Level Responsibilities**:
- Accept array-like objects
- Basic type checking
- Defer detailed validation to layer constructor
- Handle conversion errors gracefully

### 2.2 Array-Like Object Support

The viewer accepts various array-like objects:

```python
# All of these are accepted:
viewer.add_image(np.array([1, 2, 3]))           # NumPy array
viewer.add_image(dask.array.from_array(...))    # Dask array
viewer.add_image(xr.DataArray(...))              # Xarray
viewer.add_image(zarr.open_array(...))          # Zarr array
viewer.add_image([1, 2, 3])                     # List (converted)
viewer.add_image((1, 2, 3))                     # Tuple (converted)

# Conversion happens via np.asarray()
# Deferred until layer constructor for lazy arrays
```

---

## 3. Layer Constructor Validation

### 3.1 Image Layer Constructor

The Image layer constructor performs comprehensive validation:

```python
# Location: napari/layers/image/image.py
class Image(Layer):
    def __init__(
        self,
        data: ArrayLike,
        *,
        rgb: bool = None,
        colormap: str = None,
        contrast_limits: Tuple[float, float] = None,
        **kwargs
    ):
        # Step 1: Convert to array (if not already)
        if not isinstance(data, np.ndarray):
            try:
                data = np.asarray(data)
            except (TypeError, ValueError) as e:
                raise TypeError(
                    f"Cannot convert {type(data)} to numpy array: {e}"
                )
        
        # Step 2: Shape validation
        if data.ndim < 2:
            raise ValueError(
                f"Image data must be at least 2D, got {data.ndim}D array "
                f"with shape {data.shape}"
            )
        
        if data.ndim > 5:
            raise ValueError(
                f"Image data has too many dimensions ({data.ndim}), "
                f"maximum is 5D"
            )
        
        # Step 3: RGB/RGBA detection
        if rgb is None:
            # Auto-detect RGB/RGBA
            if data.ndim >= 3 and data.shape[-1] in (3, 4):
                rgb = True
        
        # Step 4: Dtype validation and normalization
        data = self._validate_and_normalize_dtype(data)
        
        # Step 5: Shape normalization
        data = self._normalize_shape(data, rgb=rgb)
        
        # Step 6: Store validated data
        # EventedModel will validate via Pydantic
        super().__init__(data=data, **kwargs)
    
    def _validate_and_normalize_dtype(self, data: np.ndarray) -> np.ndarray:
        """Validate and normalize dtype"""
        # Allowed dtypes for images
        allowed_dtypes = [
            np.float32, np.float64,
            np.uint8, np.uint16, np.uint32,
            np.int8, np.int16, np.int32,
            bool
        ]
        
        # Check if dtype is allowed
        if data.dtype == bool:
            # Convert bool to uint8
            return data.astype(np.uint8)
        
        if not any(np.issubdtype(data.dtype, dt) for dt in allowed_dtypes):
            # Try to convert to float32
            try:
                return data.astype(np.float32)
            except (TypeError, ValueError):
                raise TypeError(
                    f"Image dtype {data.dtype} not supported. "
                    f"Allowed dtypes: {allowed_dtypes}"
                )
        
        return data
    
    def _normalize_shape(self, data: np.ndarray, rgb: bool = None) -> np.ndarray:
        """Normalize array shape"""
        # Handle RGB/RGBA
        if rgb and data.ndim >= 3:
            # Ensure last dimension is 3 or 4
            if data.shape[-1] not in (3, 4):
                raise ValueError(
                    f"RGB/RGBA data must have last dimension of 3 or 4, "
                    f"got {data.shape[-1]}"
                )
        
        # No shape normalization needed for standard images
        # Shape is preserved as-is
        return data
```

### 3.2 Validation Error Messages

Layer constructor provides specific error messages:

```python
# Example error messages:

# Shape errors
ValueError: "Image data must be at least 2D, got 1D array with shape (100,)"

# Dtype errors
TypeError: "Image dtype complex128 not supported. Allowed dtypes: [float32, float64, uint8, ...]"

# RGB shape errors
ValueError: "RGB/RGBA data must have last dimension of 3 or 4, got 5"
```

---

## 4. Pydantic Validators

### 4.1 EventedModel Validation

EventedModel uses Pydantic for additional validation:

```python
# Location: napari/layers/image/image.py
class Image(EventedModel):
    data: np.ndarray
    
    @validator('data')
    def validate_data_type(cls, v):
        """Validate data is numpy array"""
        if not isinstance(v, np.ndarray):
            # Try to convert
            try:
                v = np.asarray(v)
            except (TypeError, ValueError) as e:
                raise TypeError(f"Data must be numpy array, got {type(v)}: {e}")
        return v
    
    @validator('data')
    def validate_data_shape(cls, v):
        """Validate data shape"""
        if v.ndim < 2:
            raise ValueError(
                f"Image data must be at least 2D, got {v.ndim}D"
            )
        if v.ndim > 5:
            raise ValueError(
                f"Image data has too many dimensions ({v.ndim}), max is 5"
            )
        return v
    
    @validator('data')
    def validate_data_dtype(cls, v):
        """Validate and coerce dtype"""
        if v.dtype == bool:
            # Convert bool to uint8
            return v.astype(np.uint8)
        
        # Check if dtype is numeric
        if not np.issubdtype(v.dtype, np.number):
            raise TypeError(f"Image data must be numeric, got {v.dtype}")
        
        return v
```

### 4.2 Pydantic Validation Flow

Pydantic validators run after layer constructor:

```python
# Validation order:
# 1. Layer constructor validates
# 2. EventedModel.__init__() called
# 3. Pydantic validators run
# 4. Values stored if valid
# 5. Events emitted

class Image(EventedModel):
    def __init__(self, data, **kwargs):
        # Constructor validation happens first
        data = self._validate_in_constructor(data)
        
        # Then Pydantic validation
        super().__init__(data=data, **kwargs)
        # ↑ Pydantic validators run here
```

---

## 5. Dtype Validation and Normalization

### 5.1 Supported Dtypes

Image layers support specific dtypes:

```python
# Supported dtypes:
SUPPORTED_DTYPES = [
    # Floating point
    np.float32, np.float64,
    
    # Unsigned integers
    np.uint8, np.uint16, np.uint32,
    
    # Signed integers
    np.int8, np.int16, np.int32,
    
    # Boolean (converted to uint8)
    bool
]

# Not supported:
# - complex64, complex128
# - object dtype
# - string dtype
# - datetime64
```

### 5.2 Dtype Normalization

Invalid dtypes are normalized or rejected:

```python
def normalize_dtype(data: np.ndarray) -> np.ndarray:
    """Normalize dtype to supported type"""
    
    # Case 1: Boolean → uint8
    if data.dtype == bool:
        return data.astype(np.uint8)
    
    # Case 2: Complex → Reject
    if np.issubdtype(data.dtype, np.complexfloating):
        raise TypeError(
            f"Complex dtypes not supported for images, got {data.dtype}"
        )
    
    # Case 3: Object dtype → Reject
    if data.dtype == object:
        raise TypeError(
            f"Object dtype not supported for images, got {data.dtype}"
        )
    
    # Case 4: String dtype → Reject
    if np.issubdtype(data.dtype, np.str_):
        raise TypeError(
            f"String dtype not supported for images, got {data.dtype}"
        )
    
    # Case 5: Unsigned integer → Keep or upcast
    if np.issubdtype(data.dtype, np.unsignedinteger):
        if data.dtype not in [np.uint8, np.uint16, np.uint32]:
            # Upcast to nearest supported
            if data.dtype.itemsize <= 1:
                return data.astype(np.uint8)
            elif data.dtype.itemsize <= 2:
                return data.astype(np.uint16)
            else:
                return data.astype(np.uint32)
    
    # Case 6: Signed integer → Keep or upcast
    if np.issubdtype(data.dtype, np.integer):
        if data.dtype not in [np.int8, np.int16, np.int32]:
            # Upcast to nearest supported
            if data.dtype.itemsize <= 1:
                return data.astype(np.int8)
            elif data.dtype.itemsize <= 2:
                return data.astype(np.int16)
            else:
                return data.astype(np.int32)
    
    # Case 7: Floating point → Keep or convert
    if np.issubdtype(data.dtype, np.floating):
        if data.dtype not in [np.float32, np.float64]:
            # Convert to float32 (most common)
            return data.astype(np.float32)
    
    # Case 8: Unknown → Try float32
    try:
        return data.astype(np.float32)
    except (TypeError, ValueError):
        raise TypeError(
            f"Cannot convert dtype {data.dtype} to supported image dtype"
        )
```

### 5.3 Dtype Conversion Examples

```python
# Example conversions:

# Boolean → uint8
bool_array = np.array([True, False, True])
image = viewer.add_image(bool_array)
# ✅ Converted to uint8: [1, 0, 1]

# Complex → Rejected
complex_array = np.array([1+2j, 3+4j])
try:
    image = viewer.add_image(complex_array)
except TypeError as e:
    # ❌ Error: "Complex dtypes not supported"

# Float16 → Float32 (for operations)
float16_array = np.array([1.0, 2.0], dtype=np.float16)
image = viewer.add_image(float16_array)
# ✅ May be converted to float32 for compatibility

# Object → Rejected
object_array = np.array([1, 'a', 3], dtype=object)
try:
    image = viewer.add_image(object_array)
except TypeError as e:
    # ❌ Error: "Object dtype not supported"
```

---

## 6. Shape Validation and Normalization

### 6.1 Shape Constraints

Image layers have specific shape constraints:

```python
# Minimum dimensions: 2D
# Maximum dimensions: 5D
# RGB/RGBA: Last dimension must be 3 or 4

def validate_shape(data: np.ndarray, rgb: bool = None) -> np.ndarray:
    """Validate and normalize shape"""
    
    # Check minimum dimensions
    if data.ndim < 2:
        raise ValueError(
            f"Image data must be at least 2D, got {data.ndim}D array "
            f"with shape {data.shape}"
        )
    
    # Check maximum dimensions
    if data.ndim > 5:
        raise ValueError(
            f"Image data has too many dimensions ({data.ndim}), "
            f"maximum is 5D. Shape: {data.shape}"
        )
    
    # Check RGB/RGBA shape
    if rgb is True:
        if data.ndim < 3:
            raise ValueError(
                f"RGB/RGBA data must be at least 3D, got {data.ndim}D"
            )
        if data.shape[-1] not in (3, 4):
            raise ValueError(
                f"RGB/RGBA data must have last dimension of 3 or 4, "
                f"got {data.shape[-1]}. Shape: {data.shape}"
            )
    
    # Shape is preserved (no normalization)
    return data
```

### 6.2 Shape Normalization

Most shapes are preserved, but some edge cases are handled:

```python
# Case 1: 1D array → Rejected
data_1d = np.array([1, 2, 3])
try:
    image = viewer.add_image(data_1d)
except ValueError as e:
    # ❌ Error: "Image data must be at least 2D"

# Case 2: 2D array → Accepted
data_2d = np.array([[1, 2], [3, 4]])
image = viewer.add_image(data_2d)
# ✅ Accepted as-is

# Case 3: 3D array → Accepted
data_3d = np.random.random((10, 20, 30))
image = viewer.add_image(data_3d)
# ✅ Accepted as 3D image

# Case 4: RGB array → Auto-detected
data_rgb = np.random.random((100, 100, 3))
image = viewer.add_image(data_rgb)
# ✅ Auto-detected as RGB

# Case 5: 6D array → Rejected
data_6d = np.random.random((2, 3, 4, 5, 6, 7))
try:
    image = viewer.add_image(data_6d)
except ValueError as e:
    # ❌ Error: "Image data has too many dimensions (6), maximum is 5D"
```

### 6.3 RGB/RGBA Shape Handling

RGB/RGBA data has special shape handling:

```python
# RGB detection
def detect_rgb(data: np.ndarray) -> bool:
    """Detect if data is RGB/RGBA"""
    if data.ndim >= 3 and data.shape[-1] in (3, 4):
        return True
    return False

# RGB validation
def validate_rgb_shape(data: np.ndarray) -> np.ndarray:
    """Validate RGB/RGBA shape"""
    if data.ndim < 3:
        raise ValueError("RGB/RGBA data must be at least 3D")
    
    if data.shape[-1] not in (3, 4):
        raise ValueError(
            f"RGB/RGBA data must have last dimension of 3 or 4, "
            f"got {data.shape[-1]}"
        )
    
    return data

# Examples:
# ✅ Valid RGB: (100, 100, 3)
# ✅ Valid RGBA: (100, 100, 4)
# ❌ Invalid: (100, 100, 5) - wrong channel count
# ❌ Invalid: (100, 3) - not enough dimensions
```

---

## 7. Conversion Points

### 7.1 Where Conversions Happen

Conversions occur at specific points:

```
┌─────────────────────────────────────────────────────────┐
│  Conversion Point 1: Viewer Level                        │
│  viewer.add_image(data)                                 │
│  - Array-like → NumPy array (if needed)                 │
│  - Deferred for lazy arrays (Dask, Zarr)                 │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Conversion Point 2: Layer Constructor                   │
│  Image(data, ...)                                       │
│  - Array-like → NumPy array (if not already)            │
│  - Dtype normalization (bool → uint8, etc.)            │
│  - Shape validation (no normalization)                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Conversion Point 3: Pydantic Validators                 │
│  EventedModel validators                                 │
│  - Final dtype coercion                                  │
│  - Final shape validation                                 │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Conversion Point 4: Runtime Operations                  │
│  Thumbnail generation, etc.                              │
│  - float16 → float32 (for operations)                    │
│  - Lazy array → NumPy (for rendering)                    │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Viewer Level Conversions

```python
# Location: napari/viewer.py
def add_image(self, data: ArrayLike, **kwargs) -> Image:
    """Viewer-level conversion"""
    
    # Convert array-like to NumPy (if not lazy)
    if isinstance(data, (np.ndarray, list, tuple)):
        # Immediate conversion
        data = np.asarray(data)
    elif isinstance(data, (dask.array.Array, zarr.Array)):
        # Defer conversion (lazy evaluation)
        # Conversion happens later during rendering
        pass
    
    # Pass to layer constructor
    layer = Image(data=data, **kwargs)
    return layer
```

### 7.3 Layer Constructor Conversions

```python
# Location: napari/layers/image/image.py
class Image(Layer):
    def __init__(self, data: ArrayLike, **kwargs):
        """Layer constructor conversions"""
        
        # Convert to NumPy (if not already)
        if not isinstance(data, np.ndarray):
            data = np.asarray(data)
        
        # Normalize dtype
        data = self._normalize_dtype(data)
        
        # Validate shape
        data = self._validate_shape(data)
        
        # Store
        super().__init__(data=data, **kwargs)
```

### 7.4 Runtime Conversions

```python
# Location: napari/layers/image/image.py
def _update_thumbnail(self):
    """Runtime conversion for thumbnail generation"""
    # Get data
    data = self.data
    
    # Convert lazy arrays
    if isinstance(data, (dask.array.Array, zarr.Array)):
        data = np.asarray(data)
    
    # Convert float16 to float32 (for operations)
    if data.dtype == np.float16:
        data = data.astype(np.float32)
    
    # Generate thumbnail
    thumbnail = generate_thumbnail(data)
    self.thumbnail = thumbnail
```

---

## 8. Plugin Level Validation

### 8.1 Plugin Input Validation

Plugins should validate inputs before processing:

```python
@napari_hook_implementation
def filter_plugin(viewer: Viewer):
    """Plugin with input validation"""
    layer = viewer.layers.selection.active
    
    # Validate layer type
    if not isinstance(layer, Image):
        raise TypeError(f"Plugin requires Image layer, got {type(layer)}")
    
    # Validate dtype
    if not np.issubdtype(layer.data.dtype, np.floating):
        raise TypeError(
            f"Plugin requires float image, got {layer.data.dtype}"
        )
    
    # Validate shape
    if layer.data.ndim != 2:
        raise ValueError(
            f"Plugin requires 2D image, got {layer.data.ndim}D"
        )
    
    # Process data
    result = apply_filter(layer.data)
    
    # Validate output
    if not isinstance(result, np.ndarray):
        result = np.asarray(result)
    
    # Return LayerData tuple
    return (result, {'name': 'Filtered'}, 'image')
```

### 8.2 Plugin Output Validation

Plugins should ensure outputs are valid:

```python
@napari_hook_implementation
def process_plugin(viewer: Viewer) -> LayerDataTuple:
    """Plugin with output validation"""
    layer = viewer.layers[0]
    
    # Process
    result = process(layer.data)
    
    # Validate output dtype
    if result.dtype not in [np.float32, np.float64, np.uint8]:
        # Normalize to float32
        result = result.astype(np.float32)
    
    # Validate output shape
    if result.ndim < 2:
        raise ValueError(f"Output must be at least 2D, got {result.ndim}D")
    
    # Return validated output
    return (result, {'name': 'Processed'}, 'image')
```

---

## 9. Error Handling

### 9.1 Validation Error Types

Different validation errors are raised:

```python
# TypeError: Invalid dtype or type
try:
    viewer.add_image([1, 2, 3, 'a'])  # Mixed types
except TypeError as e:
    # Error: "Cannot convert to numpy array"

# ValueError: Invalid shape
try:
    viewer.add_image(np.array([1, 2, 3]))  # 1D array
except ValueError as e:
    # Error: "Image data must be at least 2D"

# ValueError: Invalid RGB shape
try:
    viewer.add_image(np.random.random((100, 100, 5)), rgb=True)
except ValueError as e:
    # Error: "RGB/RGBA data must have last dimension of 3 or 4"
```

### 9.2 Error Message Format

Error messages provide specific information:

```python
# Format: "Expected {requirement}, got {actual}"

# Example 1: Shape error
ValueError(
    "Image data must be at least 2D, got 1D array with shape (100,)"
)

# Example 2: Dtype error
TypeError(
    "Image dtype complex128 not supported. "
    "Allowed dtypes: [float32, float64, uint8, uint16, ...]"
)

# Example 3: RGB shape error
ValueError(
    "RGB/RGBA data must have last dimension of 3 or 4, "
    "got 5. Shape: (100, 100, 5)"
)
```

---

## 10. Complete Validation Flow Example

### 10.1 Example: Adding Invalid Data

```python
# User input
invalid_data = np.array([1, 2, 3])  # 1D array

# Step 1: Viewer level
viewer.add_image(invalid_data)
# ✅ Accepted (basic type check passes)

# Step 2: Layer constructor
Image(data=invalid_data)
# ❌ ValueError: "Image data must be at least 2D, got 1D array with shape (3,)"

# Error raised, layer not created
```

### 10.2 Example: Adding Valid Data with Conversion

```python
# User input
bool_data = np.array([[True, False], [False, True]])

# Step 1: Viewer level
viewer.add_image(bool_data)
# ✅ Accepted

# Step 2: Layer constructor
Image(data=bool_data)
# - Detects bool dtype
# - Converts to uint8: [[1, 0], [0, 1]]
# ✅ Layer created

# Step 3: Pydantic validation
# - Validates dtype is uint8
# - Validates shape is 2D
# ✅ Validation passes

# Result: Layer created with uint8 data
```

### 10.3 Example: Adding Complex Data

```python
# User input
complex_data = np.array([[1+2j, 3+4j], [5+6j, 7+8j]])

# Step 1: Viewer level
viewer.add_image(complex_data)
# ✅ Accepted (basic type check passes)

# Step 2: Layer constructor
Image(data=complex_data)
# - Detects complex dtype
# ❌ TypeError: "Complex dtypes not supported for images, got complex128"

# Error raised, layer not created
```

---

## 11. Summary

Napari implements a comprehensive multi-tier validation system for array dtype and shape:

### Validation Layers

1. **Viewer Level**: Basic type checking, accepts array-like objects
2. **Layer Constructor**: Comprehensive validation and normalization
3. **Pydantic Validators**: Final type coercion and constraint checking
4. **Runtime**: Additional conversions for specific operations

### Dtype Handling

- **Supported**: float32, float64, uint8, uint16, uint32, int8, int16, int32, bool
- **Normalized**: bool → uint8, float16 → float32 (for operations)
- **Rejected**: complex, object, string dtypes

### Shape Handling

- **Minimum**: 2D arrays required
- **Maximum**: 5D arrays supported
- **RGB/RGBA**: Last dimension must be 3 or 4
- **Preserved**: Shapes are generally preserved (not normalized)

### Conversion Points

- **Viewer**: Array-like → NumPy (immediate or deferred)
- **Constructor**: Dtype normalization, shape validation
- **Pydantic**: Final validation and coercion
- **Runtime**: Lazy array conversion, operation-specific conversions

### Error Handling

- **TypeError**: Invalid dtype or type
- **ValueError**: Invalid shape
- **Clear Messages**: Specific error messages with context

This system ensures that only valid image data reaches the layer, while providing helpful error messages for invalid inputs and automatic normalization where appropriate.

