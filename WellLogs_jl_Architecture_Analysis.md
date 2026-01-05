# WellLogs.jl Architecture & Runtime Behavior Analysis

## Executive Summary

WellLogs.jl is a Julia package for visualizing well log data that integrates with Plots.jl through the recipe system. It provides abstractions for tracks, curves, wells, and layouts, enabling flexible composition of multi-track and multi-well visualizations with synchronized depth axes.

---

## 1. Core Abstractions

### 1.1 Type Hierarchy

WellLogs.jl uses Julia's type system to define core abstractions:

```julia
# Core data structures
abstract type WellLogData end
abstract type TrackType end
abstract type CurveType end

# Well representation
struct Well
    name::String
    data::DataFrame  # Depth-indexed data
    tracks::Vector{Track}
    metadata::Dict{String, Any}
end

# Track representation
struct Track
    id::String
    curves::Vector{Curve}
    scale::ScaleType  # Linear, Logarithmic, etc.
    width::Float64
    position::Int  # Horizontal position in layout
    depth_range::Tuple{Float64, Float64}
end

# Curve representation
struct Curve
    name::String
    mnemonic::String
    data::Vector{Float64}  # Values
    depth::Vector{Float64}  # Depth values (shared across curves in well)
    unit::String
    color::Color
    linestyle::LineStyle
    track_id::String
end

# Layout representation
struct WellLogLayout
    wells::Vector{Well}
    tracks_per_well::Vector{Vector{Track}}
    depth_sync::Bool  # Whether depth axes are synchronized
    track_widths::Dict{String, Float64}
    spacing::Float64
end
```

### 1.2 Abstractions Overview

**Wells:**
- **Purpose**: Container for all data and tracks associated with a single well
- **Properties**:
  - Well name/identifier
  - DataFrame containing all curve data (depth-indexed)
  - Collection of tracks
  - Metadata (location, dates, etc.)

**Tracks:**
- **Purpose**: Vertical sections displaying specific curve types
- **Properties**:
  - Track identifier
  - Collection of curves to display
  - Scale type (linear, logarithmic)
  - Width and position in layout
  - Depth range (may be subset of well depth range)

**Curves:**
- **Purpose**: Individual measurement series (e.g., Gamma Ray, Resistivity)
- **Properties**:
  - Curve name and mnemonic
  - Data values array
  - Depth array (typically shared reference)
  - Visual styling (color, line style)
  - Associated track

**Layouts:**
- **Purpose**: Spatial arrangement of tracks and wells
- **Properties**:
  - Collection of wells
  - Track organization per well
  - Depth synchronization flag
  - Track widths and spacing configuration

---

## 2. Depth Sharing and Synchronization

### 2.1 Shared Depth Axis

WellLogs.jl ensures all tracks within a well (or across wells in a layout) share the same depth axis:

```julia
# Depth is stored once per well
struct Well
    depth::Vector{Float64}  # Shared depth array
    curves::Dict{String, Vector{Float64}}  # Curve data keyed by mnemonic
    # ...
end

# All curves reference the same depth array
function get_curve(well::Well, mnemonic::String)
    return Curve(
        mnemonic,
        well.curves[mnemonic],
        well.depth,  # Shared reference
        # ...
    )
end
```

### 2.2 Depth Synchronization Mechanism

**Within a Single Well:**
- All tracks share the same depth vector (reference, not copy)
- Depth range is determined from the well's overall depth range
- Tracks can have subset depth ranges for zooming, but share the same scale

**Across Multiple Wells:**
- When `depth_sync = true` in layout, all wells use the same depth domain
- Depth domain is calculated as: `[min(all well depths), max(all well depths)]`
- Each well's tracks are scaled to this shared domain

```julia
function synchronize_depth(layout::WellLogLayout)
    if layout.depth_sync
        # Calculate global depth range
        all_depths = vcat([w.depth for w in layout.wells]...)
        global_min = minimum(all_depths)
        global_max = maximum(all_depths)
        
        # Apply to all wells
        for well in layout.wells
            well.depth_domain = (global_min, global_max)
        end
    end
end
```

### 2.3 Depth Scale in Plots.jl

When rendering with Plots.jl, the depth axis is configured as:

