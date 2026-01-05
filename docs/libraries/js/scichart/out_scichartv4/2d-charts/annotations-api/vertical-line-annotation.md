On this page

# VerticalLineAnnotation

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" rel="noopener noreferrer" target="_blank">VerticalLineAnnotationðŸ“˜</a> allows toÂ draw a **vertical line**Â between Y1, Y2 coordinates at X1.

<img src="out_scichartv4/2d-charts/annotations-api/vertical-line-annotation/index_media/53d88b90390a60789d34d9ffe0bb8b5707253bbf.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Declaring a VerticalLineAnnotation in code<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation/#declaring-a-verticallineannotation-in-code" class="hash-link" aria-label="Direct link to Declaring a VerticalLineAnnotation in code" translate="no" title="Direct link to Declaring a VerticalLineAnnotation in code">â€‹</a>

The following code will declare aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" rel="noopener noreferrer" target="_blank">VerticalLineAnnotationðŸ“˜</a> and add it to the chart.

- TS
- Builder API (JSON Config)

``` prism-code
const { VerticalLineAnnotation, NumericAxis, SciChartSurface, ELabelPlacement, SciChartJsNavyTheme } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Add a selection of annotations to the chart
    sciChartSurface.annotations.add(
        // Vertically line stretched across the viewport, showing label value = X (9)
        new VerticalLineAnnotation({
            labelPlacement: ELabelPlacement.Axis,
            showLabel: true,
            stroke: "#FF6600",
            strokeThickness: 2,
            x1: 9,
            axisLabelFill: "#FF6600",
            axisLabelStroke: "#333",
            axisFontSize: 20
        }),
        // Vertically line with a custom label value
        new VerticalLineAnnotation({
            labelPlacement: ELabelPlacement.Axis,
            showLabel: true,
            stroke: "#3388FF",
            strokeThickness: 2,
            strokeDashArray: [5, 5],
            x1: 4,
            axisLabelFill: "#3388FF",
            labelValue: "Custom Label",
            axisLabelStroke: "White",
            axisFontSize: 20
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
            type: EAnnotationType.RenderContextVerticalLineAnnotation,
            options: {
                labelPlacement: ELabelPlacement.Axis,
                showLabel: true,
                stroke: "#FF6600",
                strokeThickness: 2,
                x1: 9,
                axisLabelFill: "#FF6600",
                axisLabelStroke: "#333",
                axisFontSize: 20
            }
        },
        {
            type: EAnnotationType.RenderContextVerticalLineAnnotation,
            options: {
                labelPlacement: ELabelPlacement.Axis,
                showLabel: true,
                stroke: "#3388FF",
                strokeThickness: 2,
                strokeDashArray: [5, 5],
                x1: 4,
                axisLabelFill: "#3388FF",
                labelValue: "Custom Label",
                axisLabelStroke: "White",
                axisFontSize: 20
            }
        }
    ]
});
```

This results in the following output:

## Changing Label Position or Label Value<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation/#changing-label-position-or-label-value" class="hash-link" aria-label="Direct link to Changing Label Position or Label Value" translate="no" title="Direct link to Changing Label Position or Label Value">â€‹</a>

The label may be placed on the line, or on the axis. Placemement of the label is controlled by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#labelplacement" rel="noopener noreferrer" target="_blank">VerticalLineAnnotation.labelPlacementðŸ“˜</a> property, which expects anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elabelplacement.html" rel="noopener noreferrer" target="_blank">ELabelPlacementðŸ“˜</a> enum.

Valid settings are `Axis`, `Bottom`,Â `BottomLeft`, `BottomRight`, `Top`, `TopLeft`, `TopRight`, `Left` and `Right`.

For example, setting **labelPlacement** to `ELabelPlacement.TopRight`:

- TS
- Builder API (JSON Config)

``` prism-code
// Add a selection of annotations to the chart
sciChartSurface.annotations.add(
    // Vertically line stretched across the viewport, showing label value = X (9)
    new VerticalLineAnnotation({
        labelPlacement: ELabelPlacement.TopRight,
        showLabel: true,
        stroke: "#FF6600",
        strokeThickness: 2,
        x1: 9,
        axisLabelFill: "#FF6600",
        axisLabelStroke: "#333",
        axisFontSize: 20
    }),
    // Vertically line with a custom label value
    new VerticalLineAnnotation({
        labelPlacement: ELabelPlacement.TopLeft,
        showLabel: true,
        stroke: "#3388FF",
        strokeThickness: 2,
        strokeDashArray: [5, 5],
        x1: 4,
        axisLabelFill: "#3388FF",
        labelValue: "Custom Label",
        axisLabelStroke: "White",
        axisFontSize: 20
    })
);
```

