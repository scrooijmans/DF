On this page

# Central Axis Layout

Placing Axis in the center the chart, like an oscilloscope or spectrum analyzer is possible with SciChart.js.

To do this, use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/centralaxeslayoutmanager.html" rel="noopener noreferrer" target="_blank">CentralAxesLayoutManagerðŸ“˜</a> applied to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#layoutmanager" rel="noopener noreferrer" target="_blank">SciChartSurface.LayoutManagerðŸ“˜</a> property.

Here's a code sample:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure a central axis in SciChart.js
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, CentralAxesLayoutManager } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Apply the CentralAxesLayoutManager to the SciChartSurface
sciChartSurface.layoutManager = new CentralAxesLayoutManager();

// Add an X, Y Axis
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        // To allow easier visualisation of axis position
        backgroundColor: "#50C7E022",
        axisBorder: {
            borderTop: 1,
            color: "#50C7E0"
        }
    })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        // To allow easier visualisation of axis position
        backgroundColor: "#F4842022",
        axisBorder: {
            borderRight: 1,
            color: "#F48420"
        }
    })
);
```

``` prism-code
// Demonstrates how to configure a central axis in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType, ELayoutManagerType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: {
        theme: { type: EThemeProviderType.Dark },
        layoutManager: { type: ELayoutManagerType.CentralAxes }
    },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            // To allow easier visualisation of axis position
            backgroundColor: "#50C7E022",
            axisBorder: {
                borderTop: 1,
                color: "#50C7E0"
            }
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            // To allow easier visualisation of axis position
            backgroundColor: "#F4842022",
            axisBorder: {
                borderRight: 1,
                color: "#F48420"
            }
        }
    }
});
```

This results in the following output:

## Configure the Position of Central Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/central-axis-layout/#configure-the-position-of-central-axis" class="hash-link" aria-label="Direct link to Configure the Position of Central Axis" translate="no" title="Direct link to Configure the Position of Central Axis">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/centralaxeslayoutmanager.html" rel="noopener noreferrer" target="_blank">CentralAxesLayoutManagerðŸ“˜</a>Â has some options you can use to configure the position of the horizontal and vertical axes.

The following code places an YAxis inside the chart at X=3 and an XAxis inside the chart at Y=100 pixels.

![](out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/central-axis-layout/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Options available in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/einneraxisplacementcoordinatemode.html" rel="noopener noreferrer" target="_blank">EInnerAxisPlacementCoordinateModeðŸ“˜</a> enum include: DataValue, Pixel, or Relative, which allows placement of an axis at a fraction of the viewport from 0..1.

- TS
- Builder API (JSON Config)

``` prism-code
// Apply the CentralAxesLayoutManager to the SciChartSurface
sciChartSurface.layoutManager = new CentralAxesLayoutManager({
    horizontalAxisPositionCoordinateMode: EInnerAxisPlacementCoordinateMode.DataValue,
    verticalAxisPositionCoordinateMode: EInnerAxisPlacementCoordinateMode.Pixel,
    horizontalAxisPosition: 3,
    verticalAxisPosition: 100
});

// Continue to add your X,Y axis as before
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: {
        theme: { type: EThemeProviderType.Dark },
        layoutManager: {
            type: ELayoutManagerType.CentralAxes,
            options: {
                horizontalAxisPositionCoordinateMode: EInnerAxisPlacementCoordinateMode.DataValue,
                verticalAxisPositionCoordinateMode: EInnerAxisPlacementCoordinateMode.Pixel,
                horizontalAxisPosition: 3,
                verticalAxisPosition: 100
            }
        }
    },
    // etc...
```

This results in the following output:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/central-axis-layout/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/central-axis-layout/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
