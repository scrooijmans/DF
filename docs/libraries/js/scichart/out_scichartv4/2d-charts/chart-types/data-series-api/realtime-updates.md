On this page

# DataSeries Realtime Updates

As previously covered any modification to the DataSeries e.g. via callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#append" rel="noopener noreferrer" target="_blank">append()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insert" rel="noopener noreferrer" target="_blank">insert()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#update" rel="noopener noreferrer" target="_blank">update()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeat" rel="noopener noreferrer" target="_blank">remove()ðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#clear" rel="noopener noreferrer" target="_blank">clear()ðŸ“˜</a> will trigger a redraw on the chart.

Redraws are throttled so that a redraw only occurs every 1/60th of a second, no matter how often you update data.

Below we're going to talk about the four modes of DataSeries Realtime updates and how to achieve them in SciChart.js.

## Appending Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#appending-data" class="hash-link" aria-label="Direct link to Appending Data" translate="no" title="Direct link to Appending Data">â€‹</a>

Appending data is aÂ dynamic chartÂ scenario where you start off with 0..N X,Y values then append a new batch of X,Y values viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendRange" rel="noopener noreferrer" target="_blank">dataSeries.appendRange()ðŸ“˜</a>. With the correct flags on the axis the chart will grow to fit all data. Memory grows until you stop appending or you reset the chart via callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#clear" rel="noopener noreferrer" target="_blank">dataSeries.clear()ðŸ“˜</a>.

Here's an example:

- TS

``` prism-code
// Create a DataSeries
const xyDataSeries = new XyDataSeries(wasmContext, {
    // Optional: pass X,Y values to DataSeries constructor for fast initialization
    xValues,
    yValues
});

// Create a renderableSeries and assign the dataSeries
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: xyDataSeries,
        strokeThickness: 3,
        stroke: "#50C7E0"
    })
);

// Now let's use a timeout to appendRange() 10 new values every 20ms. After N appends, reset the dataSeries
let updateCount = 0;
const updateCallback = () => {
    const xUpdate = [];
    const yUpdate = [];
    for (let j = 0; j < 10; i++, j++) {
        xUpdate.push(i);
        yUpdate.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
    }
    xyDataSeries.appendRange(xUpdate, yUpdate);

    // Just putting this in to reset the dataseries after N updates. We don't want the codepen example to grow infinitely!
    if (++updateCount % 250 === 0) {
        xyDataSeries.clear();
        updateCount = 0;
    }
};

setTimeout(() => {
    updateCallback();
    setInterval(updateCallback, 20);
}, 20);
```

This results in the following output

## Replacing Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#replacing-data" class="hash-link" aria-label="Direct link to Replacing Data" translate="no" title="Direct link to Replacing Data">â€‹</a>

Replacing data is a real-time scenario which would allow you to make a spectral-analyzer type chart, where all data is replaced every time the chart is updated.

In SciChart.js, we achieve this by usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#clear" rel="noopener noreferrer" target="_blank">dataSeries.clear()ðŸ“˜</a> followed byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendRange" rel="noopener noreferrer" target="_blank">dataSeries.appendRange()ðŸ“˜</a>.

- TS

``` prism-code
// Create a DataSeries
const xyDataSeries = new XyDataSeries(wasmContext, {
    // Optional: pass X,Y values to DataSeries constructor for fast initialization
    xValues: [],
    yValues: []
});

// Create a renderableSeries and assign the dataSeries
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: xyDataSeries,
        strokeThickness: 3,
        stroke: "#50C7E0"
    })
);

// Now let's use a timeout to clear() and appendRange() entirely new values every 20ms.
const updateCallback = () => {
    const xValues = [];
    const yValues = [];
    for (let i = 0; i < 100; i++) {
        xValues.push(i);
        yValues.push(Math.random() * Math.sin(i * 0.1) - Math.cos(i * 0.01));
    }
    xyDataSeries.clear();
    xyDataSeries.appendRange(xValues, yValues);
};

setTimeout(() => {
    updateCallback();
    setInterval(updateCallback, 20);
}, 20);
```

This results in the following output

## Scrolling Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#scrolling-data" class="hash-link" aria-label="Direct link to Scrolling Data" translate="no" title="Direct link to Scrolling Data">â€‹</a>

Scrolling data can be achieved by appending then removing data so that a fixed number of points remains in the dataSeries. This can be achieved viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeRange" rel="noopener noreferrer" target="_blank">dataSeries.removeRange()ðŸ“˜</a> thenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendRange" rel="noopener noreferrer" target="_blank">dataSeries.appendRange()ðŸ“˜</a> but also you can use the new fifoCapacity flag available in SciChart.js v3.2.

Below we have an example of each:

### Scrolling using appendRange() removeRange()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#scrolling-using-appendrange-removerange" class="hash-link" aria-label="Direct link to Scrolling using appendRange() removeRange()" translate="no" title="Direct link to Scrolling using appendRange() removeRange()">â€‹</a>

Here's an example of how to useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeRange" rel="noopener noreferrer" target="_blank">dataSeries.removeRange()ðŸ“˜</a> thenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendRange" rel="noopener noreferrer" target="_blank">dataSeries.appendRange()ðŸ“˜</a>Â  to scroll a chart.

- TS