```julia
# In the recipe definition
@recipe function f(well::Well)
    # Set depth as Y-axis (inverted for geological convention)
    yflip --> true
    ylabel --> "Depth (m)"
    
    # Depth domain from well
    ylims --> (maximum(well.depth), minimum(well.depth))  # Inverted
    
    # Return series data for each curve
    for track in well.tracks
        for curve in track.curves
            @series begin
                label --> curve.mnemonic
                color --> curve.color
                linestyle --> curve.linestyle
                well.depth, curve.data  # (x, y) = (depth, value)
            end
        end
    end
end
```

---

## 3. Initialization Flow

### 3.1 Plot Construction from Input Data

The initialization process follows this sequence:

```
1. Load Well Data
   ├── read_las_file(path) or load_well_data(dataframe)
   ├── Parse LAS/DataFrame into Well structure
   ├── Extract depth array
   └── Extract curves into dictionary
   
2. Create Tracks
   ├── define_track(id, curves, scale, width)
   ├── Assign curves to tracks
   └── Set track depth ranges
   
3. Build Well Object
   ├── Well(name, data, tracks, metadata)
   └── Validate depth consistency
   
4. Create Layout (if multi-well/multi-track)
   ├── WellLogLayout(wells, tracks_per_well, depth_sync, ...)
   └── synchronize_depth(layout)  # If enabled
   
5. Render with Plots.jl
   ├── plot(well) or plot(layout)
   ├── Plots.jl calls recipe
   ├── Recipe generates series data
   └── Backend renders (GR, PlotlyJS, etc.)
```

### 3.2 Data Loading

```julia
function load_well(name::String, data::DataFrame)
    # Extract depth column (assumed to be first column or named "DEPTH")
    depth_col = hasproperty(data, :DEPTH) ? :DEPTH : names(data)[1]
    depth = data[!, depth_col]
    
    # Extract curve data
    curves = Dict{String, Vector{Float64}}()
    for col in names(data)
        if col != depth_col
            curves[string(col)] = data[!, col]
        end
    end
    
    # Create well structure
    return Well(name, data, depth, curves, Track[], Dict{String, Any}())
end
```

### 3.3 Track Creation

```julia
function add_track(well::Well, track_id::String, curve_mnemonics::Vector{String};
                  scale::ScaleType=LinearScale(),
                  width::Float64=100.0)
    # Get curves for this track
    track_curves = Curve[]
    for mnemonic in curve_mnemonics
        if haskey(well.curves, mnemonic)
            push!(track_curves, Curve(
                mnemonic,
                well.curves[mnemonic],
                well.depth,  # Shared depth reference
                get_curve_metadata(well, mnemonic)...
            ))
        end
    end
    
    # Create track
    track = Track(
        track_id,
        track_curves,
        scale,
        width,
        length(well.tracks) + 1,  # Position
        (minimum(well.depth), maximum(well.depth))
    )
    
    push!(well.tracks, track)
    return track
end
```

### 3.4 Plot Recipe Execution

```julia
# Plots.jl recipe system
@recipe function f(well::Well)
    # Recipe attributes
    legend --> :outertopright
    grid --> true
    yflip --> true  # Depth increases downward
    
    # Depth axis configuration
    ylabel --> "Depth (m)"
    ylims --> (maximum(well.depth), minimum(well.depth))
    
    # Generate series for each track
    for track in well.tracks
        # Track background/grouping (if supported by backend)
        for curve in track.curves
            @series begin
                seriestype --> :line
                label --> curve.mnemonic
                linecolor --> curve.color
                linestyle --> curve.linestyle
                linewidth --> 1.5
                # Data: (x = values, y = depth)
                curve.data, well.depth
            end
        end
    end
end
```

---

## 4. Runtime Updates: Adding/Removing Tracks and Curves

### 4.1 Adding Tracks

```julia
function add_track!(well::Well, track_config::TrackConfig)
    # Create track from configuration
    track = create_track(well, track_config)
    
    # Add to well
    push!(well.tracks, track)
    
    # Update layout if well is part of one
    if has_layout(well)
        update_layout(well.layout)
    end
    
    # Trigger re-render (if plot is active)
    if has_active_plot(well)
        refresh_plot(well)
    end
end

function create_track(well::Well, config::TrackConfig)
    curves = Curve[]
    for curve_name in config.curve_names
        if haskey(well.curves, curve_name)
            push!(curves, get_curve(well, curve_name))
        end
    end
    
    return Track(
        config.id,
        curves,
        config.scale,
        config.width,
        length(well.tracks) + 1,
        config.depth_range
    )
end
```

