On this page

# The Pie Chart Type

In SciChart.js, theÂ <a href="https://www.scichart.com/demo/javascript-pie-chart" rel="noopener noreferrer" target="_blank">JavaScript Pie Chart</a>Â type is represented by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" rel="noopener noreferrer" target="_blank">SciChartPieSurfaceðŸ“˜</a> type.

![](out_scichartv4/2d-charts/chart-types/pie-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-pie-chart" rel="noopener noreferrer" target="_blank">JavaScript Pie Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/PieChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Pie Chart</a>Â on Github

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/pie-chart" target="_blank">Pie Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

The Pie Chart represents data in a form of circle divided into triangular wedges called segments. AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> represents a percentage that corresponds to a particular value. This value appears drawn on every segment and can be setÂ in code. AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> can be selected by clicking either on it or on the corresponding item in the Legend. This action provides a visual feedback on the chart and the Legend.

## Create a Pie Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/pie-chart-type/#create-a-pie-chart" class="hash-link" aria-label="Direct link to Create a Pie Chart" translate="no" title="Direct link to Create a Pie Chart">â€‹</a>

To create a Pie Chart, you have to create a number ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> instances and add them to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#piesegments" rel="noopener noreferrer" target="_blank">SciChartPieSurface.pieSegmentsðŸ“˜</a> collection.

EachÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a> has properties for **value**, **text** and **color**, or alternatively **colorLinearGradient** if you wish to specify a gradient fill. The property **isSelected** denotes whether theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" rel="noopener noreferrer" target="_blank">PieSegmentðŸ“˜</a>Â is in the selected state or not.

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

// Create the pie chart
const sciChartPieSurface = await SciChartPieSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    pieType: EPieType.Pie,
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
const { chartBuilder, ESciChartSurfaceType, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const sciChartPieChart = await chartBuilder.buildChart(divElementId, {
    type: ESciChartSurfaceType.Pie2D,
    options: {
        surface: { theme: { type: EThemeProviderType.Dark } },
        segments: [
            { text: "This", value: 10, color: "red" },
            { text: "That", value: 5, color: "blue" },
            { text: "Other", value: 7, color: "green" }
        ]
    }
});

// Alternative API
const pieChart = await chartBuilder.buildPieChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    segments: [
        { text: "This", value: 10, color: "red" },
        { text: "That", value: 5, color: "blue" },
        { text: "Other", value: 7, color: "green" }
    ]
});
```

This results in the following output:Â 

## Dynamically Updating a Pie Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/pie-chart-type/#dynamically-updating-a-pie-chart" class="hash-link" aria-label="Direct link to Dynamically Updating a Pie Chart" translate="no" title="Direct link to Dynamically Updating a Pie Chart">â€‹</a>

Pie Charts can be dynamically updated by setting the PieSegment.value property. When SciChartPieSurface.animate is true, updates to the pie chart will be animated.

Try this code below:

- TS

``` prism-code
// Create a Pie Chart
const sciChartPieSurface = await SciChartPieSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    pieType: EPieType.Pie,
    animate: true,
    animationFrames: 30
});

// Disable the legend for this example
sciChartPieSurface.legend.showLegend = false;

// Create pie segments and add to the chart
const pieSegment1 = new PieSegment({
    color: "#F48420",
    value: 5
});
const pieSegment2 = new PieSegment({
    color: "#30BC9A",
    value: 10
});
const pieSegment3 = new PieSegment({
    color: "#EC0F6C",
    value: 15
});
const pieSegment4 = new PieSegment({
    color: "#50C7E0",
    value: 20
});
sciChartPieSurface.pieSegments.add(pieSegment1, pieSegment2, pieSegment3, pieSegment4);

// Dynamically update the pie segments
const updateFunc = () => {
    pieSegment1.value = Math.random() * 20 + 10;
    pieSegment2.value = Math.random() * 20 + 10;
    pieSegment3.value = Math.random() * 20 + 10;
    pieSegment4.value = Math.random() * 20 + 10;
    setTimeout(() => updateFunc(), 1500);
};

