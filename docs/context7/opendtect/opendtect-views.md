# OpenDTect Views and Visualization

A comprehensive list of view types and visualization capabilities in OpenDTect.

## Core View Types

### 3D Scene Viewer
- Primary visualization environment for 3D seismic data
- Supports multiple independent scenes with separate display trees
- Display modes: two-way time, depth, and horizon-flattened data
- On-the-fly coordinate transformations
- Display elements can hold up to 8 layers with transparency controls

### 2D Viewer
- Supports two visualization modes:
  - **Variable Density** - color-mapped amplitude display
  - **Wiggles + Variable Area** - traditional wiggle trace display
- Different attributes can be assigned to each mode independently
- Used for flat (2D) viewing of seismic sections

### Specialized Viewers
| Viewer Type | Purpose |
|-------------|---------|
| Seismic Data Viewer | 2D/3D seismic display |
| Well Log Viewer | Display well log data |
| Cross-Plot Viewer | Multi-attribute analysis |
| Pre-Stack Gather Viewer | Gather display and QC |

## Display Elements (Displayable in 3D Scene or 2D Viewers)

- **In-lines** - Vertical sections along inline direction
- **Cross-lines** - Vertical sections along crossline direction
- **Z-slices (Time slices)** - Horizontal slices at constant time/depth
- **Random lines** - Arbitrary vertical sections
- **Horizons** - Interpreted surfaces
- **2D Lines** - 2D seismic line display
- **Faults** - Fault surface visualization
- **Geobodies** - 3D volumetric bodies

## Pre-Stack Visualization

- Display pre-stack gathers in 3D scene alongside seismic sections
- Flat viewer display for detailed gather analysis
- Pre-processing options: mutes, super-gathers, scaling
- AVO attribute computation and display

## Pro Version Additional Views

| View | Description |
|------|-------------|
| **Basemap** | Map-based navigation to populate 3D scenes and open 2D viewers |
| **3D Wheeler Scene** | Chronostratigraphic visualization |
| **Well Correlation Panel** | Multi-well log correlation display |
| **Semblance Gathers** | Velocity analysis display |

## Interpretation Workflows

### Horizon Picking
- Available in both 3D and 2D viewers
- Manual and auto-tracking modes

### Fault Interpretation
- 3D fault surface picking
- Fault stick display

### Attribute Analysis
- Compute and display seismic attributes
- Volume and section-based computation

## Sources

- [OpenDTect System Overview](https://doc.opendtect.org/7.0.0/doc/od_userdoc/content/getting_started/system_overview.htm)
- [OpenDTect 2D Viewer Documentation](https://doc.opendtect.org/6.6.0/doc/od_userdoc/content/getting_started/viewer_2d.htm)
- [dGB Earth Sciences - OpendTect](https://dgbes.com/software/opendtect)
