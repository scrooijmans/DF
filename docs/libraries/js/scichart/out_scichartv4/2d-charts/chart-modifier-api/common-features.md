On this page

# Common ChartModifiers Features

All the ChartModifiers provided by SciChart.js implement theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase.html" rel="noopener noreferrer" target="_blank">IChartModifierBase interfaceðŸ“˜</a> andÂ derive fromÂ theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" rel="noopener noreferrer" target="_blank">ChartModifierBaseðŸ“˜</a> class. These provide a powerful API whichÂ givesÂ the full access to internals of a chart, axes, series, annotations, mouse, touch events and more.

Please refer to theÂ [What is a ChartModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)Â article for the completeÂ list of all the ChartÂ Modifiers available out of the box in SciChart.

## CommonÂ Features of Chart Modifiers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#commonfeatures-of-chart-modifiers" class="hash-link" aria-label="Direct link to CommonÂ Features of Chart Modifiers" translate="no" title="Direct link to CommonÂ Features of Chart Modifiers">â€‹</a>

### ChartModifierBase type<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#chartmodifierbase-type" class="hash-link" aria-label="Direct link to ChartModifierBase type" translate="no" title="Direct link to ChartModifierBase type">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" rel="noopener noreferrer" target="_blank">ChartModifierBase typeðŸ“˜</a> has the following public API.

Refer to ourÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" rel="noopener noreferrer" target="_blank">TypeDoc DocumentationðŸ“˜</a> for up to date and commented / annotated functions and properties available on this type.

| **Feature** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#parentsurface" rel="noopener noreferrer" target="_blank">.parentSurfaceðŸ“˜</a> | A property to get the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> when the modifier is attached. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#isenabled" rel="noopener noreferrer" target="_blank">.isEnabledðŸ“˜</a> | A property which determines if the current modifier is enabled or not. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#isattached" rel="noopener noreferrer" target="_blank">.isAttachedðŸ“˜</a> | When true, the modifier is attached to a parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#receivehandledevents" rel="noopener noreferrer" target="_blank">.receiveHandledEventsðŸ“˜</a> | When true, the modifier will receive all events even if that event is marked as handled by a previous modifier. When false (default), the modifier will not receive events if they are handled. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#executecondition" rel="noopener noreferrer" target="_blank">.executeConditionðŸ“˜</a> | The primary action execute condition that modifier should respond to (see below). |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#secondaryexecutecondition" rel="noopener noreferrer" target="_blank">.secondaryExecuteConditionðŸ“˜</a> | The secondary action execute condition that modifier should respond to (see below). |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiergroup" rel="noopener noreferrer" target="_blank">.modifierGroupðŸ“˜</a> | Specifies a string ID to group modifiers. When one receives a mouse event, all modifiers in the same group receive the event. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#onattach" rel="noopener noreferrer" target="_blank">onAttach()ðŸ“˜</a> | Called when the modifier is attached to aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#onparentsurfacerendered" rel="noopener noreferrer" target="_blank">onParentSurfaceRendered()ðŸ“˜</a> | Called when the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> is rendered. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiermousedown" rel="noopener noreferrer" target="_blank">modifierMouseDown()ðŸ“˜</a> | Called when a mouse or touch-down event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiermousemove" rel="noopener noreferrer" target="_blank">modifierMouseMove()ðŸ“˜</a> | Called when a mouse or touch-move event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiermouseup" rel="noopener noreferrer" target="_blank">modifierMouseUp()ðŸ“˜</a> | Called when a mouse or touch-up event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiermousewheel" rel="noopener noreferrer" target="_blank">modifierMouseWheel()ðŸ“˜</a> | Called when a mouse wheel event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifierdoubleclick" rel="noopener noreferrer" target="_blank">modifierDoubleClick()ðŸ“˜</a> | Called when a mouse or touch double-click event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiermouseenter" rel="noopener noreferrer" target="_blank">modifierMouseEnter()ðŸ“˜</a> | Called when a mouse-enter event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#modifiermouseleave" rel="noopener noreferrer" target="_blank">modifierMouseLeave()ðŸ“˜</a> | Called when a mouse-leave event occurs on the parentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. |

### Execute Conditions<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#execute-conditions" class="hash-link" aria-label="Direct link to Execute Conditions" translate="no" title="Direct link to Execute Conditions">â€‹</a>

Chart modifiers can be configured to respond to specific mouse and keyboard combinations using `executeCondition` and `secondaryExecuteCondition` properties. These conditions determine when the modifier should activate.

Each condition can specify:

- A mouse button (`button`) from `EExecuteOn` enum (e.g., MouseLeftButton, MouseMiddleButton, MouseRightButton)
- A keyboard modifier key (`key`) from `EModifierMouseArgKey` enum (Shift, Ctrl, Alt, or None)

**Available Mouse Buttons:**

``` prism-code
enum EExecuteOn {
    MouseLeftButton = 0,    // Primary mouse button
    MouseMiddleButton = 1,  // Middle mouse button/wheel
    MouseRightButton = 2,   // Secondary mouse button
    BrowserBackButton = 3,  // Browser back button
    BrowserForwardButton = 4 // Browser forward button
}
```

**Available Modifier Keys:**

``` prism-code
enum EModifierMouseArgKey {
    None = 0,   // No modifier key
    Shift = 1,  // Shift key
    Ctrl = 2,   // Control key
    Alt = 4     // Alt/Option key
}
```

