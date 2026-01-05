# Segyio: Parsing SEGY Files into Structured Objects

This document explains how the Python `segyio` library parses SEGY (SEG Y) files into structured objects. SEGY is the standard format for seismic data in the oil and gas industry.

## Overview

Segyio is a Python library for reading and writing SEGY files, which contain seismic reflection data. SEGY files are binary files with a specific structure containing textual headers, binary headers, trace headers, and trace data.

## SEGY File Structure

SEGY files follow a strict binary format:

```
┌─────────────────────────────────────┐
│ Textual EBCDIC Header (3200 bytes)  │
├─────────────────────────────────────┤
│ Binary Header (400 bytes)           │
├─────────────────────────────────────┤
│ Trace 1 Header (240 bytes)          │
│ Trace 1 Data (variable length)      │
├─────────────────────────────────────┤
│ Trace 2 Header (240 bytes)          │
│ Trace 2 Data (variable length)      │
├─────────────────────────────────────┤
│ ...                                 │
└─────────────────────────────────────┘
```

### Components

1. **Textual Header**: 3200 bytes of EBCDIC text (40 lines × 80 characters)
2. **Binary Header**: 400 bytes of binary metadata
3. **Trace Headers**: 240 bytes per trace (binary)
4. **Trace Data**: Variable length per trace (binary, typically 4-byte floats)

## Installation

```bash
pip install segyio
```

## Basic Usage

### Opening a SEGY File

```python
import segyio

# Open SEGY file in read mode
with segyio.open("seismic.sgy", "r") as f:
    # Access structured data
    print(f.tracecount)      # Number of traces
    print(f.samples)         # Number of samples per trace
    print(f.bin)             # Binary header
    print(f.text[0])         # First line of textual header
```

## Structured Data Format

### SegyFile Object

The `segyio.open()` function returns a `SegyFile` object that provides structured access to all SEGY components:

```python
with segyio.open("seismic.sgy", "r") as f:
    # File metadata
    f.tracecount    # int - Total number of traces
    f.samples       # int - Number of samples per trace
    f.sample_rate   # int - Sample interval in microseconds
    
    # Headers
    f.bin           # dict-like - Binary header
    f.text          # list - Textual header (40 lines)
    
    # Trace access
    f.trace         # array-like - Trace data access
    f.header        # array-like - Trace header access
    
    # Attributes
    f.attributes    # dict - Trace attribute access
```

### Textual Header (`f.text`)

The textual header is a list of 40 strings (80 characters each):

```python
with segyio.open("seismic.sgy", "r") as f:
    # Access textual header
    print(f.text[0])   # First line (80 characters)
    print(f.text[1])   # Second line
    print(len(f.text)) # 40 lines
    
    # Print all lines
    for i, line in enumerate(f.text):
        print(f"Line {i+1}: {line}")
    
    # Search for specific information
    full_text = "".join(f.text)
    if "SURVEY" in full_text:
        print("Survey information found")
```

**Structure:**
- List of 40 strings
- Each string is 80 characters
- Originally EBCDIC, segyio converts to ASCII/UTF-8
- Contains survey metadata, processing history, etc.

### Binary Header (`f.bin`)

The binary header is a dictionary-like object containing file-wide metadata:

```python
with segyio.open("seismic.sgy", "r") as f:
    # Access binary header fields
    print(f.bin[segyio.BinField.Traces])        # Number of traces
    print(f.bin[segyio.BinField.Samples])       # Samples per trace
    print(f.bin[segyio.BinField.SampleInterval]) # Sample interval (microseconds)
    print(f.bin[segyio.BinField.Format])        # Data format code
    
    # Common fields
    print(f.bin[segyio.BinField.Interval])      # Sample interval
    print(f.bin[segyio.BinField.SweepFrequencyStart])  # Sweep start frequency
    print(f.bin[segyio.BinField.SweepFrequencyEnd])    # Sweep end frequency
    
    # Iterate over all fields
    for field, value in f.bin.items():
        print(f"{field}: {value}")
```

