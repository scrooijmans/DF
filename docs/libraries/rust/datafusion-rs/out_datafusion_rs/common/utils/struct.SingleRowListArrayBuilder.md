# Struct SingleRowListArrayBuilder Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/utils/mod.rs.html#358" class="src">Source</a>

``` rust
pub struct SingleRowListArrayBuilder { /* private fields */ }
```

Expand description

Creates single element [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray"), [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray") and [`FixedSizeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html "struct datafusion::common::arrow::array::FixedSizeListArray") from other arrays

For example this builder can convert `[1, 2, 3]` into `[[1, 2, 3]]`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
// Array is [1, 2, 3]
let arr = ListArray::from_iter_primitive::<Int64Type, _, _>(vec![
      Some(vec![Some(1), Some(2), Some(3)]),
]);
// Wrap as a list array: [[1, 2, 3]]
let list_arr = SingleRowListArrayBuilder::new(Arc::new(arr)).build_list_array();
assert_eq!(list_arr.len(), 1);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#impl-SingleRowListArrayBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.new" class="fn">new</a>(arr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

Create a new instance of [`SingleRowListArrayBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html "struct datafusion::common::utils::SingleRowListArrayBuilder")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.with_nullable" class="fn">with_nullable</a>(self, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

Set the nullable flag

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.with_field_name" class="fn">with_field_name</a>( self, field_name: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

sets the field name for the resulting array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.with_field" class="fn">with_field</a>(self, field: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

Copies field name and nullable from the specified field

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.build_list_array" class="fn">build_list_array</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Build a single element [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.build_list_scalar" class="fn">build_list_scalar</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Build a single element [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray") and wrap as [`ScalarValue::List`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.List "variant datafusion::scalar::ScalarValue::List")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.build_large_list_array" class="fn">build_large_list_array</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Build a single element [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.build_large_list_scalar" class="fn">build_large_list_scalar</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Build a single element [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray") and wrap as [`ScalarValue::LargeList`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.LargeList "variant datafusion::scalar::ScalarValue::LargeList")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.build_fixed_size_list_array" class="fn">build_fixed_size_list_array</a>(self, list_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct datafusion::common::arrow::array::FixedSizeListArray">FixedSizeListArray</a>

Build a single element [`FixedSizeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html "struct datafusion::common::arrow::array::FixedSizeListArray")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.build_fixed_size_list_scalar" class="fn">build_fixed_size_list_scalar</a>(self, list_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Build a single element [`FixedSizeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html "struct datafusion::common::arrow::array::FixedSizeListArray") and wrap as [`ScalarValue::FixedSizeList`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.FixedSizeList "variant datafusion::scalar::ScalarValue::FixedSizeList")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#impl-Clone-for-SingleRowListArrayBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#impl-Debug-for-SingleRowListArrayBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html#blanket-implementations" class="anchor">§</a>
