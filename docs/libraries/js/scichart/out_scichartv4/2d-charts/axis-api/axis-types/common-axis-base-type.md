On this page

# Common Axis Base Type and Options

Common Axis Base Type and Options

All the axis types in SciChart inherit fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" rel="noopener noreferrer" target="_blank">AxisCoreðŸ“˜</a> - a common type shared across both 2D and 3D Charts - and all 2D Axis inheritÂ **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" rel="noopener noreferrer" target="_blank">AxisBase2DðŸ“˜</a>**.Â 

The inheritance diagram for Axis in SciChart.js looks like this:

## Common Properties on an Axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type/#common-properties-on-an-axis" class="hash-link" aria-label="Direct link to Common Properties on an Axis" translate="no" title="Direct link to Common Properties on an Axis">â€‹</a>

Common properties on an axis allow you to configure the following things:

- Showing/hiding and styling of GridlinesÂ 
- Showing/hiding and styling of labels
- Alignment of the axis
- Formatting of labels
- Getting / setting visibleRange or padding
- Getting / setting ID - used in multi-axis scenarios
- Styling border, background
- Setting axis title

The properties common to theÂ **AxisBase2D** / **AxisCore** classes can be found in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html" rel="noopener noreferrer" target="_blank">TypeDoc API documentationðŸ“˜</a>.

## Specific Axis Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/common-axis-base-type/#specific-axis-types" class="hash-link" aria-label="Direct link to Specific Axis Types" translate="no" title="Direct link to Specific Axis Types">â€‹</a>

The following sections go into further details for specific axis types, as well as giving code samples on how to configure and use each axis.

| Axis Type | Description |
|----|----|
| **[NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis)** | Value Axis / Numeric Types |
| **[PolarNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis)** | Value Axis / Numeric Types for [Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type) |
| **[DateTimeNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis)** | Value Axis with additional features for Dates and Time formatting |
| **[CategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis)** | Category Axis - measures using index - Numeric Types or Dates |
| **[PolarCategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis)** | Category Axis for [Polar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-polar-surface-type) - measures using index - Numeric Types or Dates |
| **[LogarithmicAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis)** | Logarithmic Axis supporting Base2, BaseE, Base10 with or without scientific notation |

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-types/common-axis-base-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-types/common-axis-base-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
