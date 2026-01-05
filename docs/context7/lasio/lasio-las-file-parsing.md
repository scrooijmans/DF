# Lasio: Parsing LAS Files into Structured Format

This document explains how the Python `lasio` library parses LAS (Log ASCII Standard) files into a structured format.

## Overview

Lasio is a Python library for reading and writing LAS files, which are the standard format for well log data in the oil and gas industry. LAS files contain well metadata, curve definitions, parameters, and numerical log data.

## LAS File Structure

LAS files follow a structured text format with specific sections:

```
~Version Information
~Well Information
~Parameter Information
~Curve Information
~ASCII Data
```

### Section Markers

- `~V` or `~Version Information` - Version and format metadata
- `~W` or `~Well Information` - Well identification and location data
- `~P` or `~Parameter Information` - Processing parameters
- `~C` or `~Curve Information` - Curve definitions (mnemonics, units, descriptions)
- `~A` or `~ASCII` or `~Log_Data` - Numerical data values

## Installation

```bash
pip install lasio
```

## Basic Usage

### Reading a LAS File

```python
import lasio

# Read LAS file
las = lasio.read("well_log.las")

# Access structured data
print(las.version)  # Version information
print(las.well)     # Well metadata
print(las.curves)   # Curve definitions
print(las.params)   # Parameters
print(las.data)     # Numerical data as numpy array
```

## Structured Data Format

### Version Section (`las.version`)

Contains format version and wrap mode:

```python
# Access version information
print(las.version)
# Output:
# Mnemonic  Unit  Value  Description
# --------  ----  -----  -----------
# VERS           2.0    CWLS LOG ASCII STANDARD - VERSION 2.0
# WRAP            NO     ONE LINE PER DEPTH STEP

# Access specific values
version = las.version['VERS'].value  # "2.0"
wrap_mode = las.version['WRAP'].value  # "NO"
```

**Structure:**
- Dictionary-like access by mnemonic
- Each item has: `mnemonic`, `unit`, `value`, `description`

### Well Section (`las.well`)

Contains well identification and location data:

```python
# Access well information
print(las.well)
# Output:
# Mnemonic  Unit  Value       Description
# --------  ----  -----       -----------
# STRT      M     0.05        FIRST INDEX VALUE
# STOP      M     136.6       LAST INDEX VALUE
# STEP      M     0.05        STEP
# NULL            -99999      NULL VALUE
# WELL            Scorpio E1  WELL
# UWI             6038-187    WUNT

# Access specific values
well_name = las.well['WELL'].value
start_depth = las.well['STRT'].value
stop_depth = las.well['STOP'].value
null_value = las.well['NULL'].value
```

**Common Well Mnemonics:**
- `WELL` - Well name
- `STRT` - Start depth/index
- `STOP` - Stop depth/index
- `STEP` - Depth step/increment
- `NULL` - Null value representation
- `UWI` - Unique Well Identifier
- `LOC` - Location
- `DATE` - Log date

### Curve Section (`las.curves`)

Defines the log curves (columns) in the data section:

```python
# Access curve information
print(las.curves)
# Output:
# Mnemonic  Unit   Value  Description
# --------  ----   -----  -----------
# DEPT      M             DEPTH
# CALI      MM            CALI
# GR        GAPI          GAMMA RAY
# RHOB      G/CM3         BULK DENSITY

# Access curve definitions
for curve in las.curves:
    print(f"{curve.mnemonic} - {curve.unit} - {curve.description}")

# Get specific curve
depth_curve = las.curves[0]  # First curve (usually depth)
print(depth_curve.mnemonic)  # "DEPT"
print(depth_curve.unit)      # "M"
print(depth_curve.description)  # "DEPTH"
```

**Structure:**
- List of curve objects
- Each curve has: `mnemonic`, `unit`, `description`
- Order matches data columns

### Parameter Section (`las.params`)

Contains processing parameters and metadata:

```python
# Access parameters
print(las.params)
# Output:
# Mnemonic    Unit  Value                    Description
# --------    ----  -----                    -----------
# BS                216 mm                   BS
# MUD               Water                    MUD
# STEP              5 cm                     STEP

# Access specific parameter
mud_type = las.params['MUD'].value
```

### Data Section (`las.data`)

Numerical log data as a NumPy array:

```python
# Access data array
data = las.data  # NumPy array: shape (n_depth_points, n_curves)

# Access by curve mnemonic
depth = las['DEPT']  # Depth values
gamma_ray = las['GR']  # Gamma ray values

# Access by index
first_curve = las.data[:, 0]  # First column
first_depth_point = las.data[0, :]  # First row

# Convert to pandas DataFrame
df = las.df()  # DataFrame with curve mnemonics as column names
```

## Parsing Process

### 1. Section Detection

Lasio identifies sections by section markers:

