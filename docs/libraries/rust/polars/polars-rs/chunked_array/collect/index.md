# Module collect Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/mod.rs.html#18" class="src">Source</a>

Expand description

Methods for collecting into a ChunkedArray.

For types that don’t have dtype parameters: iter.(try\_)collect_ca(\_trusted) (name)

For all types: iter.(try\_)collect_ca(\_trusted)*like (other_df) Copies name/dtype from other_df iter.(try*)collect_ca(\_trusted)\_with_dtype (name, df)

The try variants work on iterators of Results, the trusted variants do not check the length of the iterator.

## Traits<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html" class="trait" title="trait polars::chunked_array::collect::ChunkedCollectInferIterExt">ChunkedCollectInferIterExt</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html" class="trait" title="trait polars::chunked_array::collect::ChunkedCollectIterExt">ChunkedCollectIterExt</a>
