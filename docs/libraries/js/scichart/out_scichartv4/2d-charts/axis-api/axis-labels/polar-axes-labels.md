On this page

# Polar Axis Labels

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarlabelmode" rel="noopener noreferrer" target="_blank">Polar Axis LabelsðŸ“˜</a> API in SciChart.js provides functionality for customizing the appearance and behavior of labels on polar axes. This includes options for angular and radial axes, allowing developers to control label visibility, alignment, and formatting.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-label-modes" target="_blank">Polar Label Modes Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

Above: The JavaScript <a href="https://www.scichart.com/demo/react/polar-label-modes" rel="noopener noreferrer" target="_blank">Polar Column Chart</a> example from the <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">SciChart.js Demo</a>.

## Key Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/polar-axes-labels/#key-properties" class="hash-link" aria-label="Direct link to Key Properties" translate="no" title="Direct link to Key Properties">â€‹</a>

![](out_scichartv4/2d-charts/axis-api/axis-labels/polar-axes-labels/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

All of these properties are available on both the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html" rel="noopener noreferrer" target="_blank">PolarNumericAxisðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcategoryaxis.html" rel="noopener noreferrer" target="_blank">PolarCategoryAxisðŸ“˜</a> classes.

### 1. The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#polarlabelmode" rel="noopener noreferrer" target="_blank">PolarAxisBase.polarLabelModeðŸ“˜</a><a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/polar-axes-labels/#1-the-polaraxisbasepolarlabelmodeblue_book" class="hash-link" aria-label="Direct link to 1-the-polaraxisbasepolarlabelmodeblue_book" translate="no" title="Direct link to 1-the-polaraxisbasepolarlabelmodeblue_book">â€‹</a>

> Defines how labels are displayed on the polar axis. The available modes are:

- **<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#horizontal" rel="noopener noreferrer" target="_blank">EPolarLabelMode.HorizontalðŸ“˜</a>**: Labels are displayed horizontally, no matter the rotation, each label will be upright. This is useful for angular axes where you want labels to be readable regardless of their position.
- **<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#parallel" rel="noopener noreferrer" target="_blank">EPolarLabelMode.ParallelðŸ“˜</a>**: Labels are displayed parallel to the axis, which means they will follow the curvature of the axis along their width.
- **<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolarlabelmode.html#perpendicular" rel="noopener noreferrer" target="_blank">EPolarLabelMode.PerpendicularðŸ“˜</a>**: Labels are displayed perpendicular to the axis, following the curvature of the axis along their height. This is useful for radial axes where you want labels to be oriented outward from the center.

### 2. The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#isinneraxis" rel="noopener noreferrer" target="_blank">PolarAxisBase.isInnerAxisðŸ“˜</a><a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/polar-axes-labels/#2-the-polaraxisbaseisinneraxisblue_book" class="hash-link" aria-label="Direct link to 2-the-polaraxisbaseisinneraxisblue_book" translate="no" title="Direct link to 2-the-polaraxisbaseisinneraxisblue_book">â€‹</a>

> Indicates whether the axis labels are drawn on the other side of the axis. For each type of <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/epolaraxismode.html" rel="noopener noreferrer" target="_blank">polarAxisModeðŸ“˜</a>, this property achieves:

- For the **Angular** axis this is a lot more important, as it determines where the labels are drawn in relation to the last angular axis gridline (the biggest circle).

  - `false`: Labels are drawn **outside** the last angular axis gridline (biggest circle), further from the center.
  - `true`: Labels are drawn **inside** the last angular axis gridline, closer to the center.

- The **Radial** axis just draws the labels on the other side of the first radial gridline, but does not entail this much control over the layout.

### 3. The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelprovider" rel="noopener noreferrer" target="_blank">PolarAxisBase.labelProviderðŸ“˜</a>:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/polar-axes-labels/#3-the-polaraxisbaselabelproviderblue_book" class="hash-link" aria-label="Direct link to 3-the-polaraxisbaselabelproviderblue_book" translate="no" title="Direct link to 3-the-polaraxisbaselabelproviderblue_book">â€‹</a>

> Provides the ability to modify / customize when and how the axis labels are formatted.

For the Polar Axes, we have created a special label provider, available out of the box, called <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" rel="noopener noreferrer" target="_blank">RadianLabelProviderðŸ“˜</a>, which formats the labels in radians. Make sure to read the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html" rel="noopener noreferrer" target="_blank">TSDoc indicationsðŸ“˜</a> before using it, and observe how the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#errortolerance" rel="noopener noreferrer" target="_blank">errorToleranceðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/radianlabelprovider.html#maxdenominator" rel="noopener noreferrer" target="_blank">maxDenominatorðŸ“˜</a> pair with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#autoticks" rel="noopener noreferrer" target="_blank">AxisBase.autoTicksðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#majordelta" rel="noopener noreferrer" target="_blank">AxisBase.majorDeltaðŸ“˜</a> to determine the label values.

## Other Base Properties that are of interest for Polar Axes:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/polar-axes-labels/#other-base-properties-that-are-of-interest-for-polar-axes" class="hash-link" aria-label="Direct link to Other Base Properties that are of interest for Polar Axes:" translate="no" title="Direct link to Other Base Properties that are of interest for Polar Axes:">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelpostfix" rel="noopener noreferrer" target="_blank">labelPostfixðŸ“˜</a>: A string that is appended to each label value.

  - For angular axes, this is often set to `Â°` to indicate degrees.
  - For radial axes, it can be set to `m`, `km`, or any other unit of measurement.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#drawminorgirdlines" rel="noopener noreferrer" target="_blank">drawMinorGridLinesðŸ“˜</a>: A boolean that determines whether minor grid lines are drawn on the axis.

  - For smaller polar charts, setting this to `false` can help improve readability by only keeping the major grid lines

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#labelprecision" rel="noopener noreferrer" target="_blank">labelPrecisionðŸ“˜</a>: A number that specifies the number of decimal places to display in the labels.

  - By default, this is set to `1`, but if you work with degrees or just larger datasets, you may want to set it to `0` to avoid showing decimal places in the labels.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/polar-axes-labels/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/polar-axes-labels/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
