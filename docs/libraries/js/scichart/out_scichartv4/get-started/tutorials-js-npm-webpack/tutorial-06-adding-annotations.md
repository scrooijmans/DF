On this page

# Tutorial 06 - Adding Annotations

InÂ [Tutorial 5 - Zoom and Pan with Realtime Updates](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-05-zoom-and-pan-with-realtime-updates), we showed you how to doÂ zooming and panning having realtime updates. In this tutorial, were going to show you how to add annotations.

The annotation API allows you to add other UIÂ elements, markers or labelsÂ to a chart, like **Lines, **Text, **Boxes, **SVG elements and more.********

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_6_Adding_Annotations" rel="noopener noreferrer" target="_blank">SciChart.JS.Examples Github Repository</a>.

# Ein Fehler ist aufgetreten.

JavaScript kann nicht ausgeführt werden.

Video tutorial for version 3. SciChart.js JavaScript Chart Tutorial 06 - Adding Annotations to Charts

  

## Chart Annotations in SciChart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/#chart-annotations-in-scichart" class="hash-link" aria-label="Direct link to Chart Annotations in SciChart" translate="no" title="Direct link to Chart Annotations in SciChart">â€‹</a>

The SciChart annotations derive from the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html" rel="noopener noreferrer" target="_blank">IAnnotationðŸ“˜</a> interface.

The annotation typesÂ included with SciChart.js out of the box are: [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation), [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation), [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation), [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation), [HorizontalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/horizontal-line-annotation),Â [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview),Â [NativeTextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation) and other.

It's also possible to add custom shapes and markers to the chart usingÂ [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation).

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The full list of supported annotations can be found on the [Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview) page.

## Adding Annotations to the Chart<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/#adding-annotations-to-the-chart" class="hash-link" aria-label="Direct link to Adding Annotations to the Chart" translate="no" title="Direct link to Adding Annotations to the Chart">â€‹</a>

In this tutorial we will create a simple example showing how to add different annotation types to a chart.

First we create aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>, then we add X and Y Axes as normal, and finally we add aÂ [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation)Â by adding an instance to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#annotations" rel="noopener noreferrer" target="_blank">SciChartSurface.annotationsðŸ“˜</a> collection.

The code is shown below:

- Part1/index.js

``` prism-code
import { SciChartSurface, NumericAxis, LineAnnotation } from "scichart";

async function initSciChart() {
    // Create the SciChartSurface in the div 'scichart-root'
    // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
    // instance must be passed to other types that exist on the same surface.
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(
        "scichart-root"
    );

    // Create an X,Y Axis and add to the chart
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Add line annotation
    sciChartSurface.annotations.add(
        new LineAnnotation({
        stroke: "#FF6600",
        strokeThickness: 3,
        x1: 1.0,
        x2: 4.0,
        y1: 6.0,
        y2: 9.0,
        })
    );
}

initSciChart();
```

This code produces this following chart with an X,Y axis and a singleÂ [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation).

Â <img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/f3fd79326c00f8b4c8fbb088e756ec2d3739e0d2.png" class="img_ev3q" decoding="async" loading="lazy" width="1021" height="996" />

In order to addÂ other annotation typesÂ to the chart passÂ appropriate annotation toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#annotations" rel="noopener noreferrer" target="_blank">sciChartSurface.annotations.add()ðŸ“˜</a>. In the code below we addÂ [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation) and [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation)Â to the chart.

- Part2/index.js region A

``` prism-code
// Add line annotation
sciChartSurface.annotations.add(
    new LineAnnotation({
    stroke: "#FF6600",
    strokeThickness: 3,
    x1: 1.0,
    x2: 4.0,
    y1: 6.0,
    y2: 9.0,
    })
);

// Add box annotation
sciChartSurface.annotations.add(
    new BoxAnnotation({
    stroke: "#33FF33",
    strokeThickness: 1,
    fill: "rgba(50, 255, 50, 0.3)",
    x1: 6.0,
    x2: 9.0,
    y1: 6.0,
    y2: 9.0,
    })
);
```

Let's also try to add aÂ [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation).

Note that this annotation type only requires x1,y1, whereasÂ [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation) and [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation)Â require x1,x2,y1,y2 to define their bounds.

Instead theÂ [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation)Â is placed at a single x,y point, and the location of that point is defined by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#horizontalanchorpoint" rel="noopener noreferrer" target="_blank">horizontalAnchorPointðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#verticalanchorpoint" rel="noopener noreferrer" target="_blank">verticalAnchorPointðŸ“˜</a> properties.

- Part2/index.js region B