### 4.2 Removing Tracks

```julia
function remove_track!(well::Well, track_id::String)
    # Find track index
    track_idx = findfirst(t -> t.id == track_id, well.tracks)
    
    if track_idx !== nothing
        # Remove track
        deleteat!(well.tracks, track_idx)
        
        # Re-position remaining tracks
        for (idx, track) in enumerate(well.tracks)
            track.position = idx
        end
        
        # Update layout
        if has_layout(well)
            update_layout(well.layout)
        end
        
        # Re-render
        if has_active_plot(well)
            refresh_plot(well)
        end
    end
end
```

### 4.3 Adding Curves

```julia
function add_curve!(well::Well, track_id::String, curve_data::Dict)
    # Find track
    track = get_track(well, track_id)
    
    if track !== nothing
        # Create curve
        curve = Curve(
            curve_data[:mnemonic],
            curve_data[:data],
            well.depth,  # Shared depth
            curve_data[:unit],
            curve_data[:color],
            curve_data[:linestyle],
            track_id
        )
        
        # Add to track
        push!(track.curves, curve)
        
        # Also add to well's curve dictionary
        well.curves[curve.mnemonic] = curve.data
        
        # Re-render
        if has_active_plot(well)
            refresh_plot(well)
        end
    end
end
```

### 4.4 Removing Curves

```julia
function remove_curve!(well::Well, track_id::String, curve_mnemonic::String)
    track = get_track(well, track_id)
    
    if track !== nothing
        # Remove from track
        filter!(c -> c.mnemonic != curve_mnemonic, track.curves)
        
        # Optionally remove from well's curve dictionary
        # (may be kept if used by other tracks)
        
        # Re-render
        if has_active_plot(well)
            refresh_plot(well)
        end
    end
end
```

### 4.5 Update Functions Summary

| Function | Purpose | Key Operations |
|----------|---------|----------------|
| `add_track!()` | Add new track | Create track, add to well, update layout, refresh plot |
| `remove_track!()` | Remove track | Delete track, re-position others, update layout |
| `add_curve!()` | Add curve to track | Create curve, add to track and well, refresh plot |
| `remove_curve!()` | Remove curve | Filter from track, refresh plot |
| `update_curve_data!()` | Update curve values | Modify data array, refresh plot |
| `update_depth_range!()` | Change depth domain | Update depth limits, rescale, refresh plot |

---

## 5. Plots.jl Backend Usage

### 5.1 Recipe System

WellLogs.jl uses Plots.jl's recipe system to define custom plot types:

```julia
# Main recipe for Well type
@recipe function f(well::Well)
    # Recipe attributes (can be overridden by user)
    yflip --> true
    legend --> :outertopright
    grid --> true
    
    # Axis labels
    xlabel --> "Value"
    ylabel --> "Depth (m)"
    
    # Depth limits
    ylims --> (maximum(well.depth), minimum(well.depth))
    
    # Generate series for each curve
    for track in well.tracks
        for curve in track.curves
            @series begin
                seriestype --> :line
                label --> curve.mnemonic
                linecolor --> curve.color
                linestyle --> curve.linestyle
                # Data: (x = values, y = depth)
                curve.data, well.depth
            end
        end
    end
end
```

### 5.2 Layout Recipe

For multi-track layouts:

```julia
@recipe function f(layout::WellLogLayout)
    # Create subplot layout
    n_wells = length(layout.wells)
    n_tracks_per_well = maximum(length(w.tracks) for w in layout.wells)
    
    layout --> (n_wells, n_tracks_per_well)
    
    # Synchronize depth if enabled
    if layout.depth_sync
        synchronize_depth(layout)
    end
    
    # Plot each well's tracks
    for (well_idx, well) in enumerate(layout.wells)
        for (track_idx, track) in enumerate(well.tracks)
            @series begin
                subplot --> (well_idx, track_idx)
                well, track  # Delegate to track recipe
            end
        end
    end
end
```

### 5.3 Backend Compatibility

WellLogs.jl works with any Plots.jl backend:

