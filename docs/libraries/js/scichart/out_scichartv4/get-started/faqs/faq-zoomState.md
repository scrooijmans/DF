On this page

# How to Detect If a User Is Zooming or Panning

## The sciChartSurface.zoomState Property<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq-zoomState/#the-scichartsurfacezoomstate-property" class="hash-link" aria-label="Direct link to The sciChartSurface.zoomState Property" translate="no" title="Direct link to The sciChartSurface.zoomState Property">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#zoomstate" rel="noopener noreferrer" target="_blank">sciChartSurface.zoomStateðŸ“˜</a> property allows us to detect if the chart has been zoomed or panned by the user, or if the chart is at extents of the data. You can take a look at the values of the <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ezoomstate.html" rel="noopener noreferrer" target="_blank">EZoomState Enum hereðŸ“˜</a>.

Here is example:

``` prism-code

const statusLabel = new TextAnnotation({
    x1: 0.1,
    y1: 0.1,
    opacity: 0.5,
    fontSize: 22,
    textColor: "white",
    horizontalAnchorPoint: EHorizontalAnchorPoint.Left,
    verticalAnchorPoint: EVerticalAnchorPoint.Bottom,
    xCoordinateMode: ECoordinateMode.Relative,
    yCoordinateMode: ECoordinateMode.Relative,
});
sciChartSurface.annotations.add(statusLabel);


sciChartSurface.rendered.subscribe(() => {

    if (sciChartSurface.zoomState === EZoomState.UserZooming) {
        statusLabel.text = "Chart has been zoomed or panned by the user"
    } else if (sciChartSurface.zoomState === EZoomState.AtExtents) {
        statusLabel.text = "Chart is at extents of the data"
    }

})
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/faqs/faq-zoomState/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/faqs/faq-zoomState/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