**Common Binary Header Fields:**
- `Traces` - Number of traces
- `Samples` - Samples per trace
- `SampleInterval` - Sample interval in microseconds
- `Format` - Data format code (1=4-byte IBM float, 5=4-byte IEEE float, etc.)
- `MeasurementSystem` - Measurement system (1=meters, 2=feet)
- `SweepFrequencyStart` - Sweep start frequency (Hz)
- `SweepFrequencyEnd` - Sweep end frequency (Hz)

### Trace Data (`f.trace`)

Trace data is accessed as an array-like object:

```python
with segyio.open("seismic.sgy", "r") as f:
    # Access individual traces
    trace_0 = f.trace[0]      # First trace (numpy array)
    trace_1 = f.trace[1]      # Second trace
    
    # Access multiple traces
    traces_0_10 = f.trace[0:10]  # Traces 0-9 (2D numpy array)
    
    # Access all traces
    all_traces = f.trace[:]   # All traces (2D array: [trace, sample])
    
    # Iterate over traces
    for i, trace in enumerate(f.trace):
        print(f"Trace {i}: {trace.shape}, mean={trace.mean()}")
    
    # Access specific sample from specific trace
    sample_100_trace_5 = f.trace[5][100]  # Sample 100 of trace 5
```

**Structure:**
- `f.trace[i]` returns a 1D numpy array for trace `i`
- `f.trace[i:j]` returns a 2D numpy array (shape: `[j-i, samples]`)
- Data type depends on format code (typically float32)
- Shape: `(tracecount, samples)`

### Trace Headers (`f.header`)

Trace headers provide metadata for each trace:

```python
import segyio

with segyio.open("seismic.sgy", "r") as f:
    # Access individual trace header
    header_0 = f.header[0]    # First trace header (dict-like)
    
    # Access specific field
    x_coord = f.header[0][segyio.TraceField.SourceX]
    y_coord = f.header[0][segyio.TraceField.SourceY]
    cdp = f.header[0][segyio.TraceField.CDP]
    
    # Access field for all traces
    all_x = f.header[:][segyio.TraceField.SourceX]  # Array of X coordinates
    
    # Iterate over trace headers
    for i, header in enumerate(f.header):
        x = header[segyio.TraceField.SourceX]
        y = header[segyio.TraceField.SourceY]
        print(f"Trace {i}: X={x}, Y={y}")
    
    # Access multiple fields
    for i in range(f.tracecount):
        header = f.header[i]
        x = header[segyio.TraceField.SourceX]
        y = header[segyio.TraceField.SourceY]
        cdp = header[segyio.TraceField.CDP]
        offset = header[segyio.TraceField.offset]
        print(f"Trace {i}: CDP={cdp}, Offset={offset}, X={x}, Y={y}")
```

**Common Trace Header Fields:**
- `SourceX`, `SourceY` - Source coordinates
- `GroupX`, `GroupY` - Receiver group coordinates
- `CDP` - Common Depth Point number
- `offset` - Source-receiver offset
- `TraceSequenceNumber` - Trace sequence number
- `TraceValueMeasurementUnit` - Measurement unit
- `CoordinateUnits` - Coordinate units
- `DelayRecordingTime` - Delay recording time
- `SampleCount` - Number of samples
- `SampleInterval` - Sample interval

### Attributes (`f.attributes`)

Attributes provide convenient access to trace header fields:

```python
with segyio.open("seismic.sgy", "r") as f:
    # Access attributes (same as header fields)
    x_coords = f.attributes(segyio.TraceField.SourceX)[:]
    y_coords = f.attributes(segyio.TraceField.SourceY)[:]
    cdps = f.attributes(segyio.TraceField.CDP)[:]
    
    # Get attribute for specific trace
    x_0 = f.attributes(segyio.TraceField.SourceX)[0]
    
    # Attributes return array-like objects
    all_offsets = f.attributes(segyio.TraceField.offset)[:]
```

## Parsing Process

### 1. File Opening

```python
# segyio.open() performs:
# 1. Opens file in binary mode
# 2. Reads textual header (3200 bytes)
# 3. Reads binary header (400 bytes)
# 4. Scans file to determine trace count and structure
# 5. Creates SegyFile object with structured access
```

### 2. Textual Header Parsing

