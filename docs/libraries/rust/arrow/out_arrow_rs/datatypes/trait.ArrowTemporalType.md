# Trait ArrowTemporalType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#299" class="src">Source</a>

``` rust
pub trait ArrowTemporalType: ArrowPrimitiveType { }
```

Expand description

A subtype of primitive type that represents temporal values.

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-Date32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-DurationMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-DurationNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-DurationSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-Time32MillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-Time32SecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-Time64MicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-TimestampMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html#impl-ArrowTemporalType-for-TimestampSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>
