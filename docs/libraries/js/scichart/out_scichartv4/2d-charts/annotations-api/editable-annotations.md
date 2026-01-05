On this page

# Editable Annotations

The annotations API allows you to mark any annotation as editable by setting isEditable true.Â  Editable annotations can be selected and dragged, and some can be resized.Â  This page describes how you can respond to a user's interaction with an annotation, and how toÂ **customise the style of the selected view** of the annotation.

## Annotation Interactions<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/editable-annotations/#annotation-interactions" class="hash-link" aria-label="Direct link to Annotation Interactions" translate="no" title="Direct link to Annotation Interactions">â€‹</a>

All annotations have the following properties andÂ events which can be used to run code on user interaction:

| **AnnotationBase Property** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#iseditable" rel="noopener noreferrer" target="_blank">isEditableðŸ“˜</a> | When true, an annotation is editable. It may be selected, dragged or resized. Individual behaviours can be controlled by the following properties. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#isselected" rel="noopener noreferrer" target="_blank">isSelectedðŸ“˜</a> | Set true when an editable annotation is clicked. This causes the selection box and the drag points to be shown. These are known as the adorners. Setting this programatically is not advised |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#selectedchanged" rel="noopener noreferrer" target="_blank">selectedChangedðŸ“˜</a> | An event that is fired when isSelected changes. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragstarted" rel="noopener noreferrer" target="_blank">dragStartedðŸ“˜</a> / <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ondragstarted" rel="noopener noreferrer" target="_blank">onDragStartedðŸ“˜</a> | dragStarted is an event which fires on mouseDown of an editable annotation. This is fired by the call to onDragStarted which is overridden in various annotations to determine which dragging point is being used, setting the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#adornerdraggingpoint" rel="noopener noreferrer" target="_blank">adornerDraggingPointðŸ“˜</a> property. If this is not set, dragging will not be performed. You can pass a callback for the event via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#ondragstarted" rel="noopener noreferrer" target="_blank">onDragStartedðŸ“˜</a> property of the IAnnotationsBase options object when constructing. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragdelta" rel="noopener noreferrer" target="_blank">dragDeltaðŸ“˜</a> / <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#onDragAdorner" rel="noopener noreferrer" target="_blank">onDragAdornerðŸ“˜</a> | dragDelta is the event which fires during dragging. This is fired by the call to onDragAdorner which translates the mouse point to xy coordinates and calls <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#calcdragdistance" rel="noopener noreferrer" target="_blank">calcDragDistanceðŸ“˜</a>, which is where the coordinates of the annotation are updated. You can pass a callback for the event via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#ondrag" rel="noopener noreferrer" target="_blank">onDragðŸ“˜</a> property of the IAnnotationsBase options object when constructing. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragended" rel="noopener noreferrer" target="_blank">dragEndedðŸ“˜</a> / <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#onDragEnded" rel="noopener noreferrer" target="_blank">onDragEndedðŸ“˜</a> | dragEnded is an event which fires on mouseUp when dragging has finished. This is fired by the call to onDragEnded. You can pass a callback for the event via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#ondragended" rel="noopener noreferrer" target="_blank">onDragEndedðŸ“˜</a> property of the IAnnotationsBase options object when constructing. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#resizedirections" rel="noopener noreferrer" target="_blank">resizeDirectionsðŸ“˜</a> | Controls which direction an annotation may be resized in, e.g. X, Y or Xy |

## Enabling and Subscribing to Drag Events in Annotations<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/editable-annotations/#enabling-and-subscribing-to-drag-events-in-annotations" class="hash-link" aria-label="Direct link to Enabling and Subscribing to Drag Events in Annotations" translate="no" title="Direct link to Enabling and Subscribing to Drag Events in Annotations">â€‹</a>

Below is an example of how to enable editing (dragging) on aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a>, as well as how to get a callback on when the annotation is updated.

- TS

``` prism-code
// A TextAnnotation which can be dragged and updates its value on drag
const textAnnotation = new TextAnnotation({
    x1: 1,
    y1: 3,
    fontSize: 24,
    fontFamily: "Default",
    text: "{{ DRAG ME! }}",
    isEditable: true
});

textAnnotation.dragDelta.subscribe(args => {
    textAnnotation.text = `I was dragged to ${textAnnotation.x1.toFixed(2)}, ${textAnnotation.y1.toFixed(2)}`;
});

sciChartSurface.annotations.add(textAnnotation);
```

