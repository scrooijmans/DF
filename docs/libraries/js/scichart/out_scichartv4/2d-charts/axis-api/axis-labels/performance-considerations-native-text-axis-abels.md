On this page

# Native Text Axis Labels

In version SciChart 4.0 rendering axis labels defaults to usingÂ a [native text api](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/).Â  This uses our in-house WebGL text rendering engine andÂ offers performance benefits in situations where you have many axes with many labels. Rotated and multiline support is better with native text than withÂ standard text, but there are also some important limitations you need to be aware of.

## Disabling Native TextÂ Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/#disabling-native-textlabels" class="hash-link" aria-label="Direct link to Disabling Native TextÂ Labels" translate="no" title="Direct link to Disabling Native TextÂ Labels">â€‹</a>

If you are using any custom fontsÂ inÂ your axes, then you can disable native text as the default for all axes by doing the following once at the start of your app:Â 

``` prism-code
// Enable native text

import { SciChartDefaults } from "scichart";

SciChartDefaults.useNativeText = false;
```

You canÂ control it for a particular axis by setting the useNativeText option when creating the axis, or by setting theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#usenativetext" rel="noopener noreferrer" target="_blank">axis.labelProvider.useNativeTextðŸ“˜</a> property.

![](out_scichartv4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

To use any font other than Arial you will need ensure that font is available on your server (as fontname.ttf), or registered usingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#registerfont" rel="noopener noreferrer" target="_blank">sciChartSurface.registerFont()ðŸ“˜</a> if coming from a remote url.Â  SeeÂ [Native Text Font Loading](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/)Â for more details.

All the normal options inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#labelstyle" rel="noopener noreferrer" target="_blank">labelStyleðŸ“˜</a> are supported except for **fontStyle** and **fontWeight**.Â 

The example below creates axes using both native and standard text.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates native text vs. standard text in SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    SciChartJsNavyTheme,
    EAxisAlignment,
    ELabelAlignment,
    SciChartDefaults,
    Thickness
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Use Native text for all axes by default
SciChartDefaults.useNativeText = true;

const labelStyle = {
    fontFamily: "Default",
    fontSize: 14,
    color: "white",
    padding: new Thickness(0, 0, 0, 0),
    alignment: ELabelAlignment.Auto
};

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Enable native text for a specific axis
sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
        useNativeText: true,
        // Most style options are supported
        // fontStyle and FontWeight are not supported for native text
        labelStyle,
        axisTitle: "Native X",
        backgroundColor: "#50C7E011"
    }),
    new NumericAxis(wasmContext, {
        // Disable native text for a specfic axis
        useNativeText: false,
        axisAlignment: EAxisAlignment.Top,
        // Same style for comparison
        labelStyle,
        axisTitle: "Normal X",
        backgroundColor: "#50C7E011"
    })
);
sciChartSurface.yAxes.add(
    // Native text with default values
    new NumericAxis(wasmContext, { axisTitle: "Native Y", labelStyle, backgroundColor: "#50C7E011" }),
    // Normal text with default values
    new NumericAxis(wasmContext, {
        labelStyle,
        useNativeText: false,
        axisAlignment: EAxisAlignment.Left,
        axisTitle: "Normal Y",
        backgroundColor: "#50C7E011"
    })
);
```

``` prism-code
// Demonstrates native text vs. standard text in SciChart.js using the Builder API
const { chartBuilder, SciChartDefaults, EAxisAlignment, ELabelAlignment, EAxisType, Thickness } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

// Use Native text for all axes by default
SciChartDefaults.useNativeText = true;

const labelStyle = {
    fontFamily: "Default",
    fontSize: 14,
    color: "white",
    padding: new Thickness(0, 0, 0, 0),
    alignment: ELabelAlignment.Auto
};

const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    xAxes: [
        {
            type: EAxisType.NumericAxis,
            options: {
                useNativeText: true,
                // Most style options are supported
                // fontStyle and FontWeight are not supported for native text
                labelStyle,
                axisTitle: "Native X",
                backgroundColor: "#50C7E011"
            }
        },
        {
            type: EAxisType.NumericAxis,
            options: {
                // Disable native text for a specfic axis
                useNativeText: false,
                axisAlignment: EAxisAlignment.Top,
                // Same style for comparison
                labelStyle,
                axisTitle: "Normal X",
                backgroundColor: "#50C7E011"
            }
        }
    ],
    yAxes: [
        {
            // Native text with default values
            type: EAxisType.NumericAxis,
            options: { axisTitle: "Native Y", labelStyle, backgroundColor: "#50C7E011" }
        },
        {
            type: EAxisType.NumericAxis,
            // Normal text with default values
            options: {
                labelStyle,
                useNativeText: false,
                axisAlignment: EAxisAlignment.Left,
                axisTitle: "Normal Y",
                backgroundColor: "#50C7E011"
            }
        }
    ]
});
```

This results in the following output:

## Rotated and Multiline Native Text Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/#rotated-and-multiline-native-text-labels" class="hash-link" aria-label="Direct link to Rotated and Multiline Native Text Labels" translate="no" title="Direct link to Rotated and Multiline Native Text Labels">â€‹</a>

The standard axis labels supported rotation, but the positioning is poor for angles outside the 0 to 90 range.Â With native text labels, this is fixed.Â Note that rotation is a property on the labelProvider, not the axis itself.

![](out_scichartv4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

When using angles that are not a multiple of 90, you probably want to setÂ **hideOverlappingLabels: false** as the overlap is calculated using the bounding rectangle of the text.Â 

Multiline labels are supported simply by using newline characters (\n)Â in the label text.Â  lineSpacing is a property on the labelProvider.Â  The alignment property on labelStyle also affects the alignment for multiple lines.Â 

![](out_scichartv4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Note: for more info aboutÂ [Text and MultiLine labels see this article](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/text-and-multi-line-labels). For rotation of labelsÂ [see this article](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/rotating-axis-labels).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

[Axis Label Formatting - TextLabelProvider](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/)

[Axis Label Formatting - Custom LabelProviders](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/)

#### Miscellaneous APIs<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/#miscellaneous-apis" class="hash-link" aria-label="Direct link to Miscellaneous APIs" translate="no" title="Direct link to Miscellaneous APIs">â€‹</a>

[Native Text Api](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
