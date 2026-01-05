On this page

# Dataset Animations

SciChart.js v2.x and above features a new API which allows you to animate between datasets on a chart.Â But the limitation is that the length of data vectors (length of X and Y values and animation values) must be the same.

Below find an example of animating between two different datasets. Note the Animation type includes style properties and is a specific animation type for the series, as per ourÂ [Style Transition Animations documentation](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations).

## Worked Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/#worked-examples" class="hash-link" aria-label="Direct link to Worked Examples" translate="no" title="Direct link to Worked Examples">â€‹</a>

### Animating Data in a Scatter Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/#animating-data-in-a-scatter-series" class="hash-link" aria-label="Direct link to Animating Data in a Scatter Series" translate="no" title="Direct link to Animating Data in a Scatter Series">â€‹</a>

You can animate the dataset in a scatter series by using the **ScatterAnimation** type. This allows you to set new data and animate to the new position. Find an example below:

``` prism-code
// Scatter dataset animation

import {SciChartSurface, NumericAxis, EllipsePointMarker, XyDataSeries, NumberRange, XyScatterRenderableSeries, SciChartJSLightTheme, ScatterAnimation, easing} from "scichart";

export async function scatterDataAnimation(divId) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId, {
        theme: new SciChartJSLightTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(0, 5) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(0, 5) }));
    // Create a scatter series with some initial data
    const scatterSeries = new XyScatterRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [1, 2, 3, 4, 5],
            yValues: [1.3, 2.3, 4, 3.3, 4.5]
        }),
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 11, height: 11, fill: "#FF3333BB", strokeThickness: 0
        })
    });
    sciChartSurface.renderableSeries.add(scatterSeries);
    // create a temp series for passing animation values
    const animationSeries = new XyDataSeries(wasmContext);
    // register this so it is deleted along with the main surface
    sciChartSurface.addDeletable(animationSeries);
    // Update data using data animations
    const animateData = () => {
        const xValues = Array.from({length: 5}, () => Math.random() * 5);
        const yValues = Array.from({length: 5}, () => Math.random() * 5);
        // Set the values on the temp series
        animationSeries.clear();
        animationSeries.appendRange(xValues, yValues);
        scatterSeries.runAnimation(new ScatterAnimation({
            duration: 500,
            ease: easing.outQuad,
            // Do not create a new DataSeries here or it will leak and eventually crash.
            dataSeries: animationSeries
        }));
        setTimeout(animateData, 1000);
    };
    setTimeout(animateData, 1000);
}
```

<img src="out_scichartv4/2d-charts/animations-api/dataset-animations/index_media/2e9f1554a990978537660813dc8e8c725acfa51a.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Combining Style and Data Animations<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/#combining-style-and-data-animations" class="hash-link" aria-label="Direct link to Combining Style and Data Animations" translate="no" title="Direct link to Combining Style and Data Animations">â€‹</a>

You can take the example above a step further and combine both style and data animations. Remember the constraint that datasets need the same amount of X,Y datapoints before and after. If this condition is met, you can achieve something like this:

``` prism-code
// Style and data animation

import {SciChartSurface, NumericAxis, EllipsePointMarker, XyDataSeries, NumberRange, XyScatterRenderableSeries, SciChartJSLightTheme, ScatterAnimation, easing, EPointMarkerType} from "scichart"

export async function scatterDataAnimationWithStyle(divId) {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId, {
        theme: new SciChartJSLightTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(0, 5) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(0, 5) }));
    // Create a scatter series with some initial data
    const scatterSeries = new XyScatterRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [1, 2, 3, 4, 5],
            yValues: [1.3, 2.3, 4, 3.3, 4.5]
        }),
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 11, height: 11, fill: "#FF3333BB", strokeThickness: 0
        })
    });
    sciChartSurface.renderableSeries.add(scatterSeries);
    // create a temp series for passing animation values
    const animationSeries = new XyDataSeries(wasmContext);
    // register this so it is deleted along with the main surface
    sciChartSurface.addDeletable(animationSeries);
    // Update data using data animations
    const animateDataAndStyle = () => {
        const xValues = Array.from({length: 5}, () => Math.random() * 5);
        const yValues = Array.from({length: 5}, () => Math.random() * 5);
        const randomColor = () => '#'+(0x1000000+Math.random()*0xffffff).toString(16).substr(1,6);
        const fillColor = randomColor();
        const strokeColor = randomColor();
        const size = Math.random() * 12 + 5;
        const pointMarkers = [EPointMarkerType.Ellipse, EPointMarkerType.Triangle, EPointMarkerType.Square];
        const randomMarker = () => pointMarkers[Math.floor(Math.random() * 3)];
        // Set the values on the temp series
        animationSeries.clear();
        animationSeries.appendRange(xValues, yValues);
        scatterSeries.runAnimation(new ScatterAnimation({
            duration: 500,
            ease: easing.outQuad,
            styles: {
                pointMarker: {
                    type: randomMarker(),
                    width: size,
                    height: size,
                    strokeThickness: 3,
                    stroke: strokeColor,
                    fill: fillColor
                }
            },
            // Do not create a new DataSeries here or it will leak and eventually crash.
            dataSeries: animationSeries
        }));
        setTimeout(animateDataAndStyle, 1000);
    };
    setTimeout(animateDataAndStyle, 1000);
}
```

<img src="out_scichartv4/2d-charts/animations-api/dataset-animations/index_media/5764edcacdfc233b985daf3ea4faab91136570ca.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Animating Data in a Column Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/#animating-data-in-a-column-series" class="hash-link" aria-label="Direct link to Animating Data in a Column Series" translate="no" title="Direct link to Animating Data in a Column Series">â€‹</a>

``` prism-code
// Column dataset animation

const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJSLightTheme()
});
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
const columnSeries = new FastColumnRenderableSeries(wasmContext, {
    fill: "rgba(176, 196, 222, 1)",
    stroke: "#4682b4",
    strokeThickness: 2,
    dataPointWidth: 0.5,
    opacity: 0.7
});
sciChartSurface.renderableSeries.add(columnSeries);
const dataSeries = new XyDataSeries(wasmContext);
for (let i = 0; i < 20; i++) {
    dataSeries.append(i, Math.sin(i * 0.5));
}
columnSeries.dataSeries = dataSeries;
const dataSeries1 = new XyDataSeries(wasmContext);
for (let i = 0; i < 20; i++) {
    dataSeries1.append(5 + i / 2, Math.cos(i * 0.5));
}
// register this so it is deleted along with the main surface
sciChartSurface.addDeletable(dataSeries1 );

columnSeries.runAnimation(
    new ColumnAnimation({
        duration: 3000,
        dataSeries: dataSeries1
    })
);
sciChartSurface.zoomExtents();
return { wasmContext, sciChartSurface };
```

Below is the result.

<img src="out_scichartv4/2d-charts/animations-api/dataset-animations/index_media/f23f563c540a71b2e8462225d834780e9f9f598b.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Animations API](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview)
- [Series Startup Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations)
- [Style Transition Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations)
- [Generic Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/generic-animations)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/animations-api/dataset-animations/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/animations-api/dataset-animations/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
