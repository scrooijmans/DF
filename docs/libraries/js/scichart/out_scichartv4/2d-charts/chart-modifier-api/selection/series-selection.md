On this page

# Series Selection and Hover

SciChart now features a native ChartModifier called the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a> which allows you to click-select series via the mouse, hover a series via the mouse, mouse-over and highlight a series and programmatically select & hover series.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a> allows you to do the following things:

1.  When <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#enableselection" rel="noopener noreferrer" target="_blank">SeriesSelectionModifier.enableSelectionðŸ“˜</a> is true, be notified via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#selectionchanged" rel="noopener noreferrer" target="_blank">selectionChangedðŸ“˜</a> event when the user selects one or more series on the chart.
2.  When <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#enablehover" rel="noopener noreferrer" target="_blank">SeriesSelectionModifier.enableHoverðŸ“˜</a> is true, be notified via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#hoverchanged" rel="noopener noreferrer" target="_blank">hoverChangedðŸ“˜</a> event when a user mouse-over hovers one or more series on the chart.
3.  Get a list of currently selected series via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#selectedseries" rel="noopener noreferrer" target="_blank">SeriesSelectionModifier.selectedSeriesðŸ“˜</a> array, or hovered series via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#hoveredseries" rel="noopener noreferrer" target="_blank">SeriesSelectionModifier.hoveredSeriesðŸ“˜</a> array.

Find an example of series selection below:

- JS
- Builder API (JSON Config)

``` prism-code
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    NumberRange,
    SeriesSelectionModifier,
    TextAnnotation,
    EHorizontalAnchorPoint,
    ECoordinateMode,
    EllipsePointMarker
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));

// Create a chart with line series with a point-marker
// Subscribe to onSelected to change the visual of the series when isSelected = true
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        stroke: "SteelBlue",
        strokeThickness: 3,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 10,
            height: 10,
            strokeThickness: 2,
            stroke: "SteelBlue",
            fill: "LightSteelBlue"
        }),
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
        }),
        // using onSelectedChanged callback, change the style of the series on selection
        onSelectedChanged: sourceSeries => {
            sourceSeries.stroke = sourceSeries.isSelected ? "white" : "SteelBlue";
            sourceSeries.pointMarker.fill = sourceSeries.isSelected ? "white" : "SteelBlue";
            sourceSeries.pointMarker.stroke = sourceSeries.isSelected ? "white" : "LightSteelBlue";
        }
    })
);

// You can also set or access onSelectedChanged via the renderableSeries
sciChartSurface.renderableSeries.get(0).selected.subscribe(args => {
    console.log(`Series ${args.sourceSeries.dataSeries.dataSeriesName} was ${args.isSelected ? "" : "de"}selected`);
});

// Add the SeriesSelectionModifier to the chart
sciChartSurface.chartModifiers.add(
    new SeriesSelectionModifier({
        enableHover: false,
        enableSelection: true
    })
);
```

``` prism-code
// Demonstrates how to configure the CursorModifier in SciChart.js using the Builder API
const { chartBuilder, ESeriesType, EThemeProviderType, EPointMarkerType, EChart2DModifierType } = SciChart;
// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { theme: { type: EThemeProviderType.Dark } },
    modifiers: [
        {
            type: EChart2DModifierType.SeriesSelection,
            options: {
                enableHover: false,
                enableSelection: true
            }
        }
    ],
    series: [
        {
            type: ESeriesType.LineSeries,
            options: {
                stroke: "SteelBlue",
                strokeThickness: 3,
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    options: {
                        width: 10,
                        height: 10,
                        strokeThickness: 2,
                        stroke: "SteelBlue",
                        fill: "LightSteelBlue"
                    }
                },
                // using onSelectedChanged callback, change the style of the series on selection
                onSelectedChanged: sourceSeries => {
                    sourceSeries.stroke = sourceSeries.isSelected ? "white" : "SteelBlue";
                    sourceSeries.pointMarker.fill = sourceSeries.isSelected ? "white" : "SteelBlue";
                    sourceSeries.pointMarker.stroke = sourceSeries.isSelected ? "white" : "LightSteelBlue";
                }
            },
            xyData: {
                dataSeriesName: "Line Series",
                xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
            }
        }
    ]
});

// You can also set or access onSelectedChanged via the renderableSeries
sciChartSurface.renderableSeries.get(0).selected.subscribe(args => {
    console.log(`Series ${args.sourceSeries.dataSeries.dataSeriesName} was ${args.isSelected ? "" : "de"}selected`);
});
```

