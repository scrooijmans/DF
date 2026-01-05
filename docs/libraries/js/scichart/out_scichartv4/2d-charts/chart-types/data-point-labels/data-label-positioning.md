On this page

# Data Label Positioning

DataLabels allow per data-point text labels to be drawn on series, or arbitrary text labels at x,y positions on the chart.

You can see several datalabel examples on the SciChart.js demo:

- <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">The Line Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">The Column Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-chart-color-points-individually-with-paletteprovider" rel="noopener noreferrer" target="_blank">The PaletteProvider example</a>
- <a href="https://www.scichart.com/demo/javascript-datalabels" rel="noopener noreferrer" target="_blank">The DataLabels demo</a>
- <a href="https://www.scichart.com/demo/javascript/stacked-column-chart" rel="noopener noreferrer" target="_blank">The Stacked Column Chart demo</a>
- <a href="https://www.scichart.com/demo/javascript/population-pyramid" rel="noopener noreferrer" target="_blank">The Population Pyramid demo</a>

Explore these for some rich examples of how to use this API.

## Label Positioning<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-positioning/#label-positioning" class="hash-link" aria-label="Direct link to Label Positioning" translate="no" title="Direct link to Label Positioning">â€‹</a>

The text positioning rules vary a little for different series. For line series the default behaviour is to place the label above the line if it is moving down, and below if it is moving up. This avoids the text overlapping the line in many situations, but often you will want to take more control.

You can disable the default behviour by settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdatalabelprovider.html#abovebelow" rel="noopener noreferrer" target="_blank">LineSeriesDataLabelProvider.aboveBelowðŸ“˜</a> false, then you can make use of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdatalabelprovider.html#horizontaltextpositionproperty" rel="noopener noreferrer" target="_blank">horizontalTextPositionðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdatalabelprovider.html#verticaltextpositionproperty" rel="noopener noreferrer" target="_blank">verticalTextPositionðŸ“˜</a> properties along with the padding on the style.

The position properties are where the text should be relative to the data point, so *horizontalTextPosition: EHorizontalTextPosition.Left* means place the text to the left of the point (ie the text is anchored on the right.) This example also demonstrates the use of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html#skipifsame" rel="noopener noreferrer" target="_blank">SkipIfSameðŸ“˜</a> option forÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" rel="noopener noreferrer" target="_blank">skipModeðŸ“˜</a> on a digital line. The other skipMode options are discussed in the section on 'Labels for many points' below.

- TS

``` prism-code
const {
    FastLineRenderableSeries,
    ELineType,
    EDataLabelSkipMode,
    EVerticalTextPosition,
    EHorizontalTextPosition,
    Thickness
} = SciChart;

// or for npm: import { FastLineRenderableSeries, ... } from "scichart"

// Shows optional positioning modes for data labels
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    stroke: "SteelBlue",
    strokeThickness: 3,
    lineType: ELineType.Digital,
    pointMarker,
    dataSeries,
    // dataLabels style must be specified to show labels
    dataLabels: {
        skipMode: EDataLabelSkipMode.SkipIfSame,
        aboveBelow: false,
        verticalTextPosition: EVerticalTextPosition.Above,
        horizontalTextPosition: EHorizontalTextPosition.Left,
        style: {
            fontFamily: "Default",
            fontSize: 18,
            padding: new Thickness(0, 5, 5, 0)
        },
        color: "#EEE"
    }
});
```

## Positioning Rules for Data Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-positioning/#positioning-rules-for-data-labels" class="hash-link" aria-label="Direct link to Positioning Rules for Data Labels" translate="no" title="Direct link to Positioning Rules for Data Labels">â€‹</a>

This table summarises the built in positioning behaviour for the various series types.

