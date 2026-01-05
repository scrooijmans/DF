# Welly Library: Architecture, Metadata, and Design

This document explains the architecture, metadata structure, and design patterns of the `welly` library, a Python package for loading, processing, and analyzing subsurface well data.

## Overview

`welly` is a Python library developed by Agile Geoscience that provides a structured approach to working with well log data, particularly from LAS (Log ASCII Standard) files. It offers tools for exploring metadata, visualizing well logs, and identifying data quality issues.

**Repository**: https://github.com/agile-geoscience/welly  
**Documentation**: https://code.agilescientific.com/welly/

## Core Architecture

### 1. **Object-Oriented Design**

Welly follows an object-oriented architecture with two primary classes:

```
Project (Container)
    └── Well (Individual Well)
        ├── Header (Metadata)
        ├── Data (Curves)
        ├── Tops (Formation Tops)
        └── Location (Spatial Information)
```

### 2. **Dependency Stack**

Welly builds upon established Python libraries:

- **pandas**: Data manipulation and DataFrame operations
- **numpy**: Numerical computations and array operations
- **matplotlib**: Visualization and plotting
- **striplog**: Stratigraphic data handling
- **lasio**: LAS file parsing (underlying library)

## The Well Object

The `Well` object is the core component that encapsulates all data and metadata associated with a single well.

### Object Structure

```python
class Well:
    """
    Represents a single well with all associated data and metadata.
    """
    def __init__(self, las_file=None, ...):
        self.header = {}      # Well metadata from LAS header
        self.data = {}       # Well log curves (pandas DataFrame)
        self.tops = {}       # Formation tops
        self.location = {}  # Spatial coordinates
        self.uwi = None     # Unique Well Identifier
        # ... other attributes
```

## Metadata Properties

### 1. **Header Metadata (`Well.header`)**

The header contains metadata extracted from the LAS file header sections. LAS files have standardized sections, and welly maps these to accessible properties:

#### Well Information Section (`~W`)

```python
well.header['WELL'] = {
    'name': 'Well Name',
    'uwi': 'Unique Well Identifier',
    'api': 'API Number',
    'country': 'Country Code',
    'state': 'State/Province',
    'county': 'County',
    'field': 'Field Name',
    'operator': 'Operator Name',
    'service_company': 'Service Company',
    'date': 'Log Date',
    'start_depth': 'Start Depth (MD)',
    'stop_depth': 'Stop Depth (MD)',
    'step': 'Depth Step',
    'null': 'Null Value',
    # ... additional well-specific metadata
}
```

**Key Properties**:
- **`name`**: Well name or identifier
- **`uwi`**: Unique Well Identifier (API number or similar)
- **`api`**: American Petroleum Institute number
- **`location`**: Geographic location information
- **`date`**: Date of logging operations
- **`depth_range`**: Start and stop depths

#### Curve Information Section (`~C`)

```python
well.header['CURVES'] = {
    'DEPTH': {
        'unit': 'FT',
        'description': 'Depth',
        'api_code': 'DEPT'
    },
    'GR': {
        'unit': 'API',
        'description': 'Gamma Ray',
        'api_code': 'GR'
    },
    # ... other curves
}
```

**Properties for Each Curve**:
- **`mnemonic`**: Curve name/mnemonic (e.g., 'GR', 'RHOB', 'NPHI')
- **`unit`**: Measurement unit (e.g., 'API', 'G/CC', 'V/V')
- **`description`**: Human-readable description
- **`api_code`**: Standard API curve code
- **`data_type`**: Data type (e.g., 'F' for float)

#### Parameter Information Section (`~P`)

```python
well.header['PARAMETERS'] = {
    'BHT': {
        'value': 150.0,
        'unit': 'F',
        'description': 'Bottom Hole Temperature'
    },
    'RMF': {
        'value': 0.75,
        'unit': 'OHMM',
        'description': 'Mud Filtrate Resistivity'
    },
    # ... other parameters
}
```

