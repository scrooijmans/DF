On this page

# Glow and DropShadow Shader Effects

SciChart.js features WebGL shader-basedÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gloweffect.html" rel="noopener noreferrer" target="_blank">GlowEffectðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/shadoweffect.html" rel="noopener noreferrer" target="_blank">ShadowEffectðŸ“˜</a> effects which may be added onto chart types throughout our library.

<img src="out_scichartv4/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/index_media/630646e7752aa731612da6d9deff0b1ddfaf2355.png" class="img_ev3q" decoding="async" loading="lazy" width="901" height="600" alt="WebGL Shader effects applied to JavaScript Charts to achieve glow and dropshadow" />

**Above**: WebGL GlowEffect added to the Real-time Ghosted Traces example

## Adding Glow Effects to Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/#adding-glow-effects-to-series" class="hash-link" aria-label="Direct link to Adding Glow Effects to Series" translate="no" title="Direct link to Adding Glow Effects to Series">â€‹</a>

A glow shader effect can be added to series to give it an oscilloscope / VDU style effect.

``` prism-code
// GlowEffect example

import { GlowEffect, Point, FastLineRenderableSeries } from "scichart";

sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
        strokeThickness: 2,
        stroke: "#FFFF00",
        effect: new GlowEffect(wasmContext, {
            range: 0,
            intensity: 1,
            color: "#333333",
            offset: new Point(10, 10)
        })
    })
);
```

This results in the following (visible in theÂ <a href="https://www.scichart.com/demo/javascript-vital-signs-ecg-medical-chart-example" rel="noopener noreferrer" target="_blank">Vital Signs monitor</a> example).

<img src="out_scichartv4/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/index_media/7bcf366cca8b2a63d73b4a96f452e0b6da3085aa.png" class="img_ev3q" decoding="async" loading="lazy" width="796" height="601" alt="WebGL Shader effects applied to JavaScript Charts to achieve glow and dropshadow" />

## Adding Shadow Effect to Series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/#adding-shadow-effect-to-series" class="hash-link" aria-label="Direct link to Adding Shadow Effect to Series" translate="no" title="Direct link to Adding Shadow Effect to Series">â€‹</a>

Drop-shadow effects are also in development, and an example will be provided soon.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