Try it out below by dragging the annotation.

## Dragging to discrete values<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/editable-annotations/#dragging-to-discrete-values" class="hash-link" aria-label="Direct link to Dragging to discrete values" translate="no" title="Direct link to Dragging to discrete values">â€‹</a>

Sometimes you want an annotation to snap to particular values as you drag.Â  The way to do this is to overrideÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ondragadorner" rel="noopener noreferrer" target="_blank">onDragAdornerðŸ“˜</a> and convert to discete points there, then pass these toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#calcdragdistancecalcDragDistance" rel="noopener noreferrer" target="_blank">calcDragDistanceðŸ“˜</a>.Â Here is an example of anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html" rel="noopener noreferrer" target="_blank">AxisMarkerAnnotationðŸ“˜</a> that can only take discrete values, from ourÂ <a href="https://www.scichart.com/example/javascript-chart/javascript-heatmap-interactions/" rel="noopener noreferrer" target="_blank">Rich Interactions Demo</a>.

- TS

``` prism-code
class DiscreteAxisMarker extends AxisMarkerAnnotation {
    stepSize = 1;
    minValue = 0;
    maxValue = 10;

    constructor(options) {
        super(options);
    }

    onDragAdorner(args) {
        super.onDragAdorner(args);
        const xyValues = this.getValuesFromCoordinates(args.mousePoint, true);
        if (xyValues) {
            let { x, y } = xyValues;
            if (this.x1 !== undefined) {
                x = Math.floor(x / this.stepSize) * this.stepSize;
            } else if (this.y1 !== undefined) {
                y = Math.floor(y / this.stepSize) * this.stepSize;
            }
            this.calcDragDistance(new Point(x, y));
            if (this.x1 !== undefined) {
                this.x1 = Math.min(Math.max(this.x1, this.minValue), this.maxValue);
            } else if (this.y1 !== undefined) {
                this.y1 = Math.min(Math.max(this.y1, this.minValue), this.maxValue);
            }
        }
        this.dragDelta.raiseEvent(new AnnotationDragDeltaEventArgs());
    }
}

// Now add to the SciChartSurface:
// sciChartSurface.annotations.add(new DiscreteAxisMarker({ y1: 5, formattedValue: "Drag Me!" }));
```

Try it out below by dragging the axis marker:

## Limiting Resize to Specific Directions (x,y)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/editable-annotations/#limiting-resize-to-specific-directions-xy" class="hash-link" aria-label="Direct link to Limiting Resize to Specific Directions (x,y)" translate="no" title="Direct link to Limiting Resize to Specific Directions (x,y)">â€‹</a>

Another property of interactable annotation is the dimension where it can be moved or resized. By default it is possible to move aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" rel="noopener noreferrer" target="_blank">BoxAnnotationðŸ“˜</a> towards each side of the chart. In the next example we will demonstrate a usage of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#resizedirections" rel="noopener noreferrer" target="_blank">AnnotationBase.resizeDirectionsðŸ“˜</a> property. We will limit the annotation to resize and move only along the X Axis.

It is also possible to restrict the drag direction of the box annotation by subscribing to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragdelta" rel="noopener noreferrer" target="_blank">dragDeltaðŸ“˜</a> event callback. Find an example below.

- TS
- Builder API (JSON Config)

``` prism-code
// A box annotation which can only be dragged in the X-direction
const boxAnnotation = new BoxAnnotation({
    x1: 3,
    x2: 7,
    y1: 3,
    y2: 7,
    isEditable: true,
    isSelected: true,
    // Restricts resize direction in the X-direction only
    resizeDirections: EXyDirection.XDirection
});
// Restricts drag direction in the X-direction only
boxAnnotation.dragDelta.subscribe(arg => {
    boxAnnotation.y1 = 3;
    boxAnnotation.y2 = 7;
});
sciChartSurface.annotations.add(boxAnnotation);
```

``` prism-code
const { chartBuilder, EAnnotationType } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                x1: 3,
                x2: 7,
                y1: 3,
                y2: 7,
                isEditable: true,
                isSelected: true,
                // custom resize direction
                resizeDirections: EXyDirection.XDirection
            }
        }
    ]
});
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/editable-annotations/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)
- [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation)
- [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/editable-annotations/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/editable-annotations/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