**Common Parameters**:
- **`BHT`**: Bottom Hole Temperature
- **`RMF`**: Mud Filtrate Resistivity
- **`RMC`**: Mud Cake Resistivity
- **`RMF_TEMP`**: Mud Filtrate Temperature
- **`MUD`**: Mud Type
- **`MUD_WEIGHT`**: Mud Weight
- **`BIT_SIZE`**: Bit Size
- **`TD`**: Total Depth

#### Other Information Section (`~O`)

```python
well.header['OTHER'] = {
    'notes': 'Additional notes and comments',
    'remarks': 'Remarks from logging engineer',
    # ... unstructured metadata
}
```

### 2. **Data Properties (`Well.data`)**

The `data` attribute contains the actual well log curves as a pandas DataFrame:

```python
well.data = pd.DataFrame({
    'DEPTH': [1000, 1001, 1002, ...],
    'GR': [45.2, 46.1, 44.8, ...],
    'RHOB': [2.65, 2.67, 2.64, ...],
    'NPHI': [0.15, 0.16, 0.14, ...],
    # ... other curves
})
```

**Properties**:
- **Index**: Typically depth values
- **Columns**: Curve mnemonics (e.g., 'GR', 'RHOB', 'NPHI')
- **Values**: Log measurements at each depth
- **Data Types**: Float arrays (with NaN for missing values)

**Accessing Curves**:
```python
# Direct access
gr_curve = well.data['GR']

# With depth
depth = well.data.index
gr_values = well.data['GR'].values
```

### 3. **Formation Tops (`Well.tops`)**

Formation tops represent geological boundaries:

```python
well.tops = {
    'Top Formation A': 1250.5,  # Depth in MD
    'Top Formation B': 1500.0,
    'Top Formation C': 1800.3,
    # ... other tops
}
```

**Properties**:
- **Keys**: Formation names
- **Values**: Depths (typically Measured Depth - MD)
- **Units**: Usually feet or meters

### 4. **Location Information (`Well.location`)**

Spatial coordinates and location metadata:

```python
well.location = {
    'latitude': 30.1234,
    'longitude': -95.5678,
    'elevation': 100.0,  # KB elevation
    'kb': 100.0,         # Kelly Bushing elevation
    'gl': 95.0,          # Ground Level elevation
    'coordinates': (30.1234, -95.5678),
    'crs': 'EPSG:4326'   # Coordinate Reference System
}
```

**Properties**:
- **`latitude`**: Well latitude
- **`longitude`**: Well longitude
- **`elevation`**: Surface elevation
- **`kb`**: Kelly Bushing elevation
- **`gl`**: Ground Level elevation
- **`coordinates`**: Tuple of (lat, lon)
- **`crs`**: Coordinate Reference System

### 5. **Unique Well Identifier (`Well.uwi`)**

```python
well.uwi = 'API-12345678901234'
# or
well.uwi = 'UWI-1234567890'
```

**Purpose**: Unique identifier for the well, typically:
- API number (American Petroleum Institute)
- UWI (Unique Well Identifier)
- Custom identifier

## Architecture Details

### 1. **Data Loading Architecture**

```
LAS File
    ↓
lasio.read()  [External library]
    ↓
Parse LAS Sections
    ├── ~V (Version)
    ├── ~W (Well)
    ├── ~C (Curves)
    ├── ~P (Parameters)
    ├── ~O (Other)
    └── ~A (ASCII Data)
    ↓
Well Object Creation
    ├── Extract header metadata
    ├── Parse curve data → DataFrame
    ├── Extract formation tops (if present)
    └── Extract location (if present)
    ↓
Well Object (Populated)
```

### 2. **Call Stack for Loading**

```python
welly.read_las('well.las')
    ↓
lasio.read('well.las')
    ↓
Parse LAS file sections
    ↓
Create Well object
    ├── Well.__init__()
    │   ├── Extract header sections
    │   ├── Parse ~W section → self.header['WELL']
    │   ├── Parse ~C section → self.header['CURVES']
    │   ├── Parse ~P section → self.header['PARAMETERS']
    │   ├── Parse ~O section → self.header['OTHER']
    │   └── Parse ~A section → self.data (DataFrame)
    │
    ├── Extract UWI
    │   └── self.uwi = header['WELL'].get('uwi')
    │
    └── Initialize other attributes
        ├── self.tops = {}
        └── self.location = {}
```

