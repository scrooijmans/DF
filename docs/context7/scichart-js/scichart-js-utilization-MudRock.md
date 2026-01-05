SciChart Usage in MudRock Project
Here's a comprehensive breakdown of how SciChart is used throughout the project:

1. Initialization (scichart-init.ts)
   SciChart is initialized once globally in the root layout:

// Root layout triggers WASM initialization
const { SciChartSurface } = await import("scichart");
SciChartSurface.useWasmFromCDN();
The +layout.svelte calls this initialization before any charts render. 2. State Architecture
Charts use a class-based state hierarchy:
Base Class Purpose Location
Chart2DBase Abstract base for all 2D charts chart-2D-base.svelte.ts
XYChartState Line/XY charts xy-chart-state.svelte.ts
ScatterChartState Scatter plots scatter-chart-state.svelte.ts
MapChartState Geographic charts map-chart-state.svelte.ts
WellCorrelationChartState Multi-well correlation well-correlation-chart-state.svelte.ts 3. Surface Creation Pattern

// In chart state class
const result = await SciChartSurface.create(container);
this.sciChartSurface = result.sciChartSurface;
this.wasmContext = result.wasmContext;

// Apply theme
const customTheme = createCustomTheme();
this.sciChartSurface.applyTheme(customTheme); 4. Theming (scichart-theme.ts)
Custom theme adapts to light/dark mode by reading CSS variables:
Axis labels, gridlines, tick colors
Series stroke colors
Background colors 5. Configuration Constants (scichart-constants.ts)
Centralized settings:
SCICHART_TYPOGRAPHY - fonts and sizes
SCICHART_AXIS - default ranges, padding (10% growBy)
SCICHART_MODIFIERS - zoom/pan settings
SCICHART_SERIES - stroke thickness, markers 6. Data Flow
UI selects curves → chart state extracts curve IDs
fetchAlignedCurveData() loads parquet data from Supabase
XyDataSeries.appendRange(xValues, yValues) populates data
FastLineRenderableSeries wraps and renders
Series tracked in renderableSeriesMap 7. Modifiers (Interactions) (chart-modifier-registry.ts)
Modifier Type Default
Pan ZoomPanModifier Disabled
Zoom RubberBandXyZoomModifier Enabled
Mouse Wheel MouseWheelZoomModifier Enabled
Zoom to Fit ZoomExtentsModifier Enabled
Crosshair CursorModifier Disabled 8. Svelte Components
Located in src/lib/components/pages/home/charts/chart-editor/sci-charts/:
sci-xy-chart.svelte - Main XY renderer
sci-xy-scatter-chart.svelte - Scatter variant
sci-map-multi-layer.svelte - Geographic charts
sci-well-correlation-chart.svelte - Multi-well panels 9. Database Sync (chart-state-sync-service.ts)
Visible ranges sync to Supabase charts.chart_config JSONB
Uses debouncing (1000ms) to avoid excessive writes
Subscribes to xAxis.visibleRangeChanged events 10. Memory Management

// Cleanup pattern
this.sciChartSurface.delete(); // Releases WASM resources
series.dataSeries.delete(); // Releases data series memory
