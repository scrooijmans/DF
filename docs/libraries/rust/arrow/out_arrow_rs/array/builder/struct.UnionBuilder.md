# Struct UnionBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/union_builder.rs.html#148" class="src">Source</a>

``` rust
pub struct UnionBuilder { /* private fields */ }
```

Expand description

Builder for [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray")

Example: **Dense Memory Layout**

``` rust

let mut builder = UnionBuilder::new_dense();
builder.append::<Int32Type>("a", 1).unwrap();
builder.append::<Float64Type>("b", 3.0).unwrap();
builder.append::<Int32Type>("a", 4).unwrap();
let union = builder.build().unwrap();

assert_eq!(union.type_id(0), 0);
assert_eq!(union.type_id(1), 1);
assert_eq!(union.type_id(2), 0);

assert_eq!(union.value_offset(0), 0);
assert_eq!(union.value_offset(1), 0);
assert_eq!(union.value_offset(2), 1);
```

Example: **Sparse Memory Layout**

``` rust

let mut builder = UnionBuilder::new_sparse();
builder.append::<Int32Type>("a", 1).unwrap();
builder.append::<Float64Type>("b", 3.0).unwrap();
builder.append::<Int32Type>("a", 4).unwrap();
let union = builder.build().unwrap();

assert_eq!(union.type_id(0), 0);
assert_eq!(union.type_id(1), 1);
assert_eq!(union.type_id(2), 0);

assert_eq!(union.value_offset(0), 0);
assert_eq!(union.value_offset(1), 1);
assert_eq!(union.value_offset(2), 2);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#impl-UnionBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.new_dense" class="fn">new_dense</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

Creates a new dense array builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.new_sparse" class="fn">new_sparse</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

Creates a new sparse array builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.with_capacity_dense" class="fn">with_capacity_dense</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

Creates a new dense array builder with capacity.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.with_capacity_sparse" class="fn">with_capacity_sparse</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

Creates a new sparse array builder with capacity.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.append_null" class="fn">append_null</a>\<T\>(&mut self, type_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Appends a null to this builder, encoding the null in the array of the `type_name` child / field.

Since `UnionArray` encodes nulls as an entry in its children (it doesn’t have a validity bitmap itself), and where the null is part of the final array, appending a NULL requires specifying which field (child) to use.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.append" class="fn">append</a>\<T\>( &mut self, type_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, v: \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Appends a value to this builder.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Builds this builder creating a new `UnionArray`.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#impl-ArrayBuilder-for-UnionBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.finish_cloned" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the underlying builder

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#impl-Debug-for-UnionBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#impl-Default-for-UnionBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.UnionBuilder.html#blanket-implementations" class="anchor">§</a>