### 3. **Metadata Access Patterns**

#### Direct Access

```python
# Well name
well_name = well.header['WELL']['name']

# UWI
uwi = well.uwi

# Specific curve metadata
gr_unit = well.header['CURVES']['GR']['unit']

# Parameter value
bht = well.header['PARAMETERS']['BHT']['value']
```

#### Property-Based Access (if implemented)

```python
# Some versions may support:
well.name
well.uwi
well.depth_range
well.curves  # List of curve names
```

#### Dictionary-Like Access

```python
# Header sections
well.header['WELL']
well.header['CURVES']
well.header['PARAMETERS']
well.header['OTHER']

# Data curves
well.data['GR']
well.data['RHOB']
```

## Design Patterns

### 1. **Container Pattern: Project Class**

The `Project` class acts as a container for multiple `Well` objects:

```python
class Project:
    """
    Container for multiple Well objects.
    """
    def __init__(self, wells=None):
        self.wells = wells or []
    
    def __getitem__(self, index):
        return self.wells[index]
    
    def __len__(self):
        return len(self.wells)
    
    # ... methods for batch operations
```

**Usage**:
```python
# Load multiple wells
project = welly.read_las('well_*.las')

# Access individual wells
well_1 = project[0]
well_2 = project[1]

# Iterate over wells
for well in project:
    print(well.uwi)
```

### 2. **Lazy Loading**

Welly may implement lazy loading for large datasets:
- Headers are loaded immediately
- Curve data may be loaded on-demand
- Metadata is always available

### 3. **Alias Management**

Welly supports alias management for curve mnemonics:

```python
# Define aliases for common curve names
aliases = {
    'GR': ['GAMMA', 'GAMMA_RAY', 'GR_RAY'],
    'RHOB': ['DENSITY', 'DEN', 'RHO'],
    'NPHI': ['NEUTRON', 'NEUT', 'PHIN']
}

# Use aliases when accessing curves
well.data['GR']  # Works even if curve is named 'GAMMA'
```

### 4. **Data Quality Assessment**

Welly provides tools for data quality assessment:

```python
# Check for missing data
missing = well.data.isnull().sum()

# Check depth consistency
depth_gaps = well.data.index.to_series().diff()

# Quality flags
quality = well.qc_curve('GR')
```

## Metadata Structure Details

### Complete Header Structure

```python
well.header = {
    'WELL': {
        # Well identification
        'name': str,
        'uwi': str,
        'api': str,
        
        # Location
        'country': str,
        'state': str,
        'county': str,
        'field': str,
        'operator': str,
        
        # Logging information
        'service_company': str,
        'date': datetime or str,
        
        # Depth information
        'start_depth': float,
        'stop_depth': float,
        'step': float,
        'null': float,  # Null value indicator
        
        # Additional metadata
        'well_type': str,
        'well_status': str,
    },
    
    'CURVES': {
        'CURVE_NAME': {
            'mnemonic': str,
            'unit': str,
            'description': str,
            'api_code': str,
            'data_type': str,
        },
        # ... more curves
    },
    
    'PARAMETERS': {
        'PARAM_NAME': {
            'value': float or str,
            'unit': str,
            'description': str,
        },
        # ... more parameters
    },
    
    'OTHER': {
        'notes': str,
        'remarks': str,
        # ... unstructured data
    }
}
```

### Metadata Properties Summary