| Series Type | DataLabelProvider type | Positioning Rules | Type Specific Options |
|----|----|----|----|
| Line | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdatalabelprovider.html" rel="noopener noreferrer" target="_blank">LineSeriesDataLabelProviderðŸ“˜</a> | If **aboveBelow** is true (default), place the label above the line if it is moving down, and below if it is moving up. Otherwise use **horizontalTextPosition** and **verticalTextPosition** (default: Right, Above) | `aboveBelow: boolean` |
| Column / Impulse | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnseriesdatalabelprovider.html" rel="noopener noreferrer" target="_blank">ColumnSeriesDataLabelProviderðŸ“˜</a> | Label is centered and outside the column (above for columns above the zeroLine, below if below). **positionMode** can be set to Inside to reverse this, or to use the value of **verticalTextPosition** (or **horizontalTextPosition** for a vertical chart). | `positionMode`: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumndatalabelposition.html" rel="noopener noreferrer" target="_blank">EColumnDataLabelPositionðŸ“˜</a> |
| Bubble | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bubbleseriesdatalabelprovider.html" rel="noopener noreferrer" target="_blank">BubbleSeriesDataLabelProviderðŸ“˜</a> | Label is centered within the bubble. If **horizontalTextPosition** or **verticalTextPosition** is not Center, label is placed outside the bubble on the specified side | â€“ |
| Band | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bandseriesdatalabelprovider.html" rel="noopener noreferrer" target="_blank">BandSeriesDataLabelProviderðŸ“˜</a> | By default, each line of the band has its own label which follows the rules for line series. Set **singleLabel** to true to render a single label in the middle of the band, containing both y and y1 values. | `singleLabel: boolean` |
| Heatmap | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapdatalabelprovider.html" rel="noopener noreferrer" target="_blank">HeatmapDataLabelProviderðŸ“˜</a> | Labels are centered in the cell | â€“ |
| NonUniformHeatmap | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapdatalabelprovider.html" rel="noopener noreferrer" target="_blank">HeatmapDataLabelProviderðŸ“˜</a> | Labels are centered in the cell | â€“ |
| Contours | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/contoursdatalabelprovider.html" rel="noopener noreferrer" target="_blank">ContoursDataLabelProviderðŸ“˜</a> | Places 10 rows of labels on the contour lines. The rows are evenly spaced. Set **labelRowCount** to adjust the number of rows | `labelRowCount: number` |
| Text | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textdatalabelprovider.html" rel="noopener noreferrer" target="_blank">TextDataLabelProviderðŸ“˜</a> | Labels placed above and to the right of the point. Set **calculateTextBounds** to false for a performance boost if rendering many labels and their size doesn't matter | `calculateTextBounds: boolean` |
| XYScatter | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html" rel="noopener noreferrer" target="_blank">DataLabelProviderðŸ“˜</a> | Labels placed above and to the right of the point | â€“ |
| CandleStick/Ohlc | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html" rel="noopener noreferrer" target="_blank">DataLabelProviderðŸ“˜</a> | Labels placed above and to the right of the close value | â€“ |
| Stacked Column | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcollectiondatalabelprovider.html" rel="noopener noreferrer" target="_blank">StackedCollectionDataLabelProviderðŸ“˜</a> | Label is centered and outside the column (above for columns above the zeroLine, below if below). **positionMode** can be set to Inside to reverse this, or to use the value of **verticalTextPosition** (or **horizontalTextPosition** for a vertical chart). | `positionMode`: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumndatalabelposition.html" rel="noopener noreferrer" target="_blank">EColumnDataLabelPositionðŸ“˜</a> |

## Custom Positioning<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-positioning/#custom-positioning" class="hash-link" aria-label="Direct link to Custom Positioning" translate="no" title="Direct link to Custom Positioning">â€‹</a>

To take full control of label positioning, override theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getposition" rel="noopener noreferrer" target="_blank">dataLabelProvider.getPosition()ðŸ“˜</a> function. This takesÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" rel="noopener noreferrer" target="_blank">DataLabelStateðŸ“˜</a> and a **TSRTextBounds** (a WebAssembly exported type) which describes the size of the label.

It should return a Point `{ x: number, y: number }` which will be the left, baseline point for the label. See **Native Text Api** for details on **TSRTextBounds**.

## Positioning Labels from Multiple Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-positioning/#positioning-labels-from-multiple-series" class="hash-link" aria-label="Direct link to Positioning Labels from Multiple Series" translate="no" title="Direct link to Positioning Labels from Multiple Series">â€‹</a>

