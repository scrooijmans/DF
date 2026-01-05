# Trait StringArrayType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#596" class="src">Source</a>

``` rust
pub trait StringArrayType<'a>: Sized + ArrayAccessor<Item = &'a str> {
    // Required methods
    fn is_ascii(&self) -> bool;
    fn iter(&self) -> ArrayIter<Self> ⓘ;
}
```

Expand description

A trait for Arrow String Arrays, currently three types are supported:

- `StringArray`
- `LargeStringArray`
- `StringViewArray`

This trait helps to abstract over the different types of string arrays so that we don’t need to duplicate the implementation for each type.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#tymethod.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if all data within this string array is ASCII

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#tymethod.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<Self\> <a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#" class="tooltip" data-notable-ty="ArrayIter&lt;Self&gt;">ⓘ</a>

Constructs a new iterator

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#impl-StringArrayType%3C&#39;a%3E-for-%26GenericByteViewArray%3CStringViewType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.StringArrayType.html" class="trait" title="trait arrow::array::StringArrayType">StringArrayType</a>\<'a\> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html#impl-StringArrayType%3C&#39;a%3E-for-%26GenericByteArray%3CGenericStringType%3CO%3E%3E" class="anchor">§</a>

### impl\<'a, O\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.StringArrayType.html" class="trait" title="trait arrow::array::StringArrayType">StringArrayType</a>\<'a\> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,
