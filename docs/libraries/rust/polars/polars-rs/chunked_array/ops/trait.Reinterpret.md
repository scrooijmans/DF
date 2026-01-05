# Trait Reinterpret Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#55" class="src">Source</a>

``` rust
pub trait Reinterpret {
    // Provided methods
    fn reinterpret_signed(&self) -> Series { ... }
    fn reinterpret_unsigned(&self) -> Series { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#method.reinterpret_signed" class="fn">reinterpret_signed</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#method.reinterpret_unsigned" class="fn">reinterpret_unsigned</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CFloat32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CFloat64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CInt8Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CInt16Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CUInt8Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CUInt16Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CUInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html#impl-Reinterpret-for-ChunkedArray%3CUInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Reinterpret.html" class="trait" title="trait polars::prelude::Reinterpret">Reinterpret</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>