Normally, the layout for dataLabels is done per series, so labels from different series could overlap. If you want to prevent this or want to do some other adjustment of label positioning after all labels for all series have been generated, but before they are drawn, you can create anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatalabellayoutmanager.html" rel="noopener noreferrer" target="_blank">IDataLabelLayoutManagerðŸ“˜</a> and attach it to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#datalabellayoutmanager" rel="noopener noreferrer" target="_blank">sciChartSurface.dataLabelLayoutManagerðŸ“˜</a> property.

This has a single method,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatalabellayoutmanager.html#performtextlayout" rel="noopener noreferrer" target="_blank">performTextLayoutðŸ“˜</a> where you can access and update theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabelProvider.dataLabelsðŸ“˜</a> array on all the series.

Although you have access to the full surface andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassinfo.html" rel="noopener noreferrer" target="_blank">renderPassInfoðŸ“˜</a> in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatalabellayoutmanager.html#performtextlayout" rel="noopener noreferrer" target="_blank">performTextLayoutðŸ“˜</a>Â function, be aware that this is run at the very end of the render process, so only changes to the contents of the dataLabels arrays will have an effect on what is drawn. Updating other things on the surface from this function is not advised.

The example below hides labels from the second series which overlap those on the first.

- TS

``` prism-code
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    EllipsePointMarker,
    XyDataSeries,
    NumberRange,
    testIsInBounds,
    SciChartJsNavyTheme
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

// Create a chart with two line series
//
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    title: "Skip overlapping labels across series",
    titleStyle: { fontSize: 20 }
});

sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));

const lineSeries = new FastLineRenderableSeries(wasmContext, {
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
        yValues: [4.3, 5, 5, 6, 8, 6.8, 7, 7, 7.2, 6.5, 6.5, 7]
    }),
    // dataLabels style must be specified to show labels
    dataLabels: {
        style: {
            fontFamily: "Default",
            fontSize: 18
        },
        color: "SteelBlue"
    }
});

sciChartSurface.renderableSeries.add(lineSeries);

const lineSeries2 = new FastLineRenderableSeries(wasmContext, {
    stroke: "Darkorange",
    strokeThickness: 3,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        strokeThickness: 2,
        stroke: "Darkorange",
        fill: "Tan"
    }),
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [4.5, 4.9, 5.1, 6.2, 7, 6.5, 7, 7.5, 7.1, 6.2, 5.5, 6]
    }),
    // dataLabels style must be specified to show labels
    dataLabels: {
        style: {
            fontFamily: "Default",
            fontSize: 18
        },
        color: "Darkorange"
    }
});

sciChartSurface.renderableSeries.add(lineSeries2);

// Override the default data label layout manager and perform custom label layout
sciChartSurface.dataLabelLayoutManager = {
    performTextLayout(sciChartSurface, renderPassInfo) {
        const firstLabels = renderPassInfo.renderableSeriesArray[0].dataLabelProvider.dataLabels;
        const secondLabels = renderPassInfo.renderableSeriesArray[1].dataLabelProvider.dataLabels;
        for (const label of secondLabels) {
            let overlap = false;
            for (const existing of firstLabels) {
                const padding = 2;
                const top = existing.rect.top - padding;
                const bottom = existing.rect.bottom + padding;
                const left = existing.rect.left - padding;
                const right = existing.rect.right + padding;
                if (
                    testIsInBounds(label.rect.left, label.rect.top, left, bottom, right, top) ||
                    testIsInBounds(label.rect.right, label.rect.top, left, bottom, right, top) ||
                    testIsInBounds(label.rect.left, label.rect.bottom, left, bottom, right, top) ||
                    testIsInBounds(label.rect.right, label.rect.bottom, left, bottom, right, top)
                ) {
                    // console.log(`Label ${label.text} overlaps ${existing.text}, skipping...`);
                    overlap = true;
                    break;
                }
            }
            if (overlap) {
                // Labels overlaps another so blank it
                label.text = "";
            }
        }
    }
};
```

![](out_scichartv4/2d-charts/chart-types/data-point-labels/data-label-positioning/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Above: Text layout is overridden to take into account label bounds across series. For each label, if the label overlaps an existing label (or is within 2 pixels of the edge of an existing label), skip drawing the label.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-point-labels/data-label-positioning/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-point-labels/data-label-positioning/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
