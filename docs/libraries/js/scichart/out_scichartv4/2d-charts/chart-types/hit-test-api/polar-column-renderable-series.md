On this page

# Hit-Test API for Polar Column Series

The IHitTestProvider.hitTest method on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a> tests if the click was within the Column's body and returns a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" rel="noopener noreferrer" target="_blank">HitTestInfoðŸ“˜</a> object with the following properties:

### HitTest on 1 particular Polar Column Series:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-column-renderable-series/#hittest-on-1-particular-polar-column-series" class="hash-link" aria-label="Direct link to HitTest on 1 particular Polar Column Series:" translate="no" title="Direct link to HitTest on 1 particular Polar Column Series:">â€‹</a>

Calling the `hitTest` method on one specific series is very easy and can be done this way:

``` prism-code
const x = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
const y = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;

const colHitTestInfo: HitTestInfo = colSeries1.hitTestProvider.hitTest(x, y);
```

The algorithm is as follows:

1.  Find the nearest column in X direction.
2.  Test if the click was within column body and update <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html#isHit" rel="noopener noreferrer" target="_blank">HitTestInfo.isHitðŸ“˜</a> property.

### Here is how you would implement it on multiple Polar Column Series:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-column-renderable-series/#here-is-how-you-would-implement-it-on-multiple-polar-column-series" class="hash-link" aria-label="Direct link to Here is how you would implement it on multiple Polar Column Series:" translate="no" title="Direct link to Here is how you would implement it on multiple Polar Column Series:">â€‹</a>

> First, you need to add the renderable series you plan to hit-test on:

``` prism-code
// add a couple of polar columns to the chart
const polarColumn1 = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        yValues: [2.5, 1.8, 3.0, 1.4, 2.0, 1.75, 2.4, 1.5, 3.0, 4.5, 2],
        dataSeriesName: "Red Columns"
    }),
    stroke: "white",
    fill: "#883333",
    dataPointWidth: 0.6
});

const polarColumn2 = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [9, 10, 11, 12, 13, 14, 15, 16],
        yValues: [4.5, 3.2, 3.9, 2.8, 4.0, 3.5, 4.8, 3.0],
        dataSeriesName: "Blue Columns"
    }),
    stroke: "black",
    fill: "#3333AA99",
    dataPointWidth: 0.8
});

sciChartSurface.renderableSeries.add(polarColumn1, polarColumn2);
```

> And then, add an event-listener, likely on the "mousedown" event:

``` prism-code
// Add an event listener for mouse down events
sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent: MouseEvent) => {
    // Use our DpiHelper class to multiply coordinates, else screens with non-100% scaling will not work very well
    const x = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
    const y = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;

    // optional - flag to stop checking for hit-test on other series once a hit is found
    let wasTheHitSuccessfulAtLeastOnce = false; 

    [...sciChartSurface.renderableSeries.asArray()] // copy the renderable series to an anonymous array to not modify the original collection
        .reverse() // The default layering of series is from bottom to top in the array, so we reverse it to check from top to bottom
        .forEach(rs => {
        console.log(`Trying hit test on: ${rs.getDataSeriesName()}`);
        
        if (rs.hitTestProvider && mouseEvent && !wasTheHitSuccessfulAtLeastOnce) {
            const hitTestInfo = rs.hitTestProvider.hitTestDataPoint(x, y, HIT_TEST_RADIUS);

            dotAnnotation.x1 = hitTestInfo.hitTestPointValues.x;
            dotAnnotation.y1 = hitTestInfo.hitTestPointValues.y;
            dotAnnotation.isHidden = false;

            if (hitTestInfo.isHit) { 
                // Successful Hit
                dotAnnotation.svgString = SUCCESSFUL_HIT_SVG;
                textAnnotation.text = `Hit (x: ${hitTestInfo.hitTestPointValues.x.toFixed(2)}, y: ${hitTestInfo.hitTestPointValues.y.toFixed(2)})\n`
                    + `- series: "${hitTestInfo.associatedSeries.getDataSeriesName()}"\n`
                    + `- column index: ${hitTestInfo.dataSeriesIndex}\n`
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

Which results in this following functionality:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/polar-column-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/polar-column-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
