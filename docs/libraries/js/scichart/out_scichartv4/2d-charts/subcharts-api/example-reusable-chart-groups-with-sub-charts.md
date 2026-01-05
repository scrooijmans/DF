On this page

# Worked Example: Re-usable Chart Groups with SubCharts

SubCharts give a way to create re-usable multi-chart components that are managed by a singleÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> instance.

For example, if in your application you are repeatedly creating groups of charts that share a single common X-Axis, and must zoom and pan together, one way you could do this is by creating three separateÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> instances (see the tutorial onÂ [Linking Multiple Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts).

Alternatively, you could create your multi-chart control using SubCharts and manage that with a single SciChartSurface. This will provide cleaner, neater code, plus also give you a performance boost, as SubCharts are faster than standard charts in multi-chart scenarios.

## Creating Re-usable Chart Groups with SubCharts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#creating-re-usable-chart-groups-with-subcharts" class="hash-link" aria-label="Direct link to Creating Re-usable Chart Groups with SubCharts" translate="no" title="Direct link to Creating Re-usable Chart Groups with SubCharts">â€‹</a>

Take the example from theÂ <a href="https://www.scichart.com/demo/react/audio-analyzer-fft-example" rel="noopener noreferrer" target="_blank">SciChart Demo - Realtime Audio Analyzer</a>. This has three charts arranged in two rows, where the bottom row has two columns. The chart types are Line, Mountain and Heatmap.

<img src="out_scichartv4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/index_media/2e000f50a205b3cdc5028cc288c597c6ce35e5ab.png" class="img_ev3q" decoding="async" loading="lazy" width="982" height="623" />

If this was a control you needed to instantiate more than once in your application you might consider creating a re-usable Chart Group using SubCharts.

Let's begin.

### Creating the Layout<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#creating-the-layout" class="hash-link" aria-label="Direct link to Creating the Layout" translate="no" title="Direct link to Creating the Layout">â€‹</a>

The layout is pretty simple. We want the top chart to occupy 100% of the width of the chart panel, and the bottom charts to occupy 50% of the width each. The top and bottom charts should both by 50% of the height.

To do this we will use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsubsurface.html" rel="noopener noreferrer" target="_blank">SciChartSubSurface.createSubSurface()ðŸ“˜</a> and pass the position property as a rectangle with relative coordinates (see theÂ [SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview) andÂ [SubChart positioning](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-charts-positioning) pages for more details).

Here's the code to do this.

- TS
- Builder API (JSON Config)

``` prism-code
// demonstrates how to create a reusable 1x2 panel of charts using SubCharts API
async function createThreePanelChart(divElementId) {
    // Create a parent (regular) SciChartSurface which will contain the sub-chart
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });

    // Add a Sub-Charts to the main surface. This will display a rectangle showing the current zoomed in area on the parent chart

    // Top chart to occupy 100% of the width and 50% height
    const subChartTop = SciChartSubSurface.createSubSurface(sciChartSurface, {
        position: new Rect(0, 0, 1, 0.5),
        theme: new SciChartJsNavyTheme(),
        title: "Audio Chart",
        titleStyle: { fontSize: 14 }
    });

    // Bottom left chart to occupy 50% of the width and 50% height
    const subChartBottomLeft = SciChartSubSurface.createSubSurface(sciChartSurface, {
        position: new Rect(0, 0.5, 0.5, 0.5),
        theme: new SciChartJsNavyTheme(),
        title: "Frequency Chart",
        titleStyle: { fontSize: 14 }
    });

    const subChartBottomRight = SciChartSubSurface.createSubSurface(sciChartSurface, {
        position: new Rect(0.5, 0.5, 0.5, 0.5),
        theme: new SciChartJsNavyTheme(),
        title: "Spectrogram Chart",
        titleStyle: { fontSize: 14 }
    });

    // Add common axis, interactivity and some data to all charts. Customize this as needed
    [subChartTop, subChartBottomLeft, subChartBottomRight].forEach(scs => {
        scs.xAxes.add(
            new NumericAxis(wasmContext, {
                axisTitle: "XAxis",
                axisTitleStyle: { fontSize: 12 }
            })
        );
        scs.yAxes.add(
            new NumericAxis(wasmContext, {
                axisTitle: "YAxis",
                axisTitleStyle: { fontSize: 12 }
            })
        );
        scs.chartModifiers.add(new ZoomPanModifier());

        // Add random data series
        const dataSeries = new XyDataSeries(wasmContext);
        const randomData = generateRandomData();
        dataSeries.appendRange(
            randomData.map(pt => pt.x),
            randomData.map(pt => pt.y)
        );

        scs.renderableSeries.add(
            new FastLineRenderableSeries(wasmContext, {
                stroke: getRandomColor(),
                strokeThickness: 2,
                dataSeries
            })
        );
    });

    // If you return the charts to the caller, you can now configure series, data and configure them
    return {
        sciChartSurface,
        subChartTop,
        subChartBottomLeft,
        subChartBottomRight
    };
}

createThreePanelChart("scichart-root");
```

``` prism-code
// Demonstrates how to create a 1x2 panel of charts using SubCharts and the Builder API
async function builderExample(divElementId) {
    // Demonstrates how to create a line chart with SciChart.js using the Builder API
    const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
        surface: { theme: { type: EThemeProviderType.Dark } },
        // Main chart
        subCharts: [
            {
                surface: {
                    position: new Rect(0, 0, 1, 0.5),
                    title: "Audio Chart",
                    titleStyle: { fontSize: 14 },
                    theme: { type: EThemeProviderType.Dark }
                },
                xAxes: {
                    type: EAxisType.NumericAxis,
                    options: { axisTitle: "XAxis", axisTitleStyle: { fontSize: 12 } }
                },
                yAxes: {
                    type: EAxisType.NumericAxis,
                    options: { axisTitle: "YAxis", axisTitleStyle: { fontSize: 12 } }
                },
                modifiers: [{ type: EChart2DModifierType.ZoomPan }]
            },
            {
                surface: {
                    position: new Rect(0, 0.5, 0.5, 0.5),
                    title: "Frequency Chart",
                    titleStyle: { fontSize: 14 },
                    theme: { type: EThemeProviderType.Dark }
                },
                xAxes: {
                    type: EAxisType.NumericAxis,
                    options: { axisTitle: "XAxis", axisTitleStyle: { fontSize: 12 } }
                },
                yAxes: {
                    type: EAxisType.NumericAxis,
                    options: { axisTitle: "YAxis", axisTitleStyle: { fontSize: 12 } }
                },
                modifiers: [{ type: EChart2DModifierType.ZoomPan }]
            },
            {
                surface: {
                    position: new Rect(0.5, 0.5, 0.5, 0.5),
                    title: "Spectrogram Chart",
                    titleStyle: { fontSize: 14 },
                    theme: { type: EThemeProviderType.Dark }
                },
                xAxes: {
                    type: EAxisType.NumericAxis,
                    options: { axisTitle: "XAxis", axisTitleStyle: { fontSize: 12 } }
                },
                yAxes: {
                    type: EAxisType.NumericAxis,
                    options: { axisTitle: "YAxis", axisTitleStyle: { fontSize: 12 } }
                },
                modifiers: [{ type: EChart2DModifierType.ZoomPan }]
            }
        ]
    });

    return {
        sciChartSurface,
        subChartTop: sciChartSurface.subCharts.at(0),
        subChartBottomLeft: sciChartSurface.subCharts.at(1),
        subChartBottomRight: sciChartSurface.subCharts.at(2)
    };
}
```

This results in the following output:

## Code Explanation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#code-explanation" class="hash-link" aria-label="Direct link to Code Explanation" translate="no" title="Direct link to Code Explanation">â€‹</a>

In the above example:

This function, `createThreePanelChart`, demonstrates how to create a **reusable 1x2 panel layout** of charts using SciChart's **SubCharts API**. It initializes a parent chart that contains three sub-charts, each positioned within a **grid layout**. This setup is useful for visualizing related datasets in **audio, frequency, and spectrogram analysis**.

- imports

``` prism-code
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, Rect, ZoomPanModifier } = SciChart;
// or, if using NPM
import { SciChartSurface, NumericAxis, SciChartJsNavyTheme, Rect, ZoomPanModifier } from "scichart";
```

### 1. Importing Required Components<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#1-importing-required-components" class="hash-link" aria-label="Direct link to 1. Importing Required Components" translate="no" title="Direct link to 1. Importing Required Components">â€‹</a>

These are key SciChart.js components:

- `SciChartSurface` â†’ The main chart container.
- `NumericAxis` â†’ Adds numeric axes for the sub-charts.
- `SciChartJsNavyTheme` â†’ A predefined theme for styling.
- `Rect` â†’ Defines sub-chart layout positions.
- `ZoomPanModifier` â†’ Enables zoom and pan interactions.

### 2. Creating the Parent SciChartSurface<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#2-creating-the-parent-scichartsurface" class="hash-link" aria-label="Direct link to 2. Creating the Parent SciChartSurface" translate="no" title="Direct link to 2. Creating the Parent SciChartSurface">â€‹</a>

- Create parent chart

``` prism-code
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
```

- Initializes the main `SciChartSurface` inside the `divElementId` container.
- Applies the `SciChartJsNavyTheme` to maintain a consistent look.
- `wasmContext` provides access to WebAssembly-powered rendering.

### 3. Adding Sub-Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#3-adding-sub-charts" class="hash-link" aria-label="Direct link to 3. Adding Sub-Charts" translate="no" title="Direct link to 3. Adding Sub-Charts">â€‹</a>

The function creates **three sub-charts** within the parent `SciChartSurface`, defining their position using `Rect(x, y, width, height)`.

**Top Chart (100% width, 50% height)**

- Adding SubChart top

``` prism-code
    const subChartTop = SciChartSubSurface.createSubSurface(sciChartSurface, {
        position: new Rect(0, 0, 1, 0.5),
        theme: new SciChartJsNavyTheme(),
        title: "Audio Chart",
        titleStyle: { fontSize: 14 }
    });
```

- Full-width, occupies the **top half** of the container.
- Labeled **"Audio Chart"**.

**Bottom Left Chart (50% Width, 50% Height)**

- Adding SubChart BottomLeft

``` prism-code
    const subChartBottomLeft = SciChartSubSurface.createSubSurface(sciChartSurface, {
        position: new Rect(0, 0.5, 0.5, 0.5),
        theme: new SciChartJsNavyTheme(),
        title: "Frequency Chart",
        titleStyle: { fontSize: 14 }
    });
```

- Takes up **half the width** and sits in the **bottom left**.

**Bottom Right Chart (50% Width, 50% Height)**

- Adding SubChart BottomRight

``` prism-code
    const subChartBottomRight = SciChartSubSurface.createSubSurface(sciChartSurface, {
        position: new Rect(0.5, 0.5, 0.5, 0.5),
        theme: new SciChartJsNavyTheme(),
        title: "Spectrogram Chart",
        titleStyle: { fontSize: 14 }
    });
```

Placed beside the bottom left chart.

### 4. Adding Axes, Interaction Controls and test data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#4-adding-axes-interaction-controls-and-test-data" class="hash-link" aria-label="Direct link to 4. Adding Axes, Interaction Controls and test data" translate="no" title="Direct link to 4. Adding Axes, Interaction Controls and test data">â€‹</a>

- Adding Axis, Interaction

``` prism-code
   [subChartTop, subChartBottomLeft, subChartBottomRight].forEach(scs => {
        scs.xAxes.add(
            new NumericAxis(wasmContext, {
                axisTitle: "XAxis",
                axisTitleStyle: { fontSize: 12 }
            })
        );
        scs.yAxes.add(
            new NumericAxis(wasmContext, {
                axisTitle: "YAxis",
                axisTitleStyle: { fontSize: 12 }
            })
        );
        scs.chartModifiers.add(new ZoomPanModifier());

        // Add random data series
        const dataSeries = new XyDataSeries(wasmContext);
        const randomData = generateRandomData();
        dataSeries.appendRange(
            randomData.map(pt => pt.x),
            randomData.map(pt => pt.y)
        );

        scs.renderableSeries.add(
            new FastLineRenderableSeries(wasmContext, {
                stroke: getRandomColor(),
                strokeThickness: 2,
                dataSeries
            })
        );
    });
```

Loops through all sub-charts to:

- Add **numeric X and Y axes** with labels.
- Attach a `ZoomPanModifier` for **interactive zooming and panning**.
- Add random data using FastLineRenderableSeries

### 5. Returning the Created Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/#5-returning-the-created-chart" class="hash-link" aria-label="Direct link to 5. Returning the Created Chart" translate="no" title="Direct link to 5. Returning the Created Chart">â€‹</a>

- Returning Charts

``` prism-code
    return {
        sciChartSurface,
        subChartTop,
        subChartBottomLeft,
        subChartBottomRight
    };
```

Finally, we return all the instances to allow further customization if needed.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