**Common Usage Patterns:**

1.  **Basic mouse button activation:**

``` prism-code
// Activate on right mouse button only
new RubberBandXyZoomModifier({
    executeCondition: { button: EExecuteOn.MouseRightButton }
})
```

2.  **Keyboard modifier combinations:**

``` prism-code
// Require Ctrl+Left mouse button
new ZoomPanModifier({
    executeCondition: {
        button: EExecuteOn.MouseLeftButton,
        key: EModifierMouseArgKey.Ctrl
    }
})
```

3.  **Different primary and secondary actions:**

``` prism-code
// Primary: Left mouse drag
// Secondary: Right mouse drag with Shift key
new CursorModifier({
    executeCondition: { button: EExecuteOn.MouseLeftButton },
    secondaryExecuteCondition: {
        button: EExecuteOn.MouseRightButton,
        key: EModifierMouseArgKey.Shift
    }
})
```

4.  **Browser navigation buttons:**

``` prism-code
// Use browser back/forward buttons for navigation
new CustomModifier({
    executeCondition: { button: EExecuteOn.BrowserBackButton },
    secondaryExecuteCondition: { button: EExecuteOn.BrowserForwardButton }
})
```

5.  **Multiple modifier combinations:**

``` prism-code
// Complex combination example
new TooltipModifier({
    executeCondition: {
        button: EExecuteOn.MouseMiddleButton,
        key: EModifierMouseArgKey.Alt | EModifierMouseArgKey.Ctrl
    }
})
```

**Important Notes:**

- The `executeCondition` is the primary activation trigger
- The `secondaryExecuteCondition` provides an alternative activation method
- Modifier keys can be combined using bitwise OR (e.g., `Ctrl|Shift`)
- When no condition is specified, most modifiers default to left mouse button with no modifiers
- The `EExecuteOn` enum values correspond to standard mouse button indices (0=left, 1=middle, 2=right)

**Advanced Example: Custom Interaction Scheme**

``` prism-code
sciChartSurface.chartModifiers.add(
    // Zoom with Ctrl+Left drag
    new RubberBandXyZoomModifier({
        executeCondition: {
            button: EExecuteOn.MouseLeftButton,
            key: EModifierMouseArgKey.Ctrl
        }
    }),
    // Pan with Middle mouse drag
    new ZoomPanModifier({
        executeCondition: { button: EExecuteOn.MouseMiddleButton }
    }),
    // Show tooltips on Alt+Right click
    new CursorModifier({
        executeCondition: {
            button: EExecuteOn.MouseRightButton,
            key: EModifierMouseArgKey.Alt
        }
    })
);
```

This configuration creates a sophisticated interaction model where:

- Ctrl+Left drag performs rectangular zoom
- Middle mouse drag pans the chart
- Alt+Right click shows cursor tooltips
- All other interactions remain available for other modifiers

### Series Interaction<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#series-interaction" class="hash-link" aria-label="Direct link to Series Interaction" translate="no" title="Direct link to Series Interaction">â€‹</a>

Chart modifiers can interact with specific series through these methods:

| **Method** | **Description** |
|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#onattachseries" rel="noopener noreferrer" target="_blank">onAttachSeries()ðŸ“˜</a> | Called when a renderable series is attached to the chart |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#ondetachseries" rel="noopener noreferrer" target="_blank">onDetachSeries()ðŸ“˜</a> | Called when a renderable series is detached from the chart |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html#includeseries" rel="noopener noreferrer" target="_blank">includeSeries()ðŸ“˜</a> | Controls whether a series should be included in the modifier's behavior |

### Including/Excluding Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#includingexcluding-series" class="hash-link" aria-label="Direct link to Including/Excluding Series" translate="no" title="Direct link to Including/Excluding Series">â€‹</a>

The `includeSeries()` method is particularly important for modifiers that display legends or tooltips (like `CursorModifier`, `LegendModifier`, `RolloverModifier`, etc.). It allows you to control which series should be included in the modifier's behavior.

Example usage:

``` prism-code
// Include a specific series in the modifier
modifier.includeSeries(mySeries, true);

// Exclude a series from the modifier
modifier.includeSeries(mySeries, false);
```

When a series is included/excluded, the modifier will update its internal state (e.g., update tooltip content or legend items) if it's currently attached to a chart.

### ChartModifierBase2D Type<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#chartmodifierbase2d-type" class="hash-link" aria-label="Direct link to ChartModifierBase2D Type" translate="no" title="Direct link to ChartModifierBase2D Type">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html" rel="noopener noreferrer" target="_blank">ChartModifierBase2DðŸ“˜</a> type provides a base class for all modifiers on 2DÂ [SciChartSurfaces](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview) and 2D [SciChartPolarSurfaces](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type). Use this type when creating custom modifiers for 2D Charts.

### ChartModifierBase3D Type<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#chartmodifierbase3d-type" class="hash-link" aria-label="Direct link to ChartModifierBase3D Type" translate="no" title="Direct link to ChartModifierBase3D Type">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html" rel="noopener noreferrer" target="_blank">ChartModifierBase3DðŸ“˜</a> type provides a base class for all modifiers on **SciChart3DSurfaces**. Use this type when creating custom modifiers for 3D Charts.Â 

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/common-features/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/common-features/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
