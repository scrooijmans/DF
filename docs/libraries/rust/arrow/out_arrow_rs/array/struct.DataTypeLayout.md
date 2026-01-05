# Struct DataTypeLayout Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/data.rs.html#1677" class="src">Source</a>

``` rust
pub struct DataTypeLayout {
    pub buffers: Vec<BufferSpec>,
    pub can_contain_null_mask: bool,
    pub variadic: bool,
}
```

Expand description

Layout specification for a data type

## Fields<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#structfield.buffers" class="anchor field">§</a>`buffers: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec"><code>BufferSpec</code></a>`>`

A vector of buffer layout specifications, one for each expected buffer

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#structfield.can_contain_null_mask" class="anchor field">§</a>`can_contain_null_mask: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Can contain a null bitmask

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#structfield.variadic" class="anchor field">§</a>`variadic: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

This field only applies to the view type [`DataType::BinaryView`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.BinaryView "variant arrow::datatypes::DataType::BinaryView") and [`DataType::Utf8View`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Utf8View "variant arrow::datatypes::DataType::Utf8View") If `variadic` is true, the number of buffers expected is only lower-bounded by buffers.len(). Buffers that exceed the lower bound are legal.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#impl-DataTypeLayout" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.new_fixed_width" class="fn">new_fixed_width</a>\<T\>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

Describes a basic numeric array where each element has type `T`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.new_nullable_empty" class="fn">new_nullable_empty</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

Describes arrays which have no data of their own but may still have a Null Bitmap (e.g. FixedSizeList)

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.new_empty" class="fn">new_empty</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

Describes arrays which have no data of their own (e.g. RunEndEncoded).

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.new_binary" class="fn">new_binary</a>\<T\>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

Describes a basic numeric array where each element has a fixed with offset buffer of type `T`, followed by a variable width data buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.new_view" class="fn">new_view</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

Describes a view type

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.new_list_view" class="fn">new_list_view</a>\<T\>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

Describes a list view type

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#impl-Debug-for-DataTypeLayout" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#impl-PartialEq-for-DataTypeLayout" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#impl-Eq-for-DataTypeLayout" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#impl-StructuralPartialEq-for-DataTypeLayout" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html#blanket-implementations" class="anchor">§</a>
