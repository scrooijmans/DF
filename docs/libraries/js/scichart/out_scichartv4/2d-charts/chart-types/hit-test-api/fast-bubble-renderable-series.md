On this page

# Hit-Test API for Bubble Series

## The hitTest method on Bubble Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series/#the-hittest-method-on-bubble-series" class="hash-link" aria-label="Direct link to The hitTest method on Bubble Series" translate="no" title="Direct link to The hitTest method on Bubble Series">â€‹</a>

The **IHitTestProvider.hitTest** method onÂ [FastBubbleRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series) tests if the click was within the **hitTestRadius** from a bubble circumference.

``` prism-code
// hitTest method on Bubble Series
const hitTestInfo = bubbleSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
```

Â The algorithm is as follows:

1.  Iterate over each of the points to find the nearest one on the XY plane.
2.  Test if the mouse click was within the **hitTestRadius** from a bubble circumference and update **HitTestInfo.isHit** property in the result returned.

This is the full example of the **hitTest** method on the Bubble Series.

- JS
- TS

``` prism-code
import {SciChartSurface, NumericAxis, NumberRange, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint, FastBubbleRenderableSeries, EllipsePointMarker, XyzDataSeries} from "scichart;

export async function hitTestBubbleTs(divId) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
    const HIT_TEST_RADIUS = 10 * DpiHelper.PIXEL_RATIO;
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    const xBubbleValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const yBubbleValues = [0.5, 1.0, 1.8, 2.9, 3.5, 3.0, 2.7, 2.4, 1.7];
    const zBubbleValues = [24, 12, 13, 16, 12, 15, 12, 19, 12];
    const bubbleSeries = new FastBubbleRenderableSeries(wasmContext, {
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 24,
            height: 24,
            fill: "white",
            strokeThickness: 2,
            stroke: "#368BC1"
        }),
        dataSeries: new XyzDataSeries(wasmContext, {
            xValues: xBubbleValues,
            yValues: yBubbleValues,
            zValues: zBubbleValues
        })
    });
    sciChartSurface.renderableSeries.add(bubbleSeries);
    // Add an SVG annotation to display the mouse click
    const svgAnnotation = new CustomAnnotation({
        svgString: `<svg width="8" height="8"><circle cx="50%" cy="50%" r="4" fill="#FF0000"/></svg>`,
        isHidden: true,
        horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
        verticalAnchorPoint: EVerticalAnchorPoint.Center
    });
    sciChartSurface.annotations.add(svgAnnotation);
    sciChartSurface.domCanvas2D.addEventListener("mousedown", mouseEvent => {
        const mouseClickX = mouseEvent.offsetX;
        const mouseClickY = mouseEvent.offsetY;
        console.log("mouseClickX", mouseClickX, "mouseClickY", mouseClickY);
        const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
        const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;
        console.log("premultipliedX", premultipliedX, "premultipliedY", premultipliedY);
        // IHitTestProvider.hitTest
        const hitTestInfo = bubbleSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
        svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
        svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
        svgAnnotation.isHidden = false;
        const resultDiv = document.getElementById("result");
        resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
        console.log("hitTestInfo", hitTestInfo);
    });
}
```

``` prism-code
import {SciChartSurface, NumericAxis, NumberRange, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, EVerticalAnchorPoint, FastBubbleRenderableSeries, EllipsePointMarker, XyzDataSeries} from "scichart;

export async function hitTestBubbleTs(divId: string) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
    const HIT_TEST_RADIUS = 10 * DpiHelper.PIXEL_RATIO;
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    const xBubbleValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const yBubbleValues = [0.5, 1.0, 1.8, 2.9, 3.5, 3.0, 2.7, 2.4, 1.7];
    const zBubbleValues = [24, 12, 13, 16, 12, 15, 12, 19, 12];
    const bubbleSeries = new FastBubbleRenderableSeries(wasmContext, {
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 24,
            height: 24,
            fill: "white",
            strokeThickness: 2,
            stroke: "#368BC1"
        }),
        dataSeries: new XyzDataSeries(wasmContext, {
            xValues: xBubbleValues,
            yValues: yBubbleValues,
            zValues: zBubbleValues
        })
    });
    sciChartSurface.renderableSeries.add(bubbleSeries);
    // Add an SVG annotation to display the mouse click
    const svgAnnotation = new CustomAnnotation({
        svgString: `<svg width="8" height="8"><circle cx="50%" cy="50%" r="4" fill="#FF0000"/></svg>`,
        isHidden: true,
        horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
        verticalAnchorPoint: EVerticalAnchorPoint.Center
    });
    sciChartSurface.annotations.add(svgAnnotation);
    sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent: MouseEvent) => {
        const mouseClickX = mouseEvent.offsetX;
        const mouseClickY = mouseEvent.offsetY;
        console.log("mouseClickX", mouseClickX, "mouseClickY", mouseClickY);
        const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
        const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;
        console.log("premultipliedX", premultipliedX, "premultipliedY", premultipliedY);
        // IHitTestProvider.hitTest
        const hitTestInfo = bubbleSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
        svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
        svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
        svgAnnotation.isHidden = false;
        const resultDiv = document.getElementById("result");
        resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
        console.log("hitTestInfo", hitTestInfo);
    });
}
```

Â If to click inside the bubble it will be hit. In the browser console you will find the **HitTestInfo** with zValue property filled for **XyzDataSeries**.

## The hitTestDataPoint method on Bubble Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series/#the-hittestdatapoint-method-on-bubble-series" class="hash-link" aria-label="Direct link to The hitTestDataPoint method on Bubble Series" translate="no" title="Direct link to The hitTestDataPoint method on Bubble Series">â€‹</a>

The **IHitTestProvider.hitTestDataPoint** method onÂ [FastBubbleRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series) tests if the click was within the **hitTestRadius** from a data point.

``` prism-code
// hitTestDataPoint method on Bubble Series
const hitTestInfo = bubbleSeries.hitTestProvider.hitTestDataPoint(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
```

Â The algorithm is as follows:

1.  Iterate over each of the points to find the nearest one on the XY plane.
2.  Compare distance to the nearest point with the **hitTestRadius** and update HitTestInfo.isHit property accordingly.

## The hitTestXSlice method on Bubble Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series/#the-hittestxslice-method-on-bubble-series" class="hash-link" aria-label="Direct link to The hitTestXSlice method on Bubble Series" translate="no" title="Direct link to The hitTestXSlice method on Bubble Series">â€‹</a>

The **IHitTestProvider.hitTestXSlice** method is used forÂ [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) andÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) to get information about the nearest point.

``` prism-code
// hitTestXSlice method on Bubble Series
const hitTestInfo = bubbleSeries.hitTestProvider.hitTestXSlice(premultipliedX, premultipliedY);
```

Â The way it works:

1.  Finds the nearest point in X direction.
2.  Sets **HitTestInfo.isHit** property to **True** if the mouse click was within the data bounds.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Hit-Test API for Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series)
- [Hit-Test API for Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series)
- [Hit-Test API for Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series)
- [Hit-Test API for Heatmap Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series)
- [Hit-Test API for Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-rectangle-renderable-series)
- [Hit-Test API for Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
