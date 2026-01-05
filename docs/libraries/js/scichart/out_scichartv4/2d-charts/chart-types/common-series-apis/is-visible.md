On this page

# Series isVisible and isVisibleChanged API

Each RenderableSeries has anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#isvisible" rel="noopener noreferrer" target="_blank">isVisibleðŸ“˜</a> property. This defines whether the series is included in rendering or not.

isVisible can be set programmatically, or is also set by SciChart.js when checking or unchecking a Legend row checkbox (seeÂ [LegendModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/miscellaneous-modifiers/legend-modifier)).

You can listen to isVisible changes via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#isvisiblechanged" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.isVisibleChangedðŸ“˜</a> event. Listen to the event (get a callback) using the following code:

- TS

``` prism-code
// Create and add a series with onIsVisibleChanged handler
const scatterSeries = new XyScatterRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues,
        dataSeriesName: "Scatter Series"
    }),
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 7,
        height: 7,
        strokeThickness: 2,
        fill: "steelblue",
        stroke: "LightSteelBlue"
    }),
    onIsVisibleChanged: (sourceSeries, isVisible) => {
        console.log(`Series ${sourceSeries.type} was set to isVisible=${isVisible}`);
    }
});

// You can also subscribe to isVisibleChanged like this
scatterSeries.isVisibleChanged.subscribe(seriesVisibleChangedArgs => {
    // See SeriesVisibleChangedArgs in typedoc
    const renderableSeries = seriesVisibleChangedArgs.sourceSeries;
    const isVisible = seriesVisibleChangedArgs.isVisible;

    console.log(`isVisibleChanged handler: Series ${renderableSeries.type} was set to isVisible=${isVisible}`);
    textAnnotation.text = `${renderableSeries.dataSeries.dataSeriesName} is ${isVisible ? "visible" : "hidden"}`;
});

// Explicitly set visibility like this
scatterSeries.isVisible = true;

sciChartSurface.renderableSeries.add(scatterSeries);
```

This can be used to get feedback about the current visibility state of a series, as in the following demo:

![](out_scichartv4/2d-charts/chart-types/common-series-apis/is-visible/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

See the onIsVisibleChanged parameter inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseriesoptions.html#onisvisiblechanged" rel="noopener noreferrer" target="_blank">IBaseRenderableSeriesOptions.onIsVisibleChangedðŸ“˜</a> for type information.

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#isvisiblechanged" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.isVisibleChangedðŸ“˜</a> event handler also has args of typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesvisiblechangedargs.html" rel="noopener noreferrer" target="_blank">SeriesVisibleChangedArgsðŸ“˜</a>. In TypeScript, the code would look like this:

**Typescript isVisibleChanged**

``` prism-code
series.isVisibleChanged.subscribe((args: SeriesVisibleChangedArgs) => {
       console.log(`isVisibleChanged handler: Series ${args.sourceSeries.type} was set to isVisible=${args.isVisible}`);
  });
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/is-visible/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseriesoptions.html#onisvisiblechanged" rel="noopener noreferrer" target="_blank">onIsVisibleChanged optionðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html#isvisiblechanged" rel="noopener noreferrer" target="_blank">RenderableSeries.isVisibleChangedðŸ“˜</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/common-series-apis/is-visible/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/common-series-apis/is-visible/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
