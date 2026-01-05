On this page

# The Donut Chart Type

In SciChart.js, the JavaScript Donut Chart type is represented by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" rel="noopener noreferrer" target="_blank">SciChartPieSurfaceðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/donut-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/donut-chart" rel="noopener noreferrer" target="_blank">JavaScript Donut Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/DonutChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Pie Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/donut-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/donut-chart" target="_blank">Donut Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

The Donut Chart represents data in a form of circle divided into segments called PieSegments. AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> represents a percentage that corresponds to a particular value. This value appears drawn on every segment and can be setÂ in code. AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> can be selected by clicking either on it or on the corresponding item in the Legend. This action provides a visual feedback on the chart and the Legend.

## Create a Donut Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/donut-chart-type/#create-a-donut-chart" class="hash-link" aria-label="Direct link to Create a Donut Chart" translate="no" title="Direct link to Create a Donut Chart">â€‹</a>

To create a Donut Chart, you have to create a number ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> instances and add them to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#piesegments" rel="noopener noreferrer" target="_blank">SciChartPieSurface.pieSegmentsðŸ“˜</a> collection. Set the propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#pietype" rel="noopener noreferrer" target="_blank">sciChartPieSurface.pieType = EPieType.DonutðŸ“˜</a> to enable a donut chart. Then the propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#holeradius" rel="noopener noreferrer" target="_blank">sciChartPieSurface.holeRadiusðŸ“˜</a> is obeyed to create the donut.

EachÂ Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a>Â has properties for <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#value" rel="noopener noreferrer" target="_blank">valueðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#text" rel="noopener noreferrer" target="_blank">textðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#color" rel="noopener noreferrer" target="_blank">colorðŸ“˜</a>, or alternatively <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#colorlineargradient" rel="noopener noreferrer" target="_blank">colorLinearGradientðŸ“˜</a> if you wish to specify a gradient fill. The property <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#isselected" rel="noopener noreferrer" target="_blank">isSelectedðŸ“˜</a> denotes whether theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a>Â is in the selected state or not.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a pie chart with SciChart.js
const {
    SciChartPieSurface,
    EPieType,
    SciChartJsNavyTheme,
    PieSegment,
    ELegendPlacement,
    ELegendOrientation,
    GradientParams,
    Point
} = SciChart;
// or, for npm, import { SciChartPieSurface, ... } from "scichart"

// Create the Donut chart
// Note: Code is the same as a pie chart, but we specify pieType and holeRadius
const sciChartPieSurface = await SciChartPieSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    pieType: EPieType.Donut,
    holeRadius: 0.6,
    animate: true
});

// Additional legend options
sciChartPieSurface.legend.showLegend = true;
sciChartPieSurface.legend.showCheckboxes = true;
sciChartPieSurface.legend.animate = true;
sciChartPieSurface.legend.placement = ELegendPlacement.TopRight;
sciChartPieSurface.legend.orientation = ELegendOrientation.Vertical;

// Create pie segments with value, colour and text
const pieSegment1 = new PieSegment({
    color: "#228B22",
    value: 40,
    text: "Green",
    colorLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#1D976C", offset: 0 },
        { color: "#93F9B9", offset: 1 }
    ])
});
const pieSegment2 = new PieSegment({
    value: 10,
    text: "Red",
    colorLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#DD5E89", offset: 0 },
        { color: "#F7BB97", offset: 1 }
    ])
});
const pieSegment3 = new PieSegment({
    value: 20,
    text: "Blue",
    colorLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#1FA2FF", offset: 0 },
        { color: "#12D8FA", offset: 0.5 },
        { color: "#A6FFCB", offset: 1 }
    ])
});
const pieSegment4 = new PieSegment({
    value: 15,
    text: "Yellow",
    colorLinearGradient: new GradientParams(new Point(0, 0), new Point(0, 1), [
        { color: "#F09819", offset: 0 },
        { color: "#EDDE5D", offset: 1 }
    ])
});

sciChartPieSurface.pieSegments.add(pieSegment1, pieSegment2, pieSegment3, pieSegment4);
```

``` prism-code
// Demonstrates how to create a pie chart with SciChart.js using the Builder API
const { chartBuilder, ESciChartSurfaceType, EPieType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const sciChartDonutChart = await chartBuilder.buildChart(divElementId, {
    type: ESciChartSurfaceType.Pie2D,
    options: {
        surface: {
            theme: { type: EThemeProviderType.Dark },
            holeRadius: 0.6,
            pieType: EPieType.Donut
        },
        segments: [
            { text: "This", value: 10, color: "red", labelStyle: { color: "white " } },
            { text: "That", value: 5, color: "blue", labelStyle: { color: "white " } },
            { text: "Other", value: 7, color: "green", labelStyle: { color: "white " } }
        ]
    }
});

// Alternative API
const donutChart = await chartBuilder.buildPieChart(divElementId, {
    surface: {
        theme: { type: EThemeProviderType.Dark },
        pieType: EPieType.Donut,
        holeRadius: 0.6
    },
    segments: [
        { text: "This", value: 10, color: "red", labelStyle: { color: "white " } },
        { text: "That", value: 5, color: "blue", labelStyle: { color: "white " } },
        { text: "Other", value: 7, color: "green", labelStyle: { color: "white " } }
    ]
});
```

This results in the following output:

## Styling Donut Chart Segments & Formatting Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/donut-chart-type/#styling-donut-chart-segments--formatting-labels" class="hash-link" aria-label="Direct link to Styling Donut Chart Segments &amp; Formatting Labels" translate="no" title="Direct link to Styling Donut Chart Segments &amp; Formatting Labels">â€‹</a>

Detailed documentation on how to style pie / donut chart segments and format labels can be found at theÂ [Pie Chart Documentation page](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/pie-chart-type).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/donut-chart-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/donut-chart-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
