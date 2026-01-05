On this page

# Axis LabelProvider API Overview

All Axis Types include theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#labelprovider" rel="noopener noreferrer" target="_blank">AxisCore.labelProviderðŸ“˜</a> property, which formats axis and cursor labels. Built in to SciChart.js are a number of labelProvider types, and you can even create your own.

The inheritance diagram for LabelProviders in SciChart.js looks like this:

All of these classesÂ inherit LabelProviderBase2D. Below there is a mapping between which axis type has which labelprovider:

| Axis Type | LabelProvider Type |
|----|----|
| **[NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis)** and **[PolarNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis)** | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericlabelprovider.html" rel="noopener noreferrer" target="_blank">NumericLabelProviderðŸ“˜</a> |
| **[DateTimeNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis)** | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/smartdatelabelprovider.html" rel="noopener noreferrer" target="_blank">SmartDateLabelProviderðŸ“˜</a> |
| **[CategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis)** and **[PolarCategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis)** | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datelabelprovider.html" rel="noopener noreferrer" target="_blank">DateLabelProviderðŸ“˜</a> |
| **[LogarithmicAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis)** | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmiclabelprovider.html" rel="noopener noreferrer" target="_blank">LogarithmicLabelProviderðŸ“˜</a> |

We've included examples of how to format labels via the built-in label providers in the above articles. Click on **NumericAxis** or **DateTimeNumericAxis** above for more info and a code sample.

## Applying a LabelProvider to an axis<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/#applying-a-labelprovider-to-an-axis" class="hash-link" aria-label="Direct link to Applying a LabelProvider to an axis" translate="no" title="Direct link to Applying a LabelProvider to an axis">â€‹</a>

All axis (above) have a built-in LabelProvider. You can also apply a labelprovider to an axis with the following code:

**SmartDate Formatting Example**

- TS

``` prism-code
import { CategoryAxis, SmartDateLabelProvider  } from "scichart";
sciChartSurface.xAxes.add(new CategoryAxis(wasmContext, {
         labelProvider: new SmartDateLabelProvider()
}));
```

If you have an axis where the date range can change as the user zooms, the **SmartDateLabelProvider** provides dynamic date labels which automatically adjust based on the axis range. You can also create custom dynamic labelproviders. See the rest of this article for steps how to do this.

## LabelProvider & LabelProviderBase2D<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/#labelprovider--labelproviderbase2d" class="hash-link" aria-label="Direct link to LabelProvider &amp; LabelProviderBase2D" translate="no" title="Direct link to LabelProvider &amp; LabelProviderBase2D">â€‹</a>

TheseÂ are the base classes for all labelproviders in SciChart.js. Some of the properties they include can be found below.

The properties common to theÂ **LabelProviderBase2D** / **LabelProvider** classes can be found in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html" rel="noopener noreferrer" target="_blank">TypeDoc API documentationðŸ“˜</a>.

There are many options to format axis labels in SciChart.js via the labelProvider.

**In particular:**

- SciChart.js callsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#formatlabel" rel="noopener noreferrer" target="_blank">axis.labelProvider.formatLabel(dataValue)ðŸ“˜</a> to format each label on the axis andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#formatcursorlabel" rel="noopener noreferrer" target="_blank">axis.labelProvider.formatCursorLabel(dataValue)ðŸ“˜</a> to format each cursor label.

- The built-in implementation of formatLabel provides text formatting based on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#numericformat" rel="noopener noreferrer" target="_blank">labelProvider.numericFormatðŸ“˜</a> &Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#cursornumericformat" rel="noopener noreferrer" target="_blank">labelProvider.cursorNumericFormatðŸ“˜</a> properties.

- The number of decimal places (if applicable numericFormat is chosen) is specified byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#precision" rel="noopener noreferrer" target="_blank">labelProvider.precisionðŸ“˜</a> property.

- A string prefix and postfix are provided by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#prefix" rel="noopener noreferrer" target="_blank">labelProvider.prefixðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#postfix" rel="noopener noreferrer" target="_blank">labelProvider.postFixðŸ“˜</a> properties.

- A rotation angle may be specified byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#rotation" rel="noopener noreferrer" target="_blank">labelProvider.rotationðŸ“˜</a>.

