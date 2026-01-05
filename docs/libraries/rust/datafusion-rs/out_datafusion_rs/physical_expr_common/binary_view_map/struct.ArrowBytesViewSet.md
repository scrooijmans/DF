# Struct ArrowBytesViewSet Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/binary_view_map.rs.html#36" class="src">Source</a>

``` rust
pub struct ArrowBytesViewSet(/* private fields */);
```

Expand description

HashSet optimized for storing string or binary values that can produce that the final set as a `GenericBinaryViewArray` with minimal copies.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#impl-ArrowBytesViewSet" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet">ArrowBytesViewSet</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.new" class="fn">new</a>(output_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/enum.OutputType.html" class="enum" title="enum datafusion::physical_expr_common::binary_map::OutputType">OutputType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet">ArrowBytesViewSet</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.insert" class="fn">insert</a>(&mut self, values: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)

Inserts each value from `values` into the set

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.take" class="fn">take</a>(&mut self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet">ArrowBytesViewSet</a>

Return the contents of this map and replace it with a new empty map with the same output type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.into_state" class="fn">into_state</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Converts this set into a `StringViewArray` or `BinaryViewArray` containing each distinct value that was interned. This is done without copying the values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of distinct values (including nulls) seen so far

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.non_null_len" class="fn">non_null_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

returns the total number of distinct values (not including nulls) seen so far

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the total size, in bytes, of memory used to store the data in this set, not including `self`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#impl-Debug-for-ArrowBytesViewSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet">ArrowBytesViewSet</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html#blanket-implementations" class="anchor">§</a>
