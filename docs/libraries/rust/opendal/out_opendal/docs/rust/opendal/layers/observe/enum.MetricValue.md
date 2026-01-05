# Enum MetricValue Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#207-259" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum MetricValue {
Show 17 variants    OperationBytes(u64),
    OperationBytesRate(f64),
    OperationEntries(u64),
    OperationEntriesRate(f64),
    OperationDurationSeconds(Duration),
    OperationErrorsTotal,
    OperationExecuting(isize),
    OperationTtfbSeconds(Duration),
    HttpExecuting(isize),
    HttpRequestBytes(u64),
    HttpRequestBytesRate(f64),
    HttpRequestDurationSeconds(Duration),
    HttpResponseBytes(u64),
    HttpResponseBytesRate(f64),
    HttpResponseDurationSeconds(Duration),
    HttpConnectionErrorsTotal,
    HttpStatusErrorsTotal,
}
```

Expand description

MetricValue is the value the opendal sends to the metrics impls.

Metrics impls can be `prometheus_client`, `metrics` etc.

Every metrics impls SHOULD implement observe over the MetricValue to make sure they provide the consistent metrics for users.

## Variants (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variants" class="anchor">Â§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationBytes" class="anchor">Â§</a>

### OperationBytes(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Record the size of data processed in bytes. Metrics impl: Update a Histogram with the given byte count.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationBytesRate" class="anchor">Â§</a>

### OperationBytesRate(<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>)

Record the rate of data processing in bytes/second. Metrics impl: Update a Histogram with the calculated rate value.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationEntries" class="anchor">Â§</a>

### OperationEntries(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Record the number of entries (files, objects, keys) processed. Metrics impl: Update a Histogram with the entry count.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationEntriesRate" class="anchor">Â§</a>

### OperationEntriesRate(<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>)

Record the rate of entries processing in entries/second. Metrics impl: Update a Histogram with the calculated rate value.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationDurationSeconds" class="anchor">Â§</a>

### OperationDurationSeconds(<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Record the total duration of an operation. Metrics impl: Update a Histogram with the duration converted to seconds (as f64).

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationErrorsTotal" class="anchor">Â§</a>

### OperationErrorsTotal

Increment the counter for operation errors. Metrics impl: Increment a Counter by 1.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationExecuting" class="anchor">Â§</a>

### OperationExecuting(<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>)

Update the current number of executing operations. Metrics impl: Add the value (positive or negative) to a Gauge.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.OperationTtfbSeconds" class="anchor">Â§</a>

### OperationTtfbSeconds(<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Record the time to first byte duration. Metrics impl: Update a Histogram with the duration converted to seconds (as f64).

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpExecuting" class="anchor">Â§</a>

### HttpExecuting(<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>)

Update the current number of executing HTTP requests. Metrics impl: Add the value (positive or negative) to a Gauge.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpRequestBytes" class="anchor">Â§</a>

### HttpRequestBytes(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Record the size of HTTP request body in bytes. Metrics impl: Update a Histogram with the given byte count.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpRequestBytesRate" class="anchor">Â§</a>

### HttpRequestBytesRate(<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>)

Record the rate of HTTP request data in bytes/second. Metrics impl: Update a Histogram with the calculated rate value.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpRequestDurationSeconds" class="anchor">Â§</a>

### HttpRequestDurationSeconds(<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Record the duration of sending an HTTP request (until first byte received). Metrics impl: Update a Histogram with the duration converted to seconds (as f64).

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpResponseBytes" class="anchor">Â§</a>

### HttpResponseBytes(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Record the size of HTTP response body in bytes. Metrics impl: Update a Histogram with the given byte count.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpResponseBytesRate" class="anchor">Â§</a>

### HttpResponseBytesRate(<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>)

Record the rate of HTTP response data in bytes/second. Metrics impl: Update a Histogram with the calculated rate value.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpResponseDurationSeconds" class="anchor">Â§</a>

### HttpResponseDurationSeconds(<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Record the duration of receiving an HTTP response (from first byte to last). Metrics impl: Update a Histogram with the duration converted to seconds (as f64).

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpConnectionErrorsTotal" class="anchor">Â§</a>

### HttpConnectionErrorsTotal

Increment the counter for HTTP connection errors. Metrics impl: Increment a Counter by 1.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#variant.HttpStatusErrorsTotal" class="anchor">Â§</a>

### HttpStatusErrorsTotal

Increment the counter for HTTP status errors (non-2xx responses). Metrics impl: Increment a Counter by 1.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#impl-MetricValue" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#method.name" class="fn">name</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the full metric name for this metric value.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#method.name_with_unit" class="fn">name_with_unit</a>(&self) -\> (&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>)

Returns the metric name along with unit for this metric value.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#notes" class="doc-anchor">Â§</a>Notes

This API is designed for the metrics impls that unit aware. They will handle the names by themselves like append `_total` for counters.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#method.help" class="fn">help</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the help text for this metric value.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#impl-Clone-for-MetricValue" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#impl-Debug-for-MetricValue" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#impl-Copy-for-MetricValue" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html#blanket-implementations" class="anchor">Â§</a>
