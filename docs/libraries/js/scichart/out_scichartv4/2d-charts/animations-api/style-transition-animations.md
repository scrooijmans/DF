On this page

# Style Transition Animations

SciChart.js v2.x and above features a new API which allows you to animate style properties on different series types. This is useful if you want to provide feedback on mouse-click such asÂ [data-point selection](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/) orÂ [series selection](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/).

## Style Animation Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations/#style-animation-types" class="hash-link" aria-label="Direct link to Style Animation Types" translate="no" title="Direct link to Style Animation Types">â€‹</a>

Style animations allow changing series styles like color, stroke thickness, point marker size, etc. These differ from series to series so there is a specific type to animate the properties of each series in SciChart.js.

Style animation types per-series are as follows:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineanimation.html" rel="noopener noreferrer" target="_blank">LineAnimationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bandanimation.html" rel="noopener noreferrer" target="_blank">BandAnimationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bubbleanimation.html" rel="noopener noreferrer" target="_blank">BubbleAnimationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcanimation.html" rel="noopener noreferrer" target="_blank">OhlcAnimationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html" rel="noopener noreferrer" target="_blank">ColumnAnimationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mountainanimation.html" rel="noopener noreferrer" target="_blank">MountainAnimationðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html" rel="noopener noreferrer" target="_blank">ScatterAnimationðŸ“˜</a>

## Worked Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations/#worked-examples" class="hash-link" aria-label="Direct link to Worked Examples" translate="no" title="Direct link to Worked Examples">â€‹</a>

### Animating PointMarkers in a Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations/#animating-pointmarkers-in-a-scatter-series" class="hash-link" aria-label="Direct link to Animating PointMarkers in a Scatter Series" translate="no" title="Direct link to Animating PointMarkers in a Scatter Series">â€‹</a>

The following exampleÂ will create a style animation for ellipse point marker:

``` prism-code
// Pointmarker Animation

const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJSLightTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
const scatterSeries = new XyScatterRenderableSeries(wasmContext, {
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 9,
        height: 9,
        strokeThickness: 2,
        fill: "LightSteelBlue",
        stroke: "steelblue"
    })
});
sciChartSurface.renderableSeries.add(scatterSeries);
const dataSeries = new XyDataSeries(wasmContext);
for (let i = 0; i < 20; i++) {
    dataSeries.append(i, Math.sin(i \* 0.5) + 1);
}
scatterSeries.dataSeries = dataSeries;
const pointMarkerAnimation = new ScatterAnimation({
    duration: 3000,
    styles: {
        pointMarker: {
            type: EPointMarkerType.Ellipse,
            width: 40,
            height: 40,
            strokeThickness: 8,
            stroke: "Purple",
            fill: "White"
        }
    }
});
scatterSeries.enqueueAnimation(pointMarkerAnimation);
sciChartSurface.zoomExtents();
```

This results in animating the pointmarker size, stroke and fill on a scatter series:

<img src="out_scichartv4/2d-charts/animations-api/style-transition-animations/index_media/3e7f9e24537533726b92da93d016315435f0b361.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Animations API](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/)
- [Series Startup Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations)
- [Dataset Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/)
- [Generic Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/generic-animations/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/animations-api/style-transition-animations/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/animations-api/style-transition-animations/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
