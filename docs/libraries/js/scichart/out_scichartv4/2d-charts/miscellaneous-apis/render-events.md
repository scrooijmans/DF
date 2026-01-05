On this page

# Render Events

A surface instance exposes several <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" rel="noopener noreferrer" target="_blank">EventHandlersðŸ“˜</a> corresponding to different stages of the chart rendering cycle.

![](out_scichartv4/2d-charts/miscellaneous-apis/render-events/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Before comparing render events, it's important to understand the following:

1.  There are two types of charts:

- with a **shared WASM context** â€“ Multiple charts share the same WebGL context. They render on a single WebGL canvas and are then copied sequentially to separate 2D canvases.
- with an **individual WASM context** â€“ Each chart has its own WebGL canvas and renders independently.

2.  The **SubCharts API** enables rendering multiple smaller charts simultaneously on a single canvas.

------------------------------------------------------------------------

For more details, check out these pages:

- [SciChartSurface.create() vs. createSingle()](https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface)
- [SciChart3DSurface.create() vs. createSingle()](https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-surface-create-and-create-single)
- [SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview)

## Lifecycle EventHandlers List<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/#lifecycle-eventhandlers-list" class="hash-link" aria-label="Direct link to Lifecycle EventHandlers List" translate="no" title="Direct link to Lifecycle EventHandlers List">â€‹</a>

Below is the list of render process event handlers, in the order they occur:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#redrawrequested" rel="noopener noreferrer" target="_blank"><strong>redrawRequested</strong> ðŸ“˜</a> Triggered on the main surface when an initial invalidate call occurs. Subsequent invalidate calls will not trigger this event until the chart has been rendered. This event is **only** fired on the main surface and does **not** apply to sub-charts.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#prerenderall" rel="noopener noreferrer" target="_blank"><strong>preRenderAll</strong> ðŸ“˜</a> Triggered on the main surface before the render loop begins. Use it to apply custom configurations such as styling or changes to the visible range. This event is **only** fired on the main surface and does **not** apply to sub-charts. It is also currently **not** applicable to 3D charts.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#prerender" rel="noopener noreferrer" target="_blank"><strong>preRender</strong> ðŸ“˜</a> Triggered on a surface or sub-surface before rendering. Use it to apply logic for layout adjustments, such as modifying the visible range aspect ratio or `PointMarker` size.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#layoutmeasured" rel="noopener noreferrer" target="_blank"><strong>layoutMeasured</strong> ðŸ“˜</a> Triggered during rendering when the visible range, size, and axis positions are measured. Use this event to hook into the rendering process when your logic depends on coordinates or offsets.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#rendered" rel="noopener noreferrer" target="_blank"><strong>rendered</strong> ðŸ“˜</a> Triggered after the render logic has executed on a surface or sub-surface.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#renderedtowebgl" rel="noopener noreferrer" target="_blank"><strong>renderedToWebGL</strong> ðŸ“˜</a> Triggered on the main surface after rendering completes. Use this to add custom drawing on the WebGL canvas.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#renderedtodestination" rel="noopener noreferrer" target="_blank"><strong>renderedToDestination</strong> ðŸ“˜</a> Triggered on the main surface after rendering completes and the image is transferred to the target canvas. Use this to add custom drawing on the 2D canvas.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#painted" rel="noopener noreferrer" target="_blank"><strong>painted</strong> ðŸ“˜</a> Triggered on the main surface after the frame has been committed by the client environment. This event is useful for confirming that the chart was drawn, for example, before exporting it as an image.

![](out_scichartv4/2d-charts/miscellaneous-apis/render-events/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Use `preRenderAll` and `renderedToDestination` to measure chart render performance. See [Performance Measurement](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/#performance-measurement).

## Helper Functions<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/#helper-functions" class="hash-link" aria-label="Direct link to Helper Functions" translate="no" title="Direct link to Helper Functions">â€‹</a>

The library provides a helper function <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#receivenextevent" rel="noopener noreferrer" target="_blank">receiveNextEventðŸ“˜</a> to promisify a single event occurrence.

Additionally, a surface exposes a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#nextstaterender" rel="noopener noreferrer" target="_blank">nextStateRenderðŸ“˜</a> method, which works similarly to `receiveNextEvent`, but subscribes only to `renderedToDestination` and allows passing options to control the [Suspend Updates API](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing).

## Usage Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/#usage-examples" class="hash-link" aria-label="Direct link to Usage Examples" translate="no" title="Direct link to Usage Examples">â€‹</a>

### Ensuring the frame was drawn<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/#ensuring-the-frame-was-drawn" class="hash-link" aria-label="Direct link to Ensuring the frame was drawn" translate="no" title="Direct link to Ensuring the frame was drawn">â€‹</a>

When exporting images or performing visual tests, you often need to ensure that the chart has rendered and any animations have completed.

The following example demonstrates how to guarantee that any logic invalidating the chart during the render loop has finished and the chart is stable (i.e., not requested to redraw).

- TS

``` prism-code
const { xValues, yValues } = generateData(10, 1);
const dataSeries = new XyDataSeries(wasmContext, { xValues, yValues, capacity: 1000 });
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    stroke: "olive",
    strokeThickness: 2,
    dataSeries,
    animation: new WaveAnimation({ duration: 3000 })
});
sciChartSurface.renderableSeries.add(lineSeries);

while (await receiveNextEvent(sciChartSurface.painted)) {}

console.log("animation completed");
```

### Performance Measurement<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/#performance-measurement" class="hash-link" aria-label="Direct link to Performance Measurement" translate="no" title="Direct link to Performance Measurement">â€‹</a>

Lifecycle events can be used to measure chart performance. To measure frame render time, use `preRenderAll` and `renderedToDestination`. Other events may also be useful.

![](out_scichartv4/2d-charts/miscellaneous-apis/render-events/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Itâ€™s recommended to initialize a chart with `createSuspended: true` to ensure rendering doesnâ€™t start before the `create` function completes.

Here is an example demonstrating how to set up rendering performance measurement. It outputs results to the console.

![](out_scichartv4/2d-charts/miscellaneous-apis/render-events/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

For thorough performance analysis, consider measuring other operations (such as data append/update times). You can also check out <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Tutorials/MemoryTests" rel="noopener noreferrer" target="_blank">Performance and Memory Usage test suites</a>.

- TS
- JS

``` prism-code
const initStartTimeStamp = performance.now();

const { sciChartSurface, wasmContext } = await SciChartSurface.createSingle(rootElement, {
    createSuspended: true
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

// The fist frame is usually the slowest, so we perform and measure it separately.
// Also in this example we include it into the initialization time.
// #region_C_start
await sciChartSurface.nextStateRender({
    resumeBefore: true,
    invalidateOnResume: true,
    suspendAfter: false
});
// #region_C_end

const firstFrameRenderedTimeStamp = performance.now();

const renderStartTimeStamps: DOMHighResTimeStamp[] = [];
const renderEndTimeStamps: DOMHighResTimeStamp[] = [];
const framePaintedTimeStamps: DOMHighResTimeStamp[] = [];

sciChartSurface.preRenderAll.subscribe(() => {
    renderStartTimeStamps.push(performance.now());
});

sciChartSurface.renderedToDestination.subscribe(() => {
    renderEndTimeStamps.push(performance.now());
});

sciChartSurface.painted.subscribe(() => {
    framePaintedTimeStamps.push(performance.now());
});

const outputPerformanceMeasurements = () => {
    if (
        renderEndTimeStamps.length !== renderStartTimeStamps.length ||
        renderEndTimeStamps.length !== framePaintedTimeStamps.length
    ) {
        // this will mean that the setup is wrong, probably due to the missing "createSuspended" flag during the initialization
        console.warn(
            "There are differences in timestamps number!",
            renderStartTimeStamps,
            renderEndTimeStamps,
            framePaintedTimeStamps
        );
    }

    const aggregatedResults = renderEndTimeStamps.map((end, index) => {
        const start = renderStartTimeStamps[index];
        return { start, end, renderDuration: end - start, frameDuration: framePaintedTimeStamps[index] - start };
    });

    console.log("Performance Measurement Results");
    console.log("Initial Frame time", firstFrameRenderedTimeStamp - initStartTimeStamp);
    console.table(aggregatedResults);

    // cleanup the results to output only new ones the next time
    renderStartTimeStamps.length = 0;
    renderEndTimeStamps.length = 0;
    framePaintedTimeStamps.length = 0;
};

// render one more time and show results
sciChartSurface.invalidateElement();
await receiveNextEvent(sciChartSurface.painted);
outputPerformanceMeasurements();
```

``` prism-code
const initStartTimeStamp = performance.now();
const { sciChartSurface, wasmContext } = await SciChartSurface.createSingle(rootElement, {
    createSuspended: true
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
// The fist frame is usually the slowest, so we perform and measure it separately.
// Also in this example we include it into the initialization time.
// #region_C_start
await sciChartSurface.nextStateRender({
    resumeBefore: true,
    invalidateOnResume: true,
    suspendAfter: false
});
// #region_C_end
const firstFrameRenderedTimeStamp = performance.now();
const renderStartTimeStamps = [];
const renderEndTimeStamps = [];
const framePaintedTimeStamps = [];
sciChartSurface.preRenderAll.subscribe(() => {
    renderStartTimeStamps.push(performance.now());
});
sciChartSurface.renderedToDestination.subscribe(() => {
    renderEndTimeStamps.push(performance.now());
});
sciChartSurface.painted.subscribe(() => {
    framePaintedTimeStamps.push(performance.now());
});
const outputPerformanceMeasurements = () => {
    if (renderEndTimeStamps.length !== renderStartTimeStamps.length ||
        renderEndTimeStamps.length !== framePaintedTimeStamps.length) {
        // this will mean that the setup is wrong, probably due to the missing "createSuspended" flag during the initialization
        console.warn("There are differences in timestamps number!", renderStartTimeStamps, renderEndTimeStamps, framePaintedTimeStamps);
    }
    const aggregatedResults = renderEndTimeStamps.map((end, index) => {
        const start = renderStartTimeStamps[index];
        return { start, end, renderDuration: end - start, frameDuration: framePaintedTimeStamps[index] - start };
    });
    console.log("Performance Measurement Results");
    console.log("Initial Frame time", firstFrameRenderedTimeStamp - initStartTimeStamp);
    console.table(aggregatedResults);
    // cleanup the results to output only new ones the next time
    renderStartTimeStamps.length = 0;
    renderEndTimeStamps.length = 0;
    framePaintedTimeStamps.length = 0;
};
// render one more time and show results
sciChartSurface.invalidateElement();
await receiveNextEvent(sciChartSurface.painted);
outputPerformanceMeasurements();
```

Additionally, this example shows an annotation displaying some rendering performance results.

The implementation:

- TS
- JS

``` prism-code
/**
 * An annotation displaying render performance stats of the surface.
 * It extends the NativeTextAnnotation so its position and styles could be easily updated.
 *
 * @remarks Since the annotation is rendered on the same surface, the annotation displays stats from the previous frame.
 * So, basically it is always a frame behind the last drawn frame...
 */
class PerformanceStatsAnnotation extends NativeTextAnnotation {
    constructor(options?: INativeTextAnnotationOptions) {
        super(options);
        this.processResults = this.processResults.bind(this);

        this.x1 = 0;
        this.y1 = 0;
        this.xCoordinateMode = ECoordinateMode.Relative;
        this.yCoordinateMode = ECoordinateMode.Relative;
        this.multiLineAlignment = EMultiLineAlignment.Left;
        this.backgroundProperty = options?.background ?? "black";
    }
    public onAttach(scs: SciChartSurface): void {
        super.onAttach(scs);

        if (scs.isSubSurface) {
            scs.hasInvalidState = true;
            throw new Error(
                `PerformanceStatsAnnotation is only supposed to be attached to a regular surface, not a sub-chart!`
            );
        }
        subscribeToPerformanceMeasurements(scs, this.processResults);
    }

    protected processResults(result: TPerformanceMeasurementResults) {
        const {
            invalidatedTimeStamp,
            renderStartTimeStamp,
            renderToWebGlEndTimeStamp,
            renderEndTimeStamp,
            paintEndTimeStamp,
            lastPaintEndTimeStamp
        } = result;
        const renderTime = renderEndTimeStamp - renderStartTimeStamp;
        const renderToWebGlTime = renderToWebGlEndTimeStamp - renderStartTimeStamp;
        const copyToCanvasTime = renderTime - renderToWebGlTime;
        const timeToRenderStart = renderStartTimeStamp - invalidatedTimeStamp;
        const timeBetweenPaints = paintEndTimeStamp - lastPaintEndTimeStamp;
        const timeFromRequestToPaint = paintEndTimeStamp - invalidatedTimeStamp;

        // updating the underlying property instead of the setter to prevent invalidation,
        // alternatively Suspend API could be used
        this.textProperty = [
            `FPS: ${(1000 / timeBetweenPaints).toFixed(3).padStart(3, "0")}`,
            `Render: ${renderTime.toFixed(2).padStart(2, "0")}ms`,
            renderToWebGlTime === renderToWebGlTime
                ? `Copy to Canvas: ${copyToCanvasTime.toFixed(2).padStart(2, "0")}ms`
                : "",
            `Since Last Paint: ${timeBetweenPaints.toFixed(2).padStart(2, "0")}ms`
        ].join("\n");
    }
}

/**
 * Collected performance timestamps
 */
type TPerformanceMeasurementResults = {
    invalidatedTimeStamp: DOMHighResTimeStamp;
    renderStartTimeStamp: DOMHighResTimeStamp;
    renderToWebGlEndTimeStamp: DOMHighResTimeStamp;
    renderEndTimeStamp: DOMHighResTimeStamp;
    paintEndTimeStamp: DOMHighResTimeStamp;
    lastPaintEndTimeStamp: DOMHighResTimeStamp;
};

function subscribeToPerformanceMeasurements(
    surface: SciChartSurface,
    callback: (result: TPerformanceMeasurementResults) => void
) {
    let invalidatedTimeStamp: DOMHighResTimeStamp;
    let renderStartTimeStamp: DOMHighResTimeStamp;
    let renderToWebGlEndTimeStamp: DOMHighResTimeStamp;
    let renderEndTimeStamp: DOMHighResTimeStamp;
    let paintEndTimeStamp: DOMHighResTimeStamp;
    let lastPaintEndTimeStamp: DOMHighResTimeStamp;

    surface.redrawRequested.subscribe(isInvalidated => {
        invalidatedTimeStamp = performance.now();
    });

    surface.preRenderAll.subscribe(() => {
        renderStartTimeStamp = performance.now();
    });

    if (surface.isCopyCanvasSurface) {
        surface.renderedToWebGl.subscribe(() => {
            renderToWebGlEndTimeStamp = performance.now();
        });
    }

    surface.renderedToDestination.subscribe(() => {
        renderEndTimeStamp = performance.now();
    });

    surface.painted.subscribe(() => {
        lastPaintEndTimeStamp = paintEndTimeStamp;
        paintEndTimeStamp = performance.now();

        callback({
            invalidatedTimeStamp,
            renderStartTimeStamp,
            renderToWebGlEndTimeStamp,
            renderEndTimeStamp,
            paintEndTimeStamp,
            lastPaintEndTimeStamp
        });
    });
}
```

``` prism-code
/**
 * An annotation displaying render performance stats of the surface.
 * It extends the NativeTextAnnotation so its position and styles could be easily updated.
 *
 * @remarks Since the annotation is rendered on the same surface, the annotation displays stats from the previous frame.
 * So, basically it is always a frame behind the last drawn frame...
 */
class PerformanceStatsAnnotation extends NativeTextAnnotation {
    constructor(options) {
        super(options);
        this.processResults = this.processResults.bind(this);
        this.x1 = 0;
        this.y1 = 0;
        this.xCoordinateMode = ECoordinateMode.Relative;
        this.yCoordinateMode = ECoordinateMode.Relative;
        this.multiLineAlignment = EMultiLineAlignment.Left;
        this.backgroundProperty = options?.background ?? "black";
    }
    onAttach(scs) {
        super.onAttach(scs);
        if (scs.isSubSurface) {
            scs.hasInvalidState = true;
            throw new Error(`PerformanceStatsAnnotation is only supposed to be attached to a regular surface, not a sub-chart!`);
        }
        subscribeToPerformanceMeasurements(scs, this.processResults);
    }
    processResults(result) {
        const { invalidatedTimeStamp, renderStartTimeStamp, renderToWebGlEndTimeStamp, renderEndTimeStamp, paintEndTimeStamp, lastPaintEndTimeStamp } = result;
        const renderTime = renderEndTimeStamp - renderStartTimeStamp;
        const renderToWebGlTime = renderToWebGlEndTimeStamp - renderStartTimeStamp;
        const copyToCanvasTime = renderTime - renderToWebGlTime;
        const timeToRenderStart = renderStartTimeStamp - invalidatedTimeStamp;
        const timeBetweenPaints = paintEndTimeStamp - lastPaintEndTimeStamp;
        const timeFromRequestToPaint = paintEndTimeStamp - invalidatedTimeStamp;
        // updating the underlying property instead of the setter to prevent invalidation,
        // alternatively Suspend API could be used
        this.textProperty = [
            `FPS: ${(1000 / timeBetweenPaints).toFixed(3).padStart(3, "0")}`,
            `Render: ${renderTime.toFixed(2).padStart(2, "0")}ms`,
            renderToWebGlTime === renderToWebGlTime
                ? `Copy to Canvas: ${copyToCanvasTime.toFixed(2).padStart(2, "0")}ms`
                : "",
            `Since Last Paint: ${timeBetweenPaints.toFixed(2).padStart(2, "0")}ms`
        ].join("\n");
    }
}
function subscribeToPerformanceMeasurements(surface, callback) {
    let invalidatedTimeStamp;
    let renderStartTimeStamp;
    let renderToWebGlEndTimeStamp;
    let renderEndTimeStamp;
    let paintEndTimeStamp;
    let lastPaintEndTimeStamp;
    surface.redrawRequested.subscribe(isInvalidated => {
        invalidatedTimeStamp = performance.now();
    });
    surface.preRenderAll.subscribe(() => {
        renderStartTimeStamp = performance.now();
    });
    if (surface.isCopyCanvasSurface) {
        surface.renderedToWebGl.subscribe(() => {
            renderToWebGlEndTimeStamp = performance.now();
        });
    }
    surface.renderedToDestination.subscribe(() => {
        renderEndTimeStamp = performance.now();
    });
    surface.painted.subscribe(() => {
        lastPaintEndTimeStamp = paintEndTimeStamp;
        paintEndTimeStamp = performance.now();
        callback({
            invalidatedTimeStamp,
            renderStartTimeStamp,
            renderToWebGlEndTimeStamp,
            renderEndTimeStamp,
            paintEndTimeStamp,
            lastPaintEndTimeStamp
        });
    });
}
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/miscellaneous-apis/render-events/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/miscellaneous-apis/render-events/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
