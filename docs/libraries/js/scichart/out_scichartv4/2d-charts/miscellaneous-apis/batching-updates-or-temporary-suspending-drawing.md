On this page

# Batching Updates or Temporary Suspending Drawing

In SciChart.js, every update you make to the chart can potentially trigger a redraw. For example:

- Calling `DataSeries.append()`
- Setting `Axis.visibleRange`
- Adding a new series to the `sciChartSurface.renderableSeries` collection

All of these operations may trigger a redraw of the `SciChartSurface`. This might not be desirable in all cases, so there is an API that allows you to temporarily suspend or pause drawing while making multiple updates.

## The Suspend Updates API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#the-suspend-updates-api" class="hash-link" aria-label="Direct link to The Suspend Updates API" translate="no" title="Direct link to The Suspend Updates API">â€‹</a>

The Suspend Updates API should be used when you want to temporarily stop drawing on a `SciChartSurface`.  
This can be useful in certain scenarios:

- It helps improve performance and reduce visual artifacts when making many changes at once (see [Data Updates Batching example](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#data-updates-batching-example-using-lockunlock)).
- It can be used in combination with [Render Events](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events) to apply custom layout calculations.
- It allows control over rendering and timing. For example, by setting up your own render loop.

## SciChart's Redraw-on-Update Behavior Overview<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#scicharts-redraw-on-update-behavior-overview" class="hash-link" aria-label="Direct link to SciChart&#39;s Redraw-on-Update Behavior Overview" translate="no" title="Direct link to SciChart&#39;s Redraw-on-Update Behavior Overview">â€‹</a>

By default, a chart instance redraws only if its state has changed. This includes modifications to properties, data, chart entities, font loading, resizing, pixel ratio changes, visibility changes, and more.

When a modification occurs, it issues an internal "invalidate" request.  
This signals the chart to rerender on the next animation frame.  
The Suspend Updates API allows the chart to ignore these invalidation requests.

![](out_scichartv4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The default behavior of the SciChart Engine relies on render operations being scheduled via <a href="https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame" rel="noopener noreferrer" target="_blank"><code>requestAnimationFrame</code></a>.

![](out_scichartv4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Some invalidation requests should not be ignored, as doing so may lead to unexpected results.  
For example, chart resizing, pixel ratio changes, or tab visibility changes call the `invalidateElement` method with the `force: true` option. This triggers a redraw regardless of the suspend state.

![](out_scichartv4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

You can override the `invalidateElement` and `notifyPropertyChanged` methods for more granular control over which updates trigger a redraw.

## Usage<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#usage" class="hash-link" aria-label="Direct link to Usage" translate="no" title="Direct link to Usage">â€‹</a>

The main logic for suspending updates is handled by an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html" rel="noopener noreferrer" target="_blank">UpdateSuspenderðŸ“˜</a> instance, accessible via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#suspender" rel="noopener noreferrer" target="_blank">sciChartSurface.suspenderðŸ“˜</a> property.

There are two mechanisms to toggle update suspension:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#suspend" rel="noopener noreferrer" target="_blank">suspender.suspendðŸ“˜</a>/<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#resume" rel="noopener noreferrer" target="_blank">suspender.resumeðŸ“˜</a> - a counter based mechanisms;
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#lock" rel="noopener noreferrer" target="_blank">suspender.lockðŸ“˜</a>/**unlock** - a token based mechanisms. Both can be used together and are applied in some areas of the SciChart rendering logic implicitly.

You can also check the current state with the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#issuspended" rel="noopener noreferrer" target="_blank"><code>suspender.isSuspended</code>ðŸ“˜</a> flag, which reflects the cumulative suspend state based on both counter and token-based controls.

The surface also exposes convenience methods for accessing the API.

### Basic<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#basic" class="hash-link" aria-label="Direct link to Basic" translate="no" title="Direct link to Basic">â€‹</a>

Two primary methods of this API are <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html#suspendupdates" rel="noopener noreferrer" target="_blank"><code>sciChartSurface.suspendUpdates</code>ðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html#resumeupdates" rel="noopener noreferrer" target="_blank"><code>sciChartSurface.resumeUpdates</code>ðŸ“˜</a>, which internally call <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#suspend" rel="noopener noreferrer" target="_blank">suspender.suspendðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#resume" rel="noopener noreferrer" target="_blank">suspender.resumeðŸ“˜</a> respectively.

The surface also exposes the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html#issuspended" rel="noopener noreferrer" target="_blank"><code>isSuspended</code>ðŸ“˜</a> property.

A few important things to note:

- Calling `suspendUpdates` multiple times requires you to call `resumeUpdates` the same number of times to fully resume updates.
- Alternatively, `resumeUpdates` accepts a `force` flag in the options, which resets the counter regardless of how many times it was suspended.
- Another option is `invalidateOnResume`, which triggers a redraw immediately after resuming.

#### Data Updates Batching Example Using Suspend/Resume<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#data-updates-batching-example-using-suspendresume" class="hash-link" aria-label="Direct link to Data Updates Batching Example Using Suspend/Resume" translate="no" title="Direct link to Data Updates Batching Example Using Suspend/Resume">â€‹</a>

- TS
- JS

``` prism-code
// In this setup data is updated approximately each 10ms,
// but the redraw is requested only when a batch of data points has been accumulated up to the specified size (`maxBatchSize`)
let batchCounter = 0;
const maxBatchSize = 1000;

let timer: NodeJS.Timeout;

const toggleUpdates = () => {
    if (timer) {
        clearInterval(timer);
        timer = null;
        return;
    }

    timer = setInterval(() => {
        // here we create arrays of additional 10 data points
        const { xValues, yValues } = generateData(10, 1, dataSeries.count());

        // Pause chart rerender requests before updating the data.
        sciChartSurface.suspendUpdates();

        // appending data points here won't trigger the chart redraw
        dataSeries.appendRange(xValues, yValues);
        console.log("Data updated. Number of points in the current batch: ", batchCounter);

        // unpause the chart
        sciChartSurface.resumeUpdates({ invalidateOnResume: false, force: false });

        // but if the accumulated data points reached the desired batch size,
        // we request a redraw explicitly
        batchCounter += xValues.length;
        if (batchCounter === maxBatchSize) {
            // reset counter
            batchCounter = 0;

            // request a redraw
            sciChartSurface.invalidateElement();
        }
    }, 10);
};

// bind the timer cleanup to the surface lifecycle
sciChartSurface.addDeletable({
    delete: () => {
        clearInterval(timer);
    }
});

toggleUpdates();
```

``` prism-code
// In this setup data is updated approximately each 10ms,
// but the redraw is requested only when a batch of data points has been accumulated up to the specified size (`maxBatchSize`)
let batchCounter = 0;
const maxBatchSize = 1000;
let timer;
const toggleUpdates = () => {
    if (timer) {
        clearInterval(timer);
        timer = null;
        return;
    }
    timer = setInterval(() => {
        // here we create arrays of additional 10 data points
        const { xValues, yValues } = generateData(10, 1, dataSeries.count());
        // Pause chart rerender requests before updating the data.
        sciChartSurface.suspendUpdates();
        // appending data points here won't trigger the chart redraw
        dataSeries.appendRange(xValues, yValues);
        console.log("Data updated. Number of points in the current batch: ", batchCounter);
        // unpause the chart
        sciChartSurface.resumeUpdates({ invalidateOnResume: false, force: false });
        // but if the accumulated data points reached the desired batch size,
        // we request a redraw explicitly
        batchCounter += xValues.length;
        if (batchCounter === maxBatchSize) {
            // reset counter
            batchCounter = 0;
            // request a redraw
            sciChartSurface.invalidateElement();
        }
    }, 10);
};
// bind the timer cleanup to the surface lifecycle
sciChartSurface.addDeletable({
    delete: () => {
        clearInterval(timer);
    }
});
toggleUpdates();
```

### Alternative Lock/Unlock Methods<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#alternative-lockunlock-methods" class="hash-link" aria-label="Direct link to Alternative Lock/Unlock Methods" translate="no" title="Direct link to Alternative Lock/Unlock Methods">â€‹</a>

Another way to suspend updates is via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#lock" rel="noopener noreferrer" target="_blank"><code>suspender.lock</code>ðŸ“˜</a>, which returns an `unlock` function that must be called to resume updates.

These methods provide a stricter suspension mechanism.

Since `suspender.isSuspended` can be affected by both `suspend`/`resume` and `lock`/`unlock`,  
the `lock`/`unlock` pair ensures that suspension is lifted only by its corresponding `unlock` call - it cannot be bypassed or force-resumed by other means.

#### Data Updates Batching Example Using Lock/Unlock<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#data-updates-batching-example-using-lockunlock" class="hash-link" aria-label="Direct link to Data Updates Batching Example Using Lock/Unlock" translate="no" title="Direct link to Data Updates Batching Example Using Lock/Unlock">â€‹</a>

- TS
- JS

``` prism-code
// In this setup data is updated approximately each 10ms,
// but the redraw is requested only when a batch of data points has been accumulated up to the specified size (`maxBatchSize`)
let batchCounter = 0;
const maxBatchSize = 1000;

let timer: NodeJS.Timeout;

const toggleUpdates = () => {
    if (timer) {
        clearInterval(timer);
        timer = null;
        return;
    }

    timer = setInterval(() => {
        // Pause chart rerender requests before updating the data.
        const unlock = sciChartSurface.suspender.lock();

        // here we create arrays of additional 10 data points
        const { xValues, yValues } = generateData(10, 1, dataSeries.count());

        // appending data points here won't trigger the chart redraw
        dataSeries.appendRange(xValues, yValues);
        batchCounter += xValues.length;
        console.log("Data updated. Number of points in the current batch: ", batchCounter);

        // unpause the chart
        unlock();

        // but if the accumulated data points reached the desired batch size,
        // we request a redraw explicitly
        if (batchCounter === maxBatchSize) {
            // reset counter
            batchCounter = 0;

            // request a redraw
            sciChartSurface.invalidateElement();
        }
    }, 10);
};

// bind the timer cleanup to the surface lifecycle
sciChartSurface.addDeletable({
    delete: () => {
        clearInterval(timer);
    }
});

toggleUpdates();
```

``` prism-code
// In this setup data is updated approximately each 10ms,
// but the redraw is requested only when a batch of data points has been accumulated up to the specified size (`maxBatchSize`)
let batchCounter = 0;
const maxBatchSize = 1000;
let timer;
const toggleUpdates = () => {
    if (timer) {
        clearInterval(timer);
        timer = null;
        return;
    }
    timer = setInterval(() => {
        // Pause chart rerender requests before updating the data.
        const unlock = sciChartSurface.suspender.lock();
        // here we create arrays of additional 10 data points
        const { xValues, yValues } = generateData(10, 1, dataSeries.count());
        // appending data points here won't trigger the chart redraw
        dataSeries.appendRange(xValues, yValues);
        batchCounter += xValues.length;
        console.log("Data updated. Number of points in the current batch: ", batchCounter);
        // unpause the chart
        unlock();
        // but if the accumulated data points reached the desired batch size,
        // we request a redraw explicitly
        if (batchCounter === maxBatchSize) {
            // reset counter
            batchCounter = 0;
            // request a redraw
            sciChartSurface.invalidateElement();
        }
    }, 10);
};
// bind the timer cleanup to the surface lifecycle
sciChartSurface.addDeletable({
    delete: () => {
        clearInterval(timer);
    }
});
toggleUpdates();
```

### Suspend Chart on Initialization<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#suspend-chart-on-initialization" class="hash-link" aria-label="Direct link to Suspend Chart on Initialization" translate="no" title="Direct link to Suspend Chart on Initialization">â€‹</a>

To prevent the chart from rendering immediately upon initialization, use the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dsurfaceoptions.html#createsuspended" rel="noopener noreferrer" target="_blank"><code>createSuspended</code>ðŸ“˜</a> option when creating the surface.

- TS
- JS

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartSurface.create(rootElement, {
    createSuspended: true
});
```

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartSurface.create(rootElement, {
    createSuspended: true
});
```

### onResumed Event Handler<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#onresumed-event-handler" class="hash-link" aria-label="Direct link to onResumed Event Handler" translate="no" title="Direct link to onResumed Event Handler">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#onresumed" rel="noopener noreferrer" target="_blank"><code>suspender.onResumed</code>ðŸ“˜</a> handler lets you subscribe to an event when a surface becomes unsuspended.

- TS
- JS

``` prism-code
sciChartSurface.suspender.onResumed.subscribe(() => {
    console.log("Updates resumed.");
});
```

``` prism-code
sciChartSurface.suspender.onResumed.subscribe(() => {
    console.log("Updates resumed.");
});
```

### Freeze Drawing for Charts Out of View<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#freeze-drawing-for-charts-out-of-view" class="hash-link" aria-label="Direct link to Freeze Drawing for Charts Out of View" translate="no" title="Direct link to Freeze Drawing for Charts Out of View">â€‹</a>

This feature also uses the Suspend API internally.  
It helps optimize performance by suspending charts that are out of view on the page.  
Check out the [Freeze Drawing for Charts](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks#21-freeze-drawing-for-charts-out-of-view) section for more details.

## Troubleshooting<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#troubleshooting" class="hash-link" aria-label="Direct link to Troubleshooting" translate="no" title="Direct link to Troubleshooting">â€‹</a>

If your chart appears frozen after using this API, it may be because you forgot to call `resume` or `unlock`.  
Ensure the suspender is resumedâ€”once active again, the chart should respond to mouse input and reflect data or property changes.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