- Caching and native text (WebGL enabled text) may be turned on or off via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#usecache" rel="noopener noreferrer" target="_blank">labelProvider.useCacheðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#usenativetext" rel="noopener noreferrer" target="_blank">useNativeTextðŸ“˜</a> property.

## Setting LabelProvider properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/#setting-labelprovider-properties" class="hash-link" aria-label="Direct link to Setting LabelProvider properties" translate="no" title="Direct link to Setting LabelProvider properties">â€‹</a>

LabelProvider properties can be set either on the labelProvider itself, or, in many cases can be set via the axis constructor options. The following code is equivalent:

- TS
- Builder API (JSON Config)

``` prism-code
// Set LabelProvider Properties in axis constructor options
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        // Enable decision labels with 4 significant figures
        labelFormat: ENumericFormat.Decimal,
        cursorLabelFormat: ENumericFormat.Decimal,
        labelPrecision: 4,
        labelPrefix: "$",
        labelPostfix: " USD"
    })
);

// Alternatively, set properties on the labelProvider itself
const yAxis = new NumericAxis(wasmContext);
yAxis.labelProvider.numericFormat = ENumericFormat.Decimal;
yAxis.labelProvider.cursorNumericFormat = ENumericFormat.Decimal;
yAxis.labelProvider.precision = 4;
yAxis.labelProvider.prefix = "$";
yAxis.labelProvider.postfix = " USD";
sciChartSurface.yAxes.add(yAxis);
```

``` prism-code
const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        // Setting the labelProvider properties via axis options
        options: {
            labelFormat: ENumericFormat.Decimal,
            cursorLabelFormat: ENumericFormat.Decimal,
            labelPrecision: 4,
            labelPrefix: "$",
            labelPostfix: " USD"
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis
    }
});

// Setting the labelprovider properties on the labelProvider itself
const labelProvider = sciChartSurface.yAxes.get(0).labelProvider;
labelProvider.numericFormat = ENumericFormat.Decimal;
labelProvider.cursorNumericFormat = ENumericFormat.Decimal;
labelProvider.precision = 4;
labelProvider.prefix = "$";
labelProvider.postfix = " USD";
```

This code sample configures label providers on the X & Y axis with exactly the same properties:

## OverridingÂ the formatLabel function<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/#overridingthe-formatlabel-function" class="hash-link" aria-label="Direct link to OverridingÂ the formatLabel function" translate="no" title="Direct link to OverridingÂ the formatLabel function">â€‹</a>

Say you wanted further customisation in the axis labels than whatÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#numericformat" rel="noopener noreferrer" target="_blank">axis.labelProvider.numericFormatðŸ“˜</a> offers, you can start by overriding the formatLabel and formatCursorLabel functions.

Take a look at the code sample below:

- TS
- Builder API (JSON Config)

``` prism-code
// Format a label as hexadecimal by overriding formatLabel
const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "X Axis with formatLabel",
    visibleRange: new NumberRange(0, 16),
    maxAutoTicks: 16
});
xAxis.labelProvider.formatLabel = dataValue => {
    return "0x" + dataValue.toString(16);
};
sciChartSurface.xAxes.add(xAxis);
```

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "X Axis with formatLabel",
            visibleRange: new NumberRange(0, 16),
            maxAutoTicks: 16
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis
    }
});

// Setting the labelprovider properties on the labelProvider itself
const labelProvider = sciChartSurface.xAxes.get(0).labelProvider;
labelProvider.formatLabel = dataValue => {
    return "0x" + dataValue.toString(16);
};
```

The function formatLabel is overridden and called for each label on the xAxis. In this function we return a string format as hexadecimal to show how to customise labels even further.

Axis labels are formatted byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelprovider.html#formatlabel" rel="noopener noreferrer" target="_blank">formatLabelðŸ“˜</a>. Tooltip values are formated byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textlabelprovider.html#formatcursorlabel" rel="noopener noreferrer" target="_blank">formatCursorLabelðŸ“˜</a>. This allows you to have different label formats for tooltips and axis.

## Custom LabelProviders<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/#custom-labelproviders" class="hash-link" aria-label="Direct link to Custom LabelProviders" translate="no" title="Direct link to Custom LabelProviders">â€‹</a>

See the following sections with worked examples on how to create custom label providers:

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Axis Label Formatting - Text / String Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/label-provider-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/label-provider-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
