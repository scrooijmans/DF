# Trait ArrowNumericType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/numeric.rs.html#21" class="src">Source</a>

``` rust
pub trait ArrowNumericType: ArrowPrimitiveType { }
```

Expand description

A subtype of primitive type that represents numeric values.

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNumericType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNumericType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNumericType.html#impl-ArrowNumericType-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNumericType.html" class="trait" title="trait arrow::array::ArrowNumericType">ArrowNumericType</a> for T

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,
