# Trait BinaryArrayType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#630" class="src">Source</a>

``` rust
pub trait BinaryArrayType<'a>: Sized + ArrayAccessor<Item = &'a [u8]> {
    // Required method
    fn iter(&self) -> ArrayIter<Self> ⓘ;
}
```

Expand description

A trait for Arrow String Arrays, currently three types are supported:

- `BinaryArray`
- `LargeBinaryArray`
- `BinaryViewArray`

This trait helps to abstract over the different types of binary arrays so that we don’t need to duplicate the implementation for each type.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#tymethod.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<Self\> <a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#" class="tooltip" data-notable-ty="ArrayIter&lt;Self&gt;">ⓘ</a>

Constructs a new iterator

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#impl-BinaryArrayType%3C&#39;a%3E-for-%26GenericByteViewArray%3CBinaryViewType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.BinaryArrayType.html" class="trait" title="trait arrow::array::BinaryArrayType">BinaryArrayType</a>\<'a\> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html#impl-BinaryArrayType%3C&#39;a%3E-for-%26GenericByteArray%3CGenericBinaryType%3CO%3E%3E" class="anchor">§</a>

### impl\<'a, O\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.BinaryArrayType.html" class="trait" title="trait arrow::array::BinaryArrayType">BinaryArrayType</a>\<'a\> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,
