On this page

# Styling Annotation Selection

TheÂ Annotations API allows to customize the interaction adorners style of an editable annotation. This includes:

1.  specifying the grip points that could be used for interaction with the annotation
2.  specifying the radius of the grip points
3.  setting a custom svg template for the grips

There is a number of common properties which could be used to customize the look and behavior of interactive annotations. They could be passed as constructor options, which are described by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html" rel="noopener noreferrer" target="_blank">IAnnotationBaseOptionsðŸ“˜</a>. And the relevant properties are defined as follows:

- Discrete Dragging

``` prism-code
/** The direction in which the annotation is allowed to be resized or dragged */
resizeDirections?: EXyDirection;
/** if true, the annotation is editable (can be dragged and manipulated by the user) */
isEditable?: boolean;
/** The stroke color for the adorner drag handle */
annotationsGripsStroke?: string;
/** The fill color for the adorner drag handle */
annotationsGripsFill?: string;
/** The radius of the adorner drag handle */
annotationsGripsRadius?: number;
/** The stroke color for the adorner selection box */
selectionBoxStroke?: string;
/** How much bigger the selection box is than the bounding box of the annotation, in pixels */
selectionBoxDelta?: number;
/** The thickness of the selection box line */
selectionBoxThickness?: number;
/** The dragPoints that should be enabled for this annotation */
dragPoints?: readonly EDraggingGripPoint[];
```

Also, the same properties could be modified using the properties on an annotation instance:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#resizedirections" rel="noopener noreferrer" target="_blank">AnnotationBase.resizeDirectionsðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#annotationsgripsstroke" rel="noopener noreferrer" target="_blank">AnnotationBase.annotationsGripsStrokeðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#annotationsgripsfill" rel="noopener noreferrer" target="_blank">AnnotationBase.annotationsGripsFillðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#annotationsgripsradius" rel="noopener noreferrer" target="_blank">AnnotationBase.annotationsGripsRadiusðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#selectionboxstroke" rel="noopener noreferrer" target="_blank">AnnotationBase.selectionBoxStrokeðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#selectionboxdelta" rel="noopener noreferrer" target="_blank">AnnotationBase.selectionBoxDeltaðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#selectionboxthickness" rel="noopener noreferrer" target="_blank">AnnotationBase.selectionBoxThicknessðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#dragpoints" rel="noopener noreferrer" target="_blank">AnnotationBase.dragPointsðŸ“˜</a>

## Default Adorners Style<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/styling-annotation-selection/#default-adorners-style" class="hash-link" aria-label="Direct link to Default Adorners Style" translate="no" title="Direct link to Default Adorners Style">â€‹</a>

We will start by creating aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" rel="noopener noreferrer" target="_blank">BoxAnnotationðŸ“˜</a> on a surface and will use it as a boilerplate for further examples (other types of annotations could be modified similarly). In order to make the annotation interactable, we will setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#iseditable" rel="noopener noreferrer" target="_blank">IAnnotationBaseOptions.isEditableðŸ“˜</a> flag. Also we will setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#isselected" rel="noopener noreferrer" target="_blank">IAnnotationBaseOptions.isSelectedðŸ“˜</a> to display adorners of the annotation.

- TS

``` prism-code
// ...
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId);

sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

const boxAnnotation = new BoxAnnotation({
    x1: 3,
    x2: 7,
    y1: 3,
    y2: 7,
    isEditable: true,
    isSelected: true,
});
sciChartSurface.annotations.add(boxAnnotation);
```

As you can see the adorners consist of the outlining **selection box** used for highlighting a selected annotation, and **dragging grip points** - used to resize or move an annotation when dragged by a cursor.Â 

<img src="out_scichartv4/2d-charts/annotations-api/styling-annotation-selection/index_media/596964f2387b0894160d2ecfc8a44d1bee9e2761.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Custom Adorners Style<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/styling-annotation-selection/#custom-adorners-style" class="hash-link" aria-label="Direct link to Custom Adorners Style" translate="no" title="Direct link to Custom Adorners Style">â€‹</a>

Here we will demonstrate how to apply custom styles for the adorners. So in this example we changed the colors and sizes of the selection box and grip points.

Modify the code as follow:

- TS
- Builder API (JSON Config)

``` prism-code
// Add a box annotation to the chart and make it editable and selected
// Style the adorners (selectiong grips) using properties
const boxAnnotation = new BoxAnnotation({
    x1: 3,
    x2: 7,
    y1: 3,
    y2: 7,
    isEditable: true,
    isSelected: true,
    // add custom styling
    annotationsGripsStroke: "Blue",
    annotationsGripsFill: "Black",
    selectionBoxStroke: "Green",
    annotationsGripsRadius: 10,
    selectionBoxDelta: 30,
    selectionBoxThickness: 9
});
sciChartSurface.annotations.add(boxAnnotation);
```

``` prism-code
const { chartBuilder, EAnnotationType } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

// Add a box annotation to the chart using the builder API and make it editable and selected
// Style the adorners (selectiong grips) using properties
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
                // add custom styling
                annotationsGripsStroke: "Blue",
                annotationsGripsFill: "Black",
                selectionBoxStroke: "Green",
                annotationsGripsRadius: 10,
                selectionBoxDelta: 30,
                selectionBoxThickness: 9
            }
        }
    ]
});
```

