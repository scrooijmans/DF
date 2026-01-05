On this page

# PolarArcAnnotation

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> is used to draw a filled sector or an arc line on a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html" rel="noopener noreferrer" target="_blank">SciChartPolarSurfaceðŸ“˜</a>. It is defined by a center point, inner and outer radii, and start and end angles. This annotation is ideal for highlighting specific angular or radial regions in polar charts.

## Create a Polar Arc Annotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation/#create-a-polar-arc-annotation" class="hash-link" aria-label="Direct link to Create a Polar Arc Annotation" translate="no" title="Direct link to Create a Polar Arc Annotation">â€‹</a>

The following code demonstrates how to declare a filled sector and a line arc using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> and add them to a polar surface.

- TS

``` prism-code
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    NumberRange,
    PolarArcAnnotation,
    SciChartJsNavyTheme,
    EPolarAxisMode,
} = SciChart;

// Create a SciChartPolarSurface
const { wasmContext, sciChartSurface } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

// Add polar axes
sciChartSurface.xAxes.add(
    new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Angular,
        visibleRange: new NumberRange(0, 360) 
    })
);
sciChartSurface.yAxes.add(
    new PolarNumericAxis(wasmContext, {
        polarAxisMode: EPolarAxisMode.Radial,
        visibleRange: new NumberRange(0, 10)
    })
);

// Add a selection of PolarArcAnnotations to the chart
sciChartSurface.annotations.add(
    // Filled Sector Annotation from 45 to 135 degrees
    new PolarArcAnnotation({
        isEditable: true,
        x1: 45, // Start Angle
        x2: 135, // End Angle
        y1: 8, // Outer Radius
        y2: 4, // Inner Radius
        fill: "rgba(255, 165, 0, 0.3)",
        stroke: "#FFA500",
        strokeThickness: 2,
    }),
    // Arc Line Annotation from 225 to 315 degrees
    new PolarArcAnnotation({
        isEditable: true,
        isLineMode: true,
        x1: 225, // Start Angle
        x2: 315, // End Angle
        y1: 9, // Radius
        // No need for y2 when `isLineMode` is true
        stroke: "#3388FF",
        strokeThickness: 3,
    })
);
```

This results in the following output:

In the code above:

- Two instances of <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> are created and added to a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html" rel="noopener noreferrer" target="_blank">SciChartPolarSurfaceðŸ“˜</a>.
- The first annotation is a **filled sector**. It is defined by start/end angles (`x1`, `x2`) and outer/inner radii (`y1`, `y2`).
- The second annotation is an **arc line**. This is achieved by setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#islinemode" rel="noopener noreferrer" target="_blank">isLineModeðŸ“˜</a>: `true`, which causes the annotation to render as a single line at the `y1` radius, ignoring the `y2` property.
- Both annotations are editable and can be dragged or resized by the user if <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#iseditable" rel="noopener noreferrer" target="_blank">isEditableðŸ“˜</a>: `true` is set.

### Unique Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation/#unique-properties" class="hash-link" aria-label="Direct link to Unique Properties" translate="no" title="Direct link to Unique Properties">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html" rel="noopener noreferrer" target="_blank">PolarArcAnnotationðŸ“˜</a> has several specific properties for its configuration:

| Property | Type | Default | Description |
|----|----|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#centerx" rel="noopener noreferrer" target="_blank">centerXðŸ“˜</a> | `number` | 0 | The X coordinate of the arc's center point. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#centery" rel="noopener noreferrer" target="_blank">centerYðŸ“˜</a> | `number` | 0 | The Y coordinate of the arc's center point. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#x1" rel="noopener noreferrer" target="_blank">x1ðŸ“˜</a> | `number` | \- | The start angle of the annotation. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#x2" rel="noopener noreferrer" target="_blank">x2ðŸ“˜</a> | `number` | \- | The end angle of the annotation. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#y1" rel="noopener noreferrer" target="_blank">y1ðŸ“˜</a> | `number` | \- | The outer radius of the annotation. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#y2" rel="noopener noreferrer" target="_blank">y2ðŸ“˜</a> | `number` | \- | The inner radius of the annotation. This property is ignored when `isLineMode` is true. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polararcannotation.html#islinemode" rel="noopener noreferrer" target="_blank">isLineModeðŸ“˜</a> | `boolean` | `false` | If `true`, the annotation renders as a line at the `y1` radius. If `false`, it renders as a filled sector between the `y1` and `y2` radii. |

![](out_scichartv4/2d-charts/annotations-api/polar-arc-annotation/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To draw a simple arc line instead of a filled sector, set the `isLineMode` property to `true`. This simplifies the annotation to only require `x1`, `x2`, and `y1` for its shape.

![](out_scichartv4/2d-charts/annotations-api/polar-arc-annotation/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To draw a full pie-slice that starts from the center, set the inner radius `y2` to `0`.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/arcannotation.html" rel="noopener noreferrer" target="_blank">ArcAnnotationðŸ“˜</a> - this a similar concept, but for 2D Cartesian surfaces.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/polar-arc-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/polar-arc-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
