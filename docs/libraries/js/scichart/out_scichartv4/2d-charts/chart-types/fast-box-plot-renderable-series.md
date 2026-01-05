On this page

# The Box Plot Series Type

A box plot (also called a box-and-whisker plot) is a statistical visualization that displays the distribution of a dataset through five key statistical measures. It's one of the most effective ways to show data distribution, identify outliers, and compare multiple datasets. The Box Plot Series Type is a powerful visualization tool that displays the distribution of data based on a five-number summary: minimum, first quartile (Q1), median (Q2), third quartile (Q3), and maximum. It is particularly useful for identifying outliers and understanding the spread of data.

## Key Configuration Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-box-plot-renderable-series/#key-configuration-options" class="hash-link" aria-label="Direct link to Key Configuration Options" translate="no" title="Direct link to Key Configuration Options">â€‹</a>

### Data Point Width Modes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-box-plot-renderable-series/#data-point-width-modes" class="hash-link" aria-label="Direct link to Data Point Width Modes" translate="no" title="Direct link to Data Point Width Modes">â€‹</a>

Sets the mode which determines how dataPointWidth is interpreted. Available values are <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatapointwidthmode.html" rel="noopener noreferrer" target="_blank">EDataPointWidthModeðŸ“˜</a>. Default is Relative.

- EDataPointWidthMode.Relative: Interprets Data Point Width as a relative to the full width which is axis length / number of columns. This assumes that there are no gaps in the data. If you are plotting sparse columns on a NumericAxis, consider Range mode
- EDataPointWidthMode.Absolute: Interprets Data Point Width as an absolute pixel value
- EDataPointWidthMode.Range: Interprets Data Point Width as the x data range per column. This is useful if you are plotting sparse columns on a NumericAxis

### Styling Components<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-box-plot-renderable-series/#styling-components" class="hash-link" aria-label="Direct link to Styling Components" translate="no" title="Direct link to Styling Components">â€‹</a>

- Main Box: Defined by stroke, fill, opacity, and strokeThickness
- Whiskers: Vertical lines extending from box to min/max values
- Caps: Horizontal lines at whisker ends
- Median Line: Horizontal line inside the box

## Create a Box Plot Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-box-plot-renderable-series/#create-a-box-plot-chart" class="hash-link" aria-label="Direct link to Create a Box Plot Chart" translate="no" title="Direct link to Create a Box Plot Chart">â€‹</a>

To create a Box Plot Chart with SciChart.js we need to use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastboxplotrenderableseries.html" rel="noopener noreferrer" target="_blank">FastBoxPlotRenderableSeriesðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxplotdataseries.html" rel="noopener noreferrer" target="_blank">BoxPlotDataSeriesðŸ“˜</a>. Start with the following code:

- TS
- JS

``` prism-code
// Demonstrates how to create a box plot chart with SciChart.js

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const isVertical = true;
const isXCategoryAxis = false;

const configCategoryAxis: ICategoryAxisOptions = {
    labelFormat: ENumericFormat.Decimal,
    labelPrecision: 0,
    cursorLabelFormat: ENumericFormat.Decimal,
    cursorLabelPrecision: 0
};

const configX = {
    axisAlignment: isVertical ? EAxisAlignment.Bottom : EAxisAlignment.Left,
    growBy: new NumberRange(0.05, 0.05),
    autoRange: EAutoRange.Once,
    flippedCoordinates: false
};

sciChartSurface.xAxes.add(
    isXCategoryAxis
        ? new CategoryAxis(wasmContext, { ...configX, ...configCategoryAxis })
        : new NumericAxis(wasmContext, configX)
);

sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisAlignment: isVertical ? EAxisAlignment.Left : EAxisAlignment.Bottom,
        growBy: new NumberRange(0.05, 0.05),
        autoRange: EAutoRange.Once,
        flippedCoordinates: !isVertical
    })
);

const xValues = [4, 5, 6];
const minimumValues = [0, 1, 0.5];
const maximumValues = [10, 9, 9.5];
const medianValues = [4.5, 5.5, 5];
const lowerQuartileValues = [3, 4, 3.5];
const upperQuartileValues = [7, 6, 6.5];

const boxPlotDataSeries = new BoxPlotDataSeries(wasmContext, {
    xValues,
    minimumValues,
    maximumValues,
    medianValues,
    lowerQuartileValues,
    upperQuartileValues
});

const boxSeries = new FastBoxPlotRenderableSeries(wasmContext, {
    dataSeries: boxPlotDataSeries,
    stroke: "black",
    strokeThickness: 1,
    dataPointWidthMode: EDataPointWidthMode.Relative,
    dataPointWidth: 0.5,
    fill: "green",
    opacity: 0.6,
    whiskers: {
        stroke: "green",
        strokeThickness: 2
    },
    cap: {
        stroke: "green",
        strokeThickness: 2,
        dataPointWidth: 0.3
    },
    medianLine: {
        stroke: "black",
        strokeThickness: 2
    }
});
sciChartSurface.renderableSeries.add(boxSeries);
```

