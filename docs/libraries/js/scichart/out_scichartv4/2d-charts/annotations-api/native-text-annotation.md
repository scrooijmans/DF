On this page

# NativeTextAnnotation

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html" rel="noopener noreferrer" target="_blank">NativeTextAnnotationðŸ“˜</a> works almost exactly like the normalÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> but instead of using SVG, it draws using the native text api that is new in SciChart.JS v3 or greater.Â ThisÂ allows for some significant benefits:

- **Performance:** you can draw hundreds or even thousands of text labels without significant slowdown.
- **Multi-line text is much easier**.Â  Separate lines with the newline (/n) character, and adjustÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html#linespacing" rel="noopener noreferrer" target="_blank">lineSpacingðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html#multilinealignment" rel="noopener noreferrer" target="_blank">multiLineAlignmentðŸ“˜</a> if needed.
- **Rotated text is much easier:**Â  If you try and rotate SVG text, you will often find it gets clipped by its own viewbox.Â  NativeText does not.Â  You can control the angle viaÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html#rotation" rel="noopener noreferrer" target="_blank">NativeTextAnnotation.rotationðŸ“˜</a>. You can control center of rotation by overridingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html#getrotationcenter" rel="noopener noreferrer" target="_blank">getRotationCenterðŸ“˜</a> if need be.
- **Text wrapping is much easier:** <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html" rel="noopener noreferrer" target="_blank">NativeTextAnnotationðŸ“˜</a>Â can wrap to the chart area, or to the width you set for it.Â  If you make the annotation editable you can see the wrapping change as you resize.
- **Allows Text Scaling:** Using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html#scale" rel="noopener noreferrer" target="_blank">NativeTextAnnotation.scaleðŸ“˜</a> property textÂ can be drawn at different sizes without creating a new font.