``` prism-code
// Create a DataSeries
const xyDataSeries = new XyDataSeries(wasmContext, {
    // Optional: pass X,Y values to DataSeries constructor for fast initialization
    xValues,
    yValues
});

// Create a renderableSeries and assign the dataSeries
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: xyDataSeries,
        strokeThickness: 3,
        stroke: "#50C7E0"
    })
);

// Now let's use a timeout to appendRange() 10 new values every 20ms.
// using removeRange() causes the number of points in the series to remain fixed and the chart to scroll
const updateCallback = () => {
    const xUpdate = [];
    const yUpdate = [];
    for (let j = 0; j < 5; i++, j++) {
        xUpdate.push(i);
        yUpdate.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
    }
    // Remove the first N points from the series
    xyDataSeries.removeRange(0, xUpdate.length);
    // Now append new points
    xyDataSeries.appendRange(xUpdate, yUpdate);
    // result: dataSeries length remains the same. as x-value increases, and xAxis.autoRange zooms to fit, the chart scrolls
};

setTimeout(() => {
    updateCallback();
    setInterval(updateCallback, 20);
}, 20);
```

This results in the following output

### Scrolling using fifoCapacity<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#scrolling-using-fifocapacity" class="hash-link" aria-label="Direct link to Scrolling using fifoCapacity" translate="no" title="Direct link to Scrolling using fifoCapacity">â€‹</a>

Since SciChart.js v3.2, we've introduced a much more efficient way to auto-discard old points. By setting **dataSeries.fifoCapacity** = N, when the capacity is exceeded, old points are discarded. FIFO series are a special case and are internally handled as a circular buffer. They cannot be resized.

- TS

``` prism-code
// Create a DataSeries
const xyDataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues,
    fifoCapacity: 1200 // set fifoCapacity to 1200. Requires scichart.js v3.2 or later
});

console.log(`version is ${libraryVersion}`);
console.log(`dataSeries.fifoCapacity is ${xyDataSeries.fifoCapacity}`);

// Create a renderableSeries and assign the dataSeries
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: xyDataSeries,
        strokeThickness: 3,
        stroke: "#50C7E0"
    })
);

// Now let's use a timeout to appendRange() new values every 20ms.
// using removeRange() causes the number of points in the series to remain fixed and the chart to scroll
const updateCallback = () => {
    const xUpdate = [];
    const yUpdate = [];
    for (let j = 0; j < 5; i++, j++) {
        xUpdate.push(i);
        yUpdate.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
    }
    // With fifoCapacity set, just append new points.
    xyDataSeries.appendRange(xUpdate, yUpdate);
    // result: dataSeries length remains the same. point counts > fifoCapacity are discarded.
    // as x-value increases, and xAxis.autoRange zooms to fit, the chart scrolls
};

setTimeout(() => {
    updateCallback();
    setInterval(updateCallback, 20);
}, 20);
```

This results in the following output

## Sweeping Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#sweeping-data" class="hash-link" aria-label="Direct link to Sweeping Data" translate="no" title="Direct link to Sweeping Data">â€‹</a>

Another mode that we've added in SciChart.js v3.2, and the last real-time update mode is Fifo Sweeping.

With **dataSeries.fifoCapacity** set, also setting **dataSeries.fifoSweeping** = true, setting an optionalÂ **dataSeries.fifoSweepingGap** and having the correct type of xAxis or modulation of x-data,Â you can achieve allowing the chart to wrap-around once the trace reaches the right edge of the viewport.

- TS

``` prism-code
// Create a DataSeries
const xyDataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues,
    fifoCapacity: 1000, // set fifoCapacity. Requires scichart.js v3.2 or later
    fifoSweeping: true,
    fifoSweepingGap: 20
});

console.log(`version is ${libraryVersion}`);
console.log(`dataSeries.fifoCapacity is ${xyDataSeries.fifoCapacity}`);

// Create a renderableSeries and assign the dataSeries
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        dataSeries: xyDataSeries,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 11,
            height: 11,
            fill: "#fff",
            lastPointOnly: true
        }),
        strokeThickness: 3,
        stroke: "#50C7E0"
    })
);

// Now let's use a timeout to appendRange() new values every 20ms.
// using removeRange() causes the number of points in the series to remain fixed and the chart to scroll
const updateCallback = () => {
    const xUpdate = [];
    const yUpdate = [];
    for (let j = 0; j < 5; i++, j++) {
        xUpdate.push(i % fifoCapacity);
        yUpdate.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
    }
    // With fifoCapacity set, just append new points.
    xyDataSeries.appendRange(xUpdate, yUpdate);
    // result: dataSeries length remains the same. point counts > fifoCapacity are discarded.
    // as x-value increases, and xAxis.autoRange zooms to fit, the chart scrolls
};

setTimeout(() => {
    updateCallback();
    setInterval(updateCallback, 20);
}, 20);
```

This results in the following output

Note: Sweeping requires a few special conditions. **fifoCapacity** must be set and **fifoSweeping** = true. Next, you must either use a **CategoryAxis** on the xAxis, or, modulate your data.

You can use **NumericAxis**Â but you must modulate your data. X must range from 0...fifoCapacity. In the example above we set **xValue\[i\] = i % fifoCapacity**

See a worked example at theÂ <a href="https://www.scichart.com/demo/javascript-vital-signs-ecg-medical-chart-example" rel="noopener noreferrer" target="_blank">ECG/Vital Signs monitor demo</a>.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-series-api/realtime-updates/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-series-api/realtime-updates/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
