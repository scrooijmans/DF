On this page

# Intro to the Builder API

The **Builder API** is a new API to SciChart.js v2. In addition to programmatically creating charts with JavaScript or TypeScript code, you can now create charts with a JSON-object API, either with objects in code with discovererable intellisense, or with JSON passed from server to client.

The Builder API is intended to:

- Provide a more familiar api to javascript developers
- Provide better discoverability of SciChart features when using typescript
- Enable charts to be defined using pure data, so that they can be serialized and deserialized

It is not intended to completely replace the existing API.Â The two can be used in combination, and the original api is more suitable for some tasks, and required for others. Currently it only supports 2D charts.

## Discovering the Builder API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/#discovering-the-builder-api" class="hash-link" aria-label="Direct link to Discovering the Builder API" translate="no" title="Direct link to Discovering the Builder API">â€‹</a>

To use the SciChart.js Builder API, you will need this import.

``` prism-code
import { chartBuilder } from "scichart";
```

**chartBuilder** exposes all the builder methods, which can be used to build parts of, or the entire chart from a JSON definition:

The top level method is **buildChart**, which takes the id of the target div, and a definition, which can be a JSON string or an object.

There are also specific function calls to <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.build2dchart" rel="noopener noreferrer" target="_blank">build2DChartðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.buildpiechart" rel="noopener noreferrer" target="_blank">buildPieChartðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.build2dpolarchart" rel="noopener noreferrer" target="_blank">build2DPolarChartðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder.build3dchart" rel="noopener noreferrer" target="_blank">build3DChartðŸ“˜</a> which can be used to build on specific chart surfaces:

- Build Surface-specific Charts

``` prism-code
const { build2DChart, buildPieChart, build2DPolarChart, build3DChart } = chartBuilder;

// build a 2D chart
const simple2d = await build2DChart("chartDivId", {
    // ...
});

// build a Pie chart
const pieChart = await buildPieChart("chartDivId", {});

// build a Polar 2D chart
const polar2d = await build2DPolarChart("chartDivId", {});

// build a 3D chart
const simple3d = await build3DChart("chartDivId", {});
```

Note that all the elements are optional. This lets you define partial definitions that you can reuse and combine.

## Worked Examples of the Builder API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/#worked-examples-of-the-builder-api" class="hash-link" aria-label="Direct link to Worked Examples of the Builder API" translate="no" title="Direct link to Worked Examples of the Builder API">â€‹</a>

We've created some worked examples of the Builder API on the following pages. Also check out ourÂ <a href="https://www.github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">Github</a> andÂ <a href="https://www.scichart.com/demo" rel="noopener noreferrer" target="_blank">SciChart demo</a> where we have published some examples of the Builder API.

### Builder API Documentation Pages<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/#builder-api-documentation-pages" class="hash-link" aria-label="Direct link to Builder API Documentation Pages" translate="no" title="Direct link to Builder API Documentation Pages">â€‹</a>

- **Documentation**:Â [Creating a Simple Chart](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/simple-chart)
- **Documentation**:Â [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)
- **Documentation**:Â [Complex Customisation](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/complex-options)
- **Documentation**:Â [Custom Subtypes](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/custom-subtypes)

### Builder API Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/#builder-api-examples" class="hash-link" aria-label="Direct link to Builder API Examples" translate="no" title="Direct link to Builder API Examples">â€‹</a>

- **Example**:Â <a href="https://www.scichart.com/demo/javascript-builder-simple" rel="noopener noreferrer" target="_blank">Simple Chart using Builder API</a>
- **Example**:Â <a href="https://www.scichart.com/demo/javascript-builder-full" rel="noopener noreferrer" target="_blank">Full Chart using Builder API</a>
- **Example**:Â <a href="https://www.scichart.com/demo/javascript-chart-from-json" rel="noopener noreferrer" target="_blank">Chart from JSON</a>
- **Example**:Â <a href="https://www.scichart.com/demo/javascript-shared-data" rel="noopener noreferrer" target="_blank">Reusable Templates with Shared Data</a>
- **Example**:Â <a href="https://www.scichart.com/demo/javascript-custom-types" rel="noopener noreferrer" target="_blank">Custom Subtypes with Builder API</a>

## TypeScript Intellisense<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/#typescript-intellisense" class="hash-link" aria-label="Direct link to TypeScript Intellisense" translate="no" title="Direct link to TypeScript Intellisense">â€‹</a>

The Builder API is best when used with Typescript, so it can guide you as to what types are available or required.

Intellisense shows which options can be passed to **buildChart**:

<img src="out_scichartv4/2d-charts/builder-api/builder-api-overview/index_media/bcf27f981d4a9866b281dc9306cfc824a8fc5b78.png" class="img_ev3q" decoding="async" loading="lazy" width="642" height="130" />

Intellisense will show you the series definition must have a type property which is an **ESeriesType**. This shows you all the series types that SciChart provides (more than shown in this screenshot).

<img src="out_scichartv4/2d-charts/builder-api/builder-api-overview/index_media/48dcacaaddfbbb62faf0540bc3206dca094ce567.png" class="img_ev3q" decoding="async" loading="lazy" width="642" height="259" />

Once you have selected a series, the properties and types will become specific to that series type:

<img src="out_scichartv4/2d-charts/builder-api/builder-api-overview/index_media/3b42e11cecf5270ec837c466e1b0ac3a02f68ba5.png" class="img_ev3q" decoding="async" loading="lazy" width="642" height="70" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

\*Â [Creating a Simple Chart](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/simple-chart)

- [Working with Data](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/builder-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/builder-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
