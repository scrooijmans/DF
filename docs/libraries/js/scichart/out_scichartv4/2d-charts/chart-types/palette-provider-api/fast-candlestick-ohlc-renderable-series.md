On this page

# Per-Point Colouring of Candlestick / OHLC Charts

Candlestick series can be colored per-point or per line-segment using theÂ [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview). To use this, we must create a class (typescript) or object (javascript)Â which implements or confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> interfaces. Then, apply this to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">FastCandlestickRenderableSeries.paletteProviderðŸ“˜</a> property. This allows you to colour data-points based on values, or custom rules with infinite extensiblity.

First let's create the PaletteProvider class.

- TS

``` prism-code
const { DefaultPaletteProvider, parseColorToUIntArgb } = SciChart;

// Create a class which inherits DefaultPaletteProvider
class CandlePaletteProvider extends DefaultPaletteProvider {
    highlightColor: number;
    constructor() {
        super();
        this.highlightColor = parseColorToUIntArgb("#FEFEFE");
    }

    // Override onAttached to get the parent FastCandlestickRenderableSeries
    onAttached(parentSeries) {
        this.parentSeries = parentSeries;
    }

    // This function is called for every data-point
    // Return undefined to use the default color for the fill, else, return
    // a custom color as ARGB color code e.g. 0xFFFF0000 is red
    overrideFillArgb(xValue, yValue, index, opacity, metadata) {
        return this.overrideColorAt(index, this.parentSeries.dataSeries);
    }
    overrideStrokeArgb(xValue, yValue, index, opacity, metadata) {
        return this.overrideColorAt(index, this.parentSeries.dataSeries);
    }

    overrideColorAt(index, ohlcDataSeries) {
        // Get the open, close values
        const close = ohlcDataSeries.getNativeCloseValues().get(index);
        const open = ohlcDataSeries.getNativeOpenValues().get(index);
        // If more than 1% change, return 'highlightColor' otherwise return undefined for default color
        if (Math.abs(1 - open / close) > 0.01) {
            return this.highlightColor;
        }
        return undefined;
    }
}
```

Next, apply the PaletteProvider to the Candlestick series.Â 

- TS
- Builder API (JSON Config)

``` prism-code
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
    strokeDown: "#ff7777",
    // Attach a paletteprovider here. Candles with > 1% range will be highlighted
    paletteProvider: new CandlePaletteProvider()
});
sciChartSurface.renderableSeries.add(candlestickSeries);
```

``` prism-code
// Register the custom CandlestickPaletteProvider with the chartBuilder
chartBuilder.registerType(EBaseType.PaletteProvider, "CandlePaletteProvider", () => new CandlePaletteProvider());

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: [{ type: EAxisType.CategoryAxis }],
    yAxes: [
        { type: EAxisType.NumericAxis, options: { labelPrefix: "$", labelPrecision: 2 } },
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
                strokeThickness: 1,
                // Apply the PaletteProvider to the candlestick series
                paletteProvider: {
                    type: EPaletteProviderType.Custom,
                    customType: "CandlePaletteProvider"
                }
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

The code above results in the following output.Â 

When the open - close is greater than a 1% range, the candle fillÂ is coloured white.

Using this or similar logic you can add an extra-dimension of data toÂ <a href="https://www.scichart.com/demo/javascript-candlestick-chart" rel="noopener noreferrer" target="_blank">JavaScript Candlestick charts</a>.

## Applying PaletteProviders to OHLC Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series/#applying-paletteproviders-to-ohlc-series" class="hash-link" aria-label="Direct link to Applying PaletteProviders to OHLC Series" translate="no" title="Direct link to Applying PaletteProviders to OHLC Series">â€‹</a>

The same technique can be applied to OHLC Bars. Just make sure when creating a class that inheritsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimenumericaxis.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProvideðŸ“˜</a> that theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html#overridestrokeargb" rel="noopener noreferrer" target="_blank">overrideStrokeArgbðŸ“˜</a> function is defined. As usual, return a color in ARGB format to override this point stroke, else, return undefined for default stroke.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
