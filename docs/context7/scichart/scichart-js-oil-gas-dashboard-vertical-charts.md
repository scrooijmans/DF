# SciChart.js Oil and Gas Dashboard: Vertical Charts and Positioning

This document explains how to create an Oil and Gas Dashboard using SciChart.js, focusing on vertical charts and how multiple charts are positioned and synchronized.

## Overview

The Oil and Gas Dashboard consists of multiple individual 2D vertical charts arranged horizontally. Each chart represents different well log measurements (Density, Resistivity, Pore Space, Sonic, Texture, Shale) displayed vertically to represent depth. The charts are synchronized so that zooming, panning, or hovering on one chart affects all charts simultaneously.

## Creating Vertical Charts

### What Makes a Chart Vertical?

A chart becomes vertical when you transpose the axis alignment:
- **X-axis** is positioned on the **Left** or **Right** (instead of Bottom/Top)
- **Y-axis** is positioned on the **Top** or **Bottom** (instead of Left/Right)

This swaps the orientation so data flows top-to-bottom instead of left-to-right, which is ideal for well log data where depth is the primary dimension.

### Configuring Vertical Charts

#### Using Standard API

```typescript
import {
    SciChartSurface,
    NumericAxis,
    EAxisAlignment,
    FastLineRenderableSeries,
    XyDataSeries
} from "scichart";

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// X-axis on the left (vertical orientation)
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "X Axis",
        axisAlignment: EAxisAlignment.Left  // Key: Left alignment makes it vertical
    })
);

// Y-axis on the top (vertical orientation)
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: "Y Axis (Depth)",
        axisAlignment: EAxisAlignment.Top  // Key: Top alignment completes vertical orientation
    })
);

// Add series - it will render vertically
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [0, 1, 2, 3, 4, 5],
            yValues: [100, 200, 300, 400, 500, 600]  // Depth values
        }),
        stroke: "#FF6600",
        strokeThickness: 2
    })
);
```

#### Using Builder API

```typescript
import { chartBuilder, EAxisType, EAxisAlignment } from "scichart";

const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    surface: {
        theme: { type: EThemeProviderType.Dark }
    },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis",
            axisAlignment: EAxisAlignment.Left  // Vertical orientation
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Y Axis (Depth)",
            axisAlignment: EAxisAlignment.Top  // Vertical orientation
        }
    },
    series: [{
        type: ESeriesType.LineSeries,
        xyData: {
            xValues: [0, 1, 2, 3, 4, 5],
            yValues: [100, 200, 300, 400, 500, 600]
        },
        options: {
            stroke: "#FF6600",
            strokeThickness: 2
        }
    }]
});
```

### Example: Density Chart (Vertical)

```typescript
import { chartBuilder, EAxisType, EAxisAlignment, ESeriesType, XyyDataSeries } from "scichart";

export const drawDensityChart = async (rootElement: string | HTMLDivElement) => {
    const yAxisId = "y-axis-id";
    
    const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(rootElement, {
        surface: {
            theme: appTheme.SciChartJsTheme,
            padding: Thickness.fromNumber(0),
            id: "densityChart",  // Unique ID for synchronization
        },
        yAxes: {
            type: EAxisType.NumericAxis,
            options: {
                id: yAxisId,
                axisAlignment: EAxisAlignment.Bottom,  // Y-axis at bottom for vertical chart
            },
        },
        modifiers: getCommonChartModifiersConfig(yAxisId),
    });

    // Set visible range for depth
    sciChartSurface.yAxes.get(0).visibleRange = new NumberRange(-0.2, 0.2);

    // Create band series data (upper and lower bounds)
    const dataSeries = new XyyDataSeries(wasmContext, { 
        dataIsSortedInX: true, 
        containsNaN: false 
    });

    // Load and append data
    const data = await getParsedData("Density.csv");
    data.forEach((dataRow) => {
        const x = dataRow[0];  // Depth
        dataSeries.append(x, dataRow[1], dataRow[2]);  // x, y1 (upper), y2 (lower)
    });

    // Create band series
    const renderableSeries = chartBuilder.buildSeries(wasmContext, {
        type: ESeriesType.BandSeries,
        options: {
            dataSeries,
            strokeThickness: 0,
            stroke: appTheme.DensityStrokeY,
            strokeY1: appTheme.DensityStrokeY1,
            fill: appTheme.DensityFillY,
            fillY1: appTheme.DensityFillY1,
        },
    });

    sciChartSurface.renderableSeries.add(...renderableSeries);

    return { sciChartSurface };
};
```

