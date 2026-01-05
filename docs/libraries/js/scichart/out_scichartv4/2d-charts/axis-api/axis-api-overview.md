On this page

# Axis API Overview

SciChart.js has a rich and configurable axis API. We believe you won't find a JavaScript Chart in the world with as many options for axis as SciChart!

This page we're going to give you an overview of what the Axis APIs can do and then show you where to look next for more detail.

# What can SciChart.js Do with Axis?

Heres a quick list of what SciChart.js can do with axis configuration, and where to go next.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-chart-axis-layout-options" target="_blank">Axis Layout Options</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing a variety of axis configurations in SciChart.js.

  

## Many Axis Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview/#many-axis-types" class="hash-link" aria-label="Direct link to Many Axis Types" translate="no" title="Direct link to Many Axis Types">â€‹</a>

There are several axis typesÂ in SciChart.js. Although they allÂ differ inÂ types of data valuesÂ that can be rendered,Â the most fundamental difference is in theirÂ behavior.

By that, the axes can be divided into two groups, Category and Value axis types.

The axis types provided by SciChart.js are listed below:

Here's the content formatted as a two-column Markdown table:

| Axis Type | Description |
|----|----|
| **[NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis)** | Value Axis / Numeric Types |
| **[PolarNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis)** | Value Axis / Numeric Types for [Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type) |
| **[DateTimeNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis)** | Value Axis with additional features for Dates and Time formatting |
| **[CategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis)** | Category Axis - measures using index - Numeric Types or Dates |
| **[PolarCategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis)** | Category Axis for [Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type) - measures using index - Numeric Types or Dates |
| **[LogarithmicAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis)** | Logarithmic Axis supporting Base2, BaseE, Base10 with or without scientific notation |
| **[Text / String Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis)** | Use LabelProviders to format axis labels as text |

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-axis-types" target="_blank">Axis Types</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

To learn more about the axis types, click one of the article links in the table above.

## Axis Layout (Multiple Axis, Axis Alignment)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview/#axis-layout-multiple-axis-axis-alignment" class="hash-link" aria-label="Direct link to Axis Layout (Multiple Axis, Axis Alignment)" translate="no" title="Direct link to Axis Layout (Multiple Axis, Axis Alignment)">â€‹</a>

### Q: Is it easier to render axes and layouts with SciChart compared to competitors?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview/#q-is-it-easier-to-render-axes-and-layouts-with-scichart-compared-to-competitors" class="hash-link" aria-label="Direct link to Q: Is it easier to render axes and layouts with SciChart compared to competitors?" translate="no" title="Direct link to Q: Is it easier to render axes and layouts with SciChart compared to competitors?">â€‹</a>

Yes, with SciChart.js, many axis configurations are possible, these are covered in detail in [What Axis Configuration Options and Axis Layout Options does SciChart.js Support?](https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-axis-configuration-options-and-axis-layout-options-does-scichartjs-support)

For an overview of axis layout options, see the documentation links below:

- [Aligning Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview) on the Left, Right
- [Adding a Secondary Axis](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis)
- [Adding Multiple X and Y Axis](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-08-adding-multiple-axis)
- [Rotating a chart 90 degrees](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis) (Vertical charts)
- [Drawing Series behind axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout)
- [Placing axis in the centre of a chart](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout), or inside a chart surface
- [Vertically Stacking Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout) - to create complex layouts
- [Horizontally Stacking Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout)Â - more complex layouts
- [Advanced Custom Axis Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/advanced-options-custom-layout-managers/) via the Layout Provider API

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-chart-with-multiple-x-axis" target="_blank">Multiple X Axis</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

To learn more about the axis layout options see the pages in the list above.

## Axis Label Configuration<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview/#axis-label-configuration" class="hash-link" aria-label="Direct link to Axis Label Configuration" translate="no" title="Direct link to Axis Label Configuration">â€‹</a>

SciChart.js has a number of label APIs, including:

- [Formatting labels the easy way](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) (using built-in flags)
- [Formatting labels - using custom code](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) (fine grained label format)
- [Having a text axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis) e.g. "Apples" "Pears" "Oranges" not 1, 2, 3
- [Turning native (WebGL) text labels on or off](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels)
- [Spacing gridlines and labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval) the easy way
- [Spacing gridlines and labels - using custom code](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/tick-provider-api) (fine grained control)
- [Rotating Labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/rotating-axis-labels)Â /Â [Multiline Labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/text-and-multi-line-labels)Â /Â [Image Labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/image-labels)
- [Label Style, Alignment, Positioning](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-style-alignment-and-positioning)

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-multiline-labels" target="_blank">Multiline Axis Labels</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

To learn more about the axis labelling options seeÂ [Axis Label Formatting](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/)Â and related pages

## Axis Ranging and Scaling<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview/#axis-ranging-and-scaling" class="hash-link" aria-label="Direct link to Axis Ranging and Scaling" translate="no" title="Direct link to Axis Ranging and Scaling">â€‹</a>

It's possible to programmatically control axis ranging, scaling and auto-fitting of data.

- [AutoRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range) (auto fitting of data)
- [Setting/Getting range programatically](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit)
- [Listening to axis range changes](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/listen-to-visible-range-changes) (callbacks on zoom)

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/zoom-pan-multiple-modifiers" target="_blank">Multiple zoom, pan behaviours</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

To learn more about the axis labelling options seeÂ [Axis Label Formatting](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview)Â and related pages

Zooming and panning of Axis (such as mouse-drag, or mousewheel zoom) is handled by the ChartModifiers. See sections in theÂ [ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier) for more details.

## Axis Styling<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview/#axis-styling" class="hash-link" aria-label="Direct link to Axis Styling" translate="no" title="Direct link to Axis Styling">â€‹</a>

Finally, SciChart.js supports Axis styling, including:

- [Styling of Gridlines, Labels, Titles and Bands](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/title-labels-gridlines-axis-band-style)
- [Styling of Axis Borders and Background](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/axis-borders-and-background)
- [Showing or Hiding of Axis parts](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/visibility-of-axis-elements)
- [Styling of Polar Axes](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling)

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/chart-styling-theming-in-code" target="_blank">Chart with custom style applied in code</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

To learn more about the axis styling options seeÂ [Axis Styling](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/title-labels-gridlines-axis-band-style)Â and related pages

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
