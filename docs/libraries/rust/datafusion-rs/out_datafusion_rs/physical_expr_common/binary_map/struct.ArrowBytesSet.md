# Struct ArrowBytesSet Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/binary_map.rs.html#54" class="src">Source</a>

``` rust
pub struct ArrowBytesSet<O>(/* private fields */)
where
    O: OffsetSizeTrait;
```

Expand description

HashSet optimized for storing string or binary values that can produce that the final set as a GenericStringArray with minimal copies.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#impl-ArrowBytesSet%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_map::ArrowBytesSet">ArrowBytesSet</a>\<O\>

where O: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.new" class="fn">new</a>(output_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/enum.OutputType.html" class="enum" title="enum datafusion::physical_expr_common::binary_map::OutputType">OutputType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_map::ArrowBytesSet">ArrowBytesSet</a>\<O\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.take" class="fn">take</a>(&mut self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_map::ArrowBytesSet">ArrowBytesSet</a>\<O\>

Return the contents of this set and replace it with a new empty set with the same output type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.insert" class="fn">insert</a>(&mut self, values: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)

Inserts each value from `values` into the set

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.into_state" class="fn">into_state</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Converts this set into a `StringArray`/`LargeStringArray` or `BinaryArray`/`LargeBinaryArray` containing each distinct value that was interned. This is done without copying the values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of distinct values (including nulls) seen so far

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.non_null_len" class="fn">non_null_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

returns the total number of distinct values (not including nulls) seen so far

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the total size, in bytes, of memory used to store the data in this set, not including `self`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#impl-Debug-for-ArrowBytesSet%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_map::ArrowBytesSet">ArrowBytesSet</a>\<O\>

where O: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html#blanket-implementations" class="anchor">§</a>
