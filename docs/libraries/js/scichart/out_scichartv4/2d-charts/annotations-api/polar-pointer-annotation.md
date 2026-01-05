On this page

# PolarPointerAnnotation

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" rel="noopener noreferrer" target="_blank">PolarPointerAnnotationðŸ“˜</a> creates a customizable SVG pointer for polar charts, ideal for gauges or radial indicators. It consists of 3 customizable elements: a pointer stick, center circle (optional), and arrowhead (also optional).

## Basic Usage<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#basic-usage" class="hash-link" aria-label="Direct link to Basic Usage" translate="no" title="Direct link to Basic Usage">â€‹</a>

To create a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" rel="noopener noreferrer" target="_blank">PolarPointerAnnotationðŸ“˜</a>, you can use the following code snippet:

``` prism-code
// Demonstrates how to use PolarPointerAnnotation in SciChart.js
const {
    SciChartPolarSurface,
    SciChartJsNavyTheme,
    PolarNumericAxis,
    EPolarAxisMode,
    PolarPointerAnnotation,
    ECoordinateMode,
    EStrokeLineJoin
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

// Create axes
sciChartSurface.xAxes.add(
    new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Angular,
        flippedCoordinates: true, // go clockwise
        totalAngle: Math.PI, // 180 degrees
        startAngle: 0,
    })
);

sciChartSurface.yAxes.add(
    new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Radial,
    })
);

const pointerExample = new PolarPointerAnnotation({
    x1: Math.random() * 10, // pointer angle
    y1: 10, // pointer length
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,
    isStrokeAboveCenter: true, // whether the pointer stick is above the center circle or not

    pointerStyle: {
        baseSize: 0.01, // slightly wider base than the pointer
        fill: "#F00",
        stroke: "#F00",
        backExtensionSize: 0.2
    },

    // optional - arrowhead at the tip of the pointer
    pointerArrowStyle: { 
        stroke: "#F00",
        strokeWidth: 2,
        fill: "#900",
        height: 0.2,
        width: 0.1,
        headDepth: 0.8
    },
    
    // optional - circle at the base of the pointer
    pointerCenterStyle: { 
        size: 0.12, // relative to the pointer height
        fill: "#111",
    },

    strokeLineJoin: EStrokeLineJoin.Round,
});
sciChartSurface.annotations.add(pointerExample);
```

Resulting in the following output:

## Key Configuration Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#key-configuration-properties" class="hash-link" aria-label="Direct link to Key Configuration Properties" translate="no" title="Direct link to Key Configuration Properties">â€‹</a>

### Core Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#core-properties" class="hash-link" aria-label="Direct link to Core Properties" translate="no" title="Direct link to Core Properties">â€‹</a>

| Property | Type | Default | Description |
|----|----|----|----|
| x1 | number | 0 | Rotation angle (radians/data-value) |
| y1 | number | 0 | Pointer length (pixels/data-value) |
| xCoordinateMode | <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ecoordinatemode" rel="noopener noreferrer" target="_blank">ECoordinateModeðŸ“˜</a> | DataValue | Units for rotation angle |
| yCoordinateMode | <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ecoordinatemode" rel="noopener noreferrer" target="_blank">ECoordinateModeðŸ“˜</a> | DataValue | Units for pointer length |
| isStrokeAboveCenter | boolean | false | Z-order: pointer above/below center |

### Style Objects<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#style-objects" class="hash-link" aria-label="Direct link to Style Objects" translate="no" title="Direct link to Style Objects">â€‹</a>

The 3 main style objects control the appearance of the pointer's key pieces are the following:

| Property | Type | Description |
|----|----|----|
| pointerStyle | <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointerstyle" rel="noopener noreferrer" target="_blank">TPointerStyleðŸ“˜</a> | Styling of the main stick of the pointer annotation. By default only this part will appear |
| pointerArrowStyle | <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointerarrowstyle" rel="noopener noreferrer" target="_blank">TPointerArrowStyleðŸ“˜</a> | Styles the arrowhead of the pointer annotation. To see the arrow end, you must set its `height` or `width`. |
| pointerCenterStyle | <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointercenterstyle" rel="noopener noreferrer" target="_blank">TPointerCenterStyleðŸ“˜</a> | Styles the center circle of the pointer annotation. To see the center circle, you must set its `size`. |

### How the style objects work in the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" rel="noopener noreferrer" target="_blank">PolarPointerAnnotationðŸ“˜</a>:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#how-the-style-objects-work-in-the-polarpointerannotationblue_book" class="hash-link" aria-label="Direct link to how-the-style-objects-work-in-the-polarpointerannotationblue_book" translate="no" title="Direct link to how-the-style-objects-work-in-the-polarpointerannotationblue_book">â€‹</a>

The **arrowhead** of the pointer does not appear by default. Set <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointerarrowstyle" rel="noopener noreferrer" target="_blank">pointerArrowStyleðŸ“˜</a>'s width & height to see it.

