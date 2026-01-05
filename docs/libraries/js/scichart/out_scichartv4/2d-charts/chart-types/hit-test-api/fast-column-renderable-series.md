On this page

# Hit-Test API for Column Series

## The hitTest method on Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/#the-hittest-method-on-column-series" class="hash-link" aria-label="Direct link to The hitTest method on Column Series" translate="no" title="Direct link to The hitTest method on Column Series">â€‹</a>

The **IHitTestProvider.hitTest** method onÂ [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series) tests if the click was within the Column.

``` prism-code
// hitTest method on Column Series
const hitTestInfo = columnSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
```

Â The algorithm is as follows:

1.  Find the nearest column in X direction.
2.  Test if the click was within column body and update **HitTestInfo.isHit** property.

This is the full example of the **hitTest** method on Column Series.

- JS
- TS

``` prism-code
import {SciChartSurface, NumericAxis, NumberRange, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, XyDataSeries, FastColumnRenderableSeries} from "scichart";

export async function hitTestColumnTs(divId) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    // Column series
    const xColumnValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const yColumnValues = [0, 0.2, 1, 2.0, 2.5, 1.9, 1.9, 1.5, 1.2];
    const columnSeries = new FastColumnRenderableSeries(wasmContext, {
        fill: 'rgba(255,255,255,0.9)',
        dataPointWidth: 0.5,
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: xColumnValues,
            yValues: yColumnValues
        })
    });
    sciChartSurface.renderableSeries.add(columnSeries);
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
        const hitTestInfo = columnSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
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
import {SciChartSurface, NumericAxis, NumberRange, DpiHelper, CustomAnnotation, EHorizontalAnchorPoint, XyDataSeries, FastColumnRenderableSeries} from "scichart";

export async function hitTestColumnTs(divId: string) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) }));
    // Column series
    const xColumnValues = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    const yColumnValues = [0, 0.2, 1, 2.0, 2.5, 1.9, 1.9, 1.5, 1.2];
    const columnSeries = new FastColumnRenderableSeries(wasmContext, {
        fill: 'rgba(255,255,255,0.9)',
        dataPointWidth: 0.5,
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: xColumnValues,
            yValues: yColumnValues
        })
    });
    sciChartSurface.renderableSeries.add(columnSeries);
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
        const hitTestInfo = columnSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
        svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
        svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
        svgAnnotation.isHidden = false;
        const resultDiv = document.getElementById('result');
        resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
        console.log('hitTestInfo', hitTestInfo);
    });
}
```

Â This gives us the chart below.

<img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/index_media/eff82058420b4951f8e8be873d09fd0875621464.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Â If to click inside the column it will be hit. In the browser console you will find output for the **HitTestInfo** object.

## The hitTestDataPoint method on Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/#the-hittestdatapoint-method-on-column-series" class="hash-link" aria-label="Direct link to The hitTestDataPoint method on Column Series" translate="no" title="Direct link to The hitTestDataPoint method on Column Series">â€‹</a>

The **IHitTestProvider.hitTestDataPoint** method onÂ [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type) tests if the click was within the **hitTestRadius** from a data point.

``` prism-code
// hitTestDataPoint method on Column Series
const hitTestInfo = columnSeries.hitTestProvider.hitTestDataPoint(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
```

Â The algorithm is as follows:

1.  Iterate over each of the points to find the nearest one on the XY plane.
2.  Compare distance to the point with the **hitTestRadius** and update **HitTestInfo.isHit** property accordingly.

## Â The hitTestXSlice method on Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/#the-hittestxslice-method-on-column-series" class="hash-link" aria-label="Direct link to Â The hitTestXSlice method on Column Series" translate="no" title="Direct link to Â The hitTestXSlice method on Column Series">â€‹</a>

The **IHitTestProvider.hitTestXSlice** method is used forÂ [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) andÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) to get information about the nearest point.

``` prism-code
// hitTestXSlice method on Column Series
const hitTestInfo = columnSeries.hitTestProvider.hitTestXSlice(premultipliedX, premultipliedY);
```

The way it works:

1.  Finds the nearest point in X direction.
2.  Sets **HitTestInfo.isHit** property to **True** if the mouse click was within the data bounds.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Hit-Test API for Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series)
- [Hit-Test API for Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series)
- [Hit-Test API for Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series)
- [Hit-Test API for Heatmap Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series)
- [Hit-Test API for Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-rectangle-renderable-series)
- [Hit-Test API for Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
