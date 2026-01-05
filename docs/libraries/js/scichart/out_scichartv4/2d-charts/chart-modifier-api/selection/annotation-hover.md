On this page

# Annotation Hover

Annotations Hover API is an opt-in functionality that allows detecting hover events on chart annotations and adding event handlers.

![](out_scichartv4/2d-charts/chart-modifier-api/selection/annotation-hover/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/stock-chart-buy-sell-markers" rel="noopener noreferrer" target="_blank">JavaScript Hoverable Trade Markers Example</a> can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/stock-chart-buy-sell-markers" target="_blank">Annotation Hover Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Using AnnotationHoverModifier<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/#using-annotationhovermodifier" class="hash-link" aria-label="Direct link to Using AnnotationHoverModifier" translate="no" title="Direct link to Using AnnotationHoverModifier">â€‹</a>

To enable the hover detection we need to add theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html" rel="noopener noreferrer" target="_blank">AnnotationHoverModifierðŸ“˜</a>.

- TS
- Builder API (JSON Config)

``` prism-code
// Add an annotation with hover behaviour
const boxAnnotation = new BoxAnnotation({
    xCoordinateMode: ECoordinateMode.Relative,
    yCoordinateMode: ECoordinateMode.Relative,
    fill: "#3d34eb",
    strokeThickness: 1,
    x1: 0.1,
    x2: 0.4,
    y1: 0.4,
    y2: 0.6,
    onHover: args => {
        const { sender, mouseArgs, isHovered } = args;
        if (mouseArgs && isHovered) {
            const relativeCoordinates = args.getRelativeCoordinates();
            console.log("The annotation is hovered at", relativeCoordinates);
        }
    }
});
sciChartSurface.annotations.add(boxAnnotation);
// Add AnnotationHoverModifier to enable hover behaviour
const annotationHoverModifier = new AnnotationHoverModifier({
    enableHover: true,
    targets: [boxAnnotation],
    hoverMode: EHoverMode.AbsoluteTopmost,
    notifyOutEvent: true,
    notifyPositionUpdate: true,
    onHover: args => {
        const { mouseArgs, includedEntities, hoveredEntities, unhoveredEntities } = args;
        const hoveredAnnotations = /** @type {BoxAnnotation[]} */ hoveredEntities;
        const unhoveredAnnotations = /** @type {BoxAnnotation[]} */ unhoveredEntities;
        hoveredAnnotations.forEach(a => {
            const annotation = a as BoxAnnotation;
            annotation.fill = "#34eb8c";
            annotation.strokeThickness = 3;
        });
        unhoveredAnnotations.forEach(a => {
            const annotation = a as BoxAnnotation;
            annotation.fill = "#3d34eb";
            annotation.strokeThickness = 1;
        });
    }
});
sciChartSurface.chartModifiers.add(annotationHoverModifier);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: {
        theme: new SciChartJsNavyTheme()
    },
    // Add an annotation with hover behaviour
    annotations: [
        {
            type: EAnnotationType.RenderContextBoxAnnotation,
            options: {
                id: "boxAnnotation",
                xCoordinateMode: ECoordinateMode.Relative,
                yCoordinateMode: ECoordinateMode.Relative,
                fill: "#3d34eb",
                strokeThickness: 1,
                x1: 0.1,
                x2: 0.4,
                y1: 0.4,
                y2: 0.6,
                onHover: args => {
                    const { sender, mouseArgs, isHovered } = args;
                    if (mouseArgs && isHovered) {
                        const relativeCoordinates = args.getRelativeCoordinates();
                        console.log("The annotation is hovered at", relativeCoordinates);
                    }
                }
            }
        }
    ],
    // Add AnnotationHoverModifier to enable hover behaviour
    modifiers: [
        {
            type: EChart2DModifierType.AnnotationHover,
            options: {
                enableHover: true,
                targets: ["boxAnnotation"],
                hoverMode: EHoverMode.AbsoluteTopmost,
                notifyOutEvent: true,
                notifyPositionUpdate: true,
                onHover: args => {
                    const { mouseArgs, includedEntities, hoveredEntities, unhoveredEntities } = args;
                    const hoveredAnnotations = hoveredEntities;
                    const unhoveredAnnotations = unhoveredEntities;
                    hoveredAnnotations.forEach(annotation => {
                        annotation.fill = "#34eb8c";
                        annotation.strokeThickness = 3;
                    });
                    unhoveredAnnotations.forEach(annotation => {
                        annotation.fill = "#3d34eb";
                        annotation.strokeThickness = 1;
                    });
                }
            }
        }
    ]
});
```

## Adding Hover Event Handlers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/#adding-hover-event-handlers" class="hash-link" aria-label="Direct link to Adding Hover Event Handlers" translate="no" title="Direct link to Adding Hover Event Handlers">â€‹</a>

There are several different options for adding a callback for the hover event.

1.  By passingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#onhover" rel="noopener noreferrer" target="_blank">onHoverðŸ“˜</a> via anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#constructor" rel="noopener noreferrer" target="_blank">annotation constructor optionsðŸ“˜</a>, orÂ subscribing to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#hovered" rel="noopener noreferrer" target="_blank">annotation.hoveredðŸ“˜</a> event handler on anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html" rel="noopener noreferrer" target="_blank">AnnotationBaseðŸ“˜</a> derived type.Â  This allows you to have different hover behaviour for each annotation, and the args has a method which allows you to find out where on the annotation the over occurred.

<!-- -->

2.  By passingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html#onhover" rel="noopener noreferrer" target="_blank">onHoverðŸ“˜</a> via the constructor of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html" rel="noopener noreferrer" target="_blank">AnnotationHoverModifierðŸ“˜</a> or subscribing to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html#hoverchanged" rel="noopener noreferrer" target="_blank">annotationHoverModifier.hoverChangedðŸ“˜</a> event handler on an instanceÂ ofÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html" rel="noopener noreferrer" target="_blank">AnnotationHoverModifierðŸ“˜</a>.Â  This gives you a single callback with access toÂ both theÂ hovered and unhovered annotations, allowing you to define common hoverÂ behaviour in one place, andÂ enabling you to update other annotations when one is hovered.

- Hover on modifier

``` prism-code
// subscribe via Event Handler
boxAnnotation.hovered.subscribe(args => {
    // ...
});
```

These approaches could be used simultaneously.

## Hover Detection Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/#hover-detection-options" class="hash-link" aria-label="Direct link to Hover Detection Options" translate="no" title="Direct link to Hover Detection Options">â€‹</a>

The hover detection functionality is managed by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html" rel="noopener noreferrer" target="_blank">AnnotationHoverModifierðŸ“˜</a>.Â  By default, the modifier checks every annotation withinÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#annotations" rel="noopener noreferrer" target="_blank">SciChartSurface.annotationsðŸ“˜</a> that is visible upon mouse move.Â Â  Then if the mouse position is over any of the annotations, only the one that is above all others is considered to be hovered.Â  An event will be raised if the hovered state of annotation has changed (e.g. it became hovered or unhovered).Â 

These behaviors can be modified via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html" rel="noopener noreferrer" target="_blank">IAnnotationHoverModifierOptionsðŸ“˜</a>.

### Hover Detection Mode<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/#hover-detection-mode" class="hash-link" aria-label="Direct link to Hover Detection Mode" translate="no" title="Direct link to Hover Detection Mode">â€‹</a>

The hover detection rules are defined by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationhovermodifier.html#hovermode" rel="noopener noreferrer" target="_blank">hoverModeðŸ“˜</a> property accepting values fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehovermode.html" rel="noopener noreferrer" target="_blank">EHoverModeðŸ“˜</a> enum.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehovermode.html#absolutetopmost" rel="noopener noreferrer" target="_blank">AbsoluteTopmostðŸ“˜</a> (default) - selects only one annotation that is not overlayed by any other annotation at the mouse position; the mode checks both included and ignored targets, but can select only an included one.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehovermode.html#topmostincluded" rel="noopener noreferrer" target="_blank">TopmostIncludedðŸ“˜</a> - selects only one annotation that is not overlayed by any other annotation at the mouse position; the mode checks and selects only included targets.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehovermode.html#multi" rel="noopener noreferrer" target="_blank">MultiðŸ“˜</a> - selects multiple included annotations at the mouse position.

### Types of actions that trigger event<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/#types-of-actions-that-trigger-event" class="hash-link" aria-label="Direct link to Types of actions that trigger event" translate="no" title="Direct link to Types of actions that trigger event">â€‹</a>

To modify the hover change condition when an event should be raised useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html#notifyoutevent" rel="noopener noreferrer" target="_blank">notifyOutEventðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html#notifypositionupdate" rel="noopener noreferrer" target="_blank">notifyPositionUpdateðŸ“˜</a> flags.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html#notifyoutevent" rel="noopener noreferrer" target="_blank">notifyOutEventðŸ“˜</a>Â - defines whether an event should be raised when any of the annotations has become unhovered. For example, if set to false an event won't be raised when all of the annotations are unhovered. Defaults to true.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html#notifypositionupdate" rel="noopener noreferrer" target="_blank">notifyPositionUpdateðŸ“˜</a>Â - defines whether an event should be raised when the list of hovered and unhovered annotations haven't changed. E.g. the mouse position changed within the bounds of an already hovered annotation. Defaults to false.

## Hover Targets<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/#hover-targets" class="hash-link" aria-label="Direct link to Hover Targets" translate="no" title="Direct link to Hover Targets">â€‹</a>

To check only a specific set of annotations use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationhovermodifieroptions.html#targets" rel="noopener noreferrer" target="_blank">targetsðŸ“˜</a> property. ItÂ can accept either

- an array of annotations
- and array ofÂ annotation ids
- aÂ function returning an array of annotations
- the name of a function registered with theÂ [Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)

<!-- -->

- Target Selector

``` prism-code
const targetsSelector = (modifier) => hoverableAnnotations;

const annotationHoverModifier = new AnnotationHoverModifier({
    targets: targetsSelector,
    hoverMode: EHoverMode.Multi
});
```

Â Here is a simpleÂ example using the methods described above

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/selection/annotation-hover/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/selection/annotation-hover/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