```python
# Textual header is:
# - 3200 bytes total (40 lines × 80 characters)
# - Originally EBCDIC encoded
# - Converted to ASCII/UTF-8 by segyio
# - Stored as list of 40 strings

# Parsing steps:
# 1. Read 3200 bytes from start of file
# 2. Decode from EBCDIC to ASCII/UTF-8
# 3. Split into 40 lines of 80 characters
# 4. Store as f.text list
```

### 3. Binary Header Parsing

```python
# Binary header is:
# - 400 bytes of binary data
# - Fixed structure defined by SEGY standard
# - Parsed into dictionary-like object

# Parsing steps:
# 1. Read 400 bytes after textual header
# 2. Parse according to SEGY binary header structure
# 3. Extract fields (traces, samples, format, etc.)
# 4. Store as f.bin dictionary
```

### 4. Trace Structure Detection

```python
# segyio determines trace structure by:
# 1. Reading sample count from binary header
# 2. Determining data format (from binary header)
# 3. Calculating trace size: header (240 bytes) + data (samples × bytes_per_sample)
# 4. Scanning file to count traces
# 5. Building index for random access
```

### 5. Trace Header Parsing

```python
# Each trace header is:
# - 240 bytes of binary data
# - Fixed structure with specific field offsets
# - Parsed on-demand when accessed

# Parsing steps:
# 1. Read 240 bytes before each trace data
# 2. Parse according to SEGY trace header structure
# 3. Extract fields using known byte offsets
# 4. Return as dictionary-like object
```

### 6. Trace Data Parsing

```python
# Trace data is:
# - Variable length (samples × bytes_per_sample)
# - Binary format (typically 4-byte floats)
# - Parsed into numpy arrays

# Parsing steps:
# 1. Determine data format from binary header
# 2. Read trace data bytes
# 3. Convert from binary to numpy array
# 4. Handle endianness (big-endian for SEGY)
# 5. Return as numpy array
```

## Complete Example

```python
import segyio
import numpy as np
import matplotlib.pyplot as plt

# Open SEGY file
with segyio.open("seismic.sgy", "r") as f:
    # Print file information
    print(f"Traces: {f.tracecount}")
    print(f"Samples per trace: {f.samples}")
    print(f"Sample interval: {f.bin[segyio.BinField.SampleInterval]} microseconds")
    print(f"Data format: {f.bin[segyio.BinField.Format]}")
    
    # Print textual header
    print("\nTextual Header:")
    for i, line in enumerate(f.text[:5]):  # First 5 lines
        print(f"Line {i+1}: {line}")
    
    # Access trace data
    trace_0 = f.trace[0]
    print(f"\nFirst trace shape: {trace_0.shape}")
    print(f"First trace stats: min={trace_0.min()}, max={trace_0.max()}, mean={trace_0.mean()}")
    
    # Access trace headers
    print("\nTrace Headers (first 5 traces):")
    for i in range(min(5, f.tracecount)):
        header = f.header[i]
        x = header[segyio.TraceField.SourceX]
        y = header[segyio.TraceField.SourceY]
        cdp = header[segyio.TraceField.CDP]
        offset = header[segyio.TraceField.offset]
        print(f"Trace {i}: CDP={cdp}, Offset={offset}, X={x}, Y={y}")
    
    # Extract coordinates
    x_coords = f.attributes(segyio.TraceField.SourceX)[:]
    y_coords = f.attributes(segyio.TraceField.SourceY)[:]
    cdps = f.attributes(segyio.TraceField.CDP)[:]
    
    # Load all traces into memory
    all_traces = f.trace[:]  # Shape: (tracecount, samples)
    
    # Create seismic section (2D array)
    seismic_section = all_traces.T  # Transpose: (samples, tracecount)
    
    # Plot seismic section
    plt.figure(figsize=(12, 8))
    plt.imshow(seismic_section, aspect='auto', cmap='seismic')
    plt.xlabel('Trace Number')
    plt.ylabel('Sample Number')
    plt.title('Seismic Section')
    plt.colorbar(label='Amplitude')
    plt.show()
```

## Advanced Features

### Reading Specific Traces

```python
with segyio.open("seismic.sgy", "r") as f:
    # Read specific traces
    traces = f.trace[10:20]  # Traces 10-19
    
    # Read every Nth trace
    every_10th = f.trace[::10]  # Every 10th trace
    
    # Read specific trace indices
    indices = [0, 5, 10, 15, 20]
    selected_traces = np.array([f.trace[i] for i in indices])
```

