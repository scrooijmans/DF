# Constant DEFAULT_TTFB_BUCKETS Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#123-134" class="src">Source</a>

``` rust
pub const DEFAULT_TTFB_BUCKETS: &[f64];
```

Expand description

Buckets for time to first byte metrics like OperationTtfbSeconds Focuses on initial response times, which are typically shorter than full operations
