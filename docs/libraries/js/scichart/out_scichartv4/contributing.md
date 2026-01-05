On this page

# SciChart.js Docs Contributing Guide

## Mark document status<a href="https://www.scichart.com/documentation/js/v4/contributing#mark-document-status" class="hash-link" aria-label="Direct link to Mark document status" translate="no" title="Direct link to Mark document status">â€‹</a>

This are documentations statuses

If no icon - document is up to date, most of the documents will have this status.

âš ï¸? - existing document needs update, this icon means that the document is not up to date and requires correction

ðŸ†• - for newly added items we can use this icon, to make it obvious. Or we can use text alternative `[NEW]`

ðŸ”„ - this icon is not used any more.

â­• - this icon is not used any more.

âœ… - this icon is not used any more.

## Use TypeScript<a href="https://www.scichart.com/documentation/js/v4/contributing#use-typescript" class="hash-link" aria-label="Direct link to Use TypeScript" translate="no" title="Direct link to Use TypeScript">â€‹</a>

Use TypeScript where it is possible.

Create a `demo.ts` file and run snippets compiler in watch mode ` npm run compileSnippets:watch` it will watch for changes and generate `demo.js` file.

In the documentation always reference the ts file, for example:

``` prism-code
ts showLineNumbers file=./PaletteProvider/demo.ts start=region_A_start end=region_A_end
```

If after changing `demo.ts` file the code snippet does not update, delete and insert it again.

## Create Search friendly titles<a href="https://www.scichart.com/documentation/js/v4/contributing#create-search-friendly-titles" class="hash-link" aria-label="Direct link to Create Search friendly titles" translate="no" title="Direct link to Create Search friendly titles">â€‹</a>

Create titles friendly for the site Search. The title h1, h2, h3 can be created using one, two or three hash symbols (#, \##, \###). The docusaurus uses these title for the search autocomplete. Therefore, give titles wisely to have a useful search. For example, in order to see PolarBandRenderableSeries in the search, I've created a h2 title with

``` prism-code
## Create PolarBandRenderableSeries
```

## Decorate TypeDoc links and reference v4<a href="https://www.scichart.com/documentation/js/v4/contributing#decorate-typedoc-links-and-reference-v4" class="hash-link" aria-label="Direct link to Decorate TypeDoc links and reference v4" translate="no" title="Direct link to Decorate TypeDoc links and reference v4">â€‹</a>

This is v4 TypeDoc - <a href="https://www.scichart.com/documentation/js/v4/typedoc" rel="noopener noreferrer" target="_blank">https://www.scichart.com/documentation/js/v4/typedoc</a>

In order to make all TypeDoc links distinct decorate the link with book icon `:blue_book:`, this is an example of SciChartSurface class typedoc link

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>

## Reference scichart.com/demo<a href="https://www.scichart.com/documentation/js/v4/contributing#reference-scichartcomdemo" class="hash-link" aria-label="Direct link to Reference scichart.com/demo" translate="no" title="Direct link to Reference scichart.com/demo">â€‹</a>

Use this link to reference scichart demo app - <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">https://www.scichart.com/demo/react</a>

## Do not reference DocumentX documents<a href="https://www.scichart.com/documentation/js/v4/contributing#do-not-reference-documentx-documents" class="hash-link" aria-label="Direct link to Do not reference DocumentX documents" translate="no" title="Direct link to Do not reference DocumentX documents">â€‹</a>

Reference pages within the docusaurus and make sure the links are not broken. The broken links show up in the build console. Do not reference old DocumentX documents.

## Use kebab-case notation for docs<a href="https://www.scichart.com/documentation/js/v4/contributing#use-kebab-case-notation-for-docs" class="hash-link" aria-label="Direct link to Use kebab-case notation for docs" translate="no" title="Direct link to Use kebab-case notation for docs">â€‹</a>

1.  It is recommended to **create a separate folder for each document** and to put `index.md` or `index.mdx` file inside. Having a separate folder is preferable because often documentation contains doc-snippets and it is nice to have them in the same folder.
2.  In order to have nice URLs it is recommended to create folder names in a **kebab-case notation** like `my-folder-name`.

## Use limited formatting styles<a href="https://www.scichart.com/documentation/js/v4/contributing#use-limited-formatting-styles" class="hash-link" aria-label="Direct link to Use limited formatting styles" translate="no" title="Direct link to Use limited formatting styles">â€‹</a>

Stick to the limited set of formatting styles.

TODO: add more formatting example

**Tip Example**

![](out_scichartv4/contributing/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Info about the properties and functions available can be found at the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" rel="noopener noreferrer" target="_blank">TypeDoc API Documentation for SciChartðŸ“˜</a>.

**Info Example**

![](out_scichartv4/contributing/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The default layers are defined in <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edefaultrenderlayer.html" rel="noopener noreferrer" target="_blank">EDefaultRenderLayerðŸ“˜</a>.

**Note Example**

![](out_scichartv4/contributing/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The order may differ depending on some configuration specifics.

**Warning**

![](out_scichartv4/contributing/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

**Error**: Could not load SciChart WebAssembly module. Check your build process and ensure that your scichart2d.wasm, scichart2d.data and scichart2d.js files are from the same version

**Quotation Example**

> For more information about Chart Modifier types in SciChart, head over to the [ChartModifier API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview) or see our <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">Examples</a>.

**Mermaid class diagram example**

**Insert Chart iFrame from scichart.com/demo**

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-uniform-heatmap-chart" target="_blank">Polar Uniform Heatmap Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

**Insert Live CodePen snippet**

``` prism-code
<LiveDocSnippet maxWidth={"100%"} name="./Basic/demo" htmlPath="./Basic/demo.html" cssPath="./Basic/demo.css" />
```

The variant with div element with id="result" useful to output something.

``` prism-code
<LiveDocSnippet maxWidth={"100%"} name="demo" htmlType="WithResult" />
```

600 px width

``` prism-code
<LiveDocSnippet name="demoGapDifferentStyle" />
```

**Code block**

```` prism-code
<CodeSnippetBlock labels={["TS", "Builder API (JSON Config)"]}>
    ```ts showLineNumbers file=./2d-charts/annotations-api/line-annotation/Basic/demo.ts start=region_A_start end=region_A_end

    ```
    ```ts showLineNumbers file=./2d-charts/annotations-api/line-annotation/Basic/demo.ts start=region_B_start end=region_B_end

    ```

</CodeSnippetBlock>
````

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/contributing.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/contributing/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