### Reading Specific Samples

```python
with segyio.open("seismic.sgy", "r") as f:
    # Read specific time/depth window
    start_sample = 100
    end_sample = 500
    windowed_traces = f.trace[:, start_sample:end_sample]
    
    # Read specific sample across all traces
    sample_100 = np.array([f.trace[i][100] for i in range(f.tracecount)])
```

### Working with Coordinates

```python
with segyio.open("seismic.sgy", "r") as f:
    # Extract coordinate arrays
    x_coords = f.attributes(segyio.TraceField.SourceX)[:]
    y_coords = f.attributes(segyio.TraceField.SourceY)[:]
    
    # Create coordinate pairs
    coordinates = np.column_stack([x_coords, y_coords])
    
    # Find traces within bounding box
    x_min, x_max = 100000, 200000
    y_min, y_max = 500000, 600000
    
    mask = ((x_coords >= x_min) & (x_coords <= x_max) &
            (y_coords >= y_min) & (y_coords <= y_max))
    
    selected_traces = f.trace[mask]
```

### Handling Different Data Formats

```python
with segyio.open("seismic.sgy", "r") as f:
    format_code = f.bin[segyio.BinField.Format]
    
    # Format codes:
    # 1 = 4-byte IBM floating point
    # 2 = 4-byte signed integer
    # 3 = 2-byte signed integer
    # 5 = 4-byte IEEE floating point
    # 8 = 1-byte signed integer
    
    print(f"Data format: {format_code}")
    
    # segyio automatically handles format conversion
    trace = f.trace[0]  # Always returns numpy array with correct dtype
    print(f"Data type: {trace.dtype}")
```

### Memory-Efficient Reading

```python
# For large files, read traces in chunks
with segyio.open("large_seismic.sgy", "r") as f:
    chunk_size = 1000
    for start in range(0, f.tracecount, chunk_size):
        end = min(start + chunk_size, f.tracecount)
        chunk = f.trace[start:end]
        # Process chunk
        process_traces(chunk)
```

## Writing SEGY Files

```python
import segyio
import numpy as np

# Create synthetic seismic data
traces = 100
samples = 1000
data = np.random.randn(traces, samples).astype(np.float32)

# Create SEGY file
with segyio.open("output.sgy", "w", ignore_geometry=True) as f:
    # Set binary header
    f.bin = {
        segyio.BinField.Traces: traces,
        segyio.BinField.Samples: samples,
        segyio.BinField.SampleInterval: 4000,  # 4ms
        segyio.BinField.Format: 5,  # IEEE float
    }
    
    # Set textual header
    f.text[0] = "C 1 Synthetic Seismic Data".ljust(80)
    
    # Write trace data
    for i in range(traces):
        f.trace[i] = data[i]
        f.header[i] = {
            segyio.TraceField.TraceSequenceNumber: i + 1,
            segyio.TraceField.SourceX: 100000 + i * 25,
            segyio.TraceField.SourceY: 500000 + i * 25,
            segyio.TraceField.CDP: i + 1,
        }
```

## Data Structure Summary

### SegyFile Object Structure

```python
f = segyio.open("file.sgy", "r")

# Metadata
f.tracecount    # int - Number of traces
f.samples       # int - Samples per trace
f.sample_rate   # int - Sample interval (microseconds)

# Headers
f.text          # list[str] - Textual header (40 lines)
f.bin           # dict-like - Binary header

# Data Access
f.trace         # array-like - Trace data (numpy arrays)
f.header        # array-like - Trace headers (dict-like objects)
f.attributes    # callable - Trace attribute accessor
```

### Trace Header Structure

```python
header = f.header[0]

# Dictionary-like access
header[segyio.TraceField.SourceX]      # X coordinate
header[segyio.TraceField.SourceY]      # Y coordinate
header[segyio.TraceField.CDP]          # CDP number
header[segyio.TraceField.offset]       # Offset
header[segyio.TraceField.SampleCount]  # Sample count
# ... many more fields
```

### Trace Data Structure

