On this page

# What is the ChartModifier API

Within the SciChart.js JavaScript Chart SDK, ChartModifiers are the classes which can be added to aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> to give it a **certain** **behavior**. For instance, all **zooming, panning operations**, **tooltips**, **legends** and even **selection** of points or lines are handled byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" rel="noopener noreferrer" target="_blank">ChartModifierBaseðŸ“˜</a> derived classes in the SciChart codebase.

There are many different ChartModifiers provided by SciChart and each one deserves an article by itself! This article is concerned with simply giving an overview of the modifiers and where you can find the examples in our Examples Suite which demonstrate them.

There are also several individual articles on the ChartModifiers and how to configure them in the SciChart.js Documentation. Please find them at the bottom of this page.

## Zoom, Pan Modifiers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview/#zoom-pan-modifiers" class="hash-link" aria-label="Direct link to Zoom, Pan Modifiers" translate="no" title="Direct link to Zoom, Pan Modifiers">â€‹</a>

The following modifiersÂ can be used if you want to add scrolling or zooming behavior to a chart:

| Modifier Name | Description |
|----|----|
| **[ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier)** | **Pans** the chart in X, Y or both directions with inertia via finger sliding. |
| **[MouseWheelZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier)** | **Zooms** the chart in or out on mouse-wheel (or two finger scroll). |
| **[XAxisDragModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier)** | **Scales** or pans an X Axis via mouse-drag. |
| **[YAxisDragModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/y-axis-drag-modifier)** | **Scales** or pans a Y Axis via mouse-drag. |
| **[RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier)** | **Zooms** a chart inside a rectangle or horizontal section that is drawn on the chart with a finger. |
| **[ZoomExtentsModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier)** | **Resets the zoom** to the data extents via double-tapping. |
| **[SciChartOverview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/overview)** | Creates an **overview chart** that allows you to **zoom and pan** the main chart. |

## Interactivity, Tooltips, Cursor Modifiers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview/#interactivity-tooltips-cursor-modifiers" class="hash-link" aria-label="Direct link to Interactivity, Tooltips, Cursor Modifiers" translate="no" title="Direct link to Interactivity, Tooltips, Cursor Modifiers">â€‹</a>

TheseÂ modifiersÂ allow to interact with chart series or inspect them:

| **Modifier Name** | **Description** |
|----|----|
| **[RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier)** | Provides a **vertical slice cursor with tooltips** and markers rolling over a series. |
| **[CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview)** | Provides a **crosshairs** with a tooltip and axis labels. |

## Miscellaneous Modifiers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview/#miscellaneous-modifiers" class="hash-link" aria-label="Direct link to Miscellaneous Modifiers" translate="no" title="Direct link to Miscellaneous Modifiers">â€‹</a>

Modifiers below are used as helpers and can be a useful addition to a chart:

| **Modifier Name** | **Description** |
|----|----|
| **[LegendModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/miscellaneous-modifiers/legend-modifier)** | AllowsÂ creationÂ and configuration a **Legend** for a chart. |

To learn more about ChartModifiers API, please read theÂ [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features) article. To find out about a specific ChartModifier type, please refer to a corresponding article about this Modifier type.

## Polar Modifiers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview/#polar-modifiers" class="hash-link" aria-label="Direct link to Polar Modifiers" translate="no" title="Direct link to Polar Modifiers">â€‹</a>

Below it the list of chart modifiers for [Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type).

| **Modifier Name** | **Description** |
|----|----|
| **[PolarArcZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-arc-zoom-modifier)** | AllowsÂ zooming to a selected arc on Polar surface. |
| **[PolarCursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-cursor-modifier)** | Displays a tooltip for closest data point when hovering over the polar chart |
| **[PolarDataPointSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-data-point-selection-modifier)** | Allows for data point selection for polar charts |
| **[PolarLegendModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-legend-modifier)** | AllowsÂ creationÂ and configuration a **Legend** for a Polar chart. |
| **[PolarMouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-mouse-wheel-zoom-modifier)** | Allows users to rotate or zoom in and out of a polar chart using the mouse wheel |
| **[PolarPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-pan-modifier)** | Allows users to pan, rotate and zoom in/out a polar chart using the mouse |
| **[PolarZoomExtends](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/polar-modifiers/polar-zoom-extents-modifier)** | Allows users to reset chart to initial state |

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/chart-modifier-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/chart-modifier-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