| Category | Property | Type | Description |
|----------|----------|------|--------------|
| **Well Info** | `name` | str | Well name/identifier |
| | `uwi` | str | Unique Well Identifier |
| | `api` | str | API number |
| | `operator` | str | Operating company |
| | `field` | str | Field name |
| | `date` | datetime | Logging date |
| **Location** | `latitude` | float | Well latitude |
| | `longitude` | float | Well longitude |
| | `elevation` | float | Surface elevation |
| | `kb` | float | Kelly Bushing elevation |
| **Depth** | `start_depth` | float | Start depth (MD) |
| | `stop_depth` | float | Stop depth (MD) |
| | `step` | float | Depth sampling interval |
| **Curves** | `mnemonic` | str | Curve name |
| | `unit` | str | Measurement unit |
| | `description` | str | Curve description |
| **Parameters** | `value` | float/str | Parameter value |
| | `unit` | str | Parameter unit |
| | `description` | str | Parameter description |

## Integration with Other Libraries

### 1. **Pandas Integration**

```python
# Well data is a pandas DataFrame
well.data.head()
well.data.describe()
well.data.plot()

# Can use pandas operations
filtered = well.data[well.data['GR'] > 50]
```

### 2. **Matplotlib Integration**

```python
import matplotlib.pyplot as plt

# Plot curves
fig, ax = plt.subplots(figsize=(8, 10))
ax.plot(well.data['GR'], well.data.index)
ax.set_xlabel('Gamma Ray (API)')
ax.set_ylabel('Depth (ft)')
plt.show()
```

### 3. **Striplog Integration**

```python
# Convert formation tops to striplog
from striplog import Striplog

striplog = well.to_striplog()
```

## Usage Examples

### Loading and Viewing Metadata

```python
import welly

# Load a well
well = welly.read_las('well.las')

# View well name
print(f"Well Name: {well.header['WELL']['name']}")
print(f"UWI: {well.uwi}")

# View all curves
print("Available Curves:")
for curve_name, curve_info in well.header['CURVES'].items():
    print(f"  {curve_name}: {curve_info['description']} ({curve_info['unit']})")

# View parameters
print("Parameters:")
for param_name, param_info in well.header['PARAMETERS'].items():
    print(f"  {param_name}: {param_info['value']} {param_info['unit']}")

# View location
if well.location:
    print(f"Location: {well.location['latitude']}, {well.location['longitude']}")
```

### Accessing Curve Data

```python
# Access specific curve
gr = well.data['GR']

# Get curve metadata
gr_unit = well.header['CURVES']['GR']['unit']
gr_desc = well.header['CURVES']['GR']['description']

# Access with depth
depth = well.data.index
gr_values = well.data['GR'].values

# Plot curve
import matplotlib.pyplot as plt
plt.figure(figsize=(4, 10))
plt.plot(gr_values, depth)
plt.xlabel(f'Gamma Ray ({gr_unit})')
plt.ylabel('Depth (ft)')
plt.gca().invert_yaxis()
plt.show()
```

### Working with Multiple Wells

```python
# Load multiple wells
project = welly.read_las('well_*.las')

# Access metadata for all wells
for well in project:
    print(f"{well.header['WELL']['name']}: {well.uwi}")
    print(f"  Curves: {list(well.data.columns)}")
    print(f"  Depth Range: {well.header['WELL']['start_depth']} - {well.header['WELL']['stop_depth']}")
```

## Best Practices

1. **Always Check Metadata**: Verify header information before processing data
2. **Handle Missing Data**: Check for null values in curves
3. **Validate Units**: Ensure unit consistency when comparing wells
4. **Use UWI**: Always use UWI for well identification
5. **Preserve Metadata**: Keep original metadata when processing data

## Conclusion

Welly provides a well-structured, object-oriented approach to handling well log data. Its architecture separates concerns between metadata (header), data (curves), and spatial information (location), making it easy to work with well data programmatically. The integration with pandas, matplotlib, and other scientific Python libraries makes it a powerful tool for well log analysis.

## References

- [Welly Documentation](https://code.agilescientific.com/welly/)
- [Welly GitHub Repository](https://github.com/agile-geoscience/welly)
- [LAS File Format Specification](https://www.cwls.org/products/las/)
- [Pandas Documentation](https://pandas.pydata.org/)
- [Matplotlib Documentation](https://matplotlib.org/)