setTimeout(updateFunc, 1000);
```

This results in the following output:

## Formatting Pie Chart Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/pie-chart-type/#formatting-pie-chart-labels" class="hash-link" aria-label="Direct link to Formatting Pie Chart Labels" translate="no" title="Direct link to Formatting Pie Chart Labels">â€‹</a>

Several options for formatting Pie Chart labels are possible with SciChart.js.

- Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#seriesspacing" rel="noopener noreferrer" target="_blank">SciChartPieSurface.seriesSpacingðŸ“˜</a> to put a padding in between pie segments.
- Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#labelstyle" rel="noopener noreferrer" target="_blank">SciChartPieSurface.labelStyleðŸ“˜</a> to set a global label font size, color and family
- Override <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#labelprovider" rel="noopener noreferrer" target="_blank">SciChartPieSurface.labelProvider.getSegmentTextðŸ“˜</a> to set a general label text override
- Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#labeloffset" rel="noopener noreferrer" target="_blank">PieSegment.labelOffsetðŸ“˜</a> to move labels further away from the pie segment.
- Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#labelstyle" rel="noopener noreferrer" target="_blank">PieSegment.labelStyleðŸ“˜</a> to set individual pie segment font and color
- Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#radiusadjustment" rel="noopener noreferrer" target="_blank">PieSegment.radiusAdjustmentðŸ“˜</a> to make the pie segment larger or smaller.
- LabelProviders are also available on individual <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html#labelprovider" rel="noopener noreferrer" target="_blank">PieSegment.labelProviderðŸ“˜</a> property

Below, an example combines several of these techniques:

- TS

``` prism-code
// Create a Pie Chart
const sciChartPieSurface = await SciChartPieSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    pieType: EPieType.Pie,
    animate: true,
    seriesSpacing: 5
});
// Set this to adjust the radial position of the labels
// When positioning outside the pie, you may want to make further fine adjustments to label positions using the labelOffset as shown below
sciChartPieSurface.labelRadiusAdjustment = 1.7;

// set default label style
sciChartPieSurface.labelStyle = { fontSize: 14, color: "White" };

// labelProvider on the surface will be used if an override is not set on a segment
sciChartPieSurface.labelProvider.getSegmentText = (segment, total) =>
    `<span>${segment.text}<span><br/><span>${segment.value.toFixed(0)} (${((100 * segment.value) / total).toFixed(
        1
    )}%)</span>`;

const pieSegment1 = new PieSegment({
    color: "#F48420",
    value: 40,
    text: "Oranges",
    radiusAdjustment: 1.05,
    labelOffset: new Point(10, 0),
    // labelStyles can be overridden per segment.
    // These will be merged onto the surface style so the resulting style here is fontSize: 14, color: "#f8f682"
    labelStyle: { color: "#F48420" }
});

const pieSegment2 = new PieSegment({
    color: "#30BC9A",
    value: 10,
    text: "Apples",
    radiusAdjustment: 1.1,
    labelOffset: new Point(0, -20)
});
// If you set a property on the segment labelProvider, it will override the one on the surface
pieSegment2.labelProvider.getSegmentText = (segment, total) => "Some Apples";

// You can also pass labelProvider options in on the constructor.
const pieSegment3 = new PieSegment({
    color: "#EC0F6C",
    value: 20,
    text: "Strawberries",
    labelProvider: new PieLabelProvider({ labelPrefix: "Strawberries: " }),
    labelOffset: new Point(-25, 0),
    labelStyle: { color: "#EC0F6C" }
});

const pieSegment4 = new PieSegment({
    color: "#50C7E0",
    value: 15,
    text: "Grapes"
});
// Overriding a property on the segment labelProvider implicitly creates a new default PieLabelProvider that overries the one set on the surface
pieSegment4.labelProvider.formatLabel = dataValue => dataValue.toFixed(2) + "%";

sciChartPieSurface.pieSegments.add(pieSegment1, pieSegment2, pieSegment3, pieSegment4);
```

Resulting in:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/pie-chart-type/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Donut Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/donut-chart-type)
- [Polar Pie Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-pie-chart)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/pie-chart-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/pie-chart-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
