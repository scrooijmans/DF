# Enum Buffer Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/buffer.rs.html#589" class="src">Source</a>

``` rust
pub enum Buffer {
Show 20 variants    Boolean(BooleanChunkedBuilder),
    Int8(PrimitiveChunkedBuilder<Int8Type>),
    Int16(PrimitiveChunkedBuilder<Int16Type>),
    Int32(PrimitiveChunkedBuilder<Int32Type>),
    Int64(PrimitiveChunkedBuilder<Int64Type>),
    Int128(PrimitiveChunkedBuilder<Int128Type>),
    UInt8(PrimitiveChunkedBuilder<UInt8Type>),
    UInt16(PrimitiveChunkedBuilder<UInt16Type>),
    UInt32(PrimitiveChunkedBuilder<UInt32Type>),
    UInt64(PrimitiveChunkedBuilder<UInt64Type>),
    Float32(PrimitiveChunkedBuilder<Float32Type>),
    Float64(PrimitiveChunkedBuilder<Float64Type>),
    Utf8(Utf8Field),
    Datetime {
        buf: DatetimeField<Int64Type>,
        time_unit: TimeUnit,
        time_zone: Option<TimeZone>,
    },
    Date(DatetimeField<Int32Type>),
    Categorical8(CategoricalField<Categorical8Type>),
    Categorical16(CategoricalField<Categorical16Type>),
    Categorical32(CategoricalField<Categorical32Type>),
    DecimalFloat32(PrimitiveChunkedBuilder<Float32Type>, Vec<u8>),
    DecimalFloat64(PrimitiveChunkedBuilder<Float64Type>, Vec<u8>),
}
```

Available on **crate feature `polars-io`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Boolean" class="anchor">§</a>

### Boolean(<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanChunkedBuilder.html" class="struct" title="struct polars::prelude::BooleanChunkedBuilder">BooleanChunkedBuilder</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Int8" class="anchor">§</a>

### Int8(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Int16" class="anchor">§</a>

### Int16(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Int32" class="anchor">§</a>

### Int32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Int64" class="anchor">§</a>

### Int64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Int128" class="anchor">§</a>

### Int128(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.UInt8" class="anchor">§</a>

### UInt8(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.UInt16" class="anchor">§</a>

### UInt16(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.UInt32" class="anchor">§</a>

### UInt32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.UInt64" class="anchor">§</a>

### UInt64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Float32" class="anchor">§</a>

### Float32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Float64" class="anchor">§</a>

### Float64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Utf8" class="anchor">§</a>

### Utf8(<a href="https://docs.rs/polars/latest/polars/prelude/buffer/struct.Utf8Field.html" class="struct" title="struct polars::prelude::buffer::Utf8Field">Utf8Field</a>)

Stores the Utf8 fields and the total string length seen for that column

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Datetime" class="anchor">§</a>

### Datetime

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Datetime.field.buf" class="anchor field">§</a>`buf: `<a href="https://docs.rs/polars/latest/polars/prelude/buffer/struct.DatetimeField.html" class="struct" title="struct polars::prelude::buffer::DatetimeField"><code>DatetimeField</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type"><code>Int64Type</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Datetime.field.time_unit" class="anchor field">§</a>`time_unit: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit"><code>TimeUnit</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Datetime.field.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone"><code>TimeZone</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Date" class="anchor">§</a>

### Date(<a href="https://docs.rs/polars/latest/polars/prelude/buffer/struct.DatetimeField.html" class="struct" title="struct polars::prelude::buffer::DatetimeField">DatetimeField</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Categorical8" class="anchor">§</a>

### Categorical8(<a href="https://docs.rs/polars/latest/polars/prelude/buffer/struct.CategoricalField.html" class="struct" title="struct polars::prelude::buffer::CategoricalField">CategoricalField</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Categorical16" class="anchor">§</a>

### Categorical16(<a href="https://docs.rs/polars/latest/polars/prelude/buffer/struct.CategoricalField.html" class="struct" title="struct polars::prelude::buffer::CategoricalField">CategoricalField</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.Categorical32" class="anchor">§</a>

### Categorical32(<a href="https://docs.rs/polars/latest/polars/prelude/buffer/struct.CategoricalField.html" class="struct" title="struct polars::prelude::buffer::CategoricalField">CategoricalField</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.DecimalFloat32" class="anchor">§</a>

### DecimalFloat32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#variant.DecimalFloat64" class="anchor">§</a>

### DecimalFloat64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#impl-Buffer" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html" class="enum" title="enum polars::prelude::buffer::Buffer">Buffer</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#method.into_series" class="fn">into_series</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#method.add_null" class="fn">add_null</a>(&mut self, valid: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#method.dtype" class="fn">dtype</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#method.add" class="fn">add</a>( &mut self, bytes: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ignore_errors: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, needs_escaping: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, missing_is_null: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/buffer/enum.Buffer.html#blanket-implementations" class="anchor">§</a>