![](out_scichartv4/2d-charts/annotations-api/native-text-annotation/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

There are also some limitations compared toÂ svg text:

- Font style and font weight are not supported.Â Fonts other than ArialÂ must be `ttf` and either be hosted on your server or registered if coming from the internet.Â SeeÂ [Native Text Font Loading](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api)

## Creating many variants ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html" rel="noopener noreferrer" target="_blank">NativeTextAnnotationðŸ“˜</a>:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation/#creating-many-variants-ofnativetextannotationblue_book" class="hash-link" aria-label="Direct link to creating-many-variants-ofnativetextannotationblue_book" translate="no" title="Direct link to creating-many-variants-ofnativetextannotationblue_book">â€‹</a>

- TS
- Builder API (JSON Config)

``` prism-code

async function addAnnotationToChart(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Add a selection of NativeTextAnnotations to the chart
    sciChartSurface.annotations.add(
        new NativeTextAnnotation({
            x1: 1,
            y1: 9,
            text: "The default font is Arial, which does not need to be hosted or registered",
            fontSize: 18
        })
    );

    // Loading a NativeTextAnnotation with a custom font
    const result = await sciChartSurface.registerFont(
        "MyCustomFont",
        "https://fonts.gstatic.com/s/opensans/v29/mem8YaGs126MiZpBA-U1UpcaXcl0Aw.ttf"
    );

    console.log("Native font was loaded? " + result);

    sciChartSurface.annotations.add(
        new NativeTextAnnotation({
            x1: 3,
            y1: 7,
            text: "This text uses a custom font",
            fontFamily: "MyCustomFont",
            fontSize: 24
        })
    );

    // Rotating a NativeTextAnnotation with multline text
    sciChartSurface.annotations.add(
        new NativeTextAnnotation({
            x1: 1,
            y1: 5,
            text: "Native text supports\nmultiline and rotation",
            fontFamily: "Default",
            fontSize: 24,
            rotation: 30,
            textColor: "orange"
        })
    );

    // Word Wrapping a NativeTextAnnotation
    sciChartSurface.annotations.add(
        new NativeTextAnnotation({
            x1: 5,
            y1: 5,
            text: "Native text can automatically wrap to the chart area or the annotation width.  ",
            fontFamily: "Default",
            fontSize: 24,
            isEditable: true,
            wrapTo: EWrapTo.ViewRect
        })
    );

    // Scaling a native text annotation
    const scaledText = new NativeTextAnnotation({
        x1: 5,
        y1: 3,
        text: "Native text can be scaled\nwithout changing the font size",
        fontFamily: "Default",
        fontSize: 16,
        scale: 1
    });
    sciChartSurface.annotations.add(scaledText);
    const scaleAnimation = new GenericAnimation({
        from: 0,
        to: 1,
        duration: 2000,
        onAnimate: (from, to, progress) => {
            if (progress < 0.5) {
                scaledText.scale = 1 + progress;
            } else {
                scaledText.scale = 1 + (1 - progress);
            }
        },
        onCompleted: () => {
            scaleAnimation.reset();
        }
    });
    sciChartSurface.addAnimation(scaleAnimation);
}

addAnnotationToChart("scichart-root");
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    annotations: [
        {
            type: EAnnotationType.RenderContextNativeTextAnnotation,
            options: {
                x1: 1,
                y1: 9,
                text: "The default font is Arial, which does not need to be hosted or registered",
                fontSize: 18
            }
        },
        {
            type: EAnnotationType.RenderContextNativeTextAnnotation,
            options: {
                x1: 1,
                y1: 5,
                text: "Native text supports\nmultiline and rotation",
                fontFamily: "Default",
                fontSize: 24,
                rotation: 30,
                textColor: "orange"
            }
        },
        {
            type: EAnnotationType.RenderContextNativeTextAnnotation,
            options: {
                x1: 3,
                y1: 7,
                text: "This text uses a font from the internet",
                fontFamily: "MyCustomFont",
                fontSize: 24
            }
        },
        {
            type: EAnnotationType.RenderContextNativeTextAnnotation,
            options: {
                x1: 5,
                y1: 5,
                text: "Native text can automatically wrap to the chart area or the annotation width.  ",
                fontFamily: "Default",
                fontSize: 24,
                isEditable: true,
                wrapTo: EWrapTo.ViewRect
            }
        },
        {
            type: EAnnotationType.RenderContextNativeTextAnnotation,
            options: {
                id: "scaleAnnotation",
                x1: 5,
                y1: 3,
                text: "Native text can be scaled\nwithout changing the font size",
                fontFamily: "Default",
                fontSize: 16
            }
        }
    ]
});

// Registering the custom font
const result = await sciChartSurface.registerFont(
    "MyCustomFont",
    "https://fonts.gstatic.com/s/opensans/v29/mem8YaGs126MiZpBA-U1UpcaXcl0Aw.ttf"
);

console.log("Native font was loaded? " + result);

// Scaling the last NativeTextAnnotation
const scaleAnnotation = sciChartSurface.annotations.getById("scaleAnnotation") as NativeTextAnnotation;

const scaleAnimation = new GenericAnimation({
    from: 0,
    to: 1,
    duration: 2000,
    onAnimate: (from, to, progress) => {
        if (progress < 0.5) {
            scaleAnnotation.scale = 1 + progress;
        } else {
            scaleAnnotation.scale = 1 + (1 - progress);
        }
    },
    onCompleted: () => {
        scaleAnimation.reset();
    }
});
sciChartSurface.addAnimation(scaleAnimation);
```

This results in the following output:

## Polar Charts with NativeTextAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation/#polar-charts-with-nativetextannotation" class="hash-link" aria-label="Direct link to Polar Charts with NativeTextAnnotation" translate="no" title="Direct link to Polar Charts with NativeTextAnnotation">â€‹</a>

To add aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html" rel="noopener noreferrer" target="_blank">NativeTextAnnotationðŸ“˜</a> to a Polar chart, use the same exact code, just change the surface and axes types. The NativeTextAnnotation will be positioned in polar coordinates.

- TS
- Builder API (JSON Config)

``` prism-code
const {
    NativeTextAnnotation,
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
        new NativeTextAnnotation({
            text: "Polar Native\nText Annotations",
            horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
            verticalAnchorPoint: EVerticalAnchorPoint.Center,
            x1: 2.5, // at 12 o'clock (if range is 0 ot 10, since startAngle is at 9 o'clock)
            y1: 2,
            fontSize: 56,
            lineSpacing: 15,
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
            type: EAnnotationType.RenderContextNativeTextAnnotation,
            options: {
                text: "Polar Native\nText Annotations",
                horizontalAnchorPoint: EHorizontalAnchorPoint.Center,
                verticalAnchorPoint: EVerticalAnchorPoint.Center,
                x1: 2.5, // at 12 o'clock (if range is 0 ot 10, since startAngle is at 9 o'clock)
                y1: 2,
                fontSize: 56,
                lineSpacing: 15,
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

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Native Text Api](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api)
- [Adding DataLabels to a Chart Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/native-text-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/native-text-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
