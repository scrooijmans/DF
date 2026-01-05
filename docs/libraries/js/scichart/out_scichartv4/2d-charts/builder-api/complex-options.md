On this page

# Complex Options

Many things in SciChart.js are customised by providing a particular subclass, eg `PointMarkers`. The type signature in the options in these cases will be something like <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarker.html" rel="noopener noreferrer" target="_blank">IPointMarkerðŸ“˜</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tpointmarkerdefinition" rel="noopener noreferrer" target="_blank">TPointMarkerDefinitionðŸ“˜</a>.

Many of these classes require a `wasmContext` in the constructor, which you wonâ€™t have if you are trying to pass everything in a single definition, so instead use the Definition style, which as usual is `{ type, options }`.

For example:

- TS

``` prism-code
return chartBuilder.buildChart(divElementId, {
    series: {
        type: ESeriesType.ScatterSeries,
        xyData: { 
            xValues: [1, 3, 4, 7, 9], 
            yValues: [10, 6, 7, 2, 16] 
        },
        options: {
            pointMarker: { 
                type: EPointMarkerType.Ellipse, 
                options: { 
                    stroke: "red",
                    fill: "white",
                }
            }
        }
    }
});
```

This works for **Themes**, **PointMarkers**, **Effects**, **Animations**, **PaletteProviders** and **LabelProviders**.

Alternatively you can take the same approach as for option 3 of creating data and call <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.build2dchart" rel="noopener noreferrer" target="_blank">build2DChartðŸ“˜</a> with a partial definition, to get your wasmContext, then create an instance of the necessary class, then call buildSeries and pass it in. This is useful if you want to keep a reference to the object to be able to update it later.

- Building with complex options

``` prism-code
const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {});

const pointMarker = new EllipsePointMarker(wasmContext, {
    stroke: "red",
    fill: "white",
});

const seriesArray = await chartBuilder.buildSeries(wasmContext, {
    type: ESeriesType.ScatterSeries,
    xyData: { 
        xValues: [1, 3, 4, 7, 9], 
        yValues: [10, 6, 7, 2, 16] 
    },
    options: {
        pointMarker: pointMarker // you can use the pointMarker instance created above
    }
});

sciChartSurface.renderableSeries.add(...seriesArray);
```

## Function Options<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/complex-options/#function-options" class="hash-link" aria-label="Direct link to Function Options" translate="no" title="Direct link to Function Options">â€‹</a>

Some options properties are actually functions, such as the templating functions on [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier), or the callbacks on [SeriesSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection). These have a signature which is essentially `function | string` eg

``` prism-code
onSelectionChanged?: ((args: SelectionChangedArgs) => void) | string;
```

Here, the choice depends very specifically on whether or not you need to be able to serialise and deserialise the chart to a JSON string. If you donâ€™t need to, just specify the option as a function as normal. If you do need to, then you will need to register your function, and pass the registered name eg:

- Building withÂ function options

``` prism-code
const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    series: { 
        type: ESeriesType.LineSeries, 
        xyData: { 
            xValues: [1, 3, 4, 7, 9], 
            yValues: [10, 6, 7, 2, 16] 
        } 
    }
});

const logOnSelectionChanged = (args: SelectionChangedArgs) => { 
    console.log(args) 
};

chartBuilder.registerFunction(EBaseType.OptionFunction, "logOnSelectionChanged", logOnSelectionChanged);

const [chartModifier] = chartBuilder.buildModifiers({
    type: EChart2DModifierType.SeriesSelection,
    options: { onSelectionChanged: "logOnSelectionChanged" }
});

sciChartSurface.chartModifiers.add(chartModifier);
```

When the modifier is built, SciChart will look up the function in its registry and assign it. When you serialize the chart, you will get the function name in the definition. It is very important when doing this that the function definition and registration actually occurs before it is needed in a chart.

## onCreated FunctionÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/complex-options/#oncreated-function" class="hash-link" aria-label="Direct link to onCreated FunctionÂ " translate="no" title="Direct link to onCreated FunctionÂ ">â€‹</a>

Specific to the builder api, there is an **onCreated** option in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichart2ddefinition.html" rel="noopener noreferrer" target="_blank">ISciChart2DDefinitionðŸ“˜</a> which is a callback that is run after the chart is built and takes the sciChartSurface as a parameter.Â  It can be used to run zoomExtents, or perform further configuration using the standard api.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/complex-options/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Intro to the Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)
- [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)
- [Custom Subtypes](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/custom-subtypes)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/complex-options/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/complex-options/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
