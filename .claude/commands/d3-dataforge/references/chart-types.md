# Existing Chart Types in DataForge-Compute

## Chart Type Registry

| Type | PaneType | Config Interface | Renderer |
|------|----------|------------------|----------|
| Line | `line` | `LineChartConfig` | ECharts |
| Scatter | `scatter` | `ScatterChartConfig` | ECharts |
| Histogram | `histogram` | `HistogramConfig` | ECharts |
| CrossPlot | `crossplot` | `CrossPlotConfig` | ECharts |
| Well Log | `welllog` | `WellLogConfig` | ECharts |
| D3 Well Log | `d3-welllog` | `D3WellLogConfig` | D3.js |
| Correlation | `correlation` | `CorrelationConfig` | Custom |

## D3 Charts (Current)

### D3WellLogTrack

**Purpose:** Single-track well log visualization with lithology interpretation

**Features:**
- Header with title, unit, scale range
- Vertical/horizontal grid lines
- Curve line with configurable color
- Area fill (left/right) for sand indication
- Lithology zone labels (Sand/Shale)
- GR cutoff-based classification

**Config Interface:**
```typescript
interface D3WellLogConfig extends BaseChartConfig {
  type: 'd3-welllog';
  curve?: AxisBinding;
  xMin?: number;
  xMax?: number;
  curveColor?: string;
  fillColor?: string;
  fillDirection?: 'left' | 'right' | 'none';
  lineWidth?: number;
  showLithologyLabels?: boolean;
  grCutoff?: number;
}
```

**Component Location:** `src/lib/components/charts/D3WellLogTrack.svelte`

**Utility Functions:** `src/lib/charts/d3-utils.ts`

## Adding a New D3 Chart Type

1. **Identify the visualization need**
2. **Design the config interface** - extend `BaseChartConfig`
3. **Implement the D3 component** - use declarative SVG pattern
4. **Integrate with ChartSettingsDialog**
5. **Add to PaneContainer**

See `SKILL.md` for detailed workflow.
