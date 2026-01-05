On this page

# PaletteProvider API Overview

SciChart.js features the ability to change color of series on a point-by-point basis, using the PaletteProvider feature.

Many series types support PaletteProvider, including:

- Line Series ([FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series))
- Mountain Series ([FastMountainRenderableSeriers](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series))
- Band Series ([FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series))
- Bubble Series ([FastBubbleRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series))
- Candlestick Series ([FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series))
- OHLC Series ([FastOhlcRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series))
- Column Series ([FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type))
- Scatter Series ([XyScatterRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series))
- Polar Band Series ([PolarBandRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series))
- Polar Column Series ([PolarColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series))
- Polar Line Series ([PolarLineRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-line-renderable-series))
- Polar Mountain Series ([PolarMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-mountain-renderable-series))
- Polar Scatter Series ([PolarXyScatterRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series))

## What is the PaletteProvider API?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/#what-is-the-paletteprovider-api" class="hash-link" aria-label="Direct link to What is the PaletteProvider API?" translate="no" title="Direct link to What is the PaletteProvider API?">â€‹</a>

The PaletteProvider API allows you to achieve per data-point colouring or styling. Here is a quick example below. The following pages have further worked examples for each series type.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/chart-color-points-individually-with-paletteprovider" target="_blank">Coloring Series per-point using PaletteProvider</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing how to color data-points based on a rule.

  

  

The PaletteProvider API is a powerful extension in SciChart.js which allows you to colour line segments, scatter points, candles/columns or mountain chart segments based on a programatic rule.

## Some common Use-cases for the PaletteProvider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/#some-common-use-cases-for-the-paletteprovider" class="hash-link" aria-label="Direct link to Some common Use-cases for the PaletteProvider" translate="no" title="Direct link to Some common Use-cases for the PaletteProvider">â€‹</a>

Some common use-cases for the PaletteProvider API include:

- Changing colour of a line series when value exceeds a threshold.
- Colouring candlesticks based on volume
- Changing the Fill of a time-based Histogram based on day of the week
- Highlighting important Scatter or Bubble points based on additional data.

![](out_scichartv4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Use this API any time you want to change the colour, fill or scatter-point colours programmatically on a per-datapoint basis.

## Enabling the PaletteProvider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/#enabling-the-paletteprovider" class="hash-link" aria-label="Direct link to Enabling the PaletteProvider" translate="no" title="Direct link to Enabling the PaletteProvider">â€‹</a>

To enable the paletting feature, you need to create a class which conforms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> orÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html" rel="noopener noreferrer" target="_blank">IPointMarkerPaletteProviderðŸ“˜</a> interfaces and assign a new instance of the class to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html#paletteprovider" rel="noopener noreferrer" target="_blank">IRenderableSeries.paletteProviderðŸ“˜</a> property.

The following articles in this section show you how to do this for each series type:

- [Per-point Colouring of Line Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-renderable-series)
- [Per-point Colouring of Mountain Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-mountain-renderable-series)
- [Per-Point Colouring of Band Segments](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series)
- [Per-Point Colouring of Bubble Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series)
- [Per-Point Colouring of Candlestick / OHLC Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-candlestick-ohlc-renderable-series)
- [Per-Point Colouring of Scatter Charts (or PointMarkers)](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/xy-scatter-renderable-series)
- [Per-Point Colouring of Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-column-renderable-series)
- [Per-Point Colouring of Impulse Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-impulse-renderable-series)
- [Per-Point Coloring for Rectangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-rectangle-renderable-series)
- [Per-Point Coloring for Line Segment Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-line-segment-renderable-series/)
- [Per-Point Coloring for Triangle Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-triangle-renderable-series/)
- [Per-Point Coloring for Polar Band Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-band-renderable-series)
- [Per-Point Coloring for Polar Column Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-column-renderable-series)
- [Per-Point Coloring for Polar Line Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-line-renderable-series)

<!-- -->

- [Per-Point Coloring for Polar Scatter Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/polar-xy-scatter-renderable-series)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
