On this page

# Retina Support and Browser Zoom

SciChart.js supports sharp graphics on high resolution Retina displays, as well as Browser Zoom functionality.

Previously in SciChart.js, retina displays would look low quality, and when the browser is zoomed the image that SciChart renders to would simply be scaled.

In SciChart.js starting from v2, every element is now rendered at the native resolution and scaled down for display. This results in the following benefits:

1.  Lines, strokes, shapes now look sharper and clearer on higher DPI displays or when browser is zoomed
2.  Text is rendered at a higher resolution. Text scales with browser zoom (good for Accessibility)
3.  Stroke thickness (line pen) increases with Browser Zoom

Take a look below to see some comparison images side by side of SciChart.js v1 vs. v2 at 200% Browser zoom in Chrome.

<img src="out_scichartv4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/index_media/3e48ee6dca60b5c5479b081059247e23f3d075e4.png" class="img_ev3q" decoding="async" loading="lazy" width="3784" height="1259" />

In particular, notice the quality of text, lines and gridlines difference between version 1 and version 2 when at 200% browser zoom:

<img src="out_scichartv4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/index_media/682d8ff819271e3009a3b8058661595635071af0.png" class="img_ev3q" decoding="async" loading="lazy" width="1050" height="580" />

## Enabling & Disabling Retina DPI / Browser Zoom Support<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/#enabling--disabling-retina-dpi--browser-zoom-support" class="hash-link" aria-label="Direct link to Enabling &amp; Disabling Retina DPI / Browser Zoom Support" translate="no" title="Direct link to Enabling &amp; Disabling Retina DPI / Browser Zoom Support">â€‹</a>

By default, Retina & high DPI support is built in, you don't have to do anything to enable it.

However, if you wanted to disable automatic scaling with DPI then you can use the following code:

- Disable DPI scaling

``` prism-code
import { DpiHelper} from "scichart";

// Note: you will need to call this before any SciChartSurface is created
DpiHelper.IsDpiScaleEnabled = false;
```

## Performance Considerations when Dpi Scaling<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/#performance-considerations-when-dpi-scaling" class="hash-link" aria-label="Direct link to Performance Considerations when Dpi Scaling" translate="no" title="Direct link to Performance Considerations when Dpi Scaling">â€‹</a>

When SciChart.js is used on a high resolution display such as Retina, the chart will be rendered at 4x the number of pixels visible on screen. For example a 1,000 x 1,000 chart (1M Pixels) will be rendered at 2,000 x 2,000 (4M Pixels) before scaling down to the correct size.

Higher number of pixels means more work for the browser to display the chart. If you notice any performance degredation on your application you can disable Dpi scaling using the code above.

Also, we recommend use of Google Chrome browser as this has by far the best performance metrics, compared to Safari or Firefox, which both struggle to render large canvases.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
