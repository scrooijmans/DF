# Trait RunEndIndexType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#290" class="src">Source</a>

``` rust
pub trait RunEndIndexType: ArrowPrimitiveType { }
```

Expand description

A subtype of primitive type that is used as run-ends index in `RunArray`. See <https://arrow.apache.org/docs/format/Columnar.html>

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.RunEndIndexType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.RunEndIndexType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.RunEndIndexType.html#impl-RunEndIndexType-for-Int16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.RunEndIndexType.html#impl-RunEndIndexType-for-Int32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.RunEndIndexType.html#impl-RunEndIndexType-for-Int64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>
