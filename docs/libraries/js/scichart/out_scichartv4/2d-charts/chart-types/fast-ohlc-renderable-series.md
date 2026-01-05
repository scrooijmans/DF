On this page

# The OHLC Series Type

Ohlc Series or JavaScript Stock Charts can be created using theÂ **FastOhlcRenderableSeries** type.

![](out_scichartv4/2d-charts/chart-types/fast-ohlc-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript/candlestick-chart" rel="noopener noreferrer" target="_blank">JavaScript Ohlc Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/OhlcChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; OHLC Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/ohlc-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/candlestick-chart" target="_blank">JavaScript Candlestick Chart example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> , which allows switching between candle & OHLC

  

## Create an Ohlc Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series/#create-an-ohlc-series" class="hash-link" aria-label="Direct link to Create an Ohlc Series" translate="no" title="Direct link to Create an Ohlc Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/javascript-ohlc-chart" rel="noopener noreferrer" target="_blank">Javascript Ohlc Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create an OHLC (or Bar) chart with SciChart.js
const {
    SciChartSurface,
    CategoryAxis,
    NumericAxis,
    FastOhlcRenderableSeries,
    OhlcDataSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
sciChartSurface.xAxes.add(new CategoryAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { labelPrefix: "$", labelPrecision: 2 }));

// Data format is { dateValues[], openValues[], highValues[], lowValues[], closeValues[] }
const { dateValues, openValues, highValues, lowValues, closeValues, volumeValues } = await getCandles(
    "BTCUSDT",
    "1h",
    100
);

// Create a OhlcDataSeries with open, high, low, close values
const dataSeries = new OhlcDataSeries(wasmContext, {
    xValues: dateValues,
    openValues,
    highValues,
    lowValues,
    closeValues
});

// Create and add the OhlcSeries series
const ohlcSeries = new FastOhlcRenderableSeries(wasmContext, {
    dataSeries,
    strokeThickness: 1,
    dataPointWidth: 1,
    strokeUp: "#77ff77",
    strokeDown: "#ff7777"
});
sciChartSurface.renderableSeries.add(ohlcSeries);
```

``` prism-code
// Demonstrates how to create an OHLC chart with SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EAxisType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

// Data format is { dateValues[], openValues[], highValues[], lowValues[], closeValues[] }
const { dateValues, openValues, highValues, lowValues, closeValues, volumeValues } = await getCandles(
    "BTCUSDT",
    "1h",
    100
);

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: [{ type: EAxisType.CategoryAxis }],
    yAxes: [{ type: EAxisType.NumericAxis, options: { labelPrefix: "$", labelPrecision: 2 } }],
    series: [
        {
            type: ESeriesType.OhlcSeries,
            ohlcData: {
                xValues: dateValues,
                openValues,
                highValues,
                lowValues,
                closeValues
            },
            options: {
                dataPointWidth: 1,
                strokeUp: "#77ff77",
                strokeDown: "#ff7777",
                strokeThickness: 1
            }
        }
    ]
});
```

This results in the following output:

In the code above:

- AÂ Ohlc Series instance is created and added to the **SciChartSurface.renderableSeries** collection.
- This requires a special dataseries type: OhlcDataSeries, which accepts `X`, `Open`, `High`, `Low`, `Close` values
- We set the up/down stroke color via properties <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastohlcrenderableseries.html#strokeup" rel="noopener noreferrer" target="_blank">strokeUpðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastohlcrenderableseries.html#strokedown" rel="noopener noreferrer" target="_blank">strokeDownðŸ“˜</a>.
- We set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastohlcrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> - which defines the fraction of width to occupy
- We use a special axis type called the CategoryAxis which removes gaps in stock market data.

![](out_scichartv4/2d-charts/chart-types/fast-ohlc-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html" rel="noopener noreferrer" target="_blank">CategoryAxisðŸ“˜</a>Â is necessaryÂ if you have Forex or Stock market data which includes weekend or overnight gaps, as this axis type measures by x-index, not by x-value. For CryptoCurrency data the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html" rel="noopener noreferrer" target="_blank">NumericAxisðŸ“˜</a> can be used as these are 24/7 markets.

You can format the date labels on the xAxis by following the instructions on theÂ [Axis Label Formatting](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/numeric-formats) page.

## Adding Volume Bars to an OHLC Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series/#adding-volume-bars-to-an-ohlc-chart" class="hash-link" aria-label="Direct link to Adding Volume Bars to an OHLC Chart" translate="no" title="Direct link to Adding Volume Bars to an OHLC Chart">â€‹</a>

TheÂ <a href="https://www.scichart.com/demo/react/candlestick-chart" rel="noopener noreferrer" target="_blank">Candlestick Chart example</a> shows a technique to add volume bars docked to the bottom of the chart. The technique is the same for OHLC series so please see theÂ [candlestick docs](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series).

## PaintingÂ Ohlc bars with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series/#paintingohlc-bars-with-different-colors" class="hash-link" aria-label="Direct link to PaintingÂ Ohlc bars with Different Colors" translate="no" title="Direct link to PaintingÂ Ohlc bars with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-ohlc-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of specific OHLC Bars using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Candlestick/Ohlc Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)
- [Common RenderableSeries Properties](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-ohlc-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-ohlc-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