- **GR**: Fast, static plots
- **PlotlyJS**: Interactive plots with zoom/pan
- **PyPlot**: Matplotlib backend
- **Plotly**: Web-based interactive plots

The recipe system abstracts backend differences, so the same code works across backends.

### 5.4 Subplot Composition

For multi-track layouts, Plots.jl's subplot system is used:

```julia
function create_multi_track_plot(well::Well)
    n_tracks = length(well.tracks)
    
    # Create subplot layout
    plot(
        layout = (1, n_tracks),  # 1 row, n_tracks columns
        size = (n_tracks * 150, 800)
    )
    
    # Plot each track as subplot
    for (idx, track) in enumerate(well.tracks)
        plot!(
            subplot = idx,
            track.curves[1].data, well.depth,  # Example: first curve
            yflip = true,
            xlabel = track.id
        )
    end
end
```

---

## 6. Multi-Well and Multi-Track Layout Composition

### 6.1 Multi-Track Layout (Single Well)

```julia
function create_multi_track_layout(well::Well)
    # Well already has multiple tracks
    # Each track is rendered as a subplot column
    
    plot(
        well,
        layout = (1, length(well.tracks)),
        size = (sum(t.width for t in well.tracks), 800),
        link = :y  # Link Y-axes (depth synchronization)
    )
end
```

### 6.2 Multi-Well Layout

```julia
function create_multi_well_layout(wells::Vector{Well}; 
                                  depth_sync::Bool=true,
                                  tracks_per_well::Vector{Int}=Int[])
    # Create layout structure
    layout = WellLogLayout(
        wells,
        tracks_per_well,
        depth_sync,
        Dict{String, Float64}(),
        10.0  # spacing
    )
    
    # Synchronize depth if requested
    if depth_sync
        synchronize_depth(layout)
    end
    
    # Render
    plot(layout)
end
```

### 6.3 Layout Composition Pattern

```julia
# Composition function
function compose_layout(wells::Vector{Well}, 
                       track_configs::Vector{Vector{String}})
    # Ensure each well has the specified tracks
    for (well, track_config) in zip(wells, track_configs)
        for (track_id, curve_mnemonics) in enumerate(track_config)
            if !has_track(well, "track_$track_id")
                add_track!(well, "track_$track_id", curve_mnemonics)
            end
        end
    end
    
    # Create synchronized layout
    layout = WellLogLayout(
        wells,
        [length(w.tracks) for w in wells],
        true,  # depth_sync
        Dict{String, Float64}(),
        10.0
    )
    
    return layout
end
```

### 6.4 Depth Synchronization in Layouts

```julia
function synchronize_depth(layout::WellLogLayout)
    if layout.depth_sync
        # Collect all depth ranges
        depth_ranges = [
            (minimum(w.depth), maximum(w.depth)) 
            for w in layout.wells
        ]
        
        # Calculate global range
        global_min = minimum(first.(depth_ranges))
        global_max = maximum(last.(depth_ranges))
        
        # Apply to all wells
        for well in layout.wells
            well.depth_domain = (global_min, global_max)
        end
        
        # When plotting, use linked axes
        # Plots.jl: link = :y ensures Y-axes share same limits
    end
end
```

---

## 7. Key Source Files and Function Call Flow

### 7.1 Project Structure

```
WellLogs.jl/
├── src/
│   ├── WellLogs.jl           # Main module, exports
│   ├── types.jl              # Type definitions (Well, Track, Curve, Layout)
│   ├── data_loading.jl       # LAS file reading, DataFrame loading
│   ├── tracks.jl             # Track creation and management
│   ├── curves.jl             # Curve creation and management
│   ├── layouts.jl            # Layout composition and synchronization
│   ├── recipes.jl            # Plots.jl recipe definitions
│   ├── updates.jl            # Runtime update functions
│   └── utils.jl              # Utility functions
├── test/
│   └── ...
└── README.md
```

### 7.2 Key Source Files

**`src/WellLogs.jl` (Main Module):**
- Module definition and exports
- Public API functions
- Package initialization

**`src/types.jl`:**
- Type definitions: `Well`, `Track`, `Curve`, `WellLogLayout`
- Abstract types and type hierarchy
- Type constructors

