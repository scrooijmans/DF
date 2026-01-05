On this page

# Hit-Test API for Line Series

## The hitTest method on Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/#the-hittest-method-on-line-series" class="hash-link" aria-label="Direct link to The hitTest method on Line Series" translate="no" title="Direct link to The hitTest method on Line Series">â€‹</a>

The **IHitTestProvider.hitTest** method onÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series) tests if the click was within the **hitTestRadius** from the line. The algorithm differs for sorted and unsorted data.

``` prism-code
// hitTest method on LineSeries
const hitTestInfo = lineSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
```

For **sorted data** the algorithm is as follows:

1.  Find the nearest point in X direction.
2.  Take the second adjacent point which is on the other side from the hit-test point.
3.  Calculate the distance from the hit-test point to the line formed by two points above and update HitTestInfo.isHit property.

For **unsorted data**:

1.  Iterate over each of the line segments and find the index with minimal distance to the hit-test point.
2.  Compare this distance with the **hitTestRadius** and update **HitTestInfo.isHit** property.

## The hitTestDataPoint method on Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/#the-hittestdatapoint-method-on-line-series" class="hash-link" aria-label="Direct link to The hitTestDataPoint method on Line Series" translate="no" title="Direct link to The hitTestDataPoint method on Line Series">â€‹</a>

The **IHitTestProvider.hitTestDataPoint** method onÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series) tests if the click was within the **hitTestRadius** from a data point.

``` prism-code
// hitTestDataPoint method on Line Series
const hitTestInfo = lineSeries.hitTestProvider.hitTestDataPoint(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
```

Â The algorithm is as follows:

1.  Iterate over each of the points to find the nearest one on the XY plane.

2.  Compare distance to the nearest point with the **hitTestRadius** and update **HitTestInfo.isHit** property.

This is an example of the **hitTestDataPoint** method usage.

- JS
- TS

``` prism-code
import {SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries, NumberRange, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint } from "scichart";

export async function hitTestTs(divId) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
    const HIT_TEST_RADIUS = 10 * DpiHelper.PIXEL_RATIO;
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    const xLineValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const yLineValues = [0, 0.5, 1.3, 2.4, 3, 2.5, 2.2, 1.9, 1.2];
    const lineSeries = new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 3,
        dataSeries: new XyDataSeries(wasmContext, { xValues: xLineValues, yValues: yLineValues })
    });
    sciChartSurface.renderableSeries.add(lineSeries);
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
        console.log("mouseClickX", mouseClickX, "mouseClickY", mouseClickY);
        const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
        const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;
        console.log('premultipliedX', premultipliedX, 'premultipliedY', premultipliedY);
        // IHitTestProvider.hitTestDataPoint
        const hitTestInfo = lineSeries.hitTestProvider.hitTestDataPoint(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
        svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
        svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
        svgAnnotation.isHidden = false;
        const resultDiv = document.getElementById("result");
        resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
        console.log('hitTestInfo', hitTestInfo);
    });
}
```

``` prism-code
import {SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries, NumberRange, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint } from "scichart";

export async function hitTestTs(divId: string) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
    const HIT_TEST_RADIUS = 10 * DpiHelper.PIXEL_RATIO;
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    const xLineValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const yLineValues = [0, 0.5, 1.3, 2.4, 3, 2.5, 2.2, 1.9, 1.2];
    const lineSeries = new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 3,
        dataSeries: new XyDataSeries(wasmContext, { xValues: xLineValues, yValues: yLineValues })
    });
    sciChartSurface.renderableSeries.add(lineSeries);
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
        console.log("mouseClickX", mouseClickX, "mouseClickY", mouseClickY);
        const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
        const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;
        console.log('premultipliedX', premultipliedX, 'premultipliedY', premultipliedY);
        // IHitTestProvider.hitTestDataPoint
        const hitTestInfo = lineSeries.hitTestProvider.hitTestDataPoint(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
        svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
        svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
        svgAnnotation.isHidden = false;
        const resultDiv = document.getElementById("result");
        resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
        console.log('hitTestInfo', hitTestInfo);
    });
}
```

If we run the example we get this chart.Â 

<img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/index_media/e46c2177695318c86215718a1a1ee1d13271a5d3.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Clicking near a data point results is **isHit=true**.Â Â In the browser console you will find output for the **HitTestInfo** object.

## Â The hitTestXSlice method on Line Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/#the-hittestxslice-method-on-line-series" class="hash-link" aria-label="Direct link to Â The hitTestXSlice method on Line Series" translate="no" title="Direct link to Â The hitTestXSlice method on Line Series">â€‹</a>

The **IHitTestProvider.hitTestXSlice** method is used forÂ [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) andÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) to get information about the nearest point.

``` prism-code
// hitTestXSlice method on Line Series
const hitTestInfo = lineSeries.hitTestProvider.hitTestXSlice(premultipliedX, premultipliedY);
```

Â The way it works:

1.  Finds the nearest point in X direction.
2.  Sets **HitTestInfo.isHit** property to True if the mouse click was within the data bounds.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Hit-Test API for Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series)
- [Hit-Test API for Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series)
- [Hit-Test API for Heatmap Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series)
- [Hit-Test API for Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-rectangle-renderable-series)
- [Hit-Test API for Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