```python
# Single trace
trace = f.trace[0]        # numpy.ndarray, shape: (samples,)

# Multiple traces
traces = f.trace[0:10]    # numpy.ndarray, shape: (10, samples)

# All traces
all_traces = f.trace[:]   # numpy.ndarray, shape: (tracecount, samples)
```

## Parsing Details

### Byte-Level Structure

```
Offset    Size    Description
─────────────────────────────────────
0         3200    Textual EBCDIC header
3200      400     Binary header
3600      240     Trace 1 header
3840      N       Trace 1 data
3840+N    240     Trace 2 header
...
```

### Binary Header Fields (Key Offsets)

```python
# Binary header is 400 bytes
# Key fields (byte offsets):
# 3200-3202: Job identification number
# 3204-3206: Line number
# 3208-3210: Reel number
# 3212-3214: Number of data traces
# 3216-3218: Number of auxiliary traces
# 3220-3222: Sample interval (microseconds)
# 3224-3226: Original sample interval
# 3228-3230: Number of samples per trace
# 3232-3234: Original number of samples
# 3236-3238: Data sample format code
# ...
```

### Trace Header Fields (Key Offsets)

```python
# Trace header is 240 bytes
# Key fields (byte offsets):
# 0-3:     Trace sequence number
# 4-7:     Trace sequence number (reel)
# 8-11:    Original field record number
# 12-15:   Trace number (field record)
# 16-19:   Energy source point number
# 20-23:   CDP ensemble number
# 24-27:   Trace number (CDP)
# 28-31:   Trace identification code
# 72-75:   Source coordinate X
# 76-79:   Source coordinate Y
# 80-83:   Group coordinate X
# 84-87:   Group coordinate Y
# 108-111: Coordinate units
# 112-115: Source-receiver offset
# ...
```

## Best Practices

### 1. Always Use Context Manager

```python
# Good
with segyio.open("file.sgy", "r") as f:
    data = f.trace[:]

# Bad - file may not be properly closed
f = segyio.open("file.sgy", "r")
data = f.trace[:]
```

### 2. Check File Validity

```python
try:
    with segyio.open("file.sgy", "r") as f:
        if f.tracecount == 0:
            raise ValueError("No traces in file")
        if f.samples == 0:
            raise ValueError("No samples in traces")
except segyio.SegyioError as e:
    print(f"SEGY file error: {e}")
```

### 3. Handle Large Files Efficiently

```python
# For very large files, avoid loading all traces
with segyio.open("large.sgy", "r") as f:
    # Process in chunks
    chunk_size = 1000
    for i in range(0, f.tracecount, chunk_size):
        chunk = f.trace[i:i+chunk_size]
        process_chunk(chunk)
```

### 4. Validate Coordinates

```python
with segyio.open("file.sgy", "r") as f:
    x_coords = f.attributes(segyio.TraceField.SourceX)[:]
    y_coords = f.attributes(segyio.TraceField.SourceY)[:]
    
    # Check for valid coordinates
    valid_mask = (x_coords != 0) & (y_coords != 0)
    if not valid_mask.all():
        print("Warning: Some traces have invalid coordinates")
```

### 5. Handle Different Endianness

```python
# segyio automatically handles endianness
# SEGY standard is big-endian, but segyio can read both
with segyio.open("file.sgy", "r", endian='big') as f:  # Explicit
    data = f.trace[:]
```

## Summary

Segyio parses SEGY files by:

1. **File Structure Detection**: Identifies textual header, binary header, and trace structure
2. **Header Parsing**: 
   - Converts EBCDIC textual header to ASCII/UTF-8
   - Parses binary header into dictionary-like object
3. **Trace Indexing**: Builds index for efficient trace access
4. **On-Demand Parsing**: Parses trace headers and data when accessed
5. **Format Conversion**: Handles different data formats (IBM float, IEEE float, integers)
6. **Structure Creation**: Organizes data into:
   - `text` - Textual header (list of strings)
   - `bin` - Binary header (dictionary)
   - `trace` - Trace data (numpy arrays)
   - `header` - Trace headers (dictionary-like objects)
   - `attributes` - Convenient attribute accessor

The result is a structured `SegyFile` object that provides efficient, numpy-based access to all SEGY file components, making it easy to work with seismic data in Python.