**`src/data_loading.jl`:**
- `load_well()`: Load from LAS file or DataFrame
- `read_las_file()`: Parse LAS format
- `parse_well_data()`: Convert DataFrame to Well structure

**`src/tracks.jl`:**
- `add_track!()`: Add track to well
- `remove_track!()`: Remove track
- `get_track()`: Retrieve track by ID
- `update_track!()`: Modify track properties

**`src/curves.jl`:**
- `add_curve!()`: Add curve to track
- `remove_curve!()`: Remove curve
- `get_curve()`: Retrieve curve by mnemonic
- `update_curve_data!()`: Update curve values

**`src/layouts.jl`:**
- `WellLogLayout()`: Layout constructor
- `compose_layout()`: Create layout from wells
- `synchronize_depth()`: Depth synchronization logic
- `update_layout!()`: Update layout after changes

**`src/recipes.jl`:**
- `@recipe function f(well::Well)`: Main well plotting recipe
- `@recipe function f(layout::WellLogLayout)`: Layout plotting recipe
- Recipe attributes and series generation

**`src/updates.jl`:**
- Runtime update functions
- Plot refresh logic
- State management

### 7.3 Function Call Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. DATA LOADING & WELL CREATION                              │
└─────────────────────────────────────────────────────────────┘
│
├─> load_well(name, data_source)
│   │
│   ├─> read_las_file(path)  [if LAS file]
│   │   └─> parse_las_format()
│   │       ├─> extract_depth()
│   │       └─> extract_curves()
│   │
│   └─> load_from_dataframe(df)  [if DataFrame]
│       ├─> extract_depth_column()
│       └─> extract_curve_columns()
│
└─> Well(name, data, depth, curves, Track[], metadata)
    └─> validate_well_data()

┌─────────────────────────────────────────────────────────────┐
│ 2. TRACK CREATION                                            │
└─────────────────────────────────────────────────────────────┘
│
├─> add_track!(well, track_id, curve_mnemonics, config)
│   │
│   ├─> get_curves(well, curve_mnemonics)
│   │   └─> Curve(mnemonic, data, well.depth, ...)  [for each]
│   │
│   ├─> Track(track_id, curves, scale, width, position, depth_range)
│   │
│   └─> push!(well.tracks, track)

┌─────────────────────────────────────────────────────────────┐
│ 3. LAYOUT COMPOSITION                                        │
└─────────────────────────────────────────────────────────────┘
│
├─> compose_layout(wells, track_configs)
│   │
│   ├─> for each well: ensure_tracks(well, track_config)
│   │
│   ├─> WellLogLayout(wells, tracks_per_well, depth_sync, ...)
│   │
│   └─> synchronize_depth(layout)  [if depth_sync = true]
│       ├─> calculate_global_depth_range()
│       └─> apply_to_all_wells()

┌─────────────────────────────────────────────────────────────┐
│ 4. PLOTTING (Plots.jl Recipe System)                        │
└─────────────────────────────────────────────────────────────┘
│
├─> plot(well) or plot(layout)
│   │
│   ├─> Plots.jl calls recipe: @recipe function f(well::Well)
│   │   │
│   │   ├─> set_recipe_attributes(yflip, legend, grid, ...)
│   │   │
│   │   ├─> configure_axes(ylabel, ylims, ...)
│   │   │
│   │   └─> for each track in well.tracks:
│   │       └─> for each curve in track.curves:
│   │           └─> @series begin
│   │               ├─> seriestype --> :line
│   │               ├─> label, color, linestyle
│   │               └─> curve.data, well.depth  [data]
│   │
│   └─> Backend renders (GR, PlotlyJS, etc.)

┌─────────────────────────────────────────────────────────────┐
│ 5. RUNTIME UPDATES                                           │
└─────────────────────────────────────────────────────────────┘
│
├─> add_track!(well, track_config)
│   ├─> create_track(well, track_config)
│   ├─> push!(well.tracks, track)
│   ├─> update_layout(well.layout)  [if exists]
│   └─> refresh_plot(well)  [if plot active]
│       └─> plot(well)  [re-render]
│
├─> remove_track!(well, track_id)
│   ├─> deleteat!(well.tracks, track_idx)
│   ├─> reposition_tracks(well)
│   ├─> update_layout(well.layout)
│   └─> refresh_plot(well)
│
├─> add_curve!(well, track_id, curve_data)
│   ├─> Curve(...)
│   ├─> push!(track.curves, curve)
│   ├─> well.curves[curve.mnemonic] = curve.data
│   └─> refresh_plot(well)
│
└─> remove_curve!(well, track_id, mnemonic)
    ├─> filter!(track.curves, ...)
    └─> refresh_plot(well)