Â This results in the following output:

## Modifying Dragging Grip Points<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/styling-annotation-selection/#modifying-dragging-grip-points" class="hash-link" aria-label="Direct link to Modifying Dragging Grip Points" translate="no" title="Direct link to Modifying Dragging Grip Points">â€‹</a>

By default, an annotation uses all of the predefined grip points for interactions (corners and body), but this can be changed to allow dragging and resizing only using specific ones. For this we will use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#dragpoints" rel="noopener noreferrer" target="_blank">IAnnotationBaseOptions.dragPointsðŸ“˜</a> (orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragpoints" rel="noopener noreferrer" target="_blank">AnnotationBase.dragPointsðŸ“˜</a>) property.

- TS
- Builder API (JSON Config)

``` prism-code
// Add a box annotation to the chart and make it editable and selected
// Style the adorners (selectiong grips) using properties
const boxAnnotation = new BoxAnnotation({
    x1: 3,
    x2: 7,
    y1: 3,
    y2: 7,
    isEditable: true,
    isSelected: true,
    // add custom styling
    annotationsGripsStroke: "Blue",
    annotationsGripsFill: "Black",
    selectionBoxStroke: "Green",
    annotationsGripsRadius: 10,
    selectionBoxDelta: 30,
    selectionBoxThickness: 9,
    // Modify the number of grip points
    dragPoints: [EDraggingGripPoint.Body, EDraggingGripPoint.x2y1]
});
sciChartSurface.annotations.add(boxAnnotation);
```

``` prism-code
const { chartBuilder, EAnnotationType } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

// Add a box annotation to the chart using the builder API and make it editable and selected
// Style the adorners (selectiong grips) using properties
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
                // add custom styling
                annotationsGripsStroke: "Blue",
                annotationsGripsFill: "Black",
                selectionBoxStroke: "Green",
                annotationsGripsRadius: 10,
                selectionBoxDelta: 30,
                selectionBoxThickness: 9,
                // Modify the number of grip points
                dragPoints: [EDraggingGripPoint.Body, EDraggingGripPoint.x2y1]
            }
        }
    ]
});
```

This modifies the output as follows:Â 

## Custom Adorners SVG<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/styling-annotation-selection/#custom-adorners-svg" class="hash-link" aria-label="Direct link to Custom Adorners SVG" translate="no" title="Direct link to Custom Adorners SVG">â€‹</a>

More advanced option to customize the adorners is to override the SVG template for the selection box and grips. To do this we can create a derived annotation class, which in this case extendsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" rel="noopener noreferrer" target="_blank">BoxAnnotationðŸ“˜</a>. In the class we are overriding theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#getannotationgripsvg" rel="noopener noreferrer" target="_blank">AnnotationBase.getAnnotationGripSvgðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#svgstringadornertemplate" rel="noopener noreferrer" target="_blank">AnnotationBase.svgStringAdornerTemplateðŸ“˜</a> methods, which are used to create the adorners.

- TS

``` prism-code
class CustomBoxAnnotation extends BoxAnnotation {
    getAnnotationGripSvg(x, y) {
        const size = this.annotationsGripsRadius;
        return `<rect x="${x - size / 2}" y="${y - size / 2}" width="${size}" height="${size}" fill="${
            this.annotationsGripsFill
        }" stroke="${this.annotationsGripsStroke}"/>`;
    }

    svgStringAdornerTemplate(x1, y1, x2, y2) {
        const width = x2 - x1;
        const height = y2 - y1;
        let svg = `<svg xmlns="http://www.w3.org/2000/svg">
                        <style type="text/css">
                            line { stroke: #474747;  }
                        </style>
                        <defs>
                            <pattern id="grid1" patternUnits="userSpaceOnUse" width="10" height="10">
                                <line x1="0" y1="0" x2="10" y2="10" />
                            </pattern>
                        </defs>
                        <rect x="${x1}" y="${y1}" width="${width}" height="${height}" fill="url(#grid1)"/>
                        `;

        const grips = this.getAdornerAnnotationBorders(false, true);
        if (this.canDragPoint(EDraggingGripPoint.x1y1)) {
            svg += this.getAnnotationGripSvg(grips.x1, grips.y1);
        }
        if (this.canDragPoint(EDraggingGripPoint.x2y2)) {
            svg += this.getAnnotationGripSvg(grips.x2, grips.y2);
        }
        if (this.canDragPoint(EDraggingGripPoint.x2y1)) {
            svg += this.getAnnotationGripSvg(grips.x2, grips.y1);
        }
        if (this.canDragPoint(EDraggingGripPoint.x1y2)) {
            svg += this.getAnnotationGripSvg(grips.x1, grips.y2);
        }
        svg += "</svg>";
        return svg;
    }
}
```

Then simply create and use an instance of the customized annotation:

- Custom Adorners SVG

``` prism-code
// Then create a box annotation using the CustomBoxAnnotation class
const boxAnnotation = new CustomBoxAnnotation({
    x1: 3,
    x2: 7,
    y1: 3,
    y2: 7,
    isEditable: true,
    isSelected: true
});
sciChartSurface.annotations.add(boxAnnotation);
```

This results in the following output:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/styling-annotation-selection/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/styling-annotation-selection/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
