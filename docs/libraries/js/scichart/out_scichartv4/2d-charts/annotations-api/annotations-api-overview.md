On this page

# Annotations API Overview

SciChart.js features a rich Annotations API, that allows you to placeÂ annotations (boxes, markers, text labels and custom shapes) over a chart:Â 

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/chart-annotations" target="_blank">Chart Annotations</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

Annotations can provide **interactive** event/news bullets, horizontal/vertical lines (thresholds), text/callouts as well as measurements such as Peak-to-peak or cycle duration. Annotations can be edited by click & drag,Â added by touching a screen, or, simply created programmatically. SciChart provides a number of built-in annotations, but you can also create your own.

## Annotation Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview/#annotation-types" class="hash-link" aria-label="Direct link to Annotation Types" translate="no" title="Direct link to Annotation Types">â€‹</a>

The following annotation types areÂ available out of the box in SciChart:

| Type | Description | Supported Chart Types |
|----|----|----|
| [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation) | Draws a **rectangle** at specific X1, X2, Y1, Y2 coordinates. | Cartesian |
| [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation) | Draws a **line** between X1, Y1 and X2, Y2 positions. | Cartesian and Polar |
| [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation) | Allows to place a piece of **text** at a specific location on a chart. | Cartesian and Polar |
| [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation) | Allows to place any SVG Content at a specific location on a chart. | Cartesian and Polar |
| [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation) | Draws a vertical line at a given x position, with various labelling options | Cartesian |
| [HorizontalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/horizontal-line-annotation) | Draws a horizontal line at a given y position, with various labelling options | Cartesian |
| [LineArrowAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-arrow-annotation) | Allows to place line arrows at a specific location on a chart | Cartesian and Polar |
| [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview) | Allows to place a marker at a specific location on an axis | Cartesian |
| [CustomAxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/image-axis-marker-annotation) | Uses an image instead of text for an axis marker | Cartesian |
| [NativeTextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation) | Draws text natively rather than using svg, supporting rotation, multiline, wordwrap and scaling | Cartesian and Polar |
| [HtmlCustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation) | Allows to render arbitrary HTML content within a chart | Cartesian |
| [HtmlTextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation) | Allows to place HTML text at a specific location on a chart | Cartesian |
| [ArcAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/arc-annotation) | Allows to place arc element at a specific location on a cartesian chart. | Cartesian |
| [PolarArcAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation) | Allows to place arc element at a specific location on a polar chart. | Polar |
| [PolarPointerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation) | Allows to place a pointer on a polar chart. Is used for gauge charts | Polar |

Annotations have <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html#surfacetypes" rel="noopener noreferrer" target="_blank">surfaceTypesðŸ“˜</a> property, which defines list of compatible surface types. <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esurfacetype.html#scichartsurfacetype" rel="noopener noreferrer" target="_blank">ESurfaceType.SciChartSurfaceTypeðŸ“˜</a> stands for regular (Cartesian) chart and <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/esurfacetype.html#scichartpolarsurfacetype" rel="noopener noreferrer" target="_blank">ESurfaceType.SciChartPolarSurfaceTypeðŸ“˜</a> stands for Polar chart.