## Positioning Multiple Vertical Charts

### Layout Structure

The Oil and Gas Dashboard uses **CSS Flexbox** to position multiple vertical charts horizontally. Each chart is contained within a `.chart-container` div, and all containers are placed in a flex container.

#### HTML Structure

```html
<div id="main-charts" class="main-charts">
    <div class="chart-container">
        <div id="shale-legend" class="legend-root"></div>
        <div id="shale-chart" class="chart-root"></div>
    </div>
    <div class="chart-container">
        <div id="density-legend" class="legend-root"></div>
        <div id="density-chart" class="chart-root"></div>
    </div>
    <div class="chart-container">
        <div id="resistivity-legend" class="legend-root"></div>
        <div id="resistivity-chart" class="chart-root"></div>
    </div>
    <!-- More chart containers... -->
</div>
```

#### CSS Positioning

```css
/* Main container - horizontal flex layout */
#main-charts {
    display: flex;
    justify-content: space-around;  /* Distributes charts evenly */
    max-width: 100%;
    height: 100%;
    overflow-x: auto;  /* Horizontal scrolling if needed */
    overflow-y: hidden;
}

/* Individual chart container - vertical flex layout */
.chart-container {
    position: relative;
    display: flex;
    flex-direction: column;  /* Legend on top, chart below */
    flex: auto;              /* Equal width distribution */
    flex-basis: 16%;         /* Each chart takes ~16% width (6 charts) */
}

/* Legend area */
.legend-root {
    min-height: 115px;
    display: flex;
    align-items: flex-end;  /* Legend at bottom of legend area */
}

/* Chart area */
.chart-root {
    flex: auto;  /* Takes remaining vertical space */
}
```

### What Controls Chart Positioning?

1. **CSS Flexbox Properties**:
   - `display: flex` on the main container creates horizontal layout
   - `flex-direction: column` on chart containers creates vertical layout (legend + chart)
   - `flex: auto` distributes available space equally
   - `flex-basis: 16%` sets approximate width per chart (for 6 charts)

2. **Container Structure**:
   - Each chart is wrapped in a `.chart-container` div
   - The container includes both legend and chart areas
   - Containers are siblings in the flex layout

3. **Chart Dimensions**:
   - Charts automatically size to their container
   - Height is controlled by parent container height
   - Width is controlled by `flex-basis` or `flex: auto`

### React/TypeScript Implementation

```typescript
import { ChartGroupLoader, SciChartReact } from "scichart-react";

export default function OilAndGasDashboardShowcase() {
    return (
        <ChartGroupLoader 
            className={commonClasses.ChartWrapper} 
            style={{ display: "flex" }} 
            onInit={onInitAllCharts}
        >
            <div className="main-container">
                <div id="main-charts" style={{ 
                    background: appTheme.SidebarBackground,
                    color: appTheme.SidebarTextColor 
                }}>
                    {/* Shale Chart */}
                    <div className="chart-container">
                        <div id="shale-legend" className="legend-root"></div>
                        <SciChartReact 
                            className="chart-root" 
                            initChart={drawShaleChart}
                        />
                    </div>

                    {/* Density Chart */}
                    <div className="chart-container">
                        <div id="density-legend" className="legend-root"></div>
                        <SciChartReact 
                            className="chart-root" 
                            initChart={drawDensityChart}
                        />
                    </div>

                    {/* Resistivity Chart */}
                    <div className="chart-container">
                        <div id="resistivity-legend" className="legend-root"></div>
                        <SciChartReact 
                            className="chart-root" 
                            initChart={drawResistivityChart}
                        />
                    </div>

                    {/* More charts... */}
                </div>
            </div>
        </ChartGroupLoader>
    );
}
```

## Synchronizing Multiple Charts

### Why Synchronize?

In well log dashboards, all charts should:
- Show the same depth range (Y-axis)
- Zoom and pan together
- Show rollover tooltips at the same depth across all charts

### Using SciChartVerticalGroup

`SciChartVerticalGroup` synchronizes the **Y-axis sizes** across multiple charts, ensuring consistent visual alignment even when axes have different ranges or label sizes.

