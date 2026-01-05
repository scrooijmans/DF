On this page

# Inner Axis Layout

SciChart allows you to customize how axes are placed around and within the surface.Â  Axes can be placed:

- **Outside the drawing area,**Â called Outer Axes.Â This is the default.Â The drawing area is reduced to give space for the axes and their labels and titles.
- **Inside the drawing area**,Â called Inner Axes.Â The drawing area fills the entire space of the chart.Â 

Inner axes can either be around the edges of the area, or bound to a coordinate so they appear in the middle of the drawing area.Â These are referred to as **Central Axes**.

## Inner Axes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/#inner-axes" class="hash-link" aria-label="Direct link to Inner Axes" translate="no" title="Direct link to Inner Axes">â€‹</a>

To create an Inner axis simply setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis.html#isinneraxis" rel="noopener noreferrer" target="_blank">isInnerAxis: trueðŸ“˜</a> on the axis options:

- TS
- Builder API (JSON Config)

``` prism-code
// Configure an axis to display inside the chart
const xAxis = new NumericAxis(wasmContext, {
    isInnerAxis: true,
    axisTitle: "Inner axis",
    // To allow easier visualisation of axis position
    backgroundColor: "#50C7E022"
});

// Add the xAxis to the chart
sciChartSurface.xAxes.add(xAxis);
```

``` prism-code
// Demonstrates how to configure an inner axis in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            isInnerAxis: true,
            axisTitle: "Inner axis",
            // To allow easier visualisation of axis position
            backgroundColor: "#50C7E022"
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "Outer axis",
            // To allow easier visualisation of axis position
            backgroundColor: "#F4842022"
        }
    }
});
```

Now the x axis is an inner axis, while the y axis is the default outer axis.Â 

## DrawSeriesBehindAxis property<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/#drawseriesbehindaxis-property" class="hash-link" aria-label="Direct link to DrawSeriesBehindAxis property" translate="no" title="Direct link to DrawSeriesBehindAxis property">â€‹</a>

SciChart.js also allows you to draw all chart series behind axis by setting a single flag on the parent SciChartSurface.

The default behaviour is to draw axis on the outside of the chart area. If you need more space on the chart (if axis are taking up too much space), you can set a single flag to draw the series behind the axis and pull the axis areas inside the chart area:

- TS

``` prism-code
sciChartSurface.drawSeriesBehindAxis = true;
```

This results in the following output.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Secondary and Multiple Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview)
- [Vertically Stacked Axis Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout)
- [Axis LabelProviders](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