``` prism-code
// Demonstrates how to create a box plot chart with SciChart.js
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
const isVertical = true;
const isXCategoryAxis = false;
const configCategoryAxis = {
    labelFormat: ENumericFormat.Decimal,
    labelPrecision: 0,
    cursorLabelFormat: ENumericFormat.Decimal,
    cursorLabelPrecision: 0
};
const configX = {
    axisAlignment: isVertical ? EAxisAlignment.Bottom : EAxisAlignment.Left,
    growBy: new NumberRange(0.05, 0.05),
    autoRange: EAutoRange.Once,
    flippedCoordinates: false
};
sciChartSurface.xAxes.add(isXCategoryAxis
    ? new CategoryAxis(wasmContext, { ...configX, ...configCategoryAxis })
    : new NumericAxis(wasmContext, configX));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, {
    axisAlignment: isVertical ? EAxisAlignment.Left : EAxisAlignment.Bottom,
    growBy: new NumberRange(0.05, 0.05),
    autoRange: EAutoRange.Once,
    flippedCoordinates: !isVertical
}));
const xValues = [4, 5, 6];
const minimumValues = [0, 1, 0.5];
const maximumValues = [10, 9, 9.5];
const medianValues = [4.5, 5.5, 5];
const lowerQuartileValues = [3, 4, 3.5];
const upperQuartileValues = [7, 6, 6.5];
const boxPlotDataSeries = new BoxPlotDataSeries(wasmContext, {
    xValues,
    minimumValues,
    maximumValues,
    medianValues,
    lowerQuartileValues,
    upperQuartileValues
});
const boxSeries = new FastBoxPlotRenderableSeries(wasmContext, {
    dataSeries: boxPlotDataSeries,
    stroke: "black",
    strokeThickness: 1,
    dataPointWidthMode: EDataPointWidthMode.Relative,
    dataPointWidth: 0.5,
    fill: "green",
    opacity: 0.6,
    whiskers: {
        stroke: "green",
        strokeThickness: 2
    },
    cap: {
        stroke: "green",
        strokeThickness: 2,
        dataPointWidth: 0.3
    },
    medianLine: {
        stroke: "black",
        strokeThickness: 2
    }
});
sciChartSurface.renderableSeries.add(boxSeries);
```

This results in the following output:

## Box Plot Complex Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-box-plot-renderable-series/#box-plot-complex-example" class="hash-link" aria-label="Direct link to Box Plot Complex Example" translate="no" title="Direct link to Box Plot Complex Example">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-box-plot-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/iframe/box-plot-chart" rel="noopener noreferrer" target="_blank">JavaScript Box Plot Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/release_v4.0/Examples/src/components/Examples/Charts2D/BasicChartTypes/BoxPlotChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Box Plot Series</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/box-plot-chart" target="_blank">Box Plot Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-box-plot-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-box-plot-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
