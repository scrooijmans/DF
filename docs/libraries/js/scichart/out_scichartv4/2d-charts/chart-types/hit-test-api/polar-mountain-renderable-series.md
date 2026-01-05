On this page

# Hit-Test API for Polar Mountain Series

The IHitTestProvider.hitTest method on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmountainrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarMountainRenderableSeriesðŸ“˜</a> tests if the click was within the mountain body, and returns a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" rel="noopener noreferrer" target="_blank">HitTestInfoðŸ“˜</a> object with the following properties:

## Hit-Test on a particular Polar Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-mountain-renderable-series/#hit-test-on-a-particular-polar-mountain-series" class="hash-link" aria-label="Direct link to Hit-Test on a particular Polar Mountain Series" translate="no" title="Direct link to Hit-Test on a particular Polar Mountain Series">â€‹</a>

Calling the `hitTest` method on one specific series can be done this way:

``` prism-code
const x = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
const y = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;

const hitTestInfo: HitTestInfo = polarMountainSeries.hitTestProvider.hitTest(x, y);
```

The algorithm is as follows:

1.  Finds two nearest points in x direction that the x-hit value falls between them.
2.  Tests if the click is within the triangle formed by two nearest points and the center of polar surface <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html#isHit" rel="noopener noreferrer" target="_blank">HitTestInfo.isHitðŸ“˜</a> property.

## Hit-Test on multiple Polar Mountain Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-mountain-renderable-series/#hit-test-on-multiple-polar-mountain-series" class="hash-link" aria-label="Direct link to Hit-Test on multiple Polar Mountain Series" translate="no" title="Direct link to Hit-Test on multiple Polar Mountain Series">â€‹</a>

First, we add two renderable series we will hit-test on:

``` prism-code
// add a couple of polar mountains to the chart
const polarMountain1 = new PolarMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7],
        yValues: [2.5, 1.8, 3.0, 1.4, 2.0, 1.75, 2.4, 1.5],
        dataSeriesName: "Red Mountain"
    }),
    stroke: "white",
    fill: "#883333",
});
const polarMountain2 = new PolarMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [8, 9, 10, 11, 12, 13, 14, 15],
        yValues: [4.5, 3.2, 5.1, 2.8, 4.0, 3.5, 4.8, 3.0],
        dataSeriesName: "Blue Mountain"
    }),
    stroke: "black",
    fill: "#3333AA",
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        strokeThickness: 2,
        stroke: "SteelBlue",
        fill: "LightSteelBlue"
    }),
});
sciChartSurface.renderableSeries.add(polarMountain1, polarMountain2);
```

Second, we add an event-listener on the "mousedown" event:

``` prism-code
// Add an event listener for mouse down events
sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent: MouseEvent) => {
    // Use our DpiHelper class to multiply coordinates, else screens with non-100% scaling will not work very well
    const x = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
    const y = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;

    // optional - flag to stop checking for hit-test on other series once a hit is found
    let wasTheHitSuccessfulAtLeastOnce = false; 

    [...sciChartSurface.renderableSeries.asArray()] // copy the renderable series to an array
        .reverse() // reverse array -> The 2nd series will be drawn on top of the 1st series, users likely want to hit-test the topmost series first (valueable for overlapping series)
        .forEach(rs => {
        console.log(`Trying hit test on: ${rs.getDataSeriesName()}`);
        
        if (rs.hitTestProvider && mouseEvent && !wasTheHitSuccessfulAtLeastOnce) {
            const hitTestInfo = rs.hitTestProvider.hitTest(x, y, HIT_TEST_RADIUS);

            dotAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
            dotAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
            dotAnnotation.isHidden = false;

            if (hitTestInfo.isHit) { 
                // Successful Hit
                dotAnnotation.svgString = SUCCESSFUL_HIT_SVG;
                textAnnotation.text = `Hit (x: ${hitTestInfo.hitTestPointValues.x.toFixed(2)}, y: ${hitTestInfo.hitTestPointValues.y.toFixed(2)})\n`
                    + `- series: "${hitTestInfo.associatedSeries.getDataSeriesName()}"\n`
                    + `- mountain index: ${hitTestInfo.dataSeriesIndex}\n`
                    + `- yValue: ${hitTestInfo.yValue}`;
                wasTheHitSuccessfulAtLeastOnce = true;
            } else { 
                // No Hit
                dotAnnotation.svgString = NO_HIT_SVG;
                textAnnotation.text = `No hit detected\n`;
            }
        }
    });
});
```

Which results in the following example

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/polar-mountain-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/polar-mountain-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
