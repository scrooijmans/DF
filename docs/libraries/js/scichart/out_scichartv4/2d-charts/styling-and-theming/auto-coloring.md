On this page

# Chart Styling - Auto Coloring

One of the challenges when creating a chart with multiple series is picking colours for each series, especially when you are trying to keep to a color scheme.Â  We faced this problem during the recent rethemeing of our demos to match the new website theme.Â  The solution was to create a way to let SciChart automatically pick colours based off a palette.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/chart-websocket-bigdata-demo" target="_blank">Client/Server Websocket Data Streaming</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Setting up Auto Coloring<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/auto-coloring/#setting-up-auto-coloring" class="hash-link" aria-label="Direct link to Setting up Auto Coloring" translate="no" title="Direct link to Setting up Auto Coloring">â€‹</a>

In order to use auto coloring you need to set up a strokePalette and/or fillPalette onÂ a theme and apply it to a surface.Â  You can then set a stroke or fill color as AUTO_COLOR and those items will pick colors based on the palette.Â  stroke properties use strokePalette and fill properties use fillPalette.Â Â 

The color is picked based on the number of series currently on the chart.Â  If the number of series is less than or equal to the number of entries in the palette, then scichart will pick values directly from the palette.Â  If there are more series than palette entries, scichart will generate a gradient using the palette and then pick colours evenly spaced on that gradient.

In the example below, the stroke and fill palettes are reversed.

``` prism-code
// Auto coloring
import {
  SciChartSurface,
  NumericAxis,
  SciChartJSLightTheme,
  FastLineRenderableSeries,
  AUTO_COLOR,
  XyDataSeries,
  EllipsePointMarker,
  NumberRange
} from "scichart";

export async function autoColoring(divId) {
    const theme = new SciChartJSLightTheme();
    // configure the palette on the theme
    theme.strokePalette = ["red", "yellow", "green", "blue"];
    theme.fillPalette = ["blue", "green", "yellow", "red"];
    // Create a sciChartSurface using the theme
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ theme });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1)}));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1)}));
    // common x values - 0 to 20
    const xValues = Array.from(Array(20)).map((_, i) => i);
    // Create 10 line series with pointmarkers
    for (let i = 0; i < 10; i++) {
        const series = new FastLineRenderableSeries(wasmContext, {
            stroke: AUTO_COLOR,
            dataSeries: new XyDataSeries(wasmContext, { xValues, yValues: xValues.map(x => Math.sin(x/2) + i)}),
            pointMarker: new EllipsePointMarker(wasmContext, {
                stroke: AUTO_COLOR,
                fill: AUTO_COLOR,
                width: 10,
                height: 10
            })
        });
        sciChartSurface.renderableSeries.add(series);
    }
}
```

<img src="out_scichartv4/2d-charts/styling-and-theming/auto-coloring/index_media/5c2cd33c50b0a36027fbf77fc51657c5001ac832.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## OverridingÂ Auto Coloring<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/auto-coloring/#overridingauto-coloring" class="hash-link" aria-label="Direct link to OverridingÂ Auto Coloring" translate="no" title="Direct link to OverridingÂ Auto Coloring">â€‹</a>

If you need to adjust the color that has been picked, you can override the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#adjustautocolor" rel="noopener noreferrer" target="_blank">adjustAutoColorðŸ“˜</a> method which exists on renerableSeries andÂ pointMarker.Â DataLabels also support AUTO_COLOR but for overrides there use getColor instead. SeeÂ Â [DataLabel Coloring](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-coloring/).

adjustAutoColor receives a propertyName which is either "stroke" or "fill" along with the color generated.Â  If we add the following code into the loop in the above example

``` prism-code
// Adjust Auto coloring

if (i === 4) {
    series.pointMarker.adjustAutoColor = (propertyName, color) => {
        return propertyName === "fill" ? "black" : color;
    };
}
```

Â we get this

<img src="out_scichartv4/2d-charts/styling-and-theming/auto-coloring/index_media/e55941eb605e265ac1a16c8b28fa9f279872996f.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Using adjustAutoColorÂ like this sets the color property so it no longer auto-colors.Â  If instead you return AUTO_COLOR + "black", then it will use black for the current render, but could be updated by subsequent auto-coloring.Â 

## Controlling When Auto Coloring Occurs<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/auto-coloring/#controlling-when-auto-coloring-occurs" class="hash-link" aria-label="Direct link to Controlling When Auto Coloring Occurs" translate="no" title="Direct link to Controlling When Auto Coloring Occurs">â€‹</a>

By default, colors are assigned to all series on the chart whenever a series is added or removed.Â  If we remove the first three series after the chart has drawn, like this:

``` prism-code
// Remove series

setTimeout(() => {
    sciChartSurface.renderableSeries.removeAt(0);
    sciChartSurface.renderableSeries.removeAt(0);
    sciChartSurface.renderableSeries.removeAt(0);
}, 1000);
```

Â Then all the series will be re-coloured using the full range of the palette.Â  Note the black fill has not changed.

<img src="out_scichartv4/2d-charts/styling-and-theming/auto-coloring/index_media/13a77bbc0fcef0272ecf6f491fb1bb49ddbcedc2.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

SciChartSurface has anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#autocolormode" rel="noopener noreferrer" target="_blank">autoColorModeðŸ“˜</a> property which is anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eautocolormode.html" rel="noopener noreferrer" target="_blank">EAutoColorModeðŸ“˜</a>.Â  This defaults to OnAddRemoveSeries but can also be Never, Once or Always.Â  With a fairly large number of series, Auto ColoringÂ can potentially have a performance impact.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/auto-coloring/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/auto-coloring/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
