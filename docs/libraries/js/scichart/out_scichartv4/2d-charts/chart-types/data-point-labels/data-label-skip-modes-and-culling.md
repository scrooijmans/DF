On this page

# DataLabel SkipModes and Culling

DataLabels allow per data-point text labels to be drawn on series, or arbitrary text labels at x,y positions on the chart.

You can see several datalabel examples on the SciChart.js demo:

- <a href="https://www.scichart.com/demo/javascript-line-chart" rel="noopener noreferrer" target="_blank">The Line Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-column-chart" rel="noopener noreferrer" target="_blank">The Column Chart example</a>
- <a href="https://www.scichart.com/demo/javascript-chart-color-points-individually-with-paletteprovider" rel="noopener noreferrer" target="_blank">The PaletteProvider example</a>
- <a href="https://www.scichart.com/demo/javascript-datalabels" rel="noopener noreferrer" target="_blank">The DataLabels demo</a>
- <a href="https://www.scichart.com/demo/javascript/stacked-column-chart" rel="noopener noreferrer" target="_blank">The Stacked Column Chart demo</a>
- <a href="https://www.scichart.com/demo/javascript/population-pyramid" rel="noopener noreferrer" target="_blank">The Population Pyramid demo</a>

Explore these for some rich examples of how to use this API.

## Labels for Many Points<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#labels-for-many-points" class="hash-link" aria-label="Direct link to Labels for Many Points" translate="no" title="Direct link to Labels for Many Points">â€‹</a>

SciChart's native text rendering means that it can potentially draw tens of thousands of labels relatively fast, but in practise with many points there is no point drawing so many labels that they become unreadable. SciChart has a number of options for dealing with this. If you really want to show all labels even if they overlap, setÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipmode" rel="noopener noreferrer" target="_blank">skipModeðŸ“˜</a> to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" rel="noopener noreferrer" target="_blank">EDataLabelSkipMode.ShowAllðŸ“˜</a>

### Hide overlapping labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#hide-overlapping-labels" class="hash-link" aria-label="Direct link to Hide overlapping labels" translate="no" title="Direct link to Hide overlapping labels">â€‹</a>

The default for theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipmode" rel="noopener noreferrer" target="_blank">skipModeðŸ“˜</a>Â property isÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" rel="noopener noreferrer" target="_blank">EDataLabelSkipMode.SkipIfOverlapPreviousðŸ“˜</a>. This means a label will not be drawn if it would overlap the bounds of the previous label. This means that even if you have 1000 points on a line series, you will only see a few dozen non-overlpping labels (depending on the shape of your data).

The downside of this is that SciChart has to calculate the text, size and position of every label, and then throw most of them away, which is potentially inefficient. Also, it is often unclear which points on the line are actually being labeled.Â  The alternative to this is to calculate less labels (useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipnumber" rel="noopener noreferrer" target="_blank">skipNumberðŸ“˜</a>) or to not draw labels at all if there are too many to display (useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold" rel="noopener noreferrer" target="_blank">pointGapThresholdðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthreshold" rel="noopener noreferrer" target="_blank">pointCountThresholdðŸ“˜</a>).

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipmode" rel="noopener noreferrer" target="_blank">skipModeðŸ“˜</a>Â  also has aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" rel="noopener noreferrer" target="_blank">EDataLabelSkipMode.SkipIfOverlapNextðŸ“˜</a> option. This is useful if you have labels of significantly varying lengths, as it means long labels tend to be skipped in favour of shorter ones.

### Improve performance with many points using skipNumber<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#improve-performance-with-many-points-using-skipnumber" class="hash-link" aria-label="Direct link to Improve performance with many points using skipNumber" translate="no" title="Direct link to Improve performance with many points using skipNumber">â€‹</a>

SettingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipnumber" rel="noopener noreferrer" target="_blank">skipNumberðŸ“˜</a>Â greater than 0 will make SciChart skip that many points between each label it generates. The number of labels generated is therefore pointCount / ( skipNumber + 1). You will see performance warnings in the console if more than 80% of labels were skipped.

### Showing Labels Past a Threshold<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#showing-labels-past-a-threshold" class="hash-link" aria-label="Direct link to Showing Labels Past a Threshold" translate="no" title="Direct link to Showing Labels Past a Threshold">â€‹</a>

The alternative is to only show labels when the chart is sufficiently zoomed in so that there are a sensible number of labels to display, or room to show them.

#### pointGapThreshold<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#pointgapthreshold" class="hash-link" aria-label="Direct link to pointGapThreshold" translate="no" title="Direct link to pointGapThreshold">â€‹</a>

If your labels are a consistent size and your data is evenly spaced and does not have large y variation (ie it is smooth, not jagged), then settingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold" rel="noopener noreferrer" target="_blank">pointGapThresholdðŸ“˜</a>Â to around 1 will cause labels to appear only when there is room to show them.Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold" rel="noopener noreferrer" target="_blank">pointGapThresholdðŸ“˜</a>Â is the gap between the first points divided by the size of the first label, so 1 means the spacing between points is equal to the label size. Values less than 1 will cause labels to be drawn sooner, but they may overlap. Values greater than 1 mean that you will need to zoom in more, but labels are less likely to overlap.

#### pointCountThreshold<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#pointcountthreshold" class="hash-link" aria-label="Direct link to pointCountThreshold" translate="no" title="Direct link to pointCountThreshold">â€‹</a>

If your data is unevenly spaced, is jagged, or your label text has significant variation in width, thenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthreshold" rel="noopener noreferrer" target="_blank">pointCountThresholdðŸ“˜</a>Â may give more predictable results. It is simply the number of points in view, below which labels will be drawn.Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipmode" rel="noopener noreferrer" target="_blank">skipModeðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipnumber" rel="noopener noreferrer" target="_blank">skipNumberðŸ“˜</a>Â still apply when these threshold options are set.

#### Custom thresholds<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/#custom-thresholds" class="hash-link" aria-label="Direct link to Custom thresholds" translate="no" title="Direct link to Custom thresholds">â€‹</a>

If you don't like either of those options, you can write your own threshold by overriding theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldgenerate" rel="noopener noreferrer" target="_blank">shouldGenerateðŸ“˜</a> function onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html" rel="noopener noreferrer" target="_blank">dataLabelProviderðŸ“˜</a>. This receives aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" rel="noopener noreferrer" target="_blank">DataLabelStateðŸ“˜</a> which will return values for the first label. IfÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldgenerate" rel="noopener noreferrer" target="_blank">shouldGenerateðŸ“˜</a>Â returns true, labels will be generated. If false, they will not. Below is the standard implementation of shouldGenerate.

``` prism-code
public shouldGenerate(state: DataLabelState): boolean {
    if (state.pointCount > this.pointCountThresholdProperty) return false;
    const firstlabel = this.getText(state);
    const bounds = getTextBounds(this.webAssemblyContext);
    state.font.CalculateStringBounds(firstlabel ?? "", bounds, this.style?.lineSpacing ?? 2);
    return state.pointGap > bounds.m_fWidth * this.pointGapThreshold;
}
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-point-labels/data-label-skip-modes-and-culling/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
