<img src="out_scichartv4/typedoc/interfaces/ibasedstackedrenderableseriesoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IBasedStackedRenderableSeriesOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html)

# Interface IBasedStackedRenderableSeriesOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options to pass to the {@link BasedStackedRenderableSeries} constructor

### Hierarchy

- IBasedStackedRenderableSeriesOptions
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istackedcolumnrenderableseriesoptions.html" class="tsd-signature-type">IStackedColumnRenderableSeriesOptions</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasestackedmountainrenderableseriesoptions.html" class="tsd-signature-type">IBaseStackedMountainRenderableSeriesOptions</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolarstackedcolumnrenderableseriesoptions.html" class="tsd-signature-type">IPolarStackedColumnRenderableSeriesOptions</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#animation" class="tsd-kind-icon">animation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#datalabelprovider" class="tsd-kind-icon">dataLabelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#onhoveredchanged" class="tsd-kind-icon">onHoveredChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#onisvisiblechanged" class="tsd-kind-icon">onIsVisibleChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#onselectedchanged" class="tsd-kind-icon">onSelectedChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#opacity" class="tsd-kind-icon">opacity</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedstackedrenderableseriesoptions.html#yarrayfilter" class="tsd-kind-icon">yArrayFilter</a>

## Properties

### Optional animation

animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tanimationdefinition" class="tsd-signature-type">TAnimationDefinition</a>

An animation that runs on the start, child class to [SeriesAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html)

### Optional dataLabelProvider

dataLabelProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelproviderdefinition" class="tsd-signature-type">TDataLabelProviderDefinition</a>

A [DataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#datalabelprovider) used for creating and drawing per-point text.

### Optional dataSeries

dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html" class="tsd-signature-type">IDataSeries</a>

The [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html) which provides a datasource for this [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) to draw

### Optional id

id: string

A unique Id for the [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

### Optional onHoveredChanged

onHoveredChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tserieshoverchangedcallback" class="tsd-signature-type">TSeriesHoverChangedCallback</a> \| string

Optional callback function when hovered changed. Also see [IRenderableSeries.hovered](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html#hovered) event handler

### Optional onIsVisibleChanged

onIsVisibleChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesvisiblechangedcallback" class="tsd-signature-type">TSeriesVisibleChangedCallback</a> \| string

Optional callback function when isVisible changed. Also see [IRenderableSeries.isVisibleChanged](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html#isvisiblechanged) event handler

### Optional onSelectedChanged

onSelectedChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tseriesselectionchangedcallback" class="tsd-signature-type">TSeriesSelectionChangedCallback</a> \| string

Optional callback function when selected changed. Also see [IRenderableSeries.selected](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html#selected) event handler

### Optional opacity

opacity: number

An Opacity factor of the Series that controls its semi-transparency level, where value 1 means the Series is opaque; 0 - transparent.

### Optional seriesName

seriesName: string

Series name

### Optional yArrayFilter

yArrayFilter: number \| string

When using XyNDataSeries, set this to determine which single set of y values to use

## Legend

- Constructor
- Property
- Method
- Accessor

<!-- -->

- Inherited constructor
- Inherited property
- Inherited method
- Inherited accessor

<!-- -->

- Property
- Method

<!-- -->

- Protected property
- Protected method

<!-- -->

- Static property
- Static method

Generated using <a href="https://typedoc.org/" target="_blank">TypeDoc</a>
