On this page

# TextAnnotation

<img src="out_scichartv4/2d-charts/annotations-api/text-annotation/index_media/593f72fc09ca5d6f9b572658038097d3967b4bbd.png" style="min-width:min(220px, 25vw);height:auto;margin-bottom:-6px" alt="Minimized Header" />

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a>Â type draws a text labelÂ at the x1,y1 locationÂ where coordinates are data-values. The TextAnnotation supportsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#text" rel="noopener noreferrer" target="_blank">textðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#fontsize" rel="noopener noreferrer" target="_blank">fontSizeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#fontweight" rel="noopener noreferrer" target="_blank">fontWeightðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#fontfamily" rel="noopener noreferrer" target="_blank">fontFamilyðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#textcolor" rel="noopener noreferrer" target="_blank">textColorðŸ“˜</a> properties.

Coordinates may be relative, absolute or data-value based, to bothÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xcoordinatemode" rel="noopener noreferrer" target="_blank">xCoordinateModeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ycoordinatemode" rel="noopener noreferrer" target="_blank">yCoordinateModeðŸ“˜</a> properties as values of <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" rel="noopener noreferrer" target="_blank">ECoordinateModeðŸ“˜</a> enum.

## Declaring a TextAnnotation in code<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation/#declaring-a-textannotation-in-code" class="hash-link" aria-label="Direct link to Declaring a TextAnnotation in code" translate="no" title="Direct link to Declaring a TextAnnotation in code">â€‹</a>

