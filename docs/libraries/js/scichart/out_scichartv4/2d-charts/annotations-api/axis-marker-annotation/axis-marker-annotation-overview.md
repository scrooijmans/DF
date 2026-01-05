On this page

# AxisMarkerAnnotation

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html" rel="noopener noreferrer" target="_blank">AxisMarkerAnnotationðŸ“˜</a> allows you to add a label on to the Axis at a specific X or Y value.

<img src="out_scichartv4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/index_media/ad715d44d977b96d24427ccb9c3d2940c629b11c.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Declaring a AxisMarkerAnnotation in code<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/#declaring-a-axismarkerannotation-in-code" class="hash-link" aria-label="Direct link to Declaring a AxisMarkerAnnotation in code" translate="no" title="Direct link to Declaring a AxisMarkerAnnotation in code">â€‹</a>

The following code will declare anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html" rel="noopener noreferrer" target="_blank">AxisMarkerAnnotationðŸ“˜</a> add itÂ to the chart.

- TS
- Builder API (JSON Config)

``` prism-code
const { AxisMarkerAnnotation, NumericAxis, SciChartSurface, ELabelPlacement, SciChartJsNavyTheme } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Add a selection of annotations to the chart
    sciChartSurface.annotations.add(
        // An AxisMarkerAnnotation at Y=5.2 showing the y-value
        new AxisMarkerAnnotation({
            y1: 5.2,
            fontSize: 20,
            fontStyle: "Bold",
            backgroundColor: "SteelBlue",
            color: "White",
            fontFamily: "Default",
            fontWeight: "700"
        }),
        // An AxisMarkerAnnotation at Y=7 with a custom label
        new AxisMarkerAnnotation({
            y1: 7,
            fontSize: 16,
            fontStyle: "Bold",
            backgroundColor: "#FF6600",
            color: "Black",
            fontFamily: "Default",
            formattedValue: "Custom Label"
        })
    );
}

addAnnotationToChart("scichart-root");
```

``` prism-code
const { chartBuilder, EAnnotationType } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.RenderContextAxisMarkerAnnotation,
            options: {
                y1: 5.2,
                fontSize: 12,
                fontStyle: "Bold",
                backgroundColor: "SteelBlue",
                color: "White",
                fontFamily: "Default"
            }
        },
        {
            type: EAnnotationType.RenderContextAxisMarkerAnnotation,
            options: {
                y1: 7,
                fontSize: 16,
                fontStyle: "Bold",
                backgroundColor: "#FF6600",
                color: "Black",
                fontFamily: "Default",
                formattedValue: "Custom Label"
            }
        }
    ]
});
```

Results in the following output:Â 

## Styling the AxisMarkerAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/#styling-the-axismarkerannotation" class="hash-link" aria-label="Direct link to Styling the AxisMarkerAnnotation" translate="no" title="Direct link to Styling the AxisMarkerAnnotation">â€‹</a>

The following properties can be set to style theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html" rel="noopener noreferrer" target="_blank">AxisMarkerAnnotationðŸ“˜</a>:

| **Property** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#backgroundColor" rel="noopener noreferrer" target="_blank">backgroundColorðŸ“˜</a> | The box fill color for the axis label |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#color" rel="noopener noreferrer" target="_blank">colorðŸ“˜</a> | The text-color for the axis label |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#fontfamily" rel="noopener noreferrer" target="_blank">fontFamilyðŸ“˜</a> | The font family for the axis label text |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#fontSize" rel="noopener noreferrer" target="_blank">fontSizeðŸ“˜</a> | The font size for the axis label text |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#fontStyle" rel="noopener noreferrer" target="_blank">fontStyleðŸ“˜</a> | The font style, e.g. Bold or Italic for the axis label text |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#formattedvalue" rel="noopener noreferrer" target="_blank">formattedValueðŸ“˜</a> | The formatted value on the axis label. This defaults to the Y-value formatted by the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#labelprovider" rel="noopener noreferrer" target="_blank">yAxis.labelProviderðŸ“˜</a>. This can be overridden by a custom label value by setting this property. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#annotationsgripsfill" rel="noopener noreferrer" target="_blank">annotationGripsFillðŸ“˜</a> | The fill color for the annotations grips when editing (dragging) |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#annotationsgripsradius" rel="noopener noreferrer" target="_blank">annotationsGripsRadiusðŸ“˜</a> | The radius for the annotations grips when editing (dragging) |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#annotationsgripsstroke" rel="noopener noreferrer" target="_blank">annotationGripsStrokeðŸ“˜</a> | The stroke color for the annotations grips when editing. |

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