``` prism-code
// Add text annotation
sciChartSurface.annotations.add(
    new TextAnnotation({
    x1: 0.25,
    y1: 0.75,
    xCoordinateMode: ECoordinateMode.Relative,
    yCoordinateMode: ECoordinateMode.Relative,
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center,
    textColor: "yellow",
    fontSize: 26,
    fontFamily: "Comic Sans MS",
    text: "TEXT ANNOTATION",
    })
);
```

The next annotation type we're going to addÂ is aÂ [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation).

This allows you toÂ create custom SVG markers and add them to the chart. CustomAnnotations require only x1,y1 properties and obeyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#horizontalanchorpoint" rel="noopener noreferrer" target="_blank">horizontalAnchorPointðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#verticalanchorpoint" rel="noopener noreferrer" target="_blank">verticalAnchorPointðŸ“˜</a>Â properties like TextAnnotations do.

- Part2/index.js region C

``` prism-code
// Add custom SVG annotation
const svgString = `
<svg baseProfile="full" width="200" height="200" xmlns="http://www.w3.org/2000/svg">
    <circle cx="100" cy="100" r="100" fill="rgba(50,50,255,0.3)" />
    <text x="100" y="125" font-size="60" text-anchor="middle" fill="white">SVG</text>
</svg>`;
sciChartSurface.annotations.add(
    new CustomAnnotation({
    x1: 7.5,
    y1: 2.5,
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center,
    svgString,
    })
);
```

This results in aÂ chart with four different annotation.

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/f6ed46915fbd9db2b6f0891f27ddda375f655511.png" class="img_ev3q" decoding="async" loading="lazy" width="1021" height="996" />

## Further APIs<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/#further-apis" class="hash-link" aria-label="Direct link to Further APIs" translate="no" title="Direct link to Further APIs">â€‹</a>

Annotations support the ability to be docked to the left/right/top/bottom of the chart viewport. You can adjust the docking using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#xcoordinatemode" rel="noopener noreferrer" target="_blank">xCoordinateModeðŸ“˜</a> /Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#ycoordinatemode" rel="noopener noreferrer" target="_blank">yCoordinateModeðŸ“˜</a> properties.

Some annotations supportÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#verticalanchorpoint" rel="noopener noreferrer" target="_blank">verticalAnchorPointðŸ“˜</a>Â /Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#horizontalanchorpoint" rel="noopener noreferrer" target="_blank">horizontalAnchorPointðŸ“˜</a>Â properties. These allow you to change the control point for annotations which have a single X,Y point.

A combination of anchor points and coordinate modes can allow you to create text watermarks on charts, or boxes which stretch horizontally or vertically over a chart.

For example:

- Part2/index.js region D

``` prism-code
// Add a watermark centered on the chart
sciChartSurface.annotations.add(
    new TextAnnotation({
    x1: 0.5,
    y1: 0.5,
    xCoordinateMode: ECoordinateMode.Relative,
    yCoordinateMode: ECoordinateMode.Relative,
    horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
    verticalAnchorPoint: EVerticalAnchorPoint.Center,
    text: "THIS IS A WATERMARK",
    opacity: 0.33,
    fontSize: 27,
    })
);

// Add a box vertically stretched between data-points X=4, X=5
sciChartSurface.annotations.add(
    new BoxAnnotation({
    x1: 4,
    x2: 5,
    // y:0-1 Relative means stretch vertically
    y1: 0,
    y2: 1,
    yCoordinateMode: ECoordinateMode.Relative,
    strokeThickness: 0,
    fill: "#ff660033",
    })
);
```

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/4013c2bd51d7e04edc71962b237c445c9eddba3c.png" class="img_ev3q" decoding="async" loading="lazy" width="1021" height="996" />

## View our Annotations Demos online<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/#view-our-annotations-demos-online" class="hash-link" aria-label="Direct link to View our Annotations Demos online" translate="no" title="Direct link to View our Annotations Demos online">â€‹</a>

You can find outÂ more about the Annotations API in the relevant section of the documentation:Â [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview).

Also, there is a couple of examplesÂ showcasingÂ annotations in our Examples Suite. See theÂ <a href="https://www.scichart.com/demo/react/chart-annotations" rel="noopener noreferrer" target="_blank">JavaScript Chart Annotations example</a>Â for more details.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/chart-annotations" target="_blank">Chart Annotations</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing how create various kinds of annotations and animate them in javascript charts.

  

In this exampleÂ we show how to create multiple annotation types, including liners, text, watermarks, stretched boxes, images, vectors (SVG) and horizontal/vertical lines.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The full source code for the Annotations demo can be found below, as well as onÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/master/Examples/src/components/Examples/Charts2D/ChartAnnotations/AnnotationsAreEasy/drawExample.ts" rel="noopener noreferrer" target="_blank">Github</a>.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Tutorial 04 - Adding Realtime Updates](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-04-adding-realtime-updates)
- [Tutorial 08 - Adding Multiple Axis](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-06-adding-annotations/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