The following code will declare a number ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationsðŸ“˜</a> and add them to the chart.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    TextAnnotation,
    NumericAxis,
    SciChartSurface,
    EHorizontalAnchorPoint,
    EVerticalAnchorPoint,
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
        // Add TextAnnotations in the top left of the chart
        new TextAnnotation({
            text: "Annotations are Easy!",
            fontSize: 24,
            x1: 0.3,
            y1: 9.7
        }),
        new TextAnnotation({
            text: "You can create text",
            fontSize: 18,
            x1: 1,
            y1: 9
        }),

        // Add TextAnnotations with anchor points
        new TextAnnotation({
            text: "Anchor Center (X1, Y1)",
            horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
            verticalAnchorPoint: EVerticalAnchorPoint.Bottom,
            x1: 2,
            y1: 8
        }),
        new TextAnnotation({
            text: "Anchor Right",
            horizontalAnchorPoint: EHorizontalAnchorPoint.Right,
            verticalAnchorPoint: EVerticalAnchorPoint.Top,
            x1: 2,
            y1: 8
        }),
        new TextAnnotation({
            text: "or Anchor Left",
            horizontalAnchorPoint: EHorizontalAnchorPoint.Left,
            verticalAnchorPoint: EVerticalAnchorPoint.Top,
            x1: 2,
            y1: 8
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
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "Annotations are Easy!",
                fontSize: 24,
                x1: 0.3,
                y1: 9.7
            }
        },
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "You can create text",
                fontSize: 18,
                x1: 1,
                y1: 9
            }
        },
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "Anchor Center (X1, Y1)",
                horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
                verticalAnchorPoint: EVerticalAnchorPoint.Bottom,
                x1: 2,
                y1: 8
            }
        },
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "Anchor Right",
                horizontalAnchorPoint: EHorizontalAnchorPoint.Right,
                verticalAnchorPoint: EVerticalAnchorPoint.Top,
                x1: 2,
                y1: 8
            }
        },
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "or Anchor Left",
                horizontalAnchorPoint: EHorizontalAnchorPoint.Left,
                verticalAnchorPoint: EVerticalAnchorPoint.Top,
                x1: 2,
                y1: 8
            }
        }
    ]
});
```

This results in the following output:

## Positioning a TextAnnotation with horizontal/vertical Anchor Points<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation/#positioning-a-textannotation-with-horizontalvertical-anchor-points" class="hash-link" aria-label="Direct link to Positioning a TextAnnotation with horizontal/vertical Anchor Points" translate="no" title="Direct link to Positioning a TextAnnotation with horizontal/vertical Anchor Points">â€‹</a>

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> only requires coordinates x1,y1 to be set. The alignment of the text around this coordinate is controlled by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#horizontalanchorpoint" rel="noopener noreferrer" target="_blank">horizontalAnchorPointðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#verticalanchorpoint" rel="noopener noreferrer" target="_blank">verticalAnchorPointðŸ“˜</a> properties.

<img src="out_scichartv4/2d-charts/annotations-api/text-annotation/index_media/3a4323d29ea490e3c8082ea4ff47d1b7c967d973.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Above: Set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#horizontalanchorpoint" rel="noopener noreferrer" target="_blank">horizontalAnchorPointðŸ“˜</a>, and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html#verticalanchorpoint" rel="noopener noreferrer" target="_blank">verticalAnchorPointðŸ“˜</a> property to determine which anchor point (horizontal: **left**, **center**, **right** or **vertical**: **top**, **center**, **bottom**) the x1, y1 coordinate is bound to.

## Aligning a LineAnnotation with x/yCoordinateModes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation/#aligning-a-lineannotation-with-xycoordinatemodes" class="hash-link" aria-label="Direct link to Aligning a LineAnnotation with x/yCoordinateModes" translate="no" title="Direct link to Aligning a LineAnnotation with x/yCoordinateModes">â€‹</a>

Like other annotation types, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> can be positioned relatively or absolute using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xcoordinatemode" rel="noopener noreferrer" target="_blank">xCoordinateModeðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ycoordinatemode" rel="noopener noreferrer" target="_blank">yCoordinateModeðŸ“˜</a> properties.

For example, to create a watermark in the centre of the chart, use this code:

- TS
- Builder API (JSON Config)

``` prism-code
// Add a TextAnnotation using CoordinateMode Relative and Horizontal/Vertical Anchor Point
// to create a watermark in a fixed position in the middle of the chart
sciChartSurface.annotations.add(
    // Watermark with CoordinateMode Relative
    new TextAnnotation({
        text: "Create a Watermark",
        horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
        verticalAnchorPoint: EVerticalAnchorPoint.Center,
        x1: 0.5,
        y1: 0.5,
        fontSize: 56,
        fontWeight: "Bold",
        textColor: "#FFFFFF22",
        xCoordinateMode: ECoordinateMode.Relative,
        yCoordinateMode: ECoordinateMode.Relative,
        annotationLayer: EAnnotationLayer.BelowChart
    })
);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "Create a Watermark",
                horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
                verticalAnchorPoint: EVerticalAnchorPoint.Center,
                x1: 0.5,
                y1: 0.5,
                fontSize: 56,
                fontWeight: "Bold",
                textColor: "#FFFFFF22",
                xCoordinateMode: ECoordinateMode.Relative,
                yCoordinateMode: ECoordinateMode.Relative,
                annotationLayer: EAnnotationLayer.BelowChart
            }
        }
    ]
});
```

This results in the following output:

## Polar Charts with TextAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation/#polar-charts-with-textannotation" class="hash-link" aria-label="Direct link to Polar Charts with TextAnnotation" translate="no" title="Direct link to Polar Charts with TextAnnotation">â€‹</a>

To add aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> to a Polar chart, use the same exact code, just change the surface and axes types. The TextAnnotation will be positioned in polar coordinates.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    LineAnnotation,
    TextAnnotation,
    PolarNumericAxis,
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    EPolarAxisMode,
    EHorizontalAnchorPoint,
    EVerticalAnchorPoint,
    ECoordinateMode,
    EAnnotationLayer
} = SciChart;
// or for npm import { SciChartPolarSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });

    const angularAxis = new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Angular,
    });
    sciChartSurface.xAxes.add(angularAxis);

    const radialAxis = new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Radial,
    });
    sciChartSurface.yAxes.add(radialAxis);

    sciChartSurface.annotations.add(
        new TextAnnotation({
            text: "Polar Text Annotations",
            horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
            verticalAnchorPoint: EVerticalAnchorPoint.Center,
            x1: 2.5, // at 12 o'clock (if range is 0 ot 10, since startAngle is at 9 o'clock)
            y1: 2,
            fontSize: 56,
            fontWeight: "Bold",
            textColor: "#FFFFFF22",
            xCoordinateMode: ECoordinateMode.DataValue,
            yCoordinateMode: ECoordinateMode.DataValue,
            annotationLayer: EAnnotationLayer.BelowChart
        })
    );
}

addAnnotationToChart("scichart-root");
```

``` prism-code
const { 
    chartBuilder, 
    EAnnotationType, 
    EHorizontalAnchorPoint, 
    EVerticalAnchorPoint, 
    ECoordinateMode, 
    EAnnotationLayer
} = SciChart;
// or for npm import { chartBuilder , ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DPolarChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "Polar Text Annotations",
                horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
                verticalAnchorPoint: EVerticalAnchorPoint.Center,
                x1: 2.5, // at 12 o'clock (if range is 0 ot 10, since startAngle is at 9 o'clock)
                y1: 2,
                fontSize: 56,
                fontWeight: "Bold",
                textColor: "#FFFFFF22",
                xCoordinateMode: ECoordinateMode.DataValue,
                yCoordinateMode: ECoordinateMode.DataValue,
                annotationLayer: EAnnotationLayer.BelowChart
            }
        }
    ]
});
```

This results in the following:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/text-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/text-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
