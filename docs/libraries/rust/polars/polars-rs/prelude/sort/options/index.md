# Module options Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/sort/mod.rs.html#6" class="src">Source</a>

## Structs<a href="https://docs.rs/polars/latest/polars/prelude/sort/options/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/sort/options/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::sort::options::SortMultipleOptions">SortMultipleOptions</a>  
Sort options for multi-series sorting.

<a href="https://docs.rs/polars/latest/polars/prelude/sort/options/struct.SortOptions.html" class="struct" title="struct polars::prelude::sort::options::SortOptions">SortOptions</a>  
Options for single series sorting.

## Traits<a href="https://docs.rs/polars/latest/polars/prelude/sort/options/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/sort/options/trait.SlicedArray.html" class="trait" title="trait polars::prelude::sort::options::SlicedArray">SlicedArray</a>  
Utility trait to slice concrete arrow arrays whilst keeping their concrete type. E.g. don’t return `Box<dyn Array>`.
