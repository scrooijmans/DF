On this page

# Voice Over

To achieve voice over the chart elements, data, and some actions, we can use theÂ [Hit Test API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/hit-test-api-overview/) and event subscription.

Possible use-cases:

- User selects axes with the voice over reading the range of the axis that is visible on the screen.

- User clicks on a series data point with the voice over reading the coordinates of the selected point.

- User zooms or pans chart with the voice over describing the new visible ranges of the axes.

## Adding VoiceOver to Data Series using HitTest API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/#adding-voiceover-to-data-series-using-hittest-api" class="hash-link" aria-label="Direct link to Adding VoiceOver to Data Series using HitTest API" translate="no" title="Direct link to Adding VoiceOver to Data Series using HitTest API">â€‹</a>

This example demonstrates how to add VoiceOver to announce points onÂ [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/). Voice over functionality is provided byÂ <a href="https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance" rel="noopener noreferrer" target="_blank">SpeechSynthesisUtterance API</a>.

1.Â We will start from a basic chart definition:

- JS
- TS

``` prism-code
import { SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries } from "scichart";

// ...
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId);
const xAxis = new NumericAxis(wasmContext, { axisTitle: "Horizontal Axis" });
const yAxis = new NumericAxis(wasmContext, { axisTitle: "Vertical Axis" });
sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);
const dataSeries = new XyDataSeries(wasmContext, {
    dataSeriesName: "Primary Data Series",
    xValues: [1, 2, 3, 4, 5],
    yValues: [8, 6, 7, 2, 16]
});
const renderableSeries = new FastLineRenderableSeries(wasmContext, { strokeThickness: 5, dataSeries });
sciChartSurface.renderableSeries.add(renderableSeries);
```

``` prism-code
import { SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries } from "scichart";

// ...
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId);
const xAxis = new NumericAxis(wasmContext, { axisTitle: "Horizontal Axis" });
const yAxis = new NumericAxis(wasmContext, { axisTitle: "Vertical Axis" });
sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);
const dataSeries = new XyDataSeries(wasmContext, {
    dataSeriesName: "Primary Data Series",
    xValues: [1, 2, 3, 4, 5],
    yValues: [8, 6, 7, 2, 16]
});
const renderableSeries = new FastLineRenderableSeries(wasmContext, { strokeThickness: 5, dataSeries });
sciChartSurface.renderableSeries.add(renderableSeries);
```

This definition instantiates a chart with named axes and line series.

2.Â Subscribe to mouse click events on the chart:

- JS
- TS

``` prism-code
const hitTestDataPoints = (point) => {
    // event handler logic
    // ...
};
// add voice over for data points
sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent) => {
    const point = new Point(mouseEvent.offsetX, mouseEvent.offsetY);
    hitTestDataPoints(point);
});
```

``` prism-code
import { Point } from "scichart";
// ...
const hitTestDataPoints = (point: Point) => {
    // event handler logic
    // ...
};
// add voice over for data points
sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent: MouseEvent) => {
    const point = new Point(mouseEvent.offsetX, mouseEvent.offsetY);
    hitTestDataPoints(point);
});
```

3.Â Add logic to check if line series are clicked in **hitTestDataPoints**:

- JS
- TS

``` prism-code
import { DpiHelper } from "scichart";
// ...
const hitTestDataPoints = (point) => {
    const HIT_TEST_RADIUS = 10 * DpiHelper.PIXEL_RATIO;
    sciChartSurface.renderableSeries.asArray().forEach(series => {
        if (series.hitTestProvider) {
            const hitTestInfo = series.hitTestProvider.hitTest(
                point.x * DpiHelper.PIXEL_RATIO,
                point.y * DpiHelper.PIXEL_RATIO,
                HIT_TEST_RADIUS,
            );
            if (hitTestInfo.isHit) {
                voiceOverDataPoint(series, hitTestInfo);
            }
        }
    });
};
```

``` prism-code
import { DpiHelper } from "scichart";
// ...
const hitTestDataPoints = (point: Point) => {
    const HIT_TEST_RADIUS = 10 * DpiHelper.PIXEL_RATIO;
    sciChartSurface.renderableSeries.asArray().forEach(series => {
        if (series.hitTestProvider) {
            const hitTestInfo = series.hitTestProvider.hitTest(
                point.x * DpiHelper.PIXEL_RATIO,
                point.y * DpiHelper.PIXEL_RATIO,
                HIT_TEST_RADIUS,
            );
            if (hitTestInfo.isHit) {
                // here we will do the VoiceOver
                voiceOverDataPoint(series, hitTestInfo);
            }
        }
    });
};
```