```typescript
import { SciChartVerticalGroup } from "scichart";

const onInitAllCharts = (initResults: IInitResult[]) => {
    // Identify vertical charts by their IDs
    const verticalChartIds = [
        "densityChart",
        "textureChart",
        "poreSpaceChart",
        "resistivityChart",
        "shaleChart",
        "sonicChart",
    ];

    // Filter to get only vertical chart surfaces
    const verticalCharts = initResults.filter(({ sciChartSurface }) =>
        verticalChartIds.includes(sciChartSurface.id)
    ) as IInitResult<SciChartSurface>[];

    // Create vertical group for axis size synchronization
    const surfaceGroup = new SciChartVerticalGroup();

    // Add each chart to the group
    verticalCharts.forEach(({ sciChartSurface }) => {
        surfaceGroup.addSurfaceToGroup(sciChartSurface);
    });
};
```

### Synchronizing Visible Ranges

To synchronize the visible Y-axis range (depth) across charts, subscribe to `visibleRangeChanged` events:

```typescript
// Synchronize Y-axis visible ranges
const masterYAxis = sciChartSurface0.yAxes.get(0);

verticalCharts.forEach(({ sciChartSurface }) => {
    const yAxis = sciChartSurface.yAxes.get(0);
    
    // When master axis changes, update all others
    masterYAxis.visibleRangeChanged.subscribe((data) => {
        yAxis.visibleRange = data.visibleRange;
    });
    
    // When any axis changes, update master and all others
    yAxis.visibleRangeChanged.subscribe((data) => {
        masterYAxis.visibleRange = data.visibleRange;
        verticalCharts.forEach(({ sciChartSurface: otherSurface }) => {
            if (otherSurface !== sciChartSurface) {
                otherSurface.yAxes.get(0).visibleRange = data.visibleRange;
            }
        });
    });
});
```

### Synchronizing Modifiers (Rollover, Zoom, Pan)

Use `modifierGroup` to link modifiers across charts:

```typescript
import { RolloverModifier, ZoomPanModifier, MouseWheelZoomModifier } from "scichart";

const modifierGroup = "VerticalChartsGroup";

verticalCharts.forEach(({ sciChartSurface }) => {
    // Add synchronized rollover modifier
    sciChartSurface.chartModifiers.add(
        new RolloverModifier({
            modifierGroup: modifierGroup,  // Same group = synchronized
            rolloverLineStroke: appTheme.RolloverLineColor,
        })
    );

    // Add synchronized zoom/pan modifiers
    sciChartSurface.chartModifiers.add(
        new ZoomPanModifier({ modifierGroup: modifierGroup }),
        new MouseWheelZoomModifier({ modifierGroup: modifierGroup })
    );
});
```

### Complete Synchronization Example

```typescript
const onInitAllCharts = (initResults: IInitResult[]) => {
    const verticalChartIds = [
        "densityChart",
        "textureChart",
        "poreSpaceChart",
        "resistivityChart",
        "shaleChart",
        "sonicChart",
    ];

    const verticalCharts = initResults.filter(({ sciChartSurface }) =>
        verticalChartIds.includes(sciChartSurface.id)
    ) as IInitResult<SciChartSurface>[];

    // 1. Synchronize Y-axis sizes
    const surfaceGroup = new SciChartVerticalGroup();
    verticalCharts.forEach(({ sciChartSurface }) => {
        surfaceGroup.addSurfaceToGroup(sciChartSurface);
    });

    // 2. Synchronize visible ranges
    if (verticalCharts.length > 0) {
        const masterYAxis = verticalCharts[0].sciChartSurface.yAxes.get(0);
        
        verticalCharts.forEach(({ sciChartSurface }) => {
            const yAxis = sciChartSurface.yAxes.get(0);
            
            // Sync to master
            masterYAxis.visibleRangeChanged.subscribe((data) => {
                if (yAxis !== masterYAxis) {
                    yAxis.visibleRange = data.visibleRange;
                }
            });
        });
    }

    // 3. Synchronize modifiers
    const modifierGroup = "VerticalChartsGroup";
    verticalCharts.forEach(({ sciChartSurface }) => {
        sciChartSurface.chartModifiers.add(
            new RolloverModifier({
                modifierGroup: modifierGroup,
                rolloverLineStroke: appTheme.RolloverLineColor,
            }),
            new ZoomPanModifier({ modifierGroup: modifierGroup }),
            new MouseWheelZoomModifier({ modifierGroup: modifierGroup })
        );
    });
};
```

