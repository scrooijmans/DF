On this page

# Hit-Test API for Band Series

## The hitTest method on Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/#the-hittest-method-on-band-series" class="hash-link" aria-label="Direct link to The hitTest method on Band Series" translate="no" title="Direct link to The hitTest method on Band Series">â€‹</a>

The **IHitTestProvider.hitTest** method onÂ [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series) tests if the click was within the band body.

**hitTest method on Band Series**

``` prism-code
const hitTestInfo = bandSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
```

The algorithm is as follows:

1.  Find the nearest data point in X direction.
2.  Test if the click was within the band body and update **HitTestInfo.isHit** property.

This is the full example of the **hitTest** method on Band Series.

- TS

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisAlignment: EAxisAlignment.Top }));
sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, { axisAlignment: EAxisAlignment.Right, growBy: new NumberRange(0.4, 0.4) })
);
const dataSeries = new XyyDataSeries(wasmContext);
const POINTS = 1000;
const STEP = (3 * Math.PI) / POINTS;
for (let i = 0; i <= 1000; i++) {
    const k = 1 - i / 2000;
    dataSeries.append(i, Math.sin(i * STEP) * k * 0.7, Math.cos(i * STEP) * k);
}
const bandSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries,
    strokeThickness: 2
});
sciChartSurface.renderableSeries.add(bandSeries);
bandSeries.fill = "#279B2733";
bandSeries.fillY1 = "#FF191933";
bandSeries.stroke = "#FF1919FF";
bandSeries.strokeY1 = "#279B27FF";
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
    const hitTestInfo = bandSeries.hitTestProvider.hitTest(premultipliedX, premultipliedY);
    svgAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
    svgAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
    svgAnnotation.isHidden = false;
    const resultDiv = document.getElementById("result");
    resultDiv.innerText = `isHit = ${hitTestInfo.isHit}`;
    console.log("hitTestInfo", hitTestInfo);
});
```

Â This gives us the chart below.

<img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/index_media/2547641d21d0bc484a3021cf719aa43f4cb2852a.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

If we click inside the band it will be hit. In the browser console you will find output for the **HitTestInfo** object containing **y1Value** and **y1Coord** properties specific for the series.

## The hitTestDataPoint method on Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/#the-hittestdatapoint-method-on-band-series" class="hash-link" aria-label="Direct link to The hitTestDataPoint method on Band Series" translate="no" title="Direct link to The hitTestDataPoint method on Band Series">â€‹</a>

The **IHitTestProvider.hitTestDataPoint** method onÂ [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series) tests if the click was within the **hitTestRadius** from XY or XY1 data point.

**hitTestDataPoint method on Band Series**

``` prism-code
const hitTestInfo = bandSeries.hitTestProvider.hitTestDataPoint(premultipliedX, premultipliedY, HIT_TEST_RADIUS);
```

Â The algorithm is as follows:

1.  Iterate over each of XY and XY1 points to find the nearest one on the plane.
2.  Compare distance to XY and XY1 points with the **hitTestRadius** and update **HitTestInfo.isHit** property accordingly.

## The hitTestXSlice method on Band Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/#the-hittestxslice-method-on-band-series" class="hash-link" aria-label="Direct link to The hitTestXSlice method on Band Series" translate="no" title="Direct link to The hitTestXSlice method on Band Series">â€‹</a>

The **IHitTestProvider.hitTestXSlice** method is used forÂ [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) andÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) to get information about the nearest point.

**hitTestXSlice method on Band Series**

``` prism-code
const hitTestInfo = bandSeries.hitTestProvider.hitTestXSlice(premultipliedX, premultipliedY);
```

Â The way it works:

1.  Finds the nearest point in X direction.
2.  Sets **HitTestInfo.isHit** property to **True** if the mouse click was within the data bounds.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Hit-Test API for Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-line-renderable-series)
- [Hit-Test API for Bubble Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-bubble-renderable-series)
- [Hit-Test API for Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-column-renderable-series)
- [Hit-Test API for Heatmap Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/uniform-heatmap-renderable-series)
- [Hit-Test API for Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/fast-rectangle-renderable-series)
- [Hit-Test API for Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/fast-band-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