Notice that coordinates passed to the Hit Test method are multiplied by **DpiHelper.PIXEL_RATIO** to be scaled accordingly to display and browser configurations. See alsoÂ [Retina Support and Browser Zoom](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/),Â [Hit Test API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/hit-test-api-overview/) Documentation.

4.Â Now let's define the **voiceOverDataPoint** function which will format the coordinates and announce them using **VoiceOver**:

- JS
- TS

``` prism-code
const voiceOverDataPoint = (series, hitTestInfo) => {
    const { hitTestPointValues } = hitTestInfo;
    const xCoordValue = series.xAxis.labelProvider.formatLabel(hitTestPointValues.x);
    const yCoordValue = series.yAxis.labelProvider.formatLabel(hitTestPointValues.y);
    const pointDescription = `Point at coordinates ${xCoordValue} and ${yCoordValue}`;
    announcePointOnDataSeries(pointDescription);
}
```

``` prism-code
import { HitTestInfo, IRenderableSeries } from "scichart";

const voiceOverDataPoint = (series: IRenderableSeries, hitTestInfo: HitTestInfo) => {
    const { hitTestPointValues } = hitTestInfo;
    const xCoordValue = series.xAxis.labelProvider.formatLabel(hitTestPointValues.x);
    const yCoordValue = series.yAxis.labelProvider.formatLabel(hitTestPointValues.y);
    const pointDescription = `Point at coordinates ${xCoordValue} and ${yCoordValue}`;
    announcePointOnDataSeries(pointDescription);
}
```

where **announcePointOnDataSeries** is implemented as following:

- JS
- TS

``` prism-code
const announceWithSpeechSynthesis = (announcement) => {
    console.log(announcement)
    const synthDescription = new SpeechSynthesisUtterance(announcement);
    window.speechSynthesis.speak(synthDescription);
};
const debounce = (func, timeout = 1000) => {
    let timer;
    return (...args) => {
        clearTimeout(timer);
        timer = setTimeout(() => func(...args), timeout);
    };
};
const announcePointOnDataSeries = debounce(announceWithSpeechSynthesis);
```

``` prism-code
const announceWithSpeechSynthesis = (announcement: string) => {
    console.log(announcement)
    const synthDescription = new SpeechSynthesisUtterance(announcement);
    window.speechSynthesis.speak(synthDescription);
};
const debounce = (func: (...params: any) => void, timeout = 1000) => {
    let timer: NodeJS.Timeout;
    return (...args: any) => {
        clearTimeout(timer);
        timer = setTimeout(() => func(...args), timeout);
    };
};
// additionally debounce the speech synthesis generation
const announcePointOnDataSeries = debounce(announceWithSpeechSynthesis);
```

Now upon clicking on a point of the line series we should get its coordinates announced with speech synthesis and logged to the console.

<img src="out_scichartv4/2d-charts/accessibility/voice-over/index_media/a4b9632ec6513fcfcb9ceec9774af3c110d91a38.jpg" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Â VoiceOver for Axes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/#voiceover-for-axes" class="hash-link" aria-label="Direct link to Â VoiceOver for Axes" translate="no" title="Direct link to Â VoiceOver for Axes">â€‹</a>

**VoiceOver** upon chart axes also could be easily implemented:

- JS
- TS

``` prism-code
const hitTestAxes = (point) => {
    sciChartSurface.xAxes.asArray().forEach(axis => {
        if (isPointWithinAxis(point, axis)) {
            announceAxis(axis);
        }
    });
    sciChartSurface.yAxes.asArray().forEach(axis => {
        if (isPointWithinAxis(point, axis)) {
            announceAxis(axis);
        }
    });
}
// add voice over for data points and axes
sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent) => {
    const point = new Point(mouseEvent.offsetX, mouseEvent.offsetY);
    hitTestAxes(point);
    hitTestDataPoints(point);
});
```

``` prism-code
const hitTestAxes = (point: Point) => {
    sciChartSurface.xAxes.asArray().forEach(axis => {
        if (isPointWithinAxis(point, axis)) {
            announceAxis(axis);
        }
    });
    sciChartSurface.yAxes.asArray().forEach(axis => {
        if (isPointWithinAxis(point, axis)) {
            announceAxis(axis);
        }
    });
}
// add voice over for data points and axes
sciChartSurface.domCanvas2D.addEventListener("mousedown", (mouseEvent: MouseEvent) => {
    const point = new Point(mouseEvent.offsetX, mouseEvent.offsetY);
    hitTestDataPoints(point);
    hitTestAxes(point);
});
```

