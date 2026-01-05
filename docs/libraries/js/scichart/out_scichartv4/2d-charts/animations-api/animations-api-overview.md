On this page

# Animations API Overview

## Animations API Fundamentals<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/#animations-api-fundamentals" class="hash-link" aria-label="Direct link to Animations API Fundamentals" translate="no" title="Direct link to Animations API Fundamentals">â€‹</a>

In SciChart.js you can use the Animations API to animateÂ [RenderableSeries (Chart Types)](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview). The Animations API allows you to define a number of different transforms you your chart series during the render pass.

Additionally there areÂ [Generic Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/generic-animations), which are run on the SciChartSurface rather than a particular series, and can be coded to animate almost anything, such as annotations, data, and even things outside the chart.

The reset of this page introduces Series Animations.

For example. here isÂ a Wave startup animation:

- Wave animation

``` prism-code
const lineSeries = new FastLineRenderableSeries(wasmContext, {
animation: new WaveAnimation({
    zeroLine: 0,
    pointDurationFraction: 0.5,
    duration: 2000,
    fadeEffect: true,
    delay: 1000
})
});
```

<img src="out_scichartv4/2d-charts/animations-api/animations-api-overview/index_media/285213a83714fc55b5d39269a7053040e78d0469.gif" class="img_ev3q" decoding="async" loading="lazy" width="623" height="446" />

## Types of Animation in SciChart.js<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/#types-of-animation-in-scichartjs" class="hash-link" aria-label="Direct link to Types of Animation in SciChart.js" translate="no" title="Direct link to Types of Animation in SciChart.js">â€‹</a>

There are three types of seriesÂ animations in the SciChart.js library:

- **[A start-up animation](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations)**, which runs on start (when a series is shown)
- **[A style animation](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations)**, which animates style properties like stroke, strokeThickness and fill
- **[A dataset animation](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations)**, which animates changing data.

Articles above cover these animation types and give you examples of how to use them.

## Running or Queueing an Animation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/#running-or-queueing-an-animation" class="hash-link" aria-label="Direct link to Running or Queueing an Animation" translate="no" title="Direct link to Running or Queueing an Animation">â€‹</a>

The API to run any of those animations is the same.

You can either run an animation immediately, or add it to the queue.

### Adding Animations to the Queue<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/#adding-animations-to-the-queue" class="hash-link" aria-label="Direct link to Adding Animations to the Queue" translate="no" title="Direct link to Adding Animations to the Queue">â€‹</a>

Internally each renderable series has an animation queue which allows chaining animations. In order to add an animation to the queue call **enqueueAnimation:**

- Queue Animation

``` prism-code
const someAnimation = new WaveAnimation({
zeroLine: 0,
pointDurationFraction: 0.5,
duration: 2000,
fadeEffect: true,
delay: 1000
});
const lineSeries = new FastLineRenderableSeries(wasmContext);
lineSeries.enqueueAnimation(someAnimation);
```

### Running an Animation Right Away<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/#running-an-animation-right-away" class="hash-link" aria-label="Direct link to Running an Animation Right Away" translate="no" title="Direct link to Running an Animation Right Away">â€‹</a>

If there is a need to cancel all already running animations and run another animation use **runAnimation** method, or, alternatively setting the **BaseRenderableSeries.animation** property or passing the animation to the series constructor.

- RunÂ Animation

``` prism-code
// Cancels running animations and executes a new one immediately
lineSeries.runAnimation(someAnimation);

// or alternatively
lineSeries.animation = someAnimation;

// or alternatively
lineSeries = new FastLineRenderableSeries(wasmContext, { animation: someAnimation });
```

See the articles in the **See Also** section below for examples of how to use the different animation types.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Style Transition Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/style-transition-animations)
- [Dataset Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/dataset-animations)
- [Series Startup Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/series-startup-animations)
- [Generic Animations](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/generic-animations)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/animations-api/animations-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/animations-api/animations-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
