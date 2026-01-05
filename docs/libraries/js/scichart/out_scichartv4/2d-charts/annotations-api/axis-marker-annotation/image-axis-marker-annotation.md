On this page

# Image AxisMarkerAnnotation

SciChart.js allows to create **custom axis marker annotations** on axes. This is done by creatingÂ [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview) and passing anÂ **image** option into the constructor.

``` prism-code
const htmlImageElement = await createImageAsync(imageUrl);

const customAxisMarkerAnnotation = new AxisMarkerAnnotation({
Â Â Â  y1: 5,
Â Â Â  isEditable: true,
Â Â Â  image: htmlImageElement,
});
```

The full example code is below:

- TS
- Builder API (JSON Config)

``` prism-code
const {
    AxisMarkerAnnotation,
    NumericAxis,
    SciChartSurface,
    SciChartJsNavyTheme,
    createImageAsync,
    TextAnnotation,
    EHorizontalAnchorPoint
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    const imageUrl = "https://www.scichart.com/demo/images/CustomMarkerImage.png";
    const htmlImageElement = await createImageAsync(imageUrl);

    // An AxisMarkerAnnotation at Y=5.2 showing an image
    sciChartSurface.annotations.add(
        new AxisMarkerAnnotation({
            y1: 5.1,
            isEditable: true,
            image: htmlImageElement,
            // Optional: Set imageWidth and imageHeight, else it will default to image size
            imageWidth: 48,
            imageHeight: 48
        })
    );

    // Add a text annotation to explain
    sciChartSurface.annotations.add(
        new TextAnnotation({
            x1: 9.5,
            y1: 5.2,
            horizontalAnchorPoint: EHorizontalAnchorPoint.Right,
            fontSize: 16,
            text: "Draggable Axis Marker with a custom image -->"
        })
    );
}

addAnnotationToChart("scichart-root");
```

``` prism-code
const { chartBuilder, EAnnotationType } = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

const imageUrl = "https://www.scichart.com/demo/images/CustomMarkerImage.png";
const htmlImageElement = await createImageAsync(imageUrl);

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.RenderContextAxisMarkerAnnotation,
            options: {
                y1: 5.1,
                isEditable: true,
                image: htmlImageElement,
                // Optional: Set imageWidth and imageHeight, else it will default to image size
                imageWidth: 48,
                imageHeight: 48
            }
        }
    ]
});
```

In this example we are usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#createimageasync" rel="noopener noreferrer" target="_blank">createImageAsync()ðŸ“˜</a> helper function to create anÂ <a href="https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement" rel="noopener noreferrer" target="_blank">htmlImageElement</a>. This is then passed toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#image" rel="noopener noreferrer" target="_blank">AxisMarkerAnnotation.imageðŸ“˜</a> property. OptionallyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#imagewidth" rel="noopener noreferrer" target="_blank">AxisMarkerAnnotation.imageWidthðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axismarkerannotation.html#imageheight" rel="noopener noreferrer" target="_blank">imageHeightðŸ“˜</a> may be set.

Here's the output:

On the chart we can see a cloud-shaped custom axis label annotation. The annotation is draggable along theÂ YAxis.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/image-axis-marker-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Annotations API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)

- [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview)

- [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/axis-marker-annotation/image-axis-marker-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/axis-marker-annotation/image-axis-marker-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