Here we have defined **hitTestAxes** function and used it in the event listener.

The **announceAxis** and **isPointWithinAxis** will look like this:

- JS
- TS

``` prism-code
const announceAxis = debounce((axis) => {
    const from = axis.labelProvider.formatLabel(axis.visibleRange.min);
    const to = axis.labelProvider.formatLabel(axis.visibleRange.max);
    const axisDescription = `${axis.axisTitle} with visible range from ${from} to ${to} `;
    announceWithSpeechSynthesis(axisDescription);
});
const isPointWithinAxis = (point, axis) => {
    return point.x * DpiHelper.PIXEL_RATIO > axis.viewRect.left 
        && point.x * DpiHelper.PIXEL_RATIO < axis.viewRect.right
        && point.y * DpiHelper.PIXEL_RATIO > axis.viewRect.top 
        && point.y * DpiHelper.PIXEL_RATIO < axis.viewRect.bottom;
};
```

``` prism-code
import { AxisBase2D } from "scichart";
// ...
const announceAxis = debounce((axis: AxisBase2D) => {
    const from = axis.labelProvider.formatLabel(axis.visibleRange.min);
    const to = axis.labelProvider.formatLabel(axis.visibleRange.max);
    const axisDescription = `${axis.axisTitle} with visible range from ${from} to ${to} `;
    announceWithSpeechSynthesis(axisDescription);
});
const isPointWithinAxis = (point: Point, axis: AxisBase2D) => {
    return point.x * DpiHelper.PIXEL_RATIO > axis.viewRect.left 
        && point.x * DpiHelper.PIXEL_RATIO < axis.viewRect.right
        && point.y * DpiHelper.PIXEL_RATIO > axis.viewRect.top 
        && point.y * DpiHelper.PIXEL_RATIO < axis.viewRect.bottom;
};
```

Â This example should result in **VoiceOver** describing axis title and its visible range upon clicking.

<img src="out_scichartv4/2d-charts/accessibility/voice-over/index_media/19771217145d329195b2ee918c8ab26a57b8aedd.jpg" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## VoiceOver for visible range updates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/#voiceover-for-visible-range-updates" class="hash-link" aria-label="Direct link to VoiceOver for visible range updates" translate="no" title="Direct link to VoiceOver for visible range updates">â€‹</a>

The visible range changes detection on an axis could be added as following:

``` prism-code
// Visible range changes detection

const announceYRangeChange = debounce(announceWithSpeechSynthesis);
const announceXRangeChange = debounce(announceWithSpeechSynthesis);
// add announcement of axis range changes
yAxis.visibleRangeChanged.subscribe((args) => {
    const { min, max } = args.visibleRange;
    const from = yAxis.labelProvider.formatLabel(min);
    const to = yAxis.labelProvider.formatLabel(max);
    const announcement = `${yAxis.axisTitle} range changed, now it's from ${from} to ${to}.`;
    announceYRangeChange(announcement);
});
xAxis.visibleRangeChanged.subscribe((args) => {
    const { min, max } = args.visibleRange;
    const from = xAxis.labelProvider.formatLabel(min);
    const to = xAxis.labelProvider.formatLabel(max);
    const announcement = `${xAxis.axisTitle} range changed, now it's from ${from} to ${to}.`;
    announceXRangeChange(announcement);
});
```

Let's add some zoom&pan modifiers to demonstrate the result:

``` prism-code
// Add modifiers

import { MouseWheelZoomModifier, ZoomExtentsModifier, ZoomPanModifier } from "scichart";
// ...
sciChartSurface.chartModifiers.add(
    new ZoomPanModifier(),
    new ZoomExtentsModifier(),
    new MouseWheelZoomModifier(),
);
```

Â Now when using the chart modifiers you should be able to observe them getting logged to the console.

<img src="out_scichartv4/2d-charts/accessibility/voice-over/index_media/27de8674de885d8bfcf6e4018811741719b34e4d.jpg" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Color and Contrast](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/color-and-contrast/)
- [Keyboard Accessibility](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/keyboard-accessibility/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/accessibility/voice-over/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/accessibility/voice-over/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
