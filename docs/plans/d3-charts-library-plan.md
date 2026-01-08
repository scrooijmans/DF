## Implementation Plan: D3.js Charts Library for DataForge

### Research Summary
- **Libraries researched**: D3.js v7, Wellioviz, ECharts (existing)
- **Projects analyzed**:
  - Wellioviz - JavaScript well log visualization using D3.js
  - Existing EChartsChart.svelte in DataForge-Compute
  - D3 Graph Gallery patterns
- **Key patterns identified**:
  - Track-based architecture for well logs
  - Segment-based data (already in place via `SegmentedCurveData`)
  - Area fills with gradient for lithology indication
  - SVG rendering for precise control and styling
  - Scale/axis patterns for depth-inverted display

### Recommended Approach

Create a dedicated D3 charts library at a shared location (`libs/charts-d3/`) that provides:

1. **D3WellLogTrack** - A Svelte 5 component for well log display using D3.js
2. **Shared D3 utilities** - Scales, axes, area generators reusable across chart types
3. **Integration with existing data structures** - Use `SegmentedCurveData` and `CorrelationCurveData`

The D3 approach offers advantages over ECharts for well logs:
- **SVG-based** - Better for lithology patterns, fills, and annotations
- **Fine-grained control** - Exact positioning for geologic markers
- **Smaller bundle** - Only import needed D3 modules
- **Publication quality** - Vector export for reports

### Implementation Steps

#### Step 1: Create D3 Charts Library Structure

- Files to create:
  - `libs/charts-d3/src/index.ts` - Main exports
  - `libs/charts-d3/src/components/D3WellLogTrack.svelte` - Well log component
  - `libs/charts-d3/src/utils/scales.ts` - D3 scale helpers
  - `libs/charts-d3/src/utils/area-generators.ts` - Area fill generators
  - `libs/charts-d3/src/types.ts` - D3 chart-specific types
- Based on: D3.js patterns and existing `correlation-types.ts`

#### Step 2: Create D3WellLogTrack Component

Key features matching the screenshot:
- **Header**: Track title (e.g., "Gamma Ray"), unit display (gAPI), scale range (0-150)
- **Grid**: Vertical grid lines at regular intervals
- **Curve**: Green line rendering with `d3.line()`
- **Fill**: Yellow area fill from curve to left edge for "Sand" zones
- **Lithology labels**: Text labels ("Shale", "Sand") positioned at correct depths

Implementation approach:
```typescript
// D3 area generator for well log fill
const area = d3.area<DataPoint>()
  .x0(0)  // Left edge baseline
  .x1(d => xScale(d.value))  // Curve value
  .y(d => yScale(d.depth))   // Depth position
  .defined(d => !isNaN(d.value))  // Handle gaps
  .curve(d3.curveLinear);

// Inverted Y scale for depth (increases downward)
const yScale = d3.scaleLinear()
  .domain([depthMin, depthMax])
  .range([0, height]);  // Top to bottom
```

#### Step 3: Integrate with Existing DataForge Architecture

- Use `SegmentedCurveData` format (already available from Rust backend)
- Connect to `chartManager` for cursor/viewport sync
- Support same props interface as `EChartsChart.svelte` where applicable

#### Step 4: Add Lithology Zone Support

For the Sand/Shale labels in the screenshot:
- Add `LithologyZone` type to define depth intervals with labels
- Render zones as `<rect>` backgrounds with text labels
- Support configurable colors per lithology type

#### Step 5: Create D3.js Slash Command

Move and configure the D3.js skill:
- Location: `.claude/commands/d3-charts.md`
- Reference the skill from `claude-d3js-skill-main/`

### File Changes Summary

| Action | File Path | Purpose |
|--------|-----------|---------|
| CREATE | `libs/charts-d3/package.json` | Library package config |
| CREATE | `libs/charts-d3/src/index.ts` | Main exports |
| CREATE | `libs/charts-d3/src/components/D3WellLogTrack.svelte` | Well log track component |
| CREATE | `libs/charts-d3/src/utils/scales.ts` | D3 scale utilities |
| CREATE | `libs/charts-d3/src/utils/area-generators.ts` | Area fill generators |
| CREATE | `libs/charts-d3/src/types.ts` | Type definitions |
| CREATE | `.claude/commands/d3-charts.md` | D3 charts slash command |
| MODIFY | `DataForge-Compute/package.json` | Add D3 dependency |

### Alternative: Inline in DataForge-Compute

For faster iteration, create the D3 component directly in DataForge-Compute first:

| Action | File Path | Purpose |
|--------|-----------|---------|
| CREATE | `DataForge-Compute/src/lib/components/charts/D3WellLogTrack.svelte` | D3 well log component |
| CREATE | `DataForge-Compute/src/lib/charts/d3-utils.ts` | D3 utility functions |
| MODIFY | `DataForge-Compute/package.json` | Add D3.js dependency |

### Dependencies
- New packages needed: `d3` (v7.x)
- Breaking changes: None (additive feature)

### Testing Strategy
- Unit tests: D3 scale/axis generation functions
- Integration tests: Component renders with mock data
- Manual verification:
  - Visual comparison with screenshot
  - Verify fill direction matches GR curve shape
  - Check depth scale accuracy
  - Confirm lithology labels position correctly

### Alternatives Considered

1. **Extend ECharts**: Already using ECharts but SVG customization limited for lithology patterns
2. **Plotly.js**: Heavier, less control over SVG structure
3. **Canvas-based**: Would lose vector export capability

### References
- [D3.js Documentation](https://d3js.org/)
- [Wellioviz](https://github.com/JustinGOSSES/wellioviz) - D3 well log library
- Existing `EChartsChart.svelte` for integration patterns
- `correlation-types.ts` for data structures
