On this page

# The Candlestick Series type

Candlestick Series or JavaScript Stock Charts can be created using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html" rel="noopener noreferrer" target="_blank">FastCandlestickRenderableSeries typeðŸ“˜</a>.

![](out_scichartv4/2d-charts/chart-types/fast-candlestick-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/react/candlestick-chart" rel="noopener noreferrer" target="_blank">JavaScript Candlestick Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/BasicChartTypes/CandlestickChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Candlestick Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript/candlestick-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/candlestick-chart" target="_blank">Candlestick Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create aÂ Candlestick Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series/#create-acandlestick-series" class="hash-link" aria-label="Direct link to Create aÂ Candlestick Series" translate="no" title="Direct link to Create aÂ Candlestick Series">â€‹</a>

To create aÂ <a href="https://www.scichart.com/demo/react/candlestick-chart" rel="noopener noreferrer" target="_blank">Javascript Candlestick Chart</a> with SciChart.js, use the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to create a Candlestick chart with SciChart.js
const {
    SciChartSurface,
    CategoryAxis,
    NumericAxis,
    FastCandlestickRenderableSeries,
    OhlcDataSeries,
    SciChartJsNavyTheme
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new CategoryAxis(wasmContext));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { 
        labelPrefix: "$", 
        labelPrecision: 2 
    })
);

// Data format is { dateValues[], openValues[], highValues[], lowValues[], closeValues[] }
const { dateValues, openValues, highValues, lowValues, closeValues, volumeValues } = await getCandles(
    "BTCUSDT",
    "1h",
    100
);

// Create and add the Candlestick series
const candlestickSeries = new FastCandlestickRenderableSeries(wasmContext, {
    dataSeries: new OhlcDataSeries(wasmContext, {
        xValues: dateValues,
        openValues,
        highValues,
        lowValues,
        closeValues
    }),
    strokeThickness: 1,
    dataPointWidth: 0.7,
    brushUp: "#33ff3377",
    brushDown: "#ff333377",
    strokeUp: "#77ff77",
    strokeDown: "#ff7777"
});
sciChartSurface.renderableSeries.add(candlestickSeries);
```

``` prism-code
// Demonstrates how to create a line chart with SciChart.js using the Builder API
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
            type: ESeriesType.CandlestickSeries,
            ohlcData: {
                xValues: dateValues,
                openValues,
                highValues,
                lowValues,
                closeValues
            },
            options: {
                dataPointWidth: 0.7,
                brushUp: "#33ff3377",
                brushDown: "#ff333377",
                strokeUp: "#77ff77",
                strokeDown: "#ff7777",
                strokeThickness: 1
            }
        }
    ]
});
```

This results in the following output:

> The above example makes a web call to Binance to fetch Bitcoin/USD prices. If you see a blank chart, check the Js console as this web call may be blocked. You can always edit the Codepen to substitute your own data!

In the example above:

- AÂ Candlestick Series instance is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection.
- This requires a special dataseries type:Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcdataseries.html" rel="noopener noreferrer" target="_blank">OhlcDataSeriesðŸ“˜</a>, which accepts X, Open, High, Low, Close values as arrays of values.
- We set the up/down stroke and fill colors via propertiesÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html#brushup" rel="noopener noreferrer" target="_blank">brushUpðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html#brushdown" rel="noopener noreferrer" target="_blank">brushDownðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html#strokeup" rel="noopener noreferrer" target="_blank">strokeUpðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html#strokedown" rel="noopener noreferrer" target="_blank">strokeDownðŸ“˜</a> properties.
- We setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html#datapointwidth" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> - which defines the fraction of width to occupy
- We use a special axis type called theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html" rel="noopener noreferrer" target="_blank">CategoryAxisðŸ“˜</a> which removes gaps in stock market data.

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/categoryaxis.html" rel="noopener noreferrer" target="_blank">CategoryAxisðŸ“˜</a>Â is necessaryÂ if you have Forex or Stock market data which includes weekend or overnight gaps, as this axis type measures by x-index, not by x-value. For CryptoCurrency data theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" rel="noopener noreferrer" target="_blank">DateTimeNumericAxisðŸ“˜</a> can be used as these are 24/7 markets.

You can format the date labels on the XAxis by following the instructions on theÂ [Axis Label Formatting](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/numeric-formats) page.

## Adding Volume Bars to a Candlestick Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series/#adding-volume-bars-to-a-candlestick-chart" class="hash-link" aria-label="Direct link to Adding Volume Bars to a Candlestick Chart" translate="no" title="Direct link to Adding Volume Bars to a Candlestick Chart">â€‹</a>

In theÂ <a href="https://www.scichart.com/demo/react/candlestick-chart" rel="noopener noreferrer" target="_blank">SciChart.js demo - Candlestick Charts</a> - volume bars are docked to the bottom of the chart. Here's how to do this with SciChart.js.

- TS
- Builder API (JSON Config)

``` prism-code
// Add a secondary axis for the volume bars
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { 
        id: "VolumeAxisId", 
        isVisible: false, 
        growBy: new NumberRange(0, 4) 
    })
);

// Fetch data. Format is { dates[], opens[], highs[], lows[], closes[], volumes[] }
const { dateValues, openValues, highValues, lowValues, closeValues, volumeValues } = await getCandles(
    "BTCUSDT",
    "4h",
    100
);

// Add a column series to render the volume bars
sciChartSurface.renderableSeries.add(
    new FastColumnRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, { 
            xValues: dateValues, 
            yValues: volumeValues 
        }),
        yAxisId: "VolumeAxisId",
        strokeThickness: 0,
        dataPointWidth: 0.7,
        opacity: 0.47
    })
);

// continue to add the candlestick series to the chart...
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    xAxes: [{ type: EAxisType.CategoryAxis }],
    yAxes: [
        { 
            type: EAxisType.NumericAxis, 
            options: { labelPrefix: "$", labelPrecision: 2 } 
        },
        {
            type: EAxisType.NumericAxis,
            options: { isVisible: false, id: "VolumeAxisId", growBy: new NumberRange(0, 4) }
        }
    ],
    series: [
        {
            type: ESeriesType.CandlestickSeries,
            ohlcData: {
                xValues: dateValues,
                openValues,
                highValues,
                lowValues,
                closeValues
            },
            options: {
                dataPointWidth: 0.7,
                brushUp: "#33ff3377",
                brushDown: "#ff333377",
                strokeUp: "#77ff77",
                strokeDown: "#ff7777",
                strokeThickness: 1
            }
        },
        {
            type: ESeriesType.ColumnSeries,
            xyData: {
                xValues: dateValues,
                yValues: volumeValues
            },
            options: {
                yAxisId: "VolumeAxisId",
                strokeThickness: 0,
                dataPointWidth: 0.7,
                opacity: 0.47
            }
        }
    ]
});
```

Here's how the example looks now:

## PaintingÂ Candles with Different Colors<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series/#paintingcandles-with-different-colors" class="hash-link" aria-label="Direct link to PaintingÂ Candles with Different Colors" translate="no" title="Direct link to PaintingÂ Candles with Different Colors">â€‹</a>

![](out_scichartv4/2d-charts/chart-types/fast-candlestick-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

It is possible to define the colour of specific OHLC Bars using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

For more info on how to do this, see theÂ [PaletteProvider - Per-point colouring of Candlestick/Ohlc Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series) documentation page.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Start Here - RenderableSeries Overview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)

- [Common RenderableSeries Properties](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/is-visible)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-candlestick-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-candlestick-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
