On this page

# LineArrowAnnotation

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html" rel="noopener noreferrer" target="_blank">LineArrowAnnotationðŸ“˜</a> class extends <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html" rel="noopener noreferrer" target="_blank">LineAnnotationðŸ“˜</a> and adds an optional arrowhead at each line endpoint. Use it to annotate charts with directional indicators.

## Create a Line Arrow Annotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-arrow-annotation/#create-a-line-arrow-annotation" class="hash-link" aria-label="Direct link to Create a Line Arrow Annotation" translate="no" title="Direct link to Create a Line Arrow Annotation">â€‹</a>

The following code will declare 2 <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html" rel="noopener noreferrer" target="_blank">LineArrowAnnotationðŸ“˜</a>s and add them to the chart.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    SciChartSurface,
    SciChartJsNavyTheme,
    LineArrowAnnotation,
    NumericAxis,
    EArrowHeadPosition,
    EDraggingGripPoint,
} = SciChart;

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

const arrow1 = new LineArrowAnnotation({
    x1: 1,
    x2: 6,
    y1: 2,
    y2: 9,
    isArrowHeadScalable: true, // the arrow head will scale with the visibleRange
    arrowStyle: {
        headLength: 40,
        headWidth: 45,
        headDepth: 0.8,
        fill: "#113388",
        stroke: "#3399FF",
        strokeThickness: 3
    },
    stroke: "#3399FF",
    strokeThickness: 3,
    isEditable: true,
    isSelected: true,
    arrowHeadPosition: EArrowHeadPosition.End, // only show arrow head at the end
    dragPoints: [ EDraggingGripPoint.x2y2 ], // only allow dragging by the end point
});

const arrow2 = new LineArrowAnnotation({
    x1: 4,
    x2: 9,
    y1: 2,
    y2: 9,
    isArrowHeadScalable: false,
    arrowStyle: {
        headLength: 25,
        headWidth: 25,
        headDepth: 1,
        fill: "#883300",
        stroke: "#FF6600",
        strokeThickness: 3
    },
    stroke: "#FF6600",
    strokeThickness: 3,
    arrowHeadPosition: EArrowHeadPosition.StartEnd, // show arrow heads on both ends
    isEditable: true,
    dragPoints: [
        EDraggingGripPoint.x1y1, 
        EDraggingGripPoint.x2y2
    ], // allow dragging by both end points
});

// append the annotations to the surface
sciChartSurface.annotations.add(arrow1, arrow2);
```

``` prism-code
const { 
    chartBuilder, 
    EAnnotationType, 
    EArrowHeadPosition,
    EDraggingGripPoint,
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: {
        theme: { type: "Navy" },
    },
    annotations: [
        {
            type: EAnnotationType.RenderContextLineArrowAnnotation,
            options: {
                stroke: "#3399FF",
                strokeThickness: 3,
                x1: 1,
                y1: 2, // start point (1, 2)
                x2: 6,
                y2: 9, // end point (6, 9)

                arrowHeadPosition: EArrowHeadPosition.End, // show arrow head at the end
                isArrowHeadScalable: true, // the arrow head will scale with the visibleRange

                isEditable: true,
                dragPoints: [ EDraggingGripPoint.x2y2 ], // allow drag editing by the end point

                arrowStyle: {
                    headLength: 40,
                    headWidth: 45,
                    headDepth: 0.8,
                    fill: "#113388",
                    stroke: "#3399FF",
                    strokeThickness: 3
                }
            }
        },
        {
            type: EAnnotationType.RenderContextLineArrowAnnotation,
            options: {
                stroke: "#FF6600",
                strokeThickness: 3,
                x1: 4,
                y1: 1, // start point (4, 1)
                x2: 9,
                y2: 8, // end point (9, 8)

                arrowHeadPosition: EArrowHeadPosition.StartEnd, // show arrow heads on both ends
                isArrowHeadScalable: false,

                isEditable: true,
                dragPoints: [
                    EDraggingGripPoint.x1y1,
                    EDraggingGripPoint.x2y2
                ], // allow drag editing by both end points

                arrowStyle: {
                    headLength: 40,
                    headWidth: 45,
                    headDepth: 0.8,
                    fill: "#883300",
                    stroke: "#FF6600",
                    strokeThickness: 3
                }
            }
        },
    ],
});
```

Resulting in the following output:

In the code above:

- We create 2 instances of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html" rel="noopener noreferrer" target="_blank">LineArrowAnnotationðŸ“˜</a> with different arrowhead styles.
- The first annotation has an arrowhead at the end, while the second has 2 arrowheads, 1 at both ends.
- Also, the first annotation's arrowhead will scale with the zooming, while the second will not.
- In case you want to use editable annotations using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html#iseditable" rel="noopener noreferrer" target="_blank">isEditableðŸ“˜</a>: `true`, you can use the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html#dragpoints" rel="noopener noreferrer" target="_blank">dragPointsðŸ“˜</a> property to pass an array of valid drag points. See <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edragginggrippoint.html" rel="noopener noreferrer" target="_blank">EDraggingGripPointðŸ“˜</a> for more details.

### Unique Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-arrow-annotation/#unique-properties" class="hash-link" aria-label="Direct link to Unique Properties" translate="no" title="Direct link to Unique Properties">â€‹</a>

| Property | Type | Default | Description |
|----|----|----|----|
| arrowHeadPosition | <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/earrowheadposition.html" rel="noopener noreferrer" target="_blank">EArrowHeadPositionðŸ“˜</a> | *End* | Where to place the arrowhead(s) -\> Start, End, Start&End |
| isArrowHeadScalable | boolean | *false* | Whether to scale the arrowhead size with zooming. |
| arrowStyle | <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iarrowstyle.html" rel="noopener noreferrer" target="_blank">IArrowStyleðŸ“˜</a> | \- | Arrowhead style options. |
| onArrowHeadSizeChanged | <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tarrowheadsizechangedcallback" rel="noopener noreferrer" target="_blank">TArrowheadSizeChangedCallbackðŸ“˜</a> | *undefined* | Callback to modify arrowhead size based on angle. |

All other properties are inherited from <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineannotation.html" rel="noopener noreferrer" target="_blank">LineAnnotationðŸ“˜</a> & <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" rel="noopener noreferrer" target="_blank">AnnotationBaseðŸ“˜</a> and work as expected.

![](out_scichartv4/2d-charts/annotations-api/line-arrow-annotation/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Use `headDepth: 0` for simple line arrows without filled heads.

![](out_scichartv4/2d-charts/annotations-api/line-arrow-annotation/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Set `isArrowHeadScalable: true` for annotations that maintain relative size during zooming.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/line-arrow-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/line-arrow-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
