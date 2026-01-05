On this page

# The Text Series Type

There are several ways to add text to a SciChart.js chart. These include the TextAnnotation, series DataLabels and also the FastTextRenderableSeries (Text Series).

Text Series should be used when you want to render a lot of text, not necessarily at X,Y positions of other chart series.

![](out_scichartv4/2d-charts/chart-types/fast-text-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-text-chart" rel="noopener noreferrer" target="_blank">JavaScript Text / Word Cloud Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/TextSeriesChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Text Series Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/text-chart" target="_blank">Text Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Creating a Text Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-text-renderable-series/#creating-a-text-series" class="hash-link" aria-label="Direct link to Creating a Text Series" translate="no" title="Direct link to Creating a Text Series">â€‹</a>

To create a chart usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttextrenderableseries.html" rel="noopener noreferrer" target="_blank">FastTextRenderableSeriesðŸ“˜</a> use the following code.Â 

**Note** that it is required to set `style: { fontSize: X }` and `color` in the dataLabels property in order for text to be drawn.Â 

FastTextRenderableSeries uses the specialÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xytextdataseries.html" rel="noopener noreferrer" target="_blank">XyTextDataSeriesðŸ“˜</a> which allows you to supplyÂ text values directly on the dataSeries, rather than having to use metadata.Â 

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a text chart with SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    FastTextRenderableSeries,
    XyTextDataSeries,
    SciChartJsNavyTheme,
    NumberRange
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(0, 9) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(0, 9) }));

// Create a chart with textSeries
const textSeries = new FastTextRenderableSeries(wasmContext, {
    dataSeries: new XyTextDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6],
        yValues: [3, 5, 6, 4, 2, 5],
        textValues: ["This", "text", "is", "drawn", "using", "FastTextRenderableSeries"]
    }),
    // font and size is required for text to be drawn
    dataLabels: {
        style: {
            fontFamily: "Default",
            fontSize: 18
        },
        color: "white"
    }
});
sciChartSurface.renderableSeries.add(textSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    series: [
        {
            type: ESeriesType.TextSeries,
            xyTextData: {
                xValues: [1, 2, 3, 4, 5, 6],
                yValues: [3, 5, 6, 4, 2, 5],
                textValues: ["This", "text", "is", "drawn", "using", "FastTextRenderableSeries"]
            },
            options: {
                dataLabels: {
                    style: {
                        fontFamily: "Default",
                        fontSize: 18
                    },
                    color: "white"
                }
            }
        }
    ]
});
```

This results in the following output:Â 

## Customising the Text SeriesÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-text-renderable-series/#customising-the-text-series" class="hash-link" aria-label="Direct link to Customising the Text SeriesÂ " translate="no" title="Direct link to Customising the Text SeriesÂ ">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttextrenderableseries.html" rel="noopener noreferrer" target="_blank">FastTextRenderableSeriesðŸ“˜</a>Â usesÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html" rel="noopener noreferrer" target="_blank">TextDataLabelProviderðŸ“˜</a> for the generation and drawing of text, which has a slightly reduced api compared with the full DataLabels api.Â 

It hasÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html#getposition" rel="noopener noreferrer" target="_blank">getPositionðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html#getcolor" rel="noopener noreferrer" target="_blank">getColorðŸ“˜</a> functions, but text is always taken from the **XyTextDataSeries**, and there is no label skipping - all labels are drawn even if they overlap.Â 

There is however an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html#onaftergenerate" rel="noopener noreferrer" target="_blank">onAfterGenerateðŸ“˜</a> function that is called with the dataLabels before they are drawn which you can use to perform additional adjustments.Â  If you need to rely on the label sizes in this function, make sure to set **calculateTextBounds:Â true** in dataLabels.

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fasttextrenderableseries.html" rel="noopener noreferrer" target="_blank">FastTextRenderableSeriesðŸ“˜</a>Â supports pointmarkers and alsoÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html#horizontaltextposition" rel="noopener noreferrer" target="_blank">horizontalTextPositionðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html#verticaltextposition" rel="noopener noreferrer" target="_blank">verticalTextPositionðŸ“˜</a> dataLabels options.

![](out_scichartv4/2d-charts/chart-types/fast-text-renderable-series/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Text is drawn using Native Text rendering, so to use any font other than Arial you will need ensure that font is available on your server (as fontname.ttf), or registered using **sciChartSurface.registerFont(...)** if coming from a remote url.Â 

- TS

``` prism-code
// Register a remote font
await sciChartSurface.registerFont(
    "notoserif",
    "https://raw.githubusercontent.com/google/fonts/main/ofl/notoserif/NotoSerif-Regular.ttf"
);

// Create a textSeries with custom fond
const textSeries = new FastTextRenderableSeries(wasmContext, {
    dataSeries: new XyTextDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6],
        yValues: [1, 1, 1, 1, 1, 1],
        textValues: ["This", "text", "is", "drawn", "using", "FastTextRenderableSeries"]
    }),
    // font and size is required for text to be drawn
    dataLabels: {
        style: {
            // Set custom font
            fontFamily: "notoserif",
            fontSize: 18
        },
        color: "white",
        // Set text position relative to the data point
        horizontalTextPosition: EHorizontalTextPosition.Center,
        verticalTextPosition: EVerticalTextPosition.Center,
        // force the label sizes to be calculated as we need them below
        calculateTextBounds: true
    }
});

// Handle further customisation of positioning and color
(textSeries.dataLabelProvider as TextDataLabelProvider).getColor = (state, text) => {
    if (state.xVal() < 4) {
        return parseColorToUIntArgb("red");
    } else {
        return state.color;
    }
};

(textSeries.dataLabelProvider as TextDataLabelProvider).onAfterGenerate = dataLabels => {
    for (let i = 0; i < dataLabels.length; i++) {
        const label = dataLabels[i];
        if (i < dataLabels.length - 1) {
            // Shift this label down if it would overlap the next one
            if (label.rect.right > dataLabels[i + 1].rect.left) {
                // @ts-ignore
                label.position.y += label.rect.height;
            }
        }
    }
};

// Add the TextSeries to the chart
sciChartSurface.renderableSeries.add(textSeries);
```

This results in the following output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-text-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Adding DataLabels to a Chart Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-text-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-text-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
