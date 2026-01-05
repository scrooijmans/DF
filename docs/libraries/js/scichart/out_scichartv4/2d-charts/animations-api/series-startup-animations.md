On this page

# Series Startup Animations

There are several series startup Animation Types provided out of the box in SciChart.js. These are:

- **FadeAnimation**
- **ScaleAnimation**
- **SweepAnimation**
- **WaveAnimation**

### Sweep Startup Animation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations/#sweep-startup-animation" class="hash-link" aria-label="Direct link to Sweep Startup Animation" translate="no" title="Direct link to Sweep Startup Animation">â€‹</a>

Let's see a simple example of using Sweep Animation on chart types in SciChart.js

``` prism-code
// Sweep Animation    

const rendSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries,
    strokeThickness: 2,
    animation: new SweepAnimation({ duration: 1000 }),
});

// Alternatively
rendSeries.enqueueAnimation(new SweepAnimation({ duration: 1000 }));
```

<img src="out_scichartv4/2d-charts/animations-api/series-startup-animations/index_media/9311a3b29f6a0807f20af5a9b06c7e75528fcb14.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Fade Startup Animation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations/#fade-startup-animation" class="hash-link" aria-label="Direct link to Fade Startup Animation" translate="no" title="Direct link to Fade Startup Animation">â€‹</a>

Now let's see an exampleÂ ofÂ using Fade Animation on some chart types in SciChart.js

``` prism-code
// Fade Animation

const rendSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries,
    strokeThickness: 2,
    animation: new FadeAnimation({ duration: 1000 }),
});

// Alternatively
rendSeries.enqueueAnimation(new FadeAnimation({ duration: 1000 }));
```

<img src="out_scichartv4/2d-charts/animations-api/series-startup-animations/index_media/d415bdc3a605661455d476b9e1041b901a4eb676.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Note: The Sweep, Scale and Wave animations also support fade/opacity by setting the Animation.fadeEffect property to true.

### Scale Startup Animation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations/#scale-startup-animation" class="hash-link" aria-label="Direct link to Scale Startup Animation" translate="no" title="Direct link to Scale Startup Animation">â€‹</a>

Now let's see an example of the Sweep animation on chart types in SciChart.js.

``` prism-code
// Scale Animation

const rendSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries,
    strokeThickness: 2,
    animation: new ScaleAnimation({ duration: 1000, zeroLineY: -1.5 }),
});

// Alternatively
rendSeries.enqueueAnimation(new ScaleAnimation({ duration: 1000 }));
```

<img src="out_scichartv4/2d-charts/animations-api/series-startup-animations/index_media/0552ac05ca356d4d801d5a0048252500ede61cb3.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

### Wave Startup Animation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations/#wave-startup-animation" class="hash-link" aria-label="Direct link to Wave Startup Animation" translate="no" title="Direct link to Wave Startup Animation">â€‹</a>

FinallyÂ let's see an example of the Wave animation on chart types in SciChart.js.

``` prism-code
// Wave Animation

const rendSeries = new FastBandRenderableSeries(wasmContext, {
    dataSeries,
    strokeThickness: 2,
    animation: new WaveAnimation({ duration: 1000, pointDurationFactor: 0.5, zeroLineY: -1.5 }),
});

// Alternatively
rendSeries.enqueueAnimation(new WaveAnimation({ duration: 1000 }));
```

<img src="out_scichartv4/2d-charts/animations-api/series-startup-animations/index_media/f4c9c286ed1cb5d2dd2243c98390b00a36e07d73.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The Animations API](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/)
- [Style Transition Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations/)
- [Dataset Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations/)
- [Generic Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/generic-animations/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/animations-api/series-startup-animations/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/animations-api/series-startup-animations/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