This results in the following output:

![](out_scichartv4/2d-charts/chart-modifier-api/selection/series-selection/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Many of the properties here are optional - they have been included to show the configuration possibilities for the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a>. See <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesselectionmodifieroptions.html" rel="noopener noreferrer" target="_blank">ISeriesSelectionModifierOptionsðŸ“˜</a> for more.

## Getting Notified on Series Hovered / Selected<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#getting-notified-on-series-hovered--selected" class="hash-link" aria-label="Direct link to Getting Notified on Series Hovered / Selected" translate="no" title="Direct link to Getting Notified on Series Hovered / Selected">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a> has two events <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#selectionchanged" rel="noopener noreferrer" target="_blank">SeriesSelectionModifier.selectionChangedðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#hoverchanged" rel="noopener noreferrer" target="_blank">SeriesSelectionModifier.hoverChangedðŸ“˜</a>. These provide callbacks to user code when a series is selected, deselected, hovered (mouse-over) or unhovered. They can be subscribed to as follows.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesselectionmodifieroptions.html" rel="noopener noreferrer" target="_blank">constructor options for the SeriesSelectionModifierðŸ“˜</a> also have <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesselectionmodifieroptions.html#onselectionchanged" rel="noopener noreferrer" target="_blank">onSelectionChangedðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iseriesselectionmodifieroptions.html#onhoverchanged" rel="noopener noreferrer" target="_blank">onHoverChangedðŸ“˜</a> functions. You can pass in selected and hovered functions in the constructor also:

- Subscribe to SeriesSelectionModifier events

``` prism-code
import { SeriesSelectionModifier } from "scichart";

const seriesSelectionModifier = new SeriesSelectionModifier({
    enableHover: true,
    enableSelection: true,
    // selection, hover callbacks can be passed to the constructor
    onSelectionChanged: args => {
        console.log("seriesSelectionModifier constructor onSelectionChanged");
    },
    onHoverChanged: args => {
        console.log("seriesSelectionModifier constructor onHoverChanged");
    }
});

// Or, subscribed to later.
seriesSelectionModifier.hoverChanged.subscribe(args => {
    console.log("seriesSelectionModifier.hoverChanged event");
});
seriesSelectionModifier.selectionChanged.subscribe(args => {
    console.log("seriesSelectionModifier.selectionChanged event");
});
```

Finally, series themselves have <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#selected" rel="noopener noreferrer" target="_blank">selectedðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#hovered" rel="noopener noreferrer" target="_blank">hoveredðŸ“˜</a> events and functions in the constructor and on the series themselves.

- Series selected hovered events

``` prism-code
const series = new FastLineRenderableSeries(wasmContext, {
/* ... other options here ... */

onSelectedChanged: (sourceSeries, isSelected) => {
    console.log("FastLineRenderableSeries constructor onSelectedChanged");
},
onHoveredChanged: (sourceSeries, isHovered) => {
    console.log("FastLineRenderableSeries constructor onSelectedChanged");
}
});

series.hovered.subscribe(args => {
    console.log("FastLineRenderableSeries.hovered event");
});
series.selected.subscribe(args => {
    console.log("FastLineRenderableSeries.selected event");
});
```

Here's a full code sample showing the four possible ways you can get notified when series selection or hover state changes in SciChart.js:

- ts

``` prism-code
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    NumberRange,
    SeriesSelectionModifier,
    TextAnnotation,
    EHorizontalAnchorPoint,
    ECoordinateMode,
    EllipsePointMarker
} = SciChart;
// or for npm import { SciChartSurface, ... } from "scichart"

// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));

// Method 1: Pass onSelectionChanged and onHoverChanged to SeriesSelectionModifier constructor options
const seriesSelectionModifier = new SeriesSelectionModifier({
    enableHover: true,
    enableSelection: true,
    onSelectionChanged: args => {
        console.log("1 seriesSelectionModifier constructor onSelectionChanged");
    },
    onHoverChanged: args => {
        console.log("1 seriesSelectionModifier constructor onHoverChanged");
    }
});

// Method 2: Use the hoverChanged and selectionChanged events
seriesSelectionModifier.hoverChanged.subscribe(args => {
    console.log("2 seriesSelectionModifier.hoverChanged event");
});

seriesSelectionModifier.selectionChanged.subscribe(args => {
    console.log("2 seriesSelectionModifier.selectionChanged event");
});

// Method 3: Use the onSelectedChanged functions on the series itself
const series = new FastLineRenderableSeries(wasmContext, {
    stroke: "SteelBlue",
    strokeThickness: 3,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        strokeThickness: 2,
        stroke: "SteelBlue",
        fill: "LightSteelBlue"
    }),
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
    }),
    onSelectedChanged: (sourceSeries, isSelected) => {
        console.log("3 FastLineRenderableSeries constructor onSelectedChanged");
    },
    onHoveredChanged: (sourceSeries, isHovered) => {
        console.log("3 FastLineRenderableSeries constructor onSelectedChanged");
    }
});

// Method 4: use the selected and hovered events on the series
series.hovered.subscribe(args => {
    console.log("4 FastLineRenderableSeries.hovered event");

    series.stroke = series.isSelected ? "White" : series.isHovered ? "Orange" : "SteelBlue";
});

series.selected.subscribe(args => {
    console.log("4 FastLineRenderableSeries.selected event");

    series.stroke = series.isSelected ? "White" : series.isHovered ? "Orange" : "SteelBlue";
});

// Add the modifier and series to chart
sciChartSurface.chartModifiers.add(seriesSelectionModifier);
sciChartSurface.renderableSeries.add(series);
```

this results in the following output:

## Customizing Selection Visuals<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#customizing-selection-visuals" class="hash-link" aria-label="Direct link to Customizing Selection Visuals" translate="no" title="Direct link to Customizing Selection Visuals">â€‹</a>

### Colour or Highlight a series when selected or hovered<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#colour-or-highlight-a-series-when-selected-or-hovered" class="hash-link" aria-label="Direct link to Colour or Highlight a series when selected or hovered" translate="no" title="Direct link to Colour or Highlight a series when selected or hovered">â€‹</a>

When a series is selected or hovered, you can use one of the callback methods (above) to change it's style to change series color, fill or highlight a series. Any property may be changed, such as `BaseRenderableSeries.stroke`, `strokeThickness`, `pointMarker` type or colours. Below is a simple code sample showing how to get a tri-state style on series selected, hovered, or selected and hovered.

- TS

``` prism-code
// Create a chart surface
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));

const defaultStroke = "SteelBlue";
const defaultFill = "LightSteelBlue";

const applyStyle = (series, isSelected, isHovered) => {
    series.stroke =
        isSelected && isHovered ? "#FFBB99" : isSelected ? "#FFF" : isHovered ? "#FF7733" : defaultStroke;
    series.pointMarker.stroke = series.stroke;
    series.pointMarker.fill =
        isSelected && isHovered ? "#FFBB99" : isSelected ? "#FFF" : isHovered ? "#FF7733" : defaultFill;
};

// Create a chart with line series with a point-marker
sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        stroke: defaultStroke,
        strokeThickness: 3,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 10,
            height: 10,
            strokeThickness: 2,
            stroke: defaultStroke,
            fill: defaultFill
        }),
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
        }),

        // Apply a style to the series on selected and hovered
        onSelectedChanged: sourceSeries => {
            applyStyle(sourceSeries, sourceSeries.isSelected, sourceSeries.isHovered);
        },

        onHoveredChanged: sourceSeries => {
            applyStyle(sourceSeries, sourceSeries.isSelected, sourceSeries.isHovered);
        }
    })
);

// Add the DatapointSelectionModifier to the chart
sciChartSurface.chartModifiers.add(
    new SeriesSelectionModifier({
        enableSelection: true,
        enableHover: true
    })
);
```

This results in the following behaviour when hovering or selecting the series.

### Animating Selection State Changes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#animating-selection-state-changes" class="hash-link" aria-label="Direct link to Animating Selection State Changes" translate="no" title="Direct link to Animating Selection State Changes">â€‹</a>

Using the Animations API built into SciChart, it is also possible to animate between style state changes on a RenderableSeries such as when a series is hovered or selected.

Update the code above to use the Animations API to call <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#enqueueanimation" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.enqueueAnimationðŸ“˜</a> as follows:

- tri state animation on selection

``` prism-code
// import { LineAnimation, easing, EPointMarkerType } from "scichart";

// Update function applyStyle to use Animations API
const applyStyle = (series: SciChart.IRenderableSeries, isSelected: boolean, isHovered: boolean) => {
    const stroke =
        isSelected && isHovered ? "#FFBB99" : isSelected ? "#FFF" : isHovered ? "#FF7733" : defaultStroke;
    const fill = isSelected && isHovered ? "#FFBB99" : isSelected ? "#FFF" : isHovered ? "#FF7733" : defaultFill;
    const strokeThickness = isHovered ? 4 : 2;

    series.enqueueAnimation(
        new LineAnimation({
            styles: {
                stroke,
                strokeThickness,
                pointMarker: {
                    type: EPointMarkerType.Ellipse,
                    stroke,
                    fill,
                    width: 10,
                    height: 10,
                    strokeThickness: 2,
                }
            },
            duration: 250,
            ease: easing.outQuad
        })
    );
};
```

This now results in the following animated style-transition behaviour when hovering or selecting the series.

![](out_scichartv4/2d-charts/chart-modifier-api/selection/series-selection/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Multiple properties can be animated in SciChart.js, including stroke, fill, strokethickness, pointmarker size, type, opacity and more. For more information about how to animate between styles or datasets, see the [Animations API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview).

## Programmatically Getting/Setting Selected Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#programmatically-gettingsetting-selected-series" class="hash-link" aria-label="Direct link to Programmatically Getting/Setting Selected Series" translate="no" title="Direct link to Programmatically Getting/Setting Selected Series">â€‹</a>

Series may also be selected and deselected programmatically. Simply set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#isselected" rel="noopener noreferrer" target="_blank">BaseRenderableSeries.isSelectedðŸ“˜</a> property to trigger this action. SciChart will automatically redraw, and selection callbacks will be called, where you can update the style.

## Excluding/Including Series from Selection<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#excludingincluding-series-from-selection" class="hash-link" aria-label="Direct link to Excluding/Including Series from Selection" translate="no" title="Direct link to Excluding/Including Series from Selection">â€‹</a>

By default, all series in the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChartSurface.renderableSeriesðŸ“˜</a> collection will be included in the selection and hovered functionality when <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a> is used.

At the moment, you can filter which series are included by overriding the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html#getallseries" rel="noopener noreferrer" target="_blank">getAllSeriesðŸ“˜</a> function.

Find a short example below:

- Exclude series from SeriesSelectionModifier

``` prism-code
const seriesSelectionModifier = new SeriesSelectionModifier({ /* options .. */});
seriesSelectionModifier.getAllSeries = () => {
    // access and return the series you want to include in the operation.
    // This function should return an array
    return [seriesSelectionModifier.parentSurface.renderableSeries.get(1)];
}
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [DataPoint Selection](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection)
- [Annotation Hover](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/selection/series-selection/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/selection/series-selection/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
