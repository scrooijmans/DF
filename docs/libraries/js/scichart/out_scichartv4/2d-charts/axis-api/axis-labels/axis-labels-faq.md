On this page

# Axis Labels FAQs

## I would like no matter what always to show axis labels, they can overlap but should not hide<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/axis-labels-faq/#i-would-like-no-matter-what-always-to-show-axis-labels-they-can-overlap-but-should-not-hide" class="hash-link" aria-label="Direct link to I would like no matter what always to show axis labels, they can overlap but should not hide" translate="no" title="Direct link to I would like no matter what always to show axis labels, they can overlap but should not hide">â€‹</a>

This scenario if often useful when we disable **autoTicks** and set **majorDelta** manually. This can be done by setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inumericaxisoptions.html#hideoverlappinglabels" rel="noopener noreferrer" target="_blank">INumericAxisOptions.hideOverlappingLabelsðŸ“˜</a> option to `false`.

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartSurface.create("scichart-root");
  
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));

sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
        labelPrecision: 0,
        autoTicks: false,
        majorDelta: 2,
        hideOverlappingLabels: false
    })
);
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/axis-labels-faq/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/axis-labels-faq/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