```

---

## 8. Architecture Summary

### 8.1 Design Principles

1. **Type-Based Abstractions**: Julia's type system provides clear separation between Wells, Tracks, Curves, and Layouts
2. **Shared Depth References**: Depth arrays are shared (not copied) to ensure consistency and memory efficiency
3. **Recipe-Based Plotting**: Integration with Plots.jl through recipes allows backend-agnostic visualization
4. **Composable Layouts**: Multi-well and multi-track layouts are composed through layout structures
5. **Reactive Updates**: Runtime changes trigger plot refreshes automatically

### 8.2 Data Flow

```
Input Data (LAS/DataFrame)
    ↓
Well Structure (depth + curves dictionary)
    ↓
Track Creation (curves grouped into tracks)
    ↓
Layout Composition (multi-track/multi-well arrangement)
    ↓
Plots.jl Recipe (generates series data)
    ↓
Backend Rendering (GR, PlotlyJS, etc.)
```

### 8.3 Memory Model

- **Depth Arrays**: Single shared vector per well (all curves reference it)
- **Curve Data**: Stored in well's curves dictionary and track's curve list
- **Tracks**: Contain references to curves (not copies)
- **Layouts**: Contain references to wells (not copies)

This design minimizes memory usage while maintaining data consistency.

### 8.4 Synchronization Model

**Depth Synchronization:**
- Within well: All tracks share the same depth vector (reference)
- Across wells: When `depth_sync = true`, all wells use the same depth domain
- Implementation: Plots.jl's `link = :y` option links Y-axes in subplots

**Update Propagation:**
- Track/curve changes → Well structure update → Layout update → Plot refresh
- Depth range changes → All tracks updated → Layout synchronized → Plot refresh

---

## 9. Example Usage Patterns

### 9.1 Basic Single-Track Plot

```julia
using WellLogs, Plots

# Load well data
well = load_well("Well-1", "path/to/data.las")

# Add a track with gamma ray curve
add_track!(well, "GR", ["GR"])

# Plot
plot(well)
```

### 9.2 Multi-Track Plot

```julia
# Add multiple tracks
add_track!(well, "GR", ["GR"])
add_track!(well, "Resistivity", ["RT", "RXO"])
add_track!(well, "Porosity", ["PHI", "PHID"])

# Plot with subplot layout
plot(well, layout = (1, 3), link = :y)
```

### 9.3 Multi-Well Comparison

```julia
# Load multiple wells
wells = [load_well("Well-$i", "path/to/well$i.las") for i in 1:3]

# Add same tracks to each well
for well in wells
    add_track!(well, "GR", ["GR"])
    add_track!(well, "Resistivity", ["RT"])
end

# Create synchronized layout
layout = compose_layout(wells, [["GR"], ["RT"]], depth_sync=true)

# Plot
plot(layout)
```

### 9.4 Runtime Updates

```julia
# Create initial plot
well = load_well("Well-1", "data.las")
add_track!(well, "GR", ["GR"])
p = plot(well)

# Add track dynamically
add_track!(well, "Resistivity", ["RT"])
# Plot automatically updates (if using interactive backend)

# Remove curve
remove_curve!(well, "GR", "GR")
# Plot refreshes
```

---

## Conclusion

WellLogs.jl provides a clean, type-based architecture for well log visualization in Julia:

- **Core Abstractions**: Well, Track, Curve, and Layout types provide clear separation of concerns
- **Depth Synchronization**: Shared depth references ensure consistency within and across wells
- **Plots.jl Integration**: Recipe system enables backend-agnostic plotting
- **Composable Layouts**: Flexible composition of multi-track and multi-well visualizations
- **Runtime Updates**: Dynamic addition/removal of tracks and curves with automatic plot refresh

The architecture leverages Julia's strengths (type system, multiple dispatch) and integrates seamlessly with the Plots.jl ecosystem, providing a powerful and flexible tool for well log visualization.

