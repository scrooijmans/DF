On this page

# Hover and Click Interactions

In SciChart.js, you can implement both click and hover events on charts using several different approaches. Here are the main methods for detecting these interactions:

## Click Events on Charts<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#click-events-on-charts" class="hash-link" aria-label="Direct link to Click Events on Charts" translate="no" title="Direct link to Click Events on Charts">â€‹</a>

### Custom Chart Modifier for Click Detection<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#custom-chart-modifier-for-click-detection" class="hash-link" aria-label="Direct link to Custom Chart Modifier for Click Detection" translate="no" title="Direct link to Custom Chart Modifier for Click Detection">â€‹</a>

The most flexible approach is creating a CustomChartModifier that extends <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html" rel="noopener noreferrer" target="_blank">ChartModifierBase2DðŸ“˜</a>

``` prism-code
class SimpleChartModifier extends CustomChartModifier2D {
    readonly type = EChart2DModifierType.Custom;

    modifierMouseDown(args: ModifierMouseArgs) {
        super.modifierMouseDown(args);
        statusLabel.text = `MouseDown at point ${args.mousePoint.x}, ${args.mousePoint.y}`;
    }

    modifierDoubleClick(args: ModifierMouseArgs) {
        super.modifierDoubleClick(args);
        statusLabel.text = `DoubleClick at point ${args.mousePoint.x}, ${args.mousePoint.y}`;
    }
}

sciChartSurface.chartModifiers.add(new SimpleChartModifier());
```

### Series Selection Modifier<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#series-selection-modifier" class="hash-link" aria-label="Direct link to Series Selection Modifier" translate="no" title="Direct link to Series Selection Modifier">â€‹</a>

For built-in series click functionality, use the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a>

``` prism-code
const seriesSelectionModifier = new SeriesSelectionModifier({
    enableSelection: true,
    onSelectionChanged: args => {
        console.log("Series selected:", args.selectedSeries[0]);
    }
});

sciChartSurface.chartModifiers.add(seriesSelectionModifier);
```

## Hover Events on Charts<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#hover-events-on-charts" class="hash-link" aria-label="Direct link to Hover Events on Charts" translate="no" title="Direct link to Hover Events on Charts">â€‹</a>

### Hover Detection with Series Selection Modifier<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#hover-detection-with-series-selection-modifier" class="hash-link" aria-label="Direct link to Hover Detection with Series Selection Modifier" translate="no" title="Direct link to Hover Detection with Series Selection Modifier">â€‹</a>

Enable hover detection using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a>

``` prism-code
const seriesSelectionModifier = new SeriesSelectionModifier({
    enableHover: true,
    onHoverChanged: args => {
        console.log("Series hovered:", args.hoveredSeries[0]);
    }
});
 sciChartSurface.chartModifiers.add(seriesSelectionModifier);
```

### Series-Level Hover Events<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#series-level-hover-events" class="hash-link" aria-label="Direct link to Series-Level Hover Events" translate="no" title="Direct link to Series-Level Hover Events">â€‹</a>

You can attach hover callbacks directly to RenderableSeries

``` prism-code
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    strokeThickness: 3,
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: xLineValues,
        yValues: yLineValues,
        dataSeriesName: "Line Series"
    }),
    onHoveredChanged: (sourceSeries, isHovered) => {
        console.log(`Series ${sourceSeries.dataSeries.dataSeriesName} hovered: ${isHovered}`);
        // Change appearance on hover
        sourceSeries.strokeThickness = isHovered ? 6 : 3;
    }
});

sciChartSurface.renderableSeries.add(lineSeries);

sciChartSurface.chartModifiers.add(
    new SeriesSelectionModifier({
        enableHover: true,
        enableSelection: false // Set to false if you only want hover, not selection
    })
);
```

### Custom Modifier for Advanced Hover Detection<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#custom-modifier-for-advanced-hover-detection" class="hash-link" aria-label="Direct link to Custom Modifier for Advanced Hover Detection" translate="no" title="Direct link to Custom Modifier for Advanced Hover Detection">â€‹</a>