```python
# Section markers start with ~
# ~V or ~Version Information
# ~W or ~Well Information
# ~P or ~Parameter Information
# ~C or ~Curve Information
# ~A or ~ASCII or ~Log_Data
```

### 2. Header Parsing

Each section header line follows a pattern:

```
Mnemonic.Unit Value :Description
```

Example:
```
DEPT.M                          :DEPTH
GR  .GAPI                       :GAMMA RAY
RHOB.G/CM3                      :BULK DENSITY
```

Lasio parses these using regex patterns to extract:
- **Mnemonic**: Curve/parameter identifier
- **Unit**: Measurement unit
- **Value**: Default or metadata value (optional)
- **Description**: Human-readable description

### 3. Data Parsing

The ASCII data section contains numerical values:

**Unwrapped Mode** (one line per depth):
```
0.05    8.5    45.2    2.65
0.10    8.6    45.5    2.66
0.15    8.7    45.8    2.67
```

**Wrapped Mode** (multiple lines per depth):
```
0.05    8.5
45.2    2.65
0.10    8.6
45.5    2.66
```

Lasio:
1. Detects wrap mode from version section
2. Parses delimiter (space, tab, comma)
3. Handles null values (replaces with NaN)
4. Converts strings to floats
5. Organizes into columns matching curve definitions

### 4. Null Value Handling

```python
# Null values are replaced with NaN
null_value = las.well['NULL'].value  # e.g., -99999

# In data array, null values become NaN
import numpy as np
mask = np.isnan(las['GR'])  # Find null values
```

## Complete Example

```python
import lasio
import pandas as pd
import numpy as np

# Read LAS file
las = lasio.read("well_log.las")

# Access metadata
print("Well Name:", las.well['WELL'].value)
print("Start Depth:", las.well['STRT'].value)
print("Stop Depth:", las.well['STOP'].value)
print("Step:", las.well['STEP'].value)
print("Null Value:", las.well['NULL'].value)

# Access version info
print("LAS Version:", las.version['VERS'].value)
print("Wrap Mode:", las.version['WRAP'].value)

# List all curves
print("\nCurves:")
for i, curve in enumerate(las.curves):
    print(f"{i}: {curve.mnemonic} ({curve.unit}) - {curve.description}")

# Access data
df = las.df()  # Convert to pandas DataFrame
print("\nData Shape:", df.shape)
print("\nFirst few rows:")
print(df.head())

# Access specific curves
depth = las['DEPT']
gamma_ray = las['GR']
density = las['RHOB']

# Filter data
mask = (depth >= 100) & (depth <= 200)
filtered_gr = gamma_ray[mask]
filtered_depth = depth[mask]

# Statistics
print(f"\nGamma Ray Statistics:")
print(f"  Min: {np.nanmin(gamma_ray)}")
print(f"  Max: {np.nanmax(gamma_ray)}")
print(f"  Mean: {np.nanmean(gamma_ray)}")
print(f"  Null count: {np.isnan(gamma_ray).sum()}")
```

## Advanced Features

### Handling Different LAS Versions

Lasio supports LAS 1.2, 2.0, and 3.0:

```python
# Check version
version = float(las.version['VERS'].value)

if version < 2.0:
    print("LAS 1.2 format")
elif version < 3.0:
    print("LAS 2.0 format")
else:
    print("LAS 3.0 format")
```

### Wrapped vs Unwrapped Data

```python
# Check wrap mode
is_wrapped = las.version['WRAP'].value.upper() == 'YES'

if is_wrapped:
    print("Data is wrapped (multiple lines per depth)")
else:
    print("Data is unwrapped (one line per depth)")
```

### Custom Delimiters

Lasio automatically detects delimiters (space, tab, comma):

```python
# Delimiter is detected from data section
# Can be space, tab, or comma
```

### Duplicate Mnemonics

Lasio handles duplicate curve mnemonics by appending numbers:

```python
# If multiple curves have same mnemonic:
# GR -> GR
# GR -> GR:1
# GR -> GR:2
```

## Data Structure Summary

### LASFile Object Structure

```python
las = lasio.read("file.las")

# Sections
las.version    # VersionSection - dict-like, keyed by mnemonic
las.well       # WellSection - dict-like, keyed by mnemonic
las.params     # ParameterSection - dict-like, keyed by mnemonic
las.curves     # CurveSection - list of CurveItem objects

# Data
las.data       # numpy.ndarray - shape (n_points, n_curves)
las.df()       # pandas.DataFrame - data with curve names as columns

# Access by mnemonic
las['DEPT']    # numpy.ndarray - depth values
las['GR']      # numpy.ndarray - gamma ray values
```

### Section Item Structure

Each item in version/well/params sections:

```python
item = las.well['WELL']
item.mnemonic     # "WELL"
item.unit         # "" (empty if no unit)
item.value        # "Scorpio E1"
item.description  # "WELL"
```

### Curve Item Structure

Each curve in curves section:

```python
curve = las.curves[0]
curve.mnemonic     # "DEPT"
curve.unit         # "M"
curve.description  # "DEPTH"
curve.value        # None (curves don't have values in header)
```

## Parsing Details

### Header Line Parsing

Lasio uses flexible regex patterns to parse header lines:

```python
# Pattern: Mnemonic.Unit Value :Description
# Examples:
# "DEPT.M                          :DEPTH"
# "GR  .GAPI                       :GAMMA RAY"
# "NULL            -99999          :NULL VALUE"

# Regex pattern (simplified):
# ^([^\s.]+)\s*\.\s*([^\s:]+)?\s*([0-9 ]*)?:\s*(.*)$
# Captures: mnemonic, unit, value, description
```

### Data Line Parsing

```python
# Unwrapped mode (space/tab/comma delimited):
# "0.05    8.5    45.2    2.65"
# Split by delimiter, convert to float, handle nulls

# Wrapped mode:
# Multiple lines per depth point
# Accumulate values, then distribute to columns
```

### Null Value Replacement

```python
# 1. Read null value from ~W section
null_value = las.well['NULL'].value  # e.g., -99999

# 2. During data parsing, replace null_value with NaN
# "0.05    -99999    45.2" -> [0.05, NaN, 45.2]
```

## Error Handling

### Common Issues

```python
import lasio

try:
    las = lasio.read("file.las")
except lasio.exceptions.LASFileError as e:
    print(f"LAS file error: {e}")
except Exception as e:
    print(f"Error reading file: {e}")

# Handle missing curves
if 'GR' in [c.mnemonic for c in las.curves]:
    gamma_ray = las['GR']
else:
    print("Gamma Ray curve not found")
```

## Integration with Pandas

```python
import lasio
import pandas as pd

# Convert to DataFrame
df = las.df()

# Set depth as index
df.index = las['DEPT']
df.index.name = 'DEPTH'

# Access by depth range
subset = df.loc[100:200]

# Plot curves
df[['GR', 'RHOB', 'NPHI']].plot()
```

## Writing LAS Files

```python
# Create new LAS file
las = lasio.LASFile()

# Set version
las.version = [('VERS', 2.0, 'CWLS LOG ASCII STANDARD - VERSION 2.0')]

# Set well information
las.well['WELL'] = lasio.HeaderItem('WELL', value='My Well')
las.well['STRT'] = lasio.HeaderItem('STRT', value=0.0, unit='M')
las.well['STOP'] = lasio.HeaderItem('STOP', value=100.0, unit='M')

# Add curves
las.curves.append(lasio.CurveItem(mnemonic='DEPT', unit='M', value='', descr='DEPTH'))
las.curves.append(lasio.CurveItem(mnemonic='GR', unit='GAPI', value='', descr='GAMMA RAY'))

# Add data
import numpy as np
depth = np.arange(0, 100, 0.5)
gr = np.random.randn(len(depth)) * 10 + 50
las.data = np.column_stack([depth, gr])

# Write to file
las.write("output.las")
```

## Best Practices

### 1. Check File Validity

```python
try:
    las = lasio.read("file.las")
    # Verify required sections exist
    assert 'WELL' in las.well
    assert len(las.curves) > 0
    assert las.data is not None
except Exception as e:
    print(f"Invalid LAS file: {e}")
```

### 2. Handle Missing Data

```python
# Check for null values
null_count = np.isnan(las['GR']).sum()
if null_count > 0:
    print(f"Warning: {null_count} null values in GR curve")

# Filter out nulls
valid_mask = ~np.isnan(las['GR'])
valid_gr = las['GR'][valid_mask]
```

### 3. Validate Curve Names

```python
# Get available curves
available_curves = [c.mnemonic for c in las.curves]

# Check before accessing
required_curves = ['DEPT', 'GR', 'RHOB']
missing = [c for c in required_curves if c not in available_curves]
if missing:
    print(f"Missing curves: {missing}")
```

### 4. Handle Different Versions

```python
version = float(las.version['VERS'].value)

if version >= 3.0:
    # LAS 3.0 specific features
    # May have additional sections like ~Log_Definition
    pass
```

## Summary

Lasio parses LAS files by:

1. **Section Detection**: Identifies sections by `~` markers
2. **Header Parsing**: Extracts mnemonics, units, values, descriptions using regex
3. **Data Parsing**: Converts ASCII numbers to NumPy arrays, handling:
   - Wrapped vs unwrapped mode
   - Multiple delimiters (space, tab, comma)
   - Null value replacement
   - Column organization
4. **Structure Creation**: Organizes data into:
   - `version` - Version metadata (dict-like)
   - `well` - Well information (dict-like)
   - `params` - Parameters (dict-like)
   - `curves` - Curve definitions (list)
   - `data` - Numerical data (NumPy array)

The result is a structured `LASFile` object that provides easy access to all LAS file components, making it simple to work with well log data in Python.

