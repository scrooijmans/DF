On this page

# Hit-Test API for Polar Line Series

The IHitTestProvider.hitTest method on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlinerenderableseries.html" rel="noopener noreferrer" target="_blank">PolarLineRenderableSeriesðŸ“˜</a> tests if the click was within the Line's bounds and returns a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" rel="noopener noreferrer" target="_blank">HitTestInfoðŸ“˜</a> object with the following properties:

### HitTest on 1 particular Polar Line Series:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series/#hittest-on-1-particular-polar-line-series" class="hash-link" aria-label="Direct link to HitTest on 1 particular Polar Line Series:" translate="no" title="Direct link to HitTest on 1 particular Polar Line Series:">â€‹</a>

Calling the `hitTest` method on one specific series is very easy and can be done this way:

``` prism-code
const x = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
const y = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;

const lineHitTestInfo: HitTestInfo = lineSeries1.hitTestProvider.hitTest(x, y);
```

The algorithm is as follows:

1.  Find the nearest line in X direction.
2.  Test if the click was within line bounds and update <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html#isHit" rel="noopener noreferrer" target="_blank">HitTestInfo.isHitðŸ“˜</a> property.

### Here is how you would implement it on multiple Polar Line Series:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series/#here-is-how-you-would-implement-it-on-multiple-polar-line-series" class="hash-link" aria-label="Direct link to Here is how you would implement it on multiple Polar Line Series:" translate="no" title="Direct link to Here is how you would implement it on multiple Polar Line Series:">â€‹</a>

> First, you need to add the renderable series you plan to hit-test on:

``` prism-code
// add a couple of polar lines to the chart
const polarLine1 = new PolarLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7],
        yValues: [4.5, 3.2, 5.1, 2.8, 4.0, 3.5, 4.8, 3.0],
        dataSeriesName: "Teal Line"
    }),
    stroke: "teal",
    strokeThickness: 3,
});
const polarLine2 = new PolarLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [4, 5, 6, 7, 8, 9, 10, 11],
        yValues: [2.5, 1.8, 3.0, 1.4, 2.0, 1.75, 2.4, 1.5],
        dataSeriesName: "Black Marker Line"
    }),
    stroke: "black",
    strokeThickness: 4,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        strokeThickness: 2,
        stroke: "black",
        fill: "white"
    }),
});
sciChartSurface.renderableSeries.add(polarLine1, polarLine2);
```

> And then, add an event-listener, likely on the "mousedown" event:

``` prism-code
const SUCCESSFUL_HIT_SVG = `<svg width="8" height="8"><circle cx="50%" cy="50%" r="4" fill="#33AA33" stroke="#000000" stroke-width="0.7"/></svg>`;
const NO_HIT_SVG = `<svg width="4" height="4"><circle cx="50%" cy="50%" r="2" fill="#FF0000"/></svg>`;
const HIT_TEST_RADIUS = 10; // Radius for hit testing

// --- Minimal HitTest and SVG Annotation ---
const dotAnnotation = new CustomAnnotation({
    svgString: NO_HIT_SVG,
    isHidden: true, // initially hidden
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center,
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue
});
const textAnnotation = new NativeTextAnnotation({
    text: "",
    fontSize: 20,
    x1: 9,
    y1: 5.5,
    lineSpacing: 10,
    multiLineAlignment: EMultiLineAlignment.Left,
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Top,
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,
});
sciChartSurface.annotations.add(dotAnnotation, textAnnotation);

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
                    + `- line index: ${hitTestInfo.dataSeriesIndex}\n`
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

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/hit-test-api/polar-line-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/hit-test-api/polar-line-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
