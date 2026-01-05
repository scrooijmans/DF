# Constant DEFAULT_BYTES_RATE_BUCKETS Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#59-75" class="src">Source</a>

``` rust
pub const DEFAULT_BYTES_RATE_BUCKETS: &[f64];
```

Expand description

Buckets for data transfer rate metrics like OperationBytesRate

Covers various network speeds from slow connections to high-speed transfers

Note: this is for single operation rate, not for total bandwidth.