For detecting hover on specific chart parts (axes, series, chart area), create a custom modifier

``` prism-code
const seriesSelectionModifier = new SeriesSelectionModifier({
    enableHover: true,
    onHoverChanged: args => {
        const hoveredSeries = args.hoveredSeries;
        if (hoveredSeries && hoveredSeries.length > 0) {
            // Get mouse coordinates from the chart surface
            sciChartSurface.domCanvas2D.addEventListener(
                "mousemove",
                mouseEvent => {
                    const premultipliedX = mouseEvent.offsetX * DpiHelper.PIXEL_RATIO;
                    const premultipliedY = mouseEvent.offsetY * DpiHelper.PIXEL_RATIO;

                    hoveredSeries.forEach(series => {
                        const hitTestInfo = series.hitTestProvider.hitTest(premultipliedX, premultipliedY, 10);

                        if (hitTestInfo.isHit) {
                            console.log("Hovered data point:");
                            console.log("- Index:", hitTestInfo.dataSeriesIndex);
                            console.log(
                                "- Values:",
                                hitTestInfo.xValue,
                                hitTestInfo.yValue,
                                hitTestInfo.x1Value,
                                hitTestInfo.y1Value
                            );
                            statusLabel.text = `Hovered data point: Index: ${hitTestInfo.dataSeriesIndex}, Values: ${hitTestInfo.xValue} ${hitTestInfo.yValue} ${hitTestInfo.x1Value} ${hitTestInfo.y1Value}`;
                        }
                    });
                },
                { once: true }
            ); // Use once to avoid multiple listeners
        } else {
            statusLabel.text = "";
        }
    }
});

sciChartSurface.chartModifiers.add(seriesSelectionModifier);
```

### Annotation Hover Events<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_hover_interactions/#annotation-hover-events" class="hash-link" aria-label="Direct link to Annotation Hover Events" translate="no" title="Direct link to Annotation Hover Events">â€‹</a>

For annotations, SciChart.js provides dedicated hover functionality

``` prism-code
// Create AnnotationHoverModifier to enable hover detection
const annotationHoverModifier = new AnnotationHoverModifier({
    enableHover: true,
    targets: [boxAnnotation, secondBoxAnnotation], // Specify which annotations to monitor
    hoverMode: EHoverMode.AbsoluteTopmost, // Only top annotation if overlapping
    notifyOutEvent: true, // Fire events when mouse leaves annotation
    notifyPositionUpdate: false, // Don't fire events on position updates within annotation

    // Global hover callback for all targeted annotations
    onHover: args => {
        const { hoveredEntities, unhoveredEntities } = args;

        // Change appearance of hovered annotations
        hoveredEntities.forEach(annotation => {
            console.log(annotation);

            if (annotation instanceof BoxAnnotation) {
                annotation.fill = "#34eb8c77"; // Semi-transparent green
                annotation.stroke = "#34eb8c";
                annotation.strokeThickness = 3;
            }
            console.log("Annotation became hovered:", annotation);
        });

        // Reset appearance of unhovered annotations
        unhoveredEntities.forEach(annotation => {
            if (annotation instanceof BoxAnnotation) {
                annotation.fill = "#3d34eb77"; // Back to blue
                annotation.stroke = "#3d34eb";
                annotation.strokeThickness = 1;
            }
            console.log("Annotation became unhovered:", annotation);
        });
    }
});

// Add the hover modifier to enable the functionality
sciChartSurface.chartModifiers.add(annotationHoverModifier);
```

The ChartModifier API provides comprehensive mouse event handling including modifierMouseDown, modifierMouseUp, modifierMouseMove, modifierDoubleClick, and modifierMouseWheel methods for creating sophisticated chart interactions.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/faqs/faq_hover_interactions/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/faqs/faq_hover_interactions/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
