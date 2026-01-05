On this page

# BoxAnnotation

<img src="out_scichartv4/2d-charts/annotations-api/box-annotation/index_media/7b7b8ca939f94ed4a494d81ddf7ef6c8e76fad07.png" style="min-width:min(220px, 25vw);height:auto;margin-bottom:-6px" alt="Minimized Header" />

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" rel="noopener noreferrer" target="_blank">BoxAnnotation typeðŸ“˜</a>Â draws a rectangle at x1, y1, x2, y2 where coordinates are data-values. The BoxAnnotation supportsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a> properties.

Coordinates may be relative, absolute or data-value based, to bothÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xcoordinatemode" rel="noopener noreferrer" target="_blank">xCoordinateModeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ycoordinatemode" rel="noopener noreferrer" target="_blank">yCoordinateModeðŸ“˜</a> properties as values of <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecoordinatemode.html" rel="noopener noreferrer" target="_blank">ECoordinateModeðŸ“˜</a> enum.

## Declaring a BoxAnnotation in code<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation/#declaring-a-boxannotation-in-code" class="hash-link" aria-label="Direct link to Declaring a BoxAnnotation in code" translate="no" title="Direct link to Declaring a BoxAnnotation in code">â€‹</a>

The following code will declare a number of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" rel="noopener noreferrer" target="_blank">BoxAnnotationsðŸ“˜</a> and add them to the chart.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    BoxAnnotation,
    TextAnnotation,
    NumericAxis,
    SciChartSurface,
    NumberRange,
    SciChartJsNavyTheme
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });

    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(2, 8) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { visibleRange: new NumberRange(2, 8) }));

    // Add a selection of annotations to the chart
    sciChartSurface.annotations.add(
        new TextAnnotation({ fontSize: 12, text: "Draw Boxes", x1: 3.3, y1: 6.3 }),

        new BoxAnnotation({
            fill: "#279B2755",
            stroke: "#279B27",
            strokeThickness: 1,
            x1: 3.5,
            x2: 5,
            y1: 4,
            y2: 5
        }),
        new BoxAnnotation({
            fill: "#FF191955",
            stroke: "#FF1919",
            strokeThickness: 1,
            x1: 4,
            x2: 5.5,
            y1: 4.5,
            y2: 5.5
        }),
        new BoxAnnotation({
            fill: "#1964FF55",
            stroke: "#1964FF",
            strokeThickness: 1,
            x1: 4.5,
            x2: 6,
            y1: 5,
            y2: 6
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
                fontSize: 12,
                text: "Draw Boxes",
                x1: 3.3,
                y1: 6.3
            }
        },
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                fill: "#279B2755",
                stroke: "#279B27",
                strokeThickness: 1,
                x1: 3.5,
                x2: 5,
                y1: 4,
                y2: 5
            }
        },
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                fill: "#FF191955",
                stroke: "#FF1919",
                strokeThickness: 1,
                x1: 4,
                x2: 5.5,
                y1: 4.5,
                y2: 5.5
            }
        },
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                fill: "#1964FF55",
                stroke: "#1964FF",
                strokeThickness: 1,
                x1: 4.5,
                x2: 6,
                y1: 5,
                y2: 6
            }
        }
    ]
});
```

Â This results in the following output:

## Aligning a BoxAnnotation with x/yCoordinateModes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation/#aligning-a-boxannotation-with-xycoordinatemodes" class="hash-link" aria-label="Direct link to Aligning a BoxAnnotation with x/yCoordinateModes" translate="no" title="Direct link to Aligning a BoxAnnotation with x/yCoordinateModes">â€‹</a>

To position a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxannotation.html" rel="noopener noreferrer" target="_blank">BoxAnnotationðŸ“˜</a> so that it stretches horizontally or vertically across the viewport, use x/yCoordinateMode. e.g. the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Add BoxAnnotations with Horizontal and Vertical Stretching
sciChartSurface.annotations.add(
    new TextAnnotation({
        text: "Horizontally Stretched Box uses xCoordinateMode.Relative",
        x1: 0.1,
        y1: 2.5
    }),
    new BoxAnnotation({
        fill: "#279B2755",
        strokeThickness: 0,
        xCoordinateMode: ECoordinateMode.Relative,
        x1: 0,
        x2: 1,
        yCoordinateMode: ECoordinateMode.DataValue,
        y1: 2,
        y2: 3
    }),


    new TextAnnotation({
        text: "Vertcally Stretched Box uses yCoordinateMode.Relative",
        x1: 2.1,
        y1: 9.2
    }),
    new BoxAnnotation({
        fill: "#FF191955",
        strokeThickness: 0,
        xCoordinateMode: ECoordinateMode.DataValue,
        x1: 2,
        x2: 3,
        yCoordinateMode: ECoordinateMode.Relative,
        y1: 0.0,
        y2: 1.0
    })
);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.SVGTextAnnotation,
            options: {
                text: "Horizontally Stretched Box uses xCoordinateMode.Relative",
                x1: 0.1,
                y1: 2.5
            }
        },
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                fill: "#279B2755",
                strokeThickness: 0,
                xCoordinateMode: ECoordinateMode.Relative,
                x1: 0,
                x2: 1,
                yCoordinateMode: ECoordinateMode.DataValue,
                y1: 2,
                y2: 3
            }
        },
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                fill: "#FF191955",
                strokeThickness: 0,
                xCoordinateMode: ECoordinateMode.DataValue,
                x1: 2,
                x2: 3,
                yCoordinateMode: ECoordinateMode.Relative,
                y1: 0.0,
                y2: 1.0
            }
        }
    ]
});
```

results in the following output:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)
- [The Arc Annotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/arc-annotation)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/box-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/box-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
