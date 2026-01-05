On this page

# Gridline and Label Spacing (Interval)

## Axis Ticks, Labels and Grid Lines<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/#axis-ticks-labels-and-grid-lines" class="hash-link" aria-label="Direct link to Axis Ticks, Labels and Grid Lines" translate="no" title="Direct link to Axis Ticks, Labels and Grid Lines">â€‹</a>

In SciChart.js, the **Ticks** are small marks around the chart on an axis.Â There are **Minor** and **Major** **Ticks**,Â where Minor Ticks are placed in betweenÂ MajorÂ ones. **Axis Labels** appears for every Major Tick, and **Grid Lines** correspond to **Ticks** on an axis.Â 

In SciChart,Â **axes are responsible** not only for drawing Ticks and Labels, but also **forÂ the chart grid**. So if you want to change the spacing of gridlines, labels or tick marks, you need to use the APIs to change Axis Tick spacing.

<img src="out_scichartv4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/index_media/8bb677ea9bb46d3f647ddd3ec10160a0e5a94a47.png" class="img_ev3q" decoding="async" loading="lazy" width="577" height="125" />

## AutomaticÂ Gridline, Label or TickÂ Spacing<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/#automaticgridline-label-or-tickspacing" class="hash-link" aria-label="Direct link to AutomaticÂ Gridline, Label or TickÂ Spacing" translate="no" title="Direct link to AutomaticÂ Gridline, Label or TickÂ Spacing">â€‹</a>

In SciChart.js, theÂ difference between two Major Gridlines is defined byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta" rel="noopener noreferrer" target="_blank">axisCore.MajorDeltaðŸ“˜</a>. The difference between two minor gridlines is defined by <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta" rel="noopener noreferrer" target="_blank">axisCore.MinorDeltaðŸ“˜</a>.

By default, **MajorDelta** and **MinorDelta** values are calculated automaticallyÂ whenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autoticks" rel="noopener noreferrer" target="_blank">axis.autoTicksðŸ“˜</a>Â = true. Major/Minor deltas are calculated dynamically according toÂ theÂ [VisibleRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/)Â and size ofÂ an axis in pixels. This means as you zoom and pan, these properties are updated and you get a dynamic gridline spacing effect.

### Configuring Automatic Tick Spacing<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/#configuring-automatic-tick-spacing" class="hash-link" aria-label="Direct link to Configuring Automatic Tick Spacing" translate="no" title="Direct link to Configuring Automatic Tick Spacing">â€‹</a>

If you want to congigure Automatic Tick Spacing, do the following:

- set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autoticks" rel="noopener noreferrer" target="_blank">axis.autoTicksðŸ“˜</a> = true
- set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.maxAutoTicksðŸ“˜</a> = a numeric value (e.g. 10)
- setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minorspermajor" rel="noopener noreferrer" target="_blank">axis.minorsPerMajorðŸ“˜</a> property = a numeric value (e.g. 5)

With a configuration like the above, SciChart.js will calculate tick spacing on the axis with up to 10 major gridlines / labels and 5 minor gridlines per major.

Note:Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.maxAutoTicksðŸ“˜</a>Â is a hint. The axis will not output this exact number of ticks, but rather dynamically vary gridline spacing depending on zoom up to this amount.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure axis autoticks in SciChart.js
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Adjust major/minor gridline style to make it clearer for the demo
const styleOptions = {
    majorGridLineStyle: { color: "#50C7E077" },
    minorGridLineStyle: { color: "#50C7E033" }
};

const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "maxAutoTicks 10, minorsPerMajor 2",
    // Default true, automatically calculate axis.MajorDelta, axis.MinorDelta
    autoTicks: true,
    // This is a hint which defines the max number of major gridlines/labels visible at any one time.
    // The actual number of gridlines may be lower than this depending on zoom level
    maxAutoTicks: 10,
    // For every major gridline, this defines how many minor gridlines there are. Default is 5.
    minorsPerMajor: 2,
    ...styleOptions
});

// Properties may also be set after instantiation, e.g.
xAxis.autoTicks = true;
xAxis.maxAutoTicks = 10;
xAxis.minorsPerMajor = 2;

const yAxis = new NumericAxis(wasmContext, {
    axisTitle: "maxAutoTicks 5, minorsPerMajor 4",
    autoTicks: true,
    maxAutoTicks: 5,
    minorsPerMajor: 4,
    ...styleOptions
});

sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);
```

``` prism-code
// Demonstrates how to configure a axis autoticks in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "maxAutoTicks 10, minorsPerMajor 2",
            autoTicks: true,
            maxAutoTicks: 10,
            minorsPerMajor: 2
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "maxAutoTicks 5, minorsPerMajor 4",
            autoTicks: true,
            maxAutoTicks: 5,
            minorsPerMajor: 4
        }
    }
});
```

This results in the following output:

## Specifying Major & Minor Deltas<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/#specifying-major--minor-deltas" class="hash-link" aria-label="Direct link to Specifying Major &amp; Minor Deltas" translate="no" title="Direct link to Specifying Major &amp; Minor Deltas">â€‹</a>

There are two methods to control gridline & label spacing in SciChart.js. The first is the simplest, by specifying axis major & minor delta.

- set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autoticks" rel="noopener noreferrer" target="_blank">axis.autoTicksðŸ“˜</a> = **false**
- set <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#maxautoticks" rel="noopener noreferrer" target="_blank">axis.majorDeltaðŸ“˜</a> = a numeric value (e.g. 2)
- setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minorspermajor" rel="noopener noreferrer" target="_blank">axis.minorDeltaðŸ“˜</a> = a numeric value (e.g. 0.4)

With a configuration like the above, SciChart.js will calculate tick spacing on the axis exactly an interval of 2 between major gridlines / labels and 0.4 between minor gridlines.

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure axis autoticks in SciChart.js
const { SciChartSurface, NumericAxis, SciChartJsNavyTheme } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

// Adjust major/minor gridline style to make it clearer for the demo
const styleOptions = {
    majorGridLineStyle: { color: "#50C7E077" },
    minorGridLineStyle: { color: "#50C7E033" }
};

const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "majorDelta 2, minorDelta 1",
    // When autoTicks is false, you must specify majorDelta and minorDelta
    autoTicks: false,
    // Have a major gridline every 2 units on the axis
    majorDelta: 2,
    // Have a minor gridline every 1 unit on the axis
    minorDelta: 1,
    ...styleOptions
});

// Properties may also be set after instantiation, e.g.
xAxis.autoTicks = false;
xAxis.majorDelta = 2;
xAxis.minorDelta = 1;

const yAxis = new NumericAxis(wasmContext, {
    axisTitle: "majorDelta 2, minorDelta 1",
    autoTicks: false,
    majorDelta: 1,
    minorDelta: 0.2,
    ...styleOptions
});

sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);
```

``` prism-code
// Demonstrates how to configure axis autoticks in SciChart.js using the Builder API
const { chartBuilder, EThemeProviderType, EAxisType } = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "majorDelta 2, minorDelta 1",
            autoTicks: false,
            majorDelta: 2,
            minorDelta: 1
        }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: {
            axisTitle: "majorDelta 2, minorDelta 1",
            autoTicks: false,
            majorDelta: 1,
            minorDelta: 0.2
        }
    }
});
```

This results in the following output:

**Note**: that Major/Minor Delta behaves differently for the Logarithmic Axis.

## Dynamic Control of Gridline, Label & Tick Spacing on Zoom<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/#dynamic-control-of-gridline-label--tick-spacing-on-zoom" class="hash-link" aria-label="Direct link to Dynamic Control of Gridline, Label &amp; Tick Spacing on Zoom" translate="no" title="Direct link to Dynamic Control of Gridline, Label &amp; Tick Spacing on Zoom">â€‹</a>

This automatic major and minor delta calculation is done by a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html" rel="noopener noreferrer" target="_blank">DeltaCalculatorðŸ“˜</a> which is specific to the axis type, so there is NumericDeltaCalculator, LogarithmicDeltaCalculator, CategoryDeltaCalculator and DateTimeDeltaCalculator.

You can override the DeltaCalculator to customise this behaviour. For instance, if you have an axis with labelPrecision: 0, you may see duplicate labels if you zoom deeply into it, because the DeltaCalcuator is returning a fractional majorDelta, but the labelProvider is rounding the resulting tick values. Here is a custom NumericDeltaCalculator which rounds up the deltas so you get integer values as a minimum.

``` prism-code
class IntegerDeltaCalculator extends NumericDeltaCalculator {
    public getDeltaFromRange(min: number, max: number, minorsPerMajor: number, maxTicks: number): NumberRange {
        const delta = super.getDeltaFromRange(min, max, minorsPerMajor, maxTicks);
        return new NumberRange(Math.ceil(delta.min), Math.ceil(delta.max));
    }
}

// Use like this
xAxis.deltaCalculator = new IntegerDeltaCalculator(wasmContext);
```

Deltas are used by the TickProvider to produce the actual tick values, so you can also customise this behaviour there. See theÂ [The Tick Provider API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/tick-provider-api).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