![](out_scichartv4/2d-charts/annotations-api/annotations-api-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

If an annotation is only compatible with Polar surfaces it has prefix "Polar" in the name. For example, PolarArcAnnotation works only with Polar surfaces. Annotations without the "Polar" prefix can be compatible with both surface types or only with Cartesian surfaces. For example, LineAnnotation is compatible with both surface types and BoxAnnotation is compatible only with Cartesian.

To learn more about any annotation type, please refer to the corresponding article.

## Adding an Annotation to a Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview/#adding-an-annotation-to-a-chart" class="hash-link" aria-label="Direct link to Adding an Annotation to a Chart" translate="no" title="Direct link to Adding an Annotation to a Chart">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>Â stores allÂ its annotations in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#annotations" rel="noopener noreferrer" target="_blank">SciChartSurface.annotationsðŸ“˜</a> collection. The following code can be used to add an annotation to a chart:

- TS
- Builder API (JSON Config)

``` prism-code
const {
    BoxAnnotation,
    TextAnnotation,
    LineAnnotation,
    NumericAxis,
    SciChartSurface,
    NumberRange,
    EHorizontalAnchorPoint,
    EVerticalAnchorPoint,
    ECoordinateMode,
    SciChartJsNavyTheme
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Add a selection of annotations to the chart
    sciChartSurface.annotations.add(
        new LineAnnotation({
            stroke: "#FF6600",
            strokeThickness: 3,
            x1: 2.0,
            x2: 8.0,
            y1: 3.0,
            y2: 7.0
        }),
        new BoxAnnotation({
            stroke: "#FF3333",
            strokeThickness: 1,
            fill: "rgba(255,50,50,0.3)",
            x1: 2.0,
            x2: 8.0,
            y1: 3.0,
            y2: 7.0
        }),
        new BoxAnnotation({
            stroke: "#33FF33",
            strokeThickness: 1,
            fill: "rgba(50, 255, 50, 0.3)",
            x1: 3.0,
            x2: 9.0,
            y1: 4.0,
            y2: 8.0
        }),
        new TextAnnotation({
            x1: 100,
            y1: 0.5,
            xCoordinateMode: ECoordinateMode.Pixel,
            yCoordinateMode: ECoordinateMode.Relative,
            horizontalAnchorPoint: EHorizontalAnchorPoint.Left,
            verticalAnchorPoint: EVerticalAnchorPoint.Center,
            textColor: "yellow",
            fontSize: 26,
            fontFamily: "Default",
            text: "TEXT ANNOTATION"
        })
    );
}

addAnnotationToChart("scichart-root");
```

``` prism-code
const { chartBuilder, EAnnotationType, ECoordinateMode, EVerticalAnchorPoint, EHorizontalAnchorPoint } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                stroke: "#FF3333",
                strokeThickness: 1,
                fill: "rgba(255,50,50,0.3)",
                x1: 2.0,
                x2: 8.0,
                y1: 3.0,
                y2: 7.0
            }
        },
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                stroke: "#33FF33",
                strokeThickness: 1,
                fill: "rgba(50,255,50,0.3)",
                x1: 3.0,
                x2: 9.0,
                y1: 4.0,
                y2: 8.0
            }
        },
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                x1: 100,
                y1: 0.5,
                xCoordinateMode: ECoordinateMode.Pixel,
                yCoordinateMode: ECoordinateMode.Relative,
                horizontalAnchorPoint: EHorizontalAnchorPoint.Left,
                verticalAnchorPoint: EVerticalAnchorPoint.Center,
                textColor: "yellow",
                fontSize: 26,
                fontFamily: "Times New Roman",
                text: "TEXT ANNOTATION"
            }
        }
    ]
});
```

This results in the following output:

Individual Annotation features are discussed in greater detail in the following pages:

- TheÂ [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation) Type
- TheÂ [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation) Type
- TheÂ [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation) Type
- TheÂ [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation) Type

## Common Annotation Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview/#common-annotation-properties" class="hash-link" aria-label="Direct link to Common Annotation Properties" translate="no" title="Direct link to Common Annotation Properties">â€‹</a>

All annotations in SciChart.js are derived from theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" rel="noopener noreferrer" target="_blank">AnnotationBaseðŸ“˜</a>Â **type. Individual Annotations have additional properties however the followingÂ common properties of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" rel="noopener noreferrer" target="_blank">AnnotationBaseðŸ“˜</a>Â class listed below can be used to control all annotation types.**

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#annotationlayer" rel="noopener noreferrer" target="_blank">annotationLayerðŸ“˜</a></strong></td>
<td>Determines which canvas the annotation should be placed on. The default is <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" rel="noopener noreferrer" target="_blank">EAnnotationLayer.AboveChartðŸ“˜</a>, where annotations are displayed above the chart series. Setting this property to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" rel="noopener noreferrer" target="_blank">EAnnotationLayer.BelowChartðŸ“˜</a> places an annotation below series but above gridlines, axis bands and axis labels. Note that this method doesn't work with SVG based annotations such as <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/customannotation.html" rel="noopener noreferrer" target="_blank">CustomAnnotationðŸ“˜</a>. Setting the property to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html#background" rel="noopener noreferrer" target="_blank">EAnnotationLayer.BackgroundðŸ“˜</a> places an annotation below all elements on the chart (series, axis bands, gridlines, axis labels). This method works with all annotation types including SVG, and is useful for placing watermarks on the chart.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xcoordinatemode" rel="noopener noreferrer" target="_blank">xCoordinateModeðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ycoordinatemode" rel="noopener noreferrer" target="_blank">yCoordinateModeðŸ“˜</a></strong></td>
<td>Determines how coordinates x1,y2,x2,y2 are used when placing the annotation. The default is <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" rel="noopener noreferrer" target="_blank">ECoordinateMode.DataValueðŸ“˜</a> where coordinates correspond to Data-values. <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" rel="noopener noreferrer" target="_blank">ECoordinateMode.RelativeðŸ“˜</a> means coordinates are relative to the viewport. <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" rel="noopener noreferrer" target="_blank">ECoordinateMode.PixelðŸ“˜</a> means coordinates are pixel values relative to the top-left of the viewport.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/svgannotationbase.html#horizontalanchorpoint" rel="noopener noreferrer" target="_blank">horizontalAnchorPointðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/svgannotationbase.html#verticalanchorpointproperty" rel="noopener noreferrer" target="_blank">verticalAnchorPointðŸ“˜</a></strong></td>
<td>Used to adjust the alignment of certain annotations. <img src="out_scichartv4/2d-charts/annotations-api/annotations-api-overview/index_media/d93f04d02116b0f0dd935b307ee75c2bbbe07215.png" class="img_ev3q" decoding="async" loading="lazy" width="407" height="192" alt="Example" /> Above: HorizontalAnchorPoint, VerticalAnchorPoint when applied to a TextAnnotation</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ishidden" rel="noopener noreferrer" target="_blank">isHiddenðŸ“˜</a></strong></td>
<td>Can be set to show or hide an annotation.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#hovered" rel="noopener noreferrer" target="_blank">hoveredðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ishovered" rel="noopener noreferrer" target="_blank">isHoveredðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#selectedchanged" rel="noopener noreferrer" target="_blank">selectedChangedðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#isselected" rel="noopener noreferrer" target="_blank">isSelectedðŸ“˜</a></strong></td>
<td>Annotations can be made interactive with selection and hover callbacks. See <a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotation-hover">Annotation Hover</a> for details.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#resizedirections" rel="noopener noreferrer" target="_blank">resizeDirectionsðŸ“˜</a></strong></td>
<td>Allows you to specify which direction (X, Y, Xy) an annotation may be resized in.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragstarted" rel="noopener noreferrer" target="_blank">dragStartedðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragdelta" rel="noopener noreferrer" target="_blank">dragDeltaðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#dragended" rel="noopener noreferrer" target="_blank">dragEndedðŸ“˜</a></strong></td>
<td>Callbacks may be registered when an annotation is dragged by the user.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#x1" rel="noopener noreferrer" target="_blank">x1ðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#x2" rel="noopener noreferrer" target="_blank">x2ðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#y1" rel="noopener noreferrer" target="_blank">y1ðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#y2" rel="noopener noreferrer" target="_blank">y2ðŸ“˜</a></strong></td>
<td>Define the position of the annotation on the parent chart. Note that annotation position is also defined by the xCoordinateMode, yCoordinateMode properties.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xaxisid" rel="noopener noreferrer" target="_blank">xAxisIdðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#yaxisid" rel="noopener noreferrer" target="_blank">yAxisIdðŸ“˜</a></strong></td>
<td>In a multiple-axis scenario, used to bind the annotation to a specific X or Y-Axis.<br />
<strong>NOTE:</strong> If the value is not supplied it will use the first axis.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#iseditable" rel="noopener noreferrer" target="_blank">isEditableðŸ“˜</a></strong></td>
<td>If true, this annotation can be selected and dragged/resized. See <a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/editable-annotations">Editable Annotations</a> for more details.</td>
</tr>
<tr>
<td><strong><a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#clicked" rel="noopener noreferrer" target="_blank">clickedðŸ“˜</a> / <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#onclick" rel="noopener noreferrer" target="_blank">onClickðŸ“˜</a></strong></td>
<td>Event fired when the annotation is clicked. Works for both editable and non-editable annotations. The event arguments contain a point which gives the coordinates of where on the annotation it was clicked, relative to the top left corner.<br />
<strong>NOTE:</strong> If an editable annotation is already selected, clicking on it will fire <strong>dragStarted</strong>, but not <strong>clicked</strong>.</td>
</tr>
</tbody>
</table>

More annotation properties and the inheritence hierachy may be viewed at theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" rel="noopener noreferrer" target="_blank">AnnotationBase Typedoc pageðŸ“˜</a>.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation)
- [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation)
- [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation)
- [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/annotations-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/annotations-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