``` prism-code
const arrowheadPointer = new PolarPointerAnnotation({
    // .....

    pointerArrowStyle: {
        width: 0.1,
        height: 0.16,
        stroke: "#222",
        fill: "#fff",
        headDepth: 0.8
    },
});
```

In simlar fashion, the **base circle** only appears if the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointercenterstyle" rel="noopener noreferrer" target="_blank">pointerCenterStyleðŸ“˜</a> option is defined.

``` prism-code
const centerCirclePointer = new PolarPointerAnnotation({
    // .....

    pointerCenterStyle: {
        size: 0.1,
        fill: "#EEE"
    }
});
```

In the case you want a **stick-less** pointer annotation, this can be achieved by doing this:

``` prism-code
const stickLessPointer = new PolarPointerAnnotation({
    // .....

    pointerStyle: { // Hide the pointer stick
        baseSize: 0,
        stroke: "none"
    },


    pointerArrowStyle: { // make the arrowhead visible so a pointer entity still exists
        // .....
        width: 0.1,
        height: 0.16,
        stroke: "#222",
    }
});
```

## Advanced Customization<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#advanced-customization" class="hash-link" aria-label="Direct link to Advanced Customization" translate="no" title="Direct link to Advanced Customization">â€‹</a>

Override SVG generation methods for full control:

``` prism-code
// Define a pointer annotation
const customPointer = new PolarPointerAnnotation({
    x1: Math.random() * 6 + 2,
    y1: 10, // pointer length
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,

    pointerStyle: {
        baseSize: 0.05, // relative to the pointer length
        fill: "#195",
        stroke: "#195",
    },

    // optional - arrowhead at the tip of the pointer
    pointerArrowStyle: { 
        stroke: "#195",
        fill: "#132",
        height: 0.2,
        width: 0.1,
        headDepth: 0.8,
    },
    
    // optional - circle at the base of the pointer
    pointerCenterStyle: { 
        size: 0.05, // relative to the pointer length
        fill: "#132",
        stroke: "#195",
    },

    strokeLineJoin: EStrokeLineJoin.Round,
});

// You can override: 
// 1. the Stick (pointer) path
customPointer.getPointerStickSvg = (pointerLength, pointerWidth, backExtensionSize) => {
    const size = pointerLength * 2;
    const halfStrokeWidth = Math.max(customPointer.pointerStyle.strokeWidth ?? 1.5, 0);
    return `<path stroke-linejoin=${customPointer.strokeLineJoin}
        d="
            M${size - halfStrokeWidth} ${pointerLength}
            L${pointerLength - pointerLength * backExtensionSize} ${pointerLength - pointerWidth / 2}
            l0 ${pointerWidth}
            L${size - halfStrokeWidth} ${pointerLength}Z
        " 
        fill="${customPointer.pointerStyle.fill}"
        stroke="${customPointer.pointerStyle.stroke}"  
        stroke-width="${customPointer.pointerStyle.strokeWidth}"
    />`; 
}

// 2. Center path
customPointer.getPointerCenterSvg = (pointerLength, centerSize) => {
    return `<rect
        x="${pointerLength - centerSize / 2}" 
        y="${pointerLength - centerSize / 2}"
        width=${centerSize} 
        height=${centerSize}
        rx=${centerSize / 2}
        ry=${centerSize / 2}
        stroke=${customPointer.pointerCenterStyle.stroke}
        stroke-width=${customPointer.pointerCenterStyle.strokeWidth}
        fill=${customPointer.pointerCenterStyle.fill}
    />`;
}

// 3. The arrowhead path
customPointer.getPointerArrowSvg = (pointerLength, height, width, headDepth) => {
    const size = 2 * pointerLength;
    const halfStrokeWidth = Math.max(customPointer.pointerArrowStyle.strokeWidth ?? 1.5, 0);
    return `<path 
        stroke="${customPointer.pointerArrowStyle.stroke}"
        stroke-width="${customPointer.pointerArrowStyle.strokeWidth}"
        fill="${customPointer.pointerArrowStyle.fill}"
        stroke-linejoin=${customPointer.strokeLineJoin}
        d="
            M${size - height / 2 - halfStrokeWidth} ${pointerLength - width / 2}
            l${height / 2} ${width / 2}
            l-${height / 2} ${width / 2}
            ${headDepth === 0
                ? ""
                : `l${((1 - headDepth) * height) / 2} ${-width / 2}Z`
            }
        " 
    />`;
}
// The above methods are the default implementations, but you can override them if you want to
```

This produces the following output:

![](out_scichartv4/2d-charts/annotations-api/polar-pointer-annotation/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpointerannotation.html" rel="noopener noreferrer" target="_blank">PolarPointerAnnotationðŸ“˜</a> plays a crucial role within a [Gauge Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart)

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Polar Gauge Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart)
- [Polar Arc Annotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/polar-pointer-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/polar-pointer-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
