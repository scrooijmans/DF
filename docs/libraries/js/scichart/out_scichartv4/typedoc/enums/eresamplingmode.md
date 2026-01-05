<img src="out_scichartv4/typedoc/enums/eresamplingmode_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [EResamplingMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html)

# Enumeration EResamplingMode

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

## Index

### Enumeration members

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#auto" class="tsd-kind-icon">Auto</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#max" class="tsd-kind-icon">Max</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#mid" class="tsd-kind-icon">Mid</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#min" class="tsd-kind-icon">Min</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#minmax" class="tsd-kind-icon">MinMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#minmaxwithunevenspacing" class="tsd-kind-icon">MinMaxWithUnevenSpacing</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#minormax" class="tsd-kind-icon">MinOrMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eresamplingmode.html#none" class="tsd-kind-icon">None</a>

## Enumeration members

### Auto

Auto: = "Auto"

Auto-detect the most suitable resampling algorithm (Fastest, plus most accurate) for the type of data appended

### Max

Max: = "Max"

Assumes Evenly-spaced data (TimeSeries). Resample by taking the maximum point of oversampled data

### Mid

Mid: = "Mid"

Assumes Evenly-spaced data (TimeSeries). Resample by taking the median point of oversampled data

### Min

Min: = "Min"

Assumes Evenly-spaced data (TimeSeries). Resample by taking the minimum point of oversampled data

### MinMax

MinMax: = "MinMax"

Assumes Evenly-spaced data (TimeSeries). Resample by taking the min-max of oversampled data. This results in the most visually accurate resampling, with the most performan

### MinMaxWithUnevenSpacing

MinMaxWithUnevenSpacing: = "MinMaxWithUnevenSpacing"

Does not assume Evenly-spaced data (TimeSeries). Resample by taking the min-max of oversampled data. This results in the most visually accurate resampling, with the most performant rendering

### MinOrMax

MinOrMax: = "MinOrMax"

EXPERIMENTAL! Assumes Evenly-spaced data (TimeSeries). Resample by taking the Min, or the Max point of oversampled data. Outputs Min and Max when points in the resampling bucket span zero

### None

None: = "None"

Do not use resampling when redrawing a series

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
