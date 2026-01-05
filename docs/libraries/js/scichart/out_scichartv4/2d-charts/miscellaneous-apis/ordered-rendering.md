On this page

# Ordered Rendering

The render order of an element on a 2D chart defines how it will be displayed while overlapping other chart elements. It is analogous to the definition of "z-index".

## Render Order of chart elements<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#render-order-of-chart-elements" class="hash-link" aria-label="Direct link to Render Order of chart elements" translate="no" title="Direct link to Render Order of chart elements">â€‹</a>

Here we will describe some details on how series, annotations, labels, grid lines, sub-charts are handling render order. This topic may require familiarity with the following points:

- SciChart has different [target canvases](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#general-annotation-layer-types-overview) where the elements are rendered.
- [SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview) allows putting multiple surfaces on a chart
- [Native Text API](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation) vs **Texture Text Rendering**

### Default behavior<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#default-behavior" class="hash-link" aria-label="Direct link to Default behavior" translate="no" title="Direct link to Default behavior">â€‹</a>

The normal order of layering is

``` prism-code
background > grid lines and bands > series > annotations > text labels
```

Thus, text labels usually are above other types of entities.

![](out_scichartv4/2d-charts/miscellaneous-apis/ordered-rendering/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

By default, elements of the same type are rendered in the insertion order.

![](out_scichartv4/2d-charts/miscellaneous-apis/ordered-rendering/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The default layers are defined in <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" rel="noopener noreferrer" target="_blank">EDefaultRenderLayerðŸ“˜</a>.

![](out_scichartv4/2d-charts/miscellaneous-apis/ordered-rendering/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The order may differ depending on some configuration specifics.

#### Series Render Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#series-render-order" class="hash-link" aria-label="Direct link to Series Render Order" translate="no" title="Direct link to Series Render Order">â€‹</a>

Series render order depends on the insertion order into <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">renderableSeriesðŸ“˜</a> collection. While it also may change when series are hovered or selected (this feature can be enabled with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesselectionmodifier.html" rel="noopener noreferrer" target="_blank">SeriesSelectionModifierðŸ“˜</a>).

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/chart-series-selection" target="_blank">Series Selection Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

#### Annotations Render Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#annotations-render-order" class="hash-link" aria-label="Direct link to Annotations Render Order" translate="no" title="Direct link to Annotations Render Order">â€‹</a>

Annotations expose <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#annotationlayer" rel="noopener noreferrer" target="_blank">annotationLayerðŸ“˜</a> property, which allows them to be placed behind grid-lines/series or above them. (Refer to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eannotationlayer.html" rel="noopener noreferrer" target="_blank">EAnnotationLayerðŸ“˜</a>)

![](out_scichartv4/2d-charts/miscellaneous-apis/ordered-rendering/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Dom Annotations have limited support of the layering options due to their rendering specifics.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/annotation-layers" target="_blank">Annotation Layers Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

#### Native Text Render Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#native-text-render-order" class="hash-link" aria-label="Direct link to Native Text Render Order" translate="no" title="Direct link to Native Text Render Order">â€‹</a>

Native Text rendering could be batched for performance optimization. However to make sure the text is rendered at the correct layer you may need to use immediate Native Text rendering. Some contexts where you can force the immediate rendering are:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#rendernativeaxislabelsimmediately" rel="noopener noreferrer" target="_blank">renderNativeAxisLabelsImmediatelyðŸ“˜</a> for axis labels
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inativetextannotationoptions.html#drawimmediate" rel="noopener noreferrer" target="_blank">drawImmediateðŸ“˜</a> on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/nativetextannotation.html" rel="noopener noreferrer" target="_blank">NativeTextAnnotationðŸ“˜</a>

Also, native text can be used in other contexts such as annotations, data labels, titles, etc...

#### SubCharts Render Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#subcharts-render-order" class="hash-link" aria-label="Direct link to SubCharts Render Order" translate="no" title="Direct link to SubCharts Render Order">â€‹</a>

Sub-charts conform to insertion order when rendering, and the main (parent) surface is processed first by default. This also means that the elements attached to the main surface are rendered before the entities on sub-charts.

#### Background rendering<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#background-rendering" class="hash-link" aria-label="Direct link to Background rendering" translate="no" title="Direct link to Background rendering">â€‹</a>

A chart <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#background" rel="noopener noreferrer" target="_blank">backgroundðŸ“˜</a> is rendered on a root `div` element <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#domchartroot" rel="noopener noreferrer" target="_blank">domChartRootðŸ“˜</a>, while a sub-chart background is rendered with WebGl on the Render Context Canvas. Thus, a parent surface background is always rendered below everything else; And a sub-chart can use only a plain color as a background.

![](out_scichartv4/2d-charts/miscellaneous-apis/ordered-rendering/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Both parent surfaces and sub-charts support transparent backgrounds. [Example of chart background stylingðŸ“˜](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds) [Example of isTransparent property on sub-chartsðŸ“˜](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-chart-sub-surface-transparency)

## Custom Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#custom-order" class="hash-link" aria-label="Direct link to Custom Order" translate="no" title="Direct link to Custom Order">â€‹</a>

SciChart allows changing the render order of some elements.

First of all, it is possible by setting the order of items within collections on a surface such as `renderableSeries`, `annotations`, `subCharts`. To do this dynamically, Use and `add` and `remove` methods on a collection.

Then, some elements (currently series and annotations) have methods for advanced control of the render order. The API is defined in <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/orderedrenderable.html" rel="noopener noreferrer" target="_blank">IOrderedRenderableðŸ“˜</a>

### Ordered Renderables<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#ordered-renderables" class="hash-link" aria-label="Direct link to Ordered Renderables" translate="no" title="Direct link to Ordered Renderables">â€‹</a>

SciChart allows setting a custom render order on instances that implement `IOrderedRenderable`.

#### Absolute order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#absolute-order" class="hash-link" aria-label="Direct link to Absolute order" translate="no" title="Direct link to Absolute order">â€‹</a>

The order can be set as a value via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#renderorder" rel="noopener noreferrer" target="_blank">renderOrderðŸ“˜</a> constructor option (or dynamically via <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iorderedrenderable.html#setrenderorder" rel="noopener noreferrer" target="_blank">setRenderOrderðŸ“˜</a>).

For example, in the following setup, we change the order of the first annotation so it is rendered on top of the second one, which is the opposite of the default behaviour.

``` prism-code
const textAnnotation1 = new NativeTextAnnotation({
    xCoordinateMode: ECoordinateMode.Pixel,
    yCoordinateMode: ECoordinateMode.Pixel,
    x1: 200,
    y1: 200,
    padding: Thickness.fromNumber(8),
    isEditable: true,
    fontSize: 32,
    background: "blue",
    text: "textAnnotation1",
    renderOrder: 3
});
const textAnnotation2 = new NativeTextAnnotation({
    xCoordinateMode: ECoordinateMode.Pixel,
    yCoordinateMode: ECoordinateMode.Pixel,
    x1: 220,
    y1: 220,
    padding: Thickness.fromNumber(8),
    isEditable: true,
    fontSize: 32,
    background: "red",
    text: "textAnnotation2"
});

sciChartSurface.annotations.add(textAnnotation1, textAnnotation2);
```

#### Relative Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#relative-order" class="hash-link" aria-label="Direct link to Relative Order" translate="no" title="Direct link to Relative Order">â€‹</a>

Alternatively, an order could be set as an offset from another Ordered Renderable instance.

In this example, we demonstrate how to place an annotation on a layer between renderable series via <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotationbaseoptions.html#rendernextto" rel="noopener noreferrer" target="_blank">renderNextToðŸ“˜</a> option (or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#setrendernextto" rel="noopener noreferrer" target="_blank">setRenderNextToðŸ“˜</a> method).

- TS
- JS

``` prism-code
const lineSeries1 = new SplineLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries1,
    stroke: "royalblue",
    strokeThickness: 5
});

const lineSeries2 = new SplineLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries2,
    stroke: "orangered",
    strokeThickness: 5
});

sciChartSurface.renderableSeries.add(lineSeries1, lineSeries2);

const boxAnnotation = new BoxAnnotation({
    renderNextTo: { renderable: lineSeries1.id, offset: 0 },
    fill: "mediumseagreen",
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.Relative,
    x1: 8,
    y1: 0,
    x2: 12,
    y2: 1
});

sciChartSurface.annotations.add(boxAnnotation);
```

``` prism-code
const lineSeries1 = new SplineLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries1,
    stroke: "royalblue",
    strokeThickness: 5
});
const lineSeries2 = new SplineLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries2,
    stroke: "orangered",
    strokeThickness: 5
});
sciChartSurface.renderableSeries.add(lineSeries1, lineSeries2);
const boxAnnotation = new BoxAnnotation({
    renderNextTo: { renderable: lineSeries1.id, offset: 0 },
    fill: "mediumseagreen",
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.Relative,
    x1: 8,
    y1: 0,
    x2: 12,
    y2: 1
});
sciChartSurface.annotations.add(boxAnnotation);
```

#### Customizing Render Layer<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#customizing-render-layer" class="hash-link" aria-label="Direct link to Customizing Render Layer" translate="no" title="Direct link to Customizing Render Layer">â€‹</a>

Setting a custom render layer allows to place an entity at a layer of different entity kinds, as well.

Here we will demonstrate how to place series at the background layer, so that grid lines are rendered above them. For that we can use <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaserenderableseriesoptions.html#renderlayer" rel="noopener noreferrer" target="_blank">renderLayerðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#setrenderlayer" rel="noopener noreferrer" target="_blank">setRenderLayerðŸ“˜</a>

``` prism-code
const lineSeries1 = new SplineMountainRenderableSeries(wasmContext, {
    renderLayer: EDefaultRenderLayer.Background,
    dataSeries: dataSeries1,
    stroke: "royalblue",
    strokeThickness: 5
});

sciChartSurface.renderableSeries.add(lineSeries1);
```

#### Surface Render Order<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/#surface-render-order" class="hash-link" aria-label="Direct link to Surface Render Order" translate="no" title="Direct link to Surface Render Order">â€‹</a>

It is also possible to change the render order of an element to appear as if it was rendered on another sub-chart, or to change the render order of a whole sub-chart

Let's consider the following setup:

- TS
- JS

``` prism-code
const [subSurface1, subSurface2] = buildSubCharts(
    [
        {
            xAxes: {
                type: EAxisType.NumericAxis,
                options: { autoRange: EAutoRange.Once }
            },
            surface: {
                position: { x: 100, y: 100, width: 200, height: 200 },
                coordinateMode: ESubSurfacePositionCoordinateMode.Pixel,
                isTransparent: true
            },
            series: [
                {
                    type: ESeriesType.SplineMountainSeries,
                    xyData: {
                        xValues: Array.from({ length: 20 }, (_, i) => i),
                        yValues: [12, 9, 10, 6, 7, 11, 13, 8, 9, 10, 14, 7, 5, 9, 8, 13, 6, 10, 11, 12]
                    },
                    options: {
                        stroke: "FireBrick",
                        fill: "Tomato",
                        strokeThickness: 5
                    }
                }
            ]
        },
        {
            surface: {
                position: { x: 150, y: 150, width: 200, height: 200 },
                coordinateMode: ESubSurfacePositionCoordinateMode.Pixel,
                isTransparent: true
            },
            series: [
                {
                    type: ESeriesType.SplineMountainSeries,
                    xyData: {
                        xValues: Array.from({ length: 20 }, (_, i) => i),
                        yValues: [6, 8, 5, 9, 3, 10, 7, 6, 4, 12, 8, 9, 10, 5, 11, 7, 3, 8, 9, 6]
                    },
                    options: {
                        stroke: "Navy",
                        fill: "DodgerBlue",
                        strokeThickness: 5
                    }
                }
            ]
        }
    ],
    sciChartSurface
) as SciChartSubSurface[];
```

``` prism-code
const [subSurface1, subSurface2] = buildSubCharts([
    {
        xAxes: {
            type: EAxisType.NumericAxis,
            options: { autoRange: EAutoRange.Once }
        },
        surface: {
            position: { x: 100, y: 100, width: 200, height: 200 },
            coordinateMode: ESubSurfacePositionCoordinateMode.Pixel,
            isTransparent: true
        },
        series: [
            {
                type: ESeriesType.SplineMountainSeries,
                xyData: {
                    xValues: Array.from({ length: 20 }, (_, i) => i),
                    yValues: [12, 9, 10, 6, 7, 11, 13, 8, 9, 10, 14, 7, 5, 9, 8, 13, 6, 10, 11, 12]
                },
                options: {
                    stroke: "FireBrick",
                    fill: "Tomato",
                    strokeThickness: 5
                }
            }
        ]
    },
    {
        surface: {
            position: { x: 150, y: 150, width: 200, height: 200 },
            coordinateMode: ESubSurfacePositionCoordinateMode.Pixel,
            isTransparent: true
        },
        series: [
            {
                type: ESeriesType.SplineMountainSeries,
                xyData: {
                    xValues: Array.from({ length: 20 }, (_, i) => i),
                    yValues: [6, 8, 5, 9, 3, 10, 7, 6, 4, 12, 8, 9, 10, 5, 11, 7, 3, 8, 9, 6]
                },
                options: {
                    stroke: "Navy",
                    fill: "DodgerBlue",
                    strokeThickness: 5
                }
            }
        ]
    }
], sciChartSurface);
```

Then, to reorder the sub-charts, we can apply the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#setsurfacerenderorder" rel="noopener noreferrer" target="_blank">setSurfaceRenderOrderðŸ“˜</a> method on a surface:

``` prism-code
subSurface1.setSurfaceRenderOrder(3);
```

Result:

Or to modify the render order only of the series from the first chart we can apply the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#setsurfacerenderorder" rel="noopener noreferrer" target="_blank">setSurfaceRenderOrderðŸ“˜</a> method on them:

- TS
- JS

``` prism-code
const firstSubSurface = sciChartSurface.subCharts[0] as SciChartSubSurface;
const firstSeries = firstSubSurface.renderableSeries.get(0) as SplineMountainRenderableSeries;
firstSeries.setSurfaceRenderOrder(3);
```

``` prism-code
const firstSubSurface = sciChartSurface.subCharts[0];
const firstSeries = firstSubSurface.renderableSeries.get(0);
firstSeries.setSurfaceRenderOrder(3);
```

Result:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/miscellaneous-apis/ordered-rendering/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/miscellaneous-apis/ordered-rendering/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