``` prism-code
const { chartBuilder, EAnnotationType } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.RenderContextVerticalLineAnnotation,
            options: {
                labelPlacement: ELabelPlacement.TopRight,
                showLabel: true,
                stroke: "#FF6600",
                strokeThickness: 2,
                x1: 9,
                axisLabelFill: "#FF6600",
                axisLabelStroke: "#333",
                axisFontSize: 20
            }
        },
        {
            type: EAnnotationType.RenderContextVerticalLineAnnotation,
            options: {
                labelPlacement: ELabelPlacement.TopLeft,
                showLabel: true,
                stroke: "#3388FF",
                strokeThickness: 2,
                strokeDashArray: [5, 5],
                x1: 4,
                axisLabelFill: "#3388FF",
                labelValue: "Custom Label",
                axisLabelStroke: "White",
                axisFontSize: 20
            }
        }
    ]
});
```

Results in the label being placed on the top right of the line.

![](out_scichartv4/2d-charts/annotations-api/vertical-line-annotation/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Labels on VerticalLineAnnotations may be placed on the `Axis`, or at `Bottom`,Â `BottomLeft`, `BottomRight`, `Top`, `TopLeft`, `TopRight`, `Left` or `Right` of the line.

## VerticalAlignment Stretch and Partially Drawn Lines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation/#verticalalignment-stretch-and-partially-drawn-lines" class="hash-link" aria-label="Direct link to VerticalAlignment Stretch and Partially Drawn Lines" translate="no" title="Direct link to VerticalAlignment Stretch and Partially Drawn Lines">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" rel="noopener noreferrer" target="_blank">VerticalLineAnnotationsðŸ“˜</a> may be drawn to stretch vertically across the viewport, or to a specific Y-value. To truncate a VerticalLineAnnotation simply specify a y1 coordinate.

For example, the two options are shown below in code:

- TS
- Builder API (JSON Config)

``` prism-code
const { VerticalLineAnnotation, NumericAxis, SciChartSurface, ELabelPlacement, SciChartJsNavyTheme } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Add a selection of annotations to the chart
    sciChartSurface.annotations.add(
        // Vertically line stretched across the viewport, showing label value = X (9)
        new VerticalLineAnnotation({
            labelPlacement: ELabelPlacement.Axis,
            showLabel: true,
            stroke: "SteelBlue",
            strokeThickness: 2,
            x1: 9,
            axisLabelFill: "SteelBlue",
            axisFontSize: 20
        }),
        // Vertically line with a custom label value
        new VerticalLineAnnotation({
            showLabel: true,
            stroke: "Orange",
            strokeThickness: 2,
            x1: 6,
            y1: 4, // only draw up to Y=4
            axisLabelFill: "Orange",
            axisFontSize: 20
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
            type: EAnnotationType.RenderContextVerticalLineAnnotation,
            options: {
                labelPlacement: ELabelPlacement.Axis,
                showLabel: true,
                stroke: "SteelBlue",
                strokeThickness: 2,
                x1: 9,
                axisLabelFill: "SteelBlue",
                axisFontSize: 20
            }
        },
        {
            type: EAnnotationType.RenderContextVerticalLineAnnotation,
            options: {
                showLabel: true,
                stroke: "Orange",
                strokeThickness: 2,
                x1: 6,
                y1: 4, // only draw up to Y=4
                axisLabelFill: "Orange",
                axisFontSize: 20
            }
        }
    ]
});
```

Result in this output:

## Styling the VerticalLineAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation/#styling-the-verticallineannotation" class="hash-link" aria-label="Direct link to Styling the VerticalLineAnnotation" translate="no" title="Direct link to Styling the VerticalLineAnnotation">â€‹</a>

The following properties can be set to style the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html" rel="noopener noreferrer" target="_blank">VerticalLineAnnotation ðŸ“˜</a>:

| **Property** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#labelplacement" rel="noopener noreferrer" target="_blank">labelPlacementðŸ“˜</a> | An enumeration defining where the vertical line label is placed. Default is on axis. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#labelvalue" rel="noopener noreferrer" target="_blank">labelValueðŸ“˜</a> | The label value. By default this will equal the x1 value with text formatting applied by the axis. However it can be overridden to any string |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#showlabel" rel="noopener noreferrer" target="_blank">showLabelðŸ“˜</a> | When true, a label is shown |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> | The stroke color of the vertical line |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#strokedasharray" rel="noopener noreferrer" target="_blank">strokeDashArrayðŸ“˜</a> | Gets or sets the strokeDashArray for the LineAnnotation |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> | The stroke thickness of the vertical line |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axislabelfill" rel="noopener noreferrer" target="_blank">axisLabelFillðŸ“˜</a> | The box fill color for the axis label |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axislabelstroke" rel="noopener noreferrer" target="_blank">axisLabelStrokeðŸ“˜</a> | The text-color for the axis label |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axisfontfamily" rel="noopener noreferrer" target="_blank">axisFontFamilyðŸ“˜</a> | The font family for the axis label text |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/verticallineannotation.html#axisfontsize" rel="noopener noreferrer" target="_blank">axisFontSizeðŸ“˜</a> | The font size for the axis label text |

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)
- [The HorizontalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/horizontal-line-annotation)
- [The LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/vertical-line-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/vertical-line-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
