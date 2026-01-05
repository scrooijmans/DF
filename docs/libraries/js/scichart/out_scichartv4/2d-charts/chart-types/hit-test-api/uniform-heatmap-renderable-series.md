On this page

# Hit-Test API for Heatmap Series

## The hitTest method on Heatmap Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series/#the-hittest-method-on-heatmap-series" class="hash-link" aria-label="Direct link to The hitTest method on Heatmap Series" translate="no" title="Direct link to The hitTest method on Heatmap Series">â€‹</a>

The **IHitTestProvider.hitTest** method onÂ [UniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type) tests if the click was within the Heatmap.

``` prism-code
// hitTest method on Heatmap Series
const hitTestInfo = heatmapSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
```

Â The algorithm is as follows:

1.  Find X and Y index of the tile being clicked.
2.  Test if the click was within the Heatmap surface and update **HitTestInfo.isHit** property.

This is the full example of the **hitTest** method on Heatmap Series.

- JS
- TS

``` prism-code
import {SciChartSurface, NumericAxis, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint, EAxisAlignment, HeatmapColorMap, HeatmapColorMap, UniformHeatmapDataSeries, UniformHeatmapRenderableSeries, NumberRange } from "scichart";

export async function hitTestHeatmapTs(divId) {
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, { axisTitle: 'Heatmap X', growBy: new NumberRange(0.05, 0.05) })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: 'Heatmap Y',
        axisAlignment: EAxisAlignment.Left,
        growBy: new NumberRange(0.05, 0.05)
    })
);
const gradientStops = [
    { offset: 0, color: 'yellow' },
    { offset: 1, color: 'red' }
];
const colorMap = new HeatmapColorMap({
    minimum: 0,
    maximum: 3,
    gradientStops
});
const initialZValues = [
    [3, 0, 1, 2],
    [0, 1, 2, 3]
];
const dataSeries = new UniformHeatmapDataSeries(wasmContext, {
    xStart: 0,
    xStep: 1,
    yStart: 3,
    yStep: 1,
    zValues: initialZValues
});
dataSeries.hasNaNs = true;
const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
    opacity: 0.5,
    dataSeries,
    colorMap
});
sciChartSurface.renderableSeries.add(heatmapSeries);
// Add an SVG annotation to display the mouse click
const svgAnnotation = new CustomAnnotation({
    svgString: `<svg width="8" height="8"><circle cx="50%" cy="50%" r="4" fill="#FF0000"/></svg>`,
    isHidden: true,
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center
});
sciChartSurface.annotations.add(svgAnnotation);
sciChartSurface.domCanvas2D.addEventListener('mousedown', (mouseEvent) => {
    const mouseClickX = mouseEvent.offsetX;
    const mouseClickY = mouseEvent.offsetY;
    console.log('mouseClickX', mouseClickX, 'mouseClickY', mouseClickY);
    const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
    const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;
    console.log('premultipliedX', premultipliedX, 'premultipliedY', premultipliedY);
    // IHitTestProvider.hitTest
    const hitTestInfo = heatmapSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
    svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
    svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
    svgAnnotation.isHidden = false;
    const resultDiv = document.getElementById('result');
    resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
    console.log('hitTestInfo', hitTestInfo);
});
}
```

``` prism-code
import {SciChartSurface, NumericAxis, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint, EAxisAlignment, HeatmapColorMap, HeatmapColorMap, UniformHeatmapDataSeries, UniformHeatmapRenderableSeries, NumberRange } from "scichart";

export async function hitTestHeatmapTs(divId: string) {
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, { axisTitle: 'Heatmap X', growBy: new NumberRange(0.05, 0.05) })
);
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        axisTitle: 'Heatmap Y',
        axisAlignment: EAxisAlignment.Left,
        growBy: new NumberRange(0.05, 0.05)
    })
);
const gradientStops = [
    { offset: 0, color: 'yellow' },
    { offset: 1, color: 'red' }
];
const colorMap = new HeatmapColorMap({
    minimum: 0,
    maximum: 3,
    gradientStops
});
const initialZValues = [
    [3, 0, 1, 2],
    [0, 1, 2, 3]
];
const dataSeries = new UniformHeatmapDataSeries(wasmContext, {
    xStart: 0,
    xStep: 1,
    yStart: 3,
    yStep: 1,
    zValues: initialZValues
});
dataSeries.hasNaNs = true;
const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
    opacity: 0.5,
    dataSeries,
    colorMap
});
sciChartSurface.renderableSeries.add(heatmapSeries);
// Add an SVG annotation to display the mouse click
const svgAnnotation = new CustomAnnotation({
    svgString: `<svg width="8" height="8"><circle cx="50%" cy="50%" r="4" fill="#FF0000"/></svg>`,
    isHidden: true,
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center
});
sciChartSurface.annotations.add(svgAnnotation);
sciChartSurface.domCanvas2D.addEventListener('mousedown', (mouseEvent: MouseEvent) => {
    const mouseClickX = mouseEvent.offsetX;
    const mouseClickY = mouseEvent.offsetY;
    console.log('mouseClickX', mouseClickX, 'mouseClickY', mouseClickY);
    const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
    const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;
    console.log('premultipliedX', premultipliedX, 'premultipliedY', premultipliedY);
    // IHitTestProvider.hitTest
    const hitTestInfo = heatmapSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
    svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
    svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
    svgAnnotation.isHidden = false;
    const resultDiv = document.getElementById('result');
    resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
    console.log('hitTestInfo', hitTestInfo);
});
}
```

Â If to click inside the Heatmap surface it will be hit. In the browser console you will find output for the **HitTestInfo** object containing **heatmapValue**, **heatmapXIndex** and **heatmapYIndex** properties which are only for theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a>.

## The hitTestDataPoint method on Heatmap Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series/#the-hittestdatapoint-method-on-heatmap-series" class="hash-link" aria-label="Direct link to The hitTestDataPoint method on Heatmap Series" translate="no" title="Direct link to The hitTestDataPoint method on Heatmap Series">â€‹</a>

The **IHitTestProvider.hitTestDataPoint** method is not supported forÂ [UniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type).

## The hitTestXSlice method on Heatmap Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series/#the-hittestxslice-method-on-heatmap-series" class="hash-link" aria-label="Direct link to The hitTestXSlice method on Heatmap Series" translate="no" title="Direct link to The hitTestXSlice method on Heatmap Series">â€‹</a>

The **IHitTestProvider.hitTestXSlice** method works identically as the **IHitTestProvider.hitTest**.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Hit-Test API for Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series)
- [Hit-Test API for Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series)
- [Hit-Test API for Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series)
- [Hit-Test API for Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series)
- [Hit-Test API for Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-rectangle-renderable-series)
- [Hit-Test API for Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