## Custom Axis Synchronizer (Advanced)

For more control, create a custom `AxisSynchroniser` class:

```typescript
import { NumericAxis, NumberRange, EventHandler } from "scichart";

class AxisSynchroniser {
    visibleRange: NumberRange = new NumberRange(0, 10);
    axes: NumericAxis[] = [];
    visibleRangeChanged: EventHandler<any>;

    constructor(initialRange?: NumberRange, axes?: NumericAxis[]) {
        this.visibleRange = initialRange ?? this.visibleRange;
        this.visibleRangeChanged = new EventHandler();

        this.publishChange = this.publishChange.bind(this);
        if (axes) {
            axes.forEach(a => this.addAxis(a));
        }
    }

    publishChange(data) {
        this.visibleRange = data.visibleRange;
        // Update all axes to match
        this.axes.forEach(a => {
            a.visibleRange = this.visibleRange;
        });
        this.visibleRangeChanged.raiseEvent(data);
    }

    addAxis(axis: NumericAxis) {
        if (!this.axes.includes(axis)) {
            this.axes.push(axis);
            axis.visibleRange = this.visibleRange;
            // Subscribe to changes
            axis.visibleRangeChanged.subscribe(this.publishChange);
        }
    }

    removeAxis(axis: NumericAxis) {
        const index = this.axes.findIndex(a => a === axis);
        if (index >= 0) {
            this.axes.splice(index, 1);
            axis.visibleRangeChanged.unsubscribe(this.publishChange);
        }
    }
}

// Usage
const axisSynchronizer = new AxisSynchroniser();
verticalCharts.forEach(({ sciChartSurface }) => {
    axisSynchronizer.addAxis(sciChartSurface.yAxes.get(0));
});
```

## Key Points Summary

### Vertical Chart Configuration
- Set X-axis `axisAlignment` to `Left` or `Right`
- Set Y-axis `axisAlignment` to `Top` or `Bottom`
- Data flows top-to-bottom (ideal for depth-based data)

### Chart Positioning
- **CSS Flexbox** controls horizontal layout
- Each chart in a `.chart-container` with `flex: auto` or `flex-basis: 16%`
- Main container uses `display: flex` with `justify-content: space-around`
- Charts automatically size to container dimensions

### Synchronization
- **SciChartVerticalGroup**: Synchronizes Y-axis sizes
- **visibleRangeChanged events**: Synchronizes visible depth ranges
- **modifierGroup**: Synchronizes rollover, zoom, pan across charts
- All charts share the same depth range and interaction behavior

### Best Practices
1. Give each chart a unique `id` for identification
2. Use consistent axis configurations across vertical charts
3. Group charts using `ChartGroupLoader` for coordinated initialization
4. Use `modifierGroup` for unified user interaction
5. Ensure container dimensions are explicit (use CSS or inline styles)

## Complete Example Structure

```typescript
// 1. Define chart initialization functions
export const drawDensityChart = async (rootElement: string | HTMLDivElement) => {
    const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(rootElement, {
        surface: {
            id: "densityChart",  // Unique ID
            theme: appTheme.SciChartJsTheme,
        },
        yAxes: {
            type: EAxisType.NumericAxis,
            options: {
                axisAlignment: EAxisAlignment.Bottom,  // Vertical
            },
        },
        // ... series configuration
    });
    return { sciChartSurface };
};

// 2. Create React component with chart containers
export default function Dashboard() {
    return (
        <ChartGroupLoader onInit={onInitAllCharts}>
            <div id="main-charts">
                <div className="chart-container">
                    <div id="density-legend" className="legend-root"></div>
                    <SciChartReact 
                        className="chart-root" 
                        initChart={drawDensityChart}
                    />
                </div>
                {/* More charts... */}
            </div>
        </ChartGroupLoader>
    );
}

// 3. Synchronize charts on initialization
const onInitAllCharts = (initResults: IInitResult[]) => {
    const verticalGroup = new SciChartVerticalGroup();
    const modifierGroup = "VerticalChartsGroup";
    
    initResults.forEach(({ sciChartSurface }) => {
        verticalGroup.addSurfaceToGroup(sciChartSurface);
        sciChartSurface.chartModifiers.add(
            new RolloverModifier({ modifierGroup })
        );
    });
};
```

This structure creates a fully synchronized, vertically-oriented dashboard perfect for oil and gas well log visualization.

