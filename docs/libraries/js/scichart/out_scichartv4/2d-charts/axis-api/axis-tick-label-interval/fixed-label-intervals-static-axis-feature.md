On this page

# Fixed Label Intervals - Static Axis Feature

Sometimes you want to have a fixed number of labels and major gridlines displayed on a chart, at specific values.

Consider the case where you have a chart withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">xAxis.visibleRangeðŸ“˜</a> from 0 to 10, and you want to display labels precisely at 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10. Zooming and panning should not change the number or spacing of the labels. In this case, you can set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#isstaticaxis" rel="noopener noreferrer" target="_blank">axis.isStaticAxisðŸ“˜</a> property.

When in this mode, the major gridline positions / label positions and spacing are fixed. If theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange" rel="noopener noreferrer" target="_blank">axis.visibleRangeðŸ“˜</a> changes then the label values update, not the position or spacing.

## Enabling Static Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/fixed-label-intervals-static-axis-feature/#enabling-static-axis" class="hash-link" aria-label="Direct link to Enabling Static Axis" translate="no" title="Direct link to Enabling Static Axis">â€‹</a>

To enable the static axis mode, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure a chart with Static Axis in SciChart.js
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme, NumberRange } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Adjust major/minor gridline style to make it clearer for the demo
const styleOptions = {
    majorGridLineStyle: { color: "#50C7E077" },
    minorGridLineStyle: { color: "#50C7E033" }
};

const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "isStaticAxis = true, maxAutoTicks = 10",
    isStaticAxis: true,
    maxAutoTicks: 10,
    ...styleOptions
});

const yAxis = new NumericAxis(wasmContext, {
    axisTitle: "yAxis.isStaticAxis = false",
    growBy: new NumberRange(0.1, 0.1),
    ...styleOptions
});

sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);
```

``` prism-code
// Demonstrates how to configure a chart with Static Axis in SciChart.js
const { chartBuilder, EThemeProviderType, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "isStaticAxis = true, maxAutoTicks = 5",
            isStaticAxis: true,
            maxAutoTicks: 5
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "yAxis.isStaticAxis = false"
        }
    }
});
```

This results in the following output

## Varying the number of Static Axis Ticks & Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/fixed-label-intervals-static-axis-feature/#varying-the-number-of-static-axis-ticks--labels" class="hash-link" aria-label="Direct link to Varying the number of Static Axis Ticks &amp; Labels" translate="no" title="Direct link to Varying the number of Static Axis Ticks &amp; Labels">â€‹</a>

WhenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#isstaticaxis" rel="noopener noreferrer" target="_blank">axis.isStaticAxisðŸ“˜</a> is set to true, the number of major ticks (major gridlines, axis labels) are constrained byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.maxAutoTicksðŸ“˜</a>.

For example settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.maxAutoTicksðŸ“˜</a> = 5 will ensure there are always 5 labels and 5 major gridlines on the chart. These wil be at fixed spacings no matter the zoom level of the chart. Label values will update instead.

## Formatting Labels and Precision (Decimal Places)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/fixed-label-intervals-static-axis-feature/#formatting-labels-and-precision-decimal-places" class="hash-link" aria-label="Direct link to Formatting Labels and Precision (Decimal Places)" translate="no" title="Direct link to Formatting Labels and Precision (Decimal Places)">â€‹</a>

When in static axis mode, the axis stil obeys formatting rules provided by the LabelProvider. Read theÂ [NumericAxis Docs](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis) or theÂ [LabelProvider API Docs](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) for more info on how to vary label precision.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-tick-label-interval/fixed-label-intervals-static-axis-feature/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-tick-label-interval/fixed-label-intervals-static-axis-feature/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
