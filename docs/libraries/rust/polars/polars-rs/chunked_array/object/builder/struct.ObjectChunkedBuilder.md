# Struct ObjectChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/builder.rs.html#8" class="src">Source</a>

``` rust
pub struct ObjectChunkedBuilder<T> { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#impl-ObjectChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.field" class="fn">field</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, v: T)

Appends a value of type `T` into the builder

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_value_from_any" class="fn">append_value_from_any</a>( &mut self, v: &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_option" class="fn">append_option</a>(&mut self, opt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#impl-AnonymousObjectBuilder-for-ObjectChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html" class="trait" title="trait polars::chunked_array::object::registry::AnonymousObjectBuilder">AnonymousObjectBuilder</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.from_chunks" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.from_chunks" class="fn">from_chunks</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>\>, chunks: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#safety" class="doc-anchor">§</a>Safety

Expects `ObjectArray<T>` arrays.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.as_array_builder" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.as_array_builder" class="fn">as_array_builder</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html" class="trait" title="trait polars_arrow::array::builder::ArrayBuilder">ArrayBuilder</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_null-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

Append a `null` value.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.append_value" class="fn">append_value</a>(&mut self, value: &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static))

Append a `T` of [`ObjectChunked<T>`](https://docs.rs/polars/latest/polars/prelude/type.ObjectChunked.html "type polars::prelude::ObjectChunked") made generic via the [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.to_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.to_series" class="fn">to_series</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take the current state and materialize as a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") the builder should not be used after that.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.get_list_builder" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.get_list_builder" class="fn">get_list_builder</a>( &self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, values_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, list_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.append_option-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#method.append_option" class="fn">append_option</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)\>)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#impl-ArrayBuilder-for-ObjectChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html" class="trait" title="trait polars_arrow::array::builder::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.reserve" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.freeze" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.freeze" class="fn">freeze</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Consume this builder returning the built array.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.freeze_reset" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.freeze_reset" class="fn">freeze_reset</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Return the built array and reset to an empty state.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of this builder (so far).

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.extend_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.extend_nulls" class="fn">extend_nulls</a>(&mut self, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Extend this builder with the given number of null elements.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.subslice_extend" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.subslice_extend" class="fn">subslice_extend</a>( &mut self, other: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static), start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given array subslice. May panic if other does not match the dtype of this array.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.subslice_extend_repeated" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.subslice_extend_repeated" class="fn">subslice_extend_repeated</a>( &mut self, other: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static), start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repeats: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

The same as subslice_extend, but repeats the extension `repeats` times.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.subslice_extend_each_repeated" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.subslice_extend_each_repeated" class="fn">subslice_extend_each_repeated</a>( &mut self, other: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static), start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repeats: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

The same as subslice_extend, but repeats each element `repeats` times.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.gather_extend" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.gather_extend" class="fn">gather_extend</a>( &mut self, other: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static), idxs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], \_share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given array at the given indices. That is, `other[idxs[i]]` is appended to this array in order, for each i=0..idxs.len(). May panic if other does not match the dtype of this array. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.gather_extend)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.opt_gather_extend" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#tymethod.opt_gather_extend" class="fn">opt_gather_extend</a>( &mut self, other: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static), idxs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], \_share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given array at the given indices. That is, `other[idxs[i]]` is appended to this array in order, for each i=0..idxs.len(). May panic if other does not match the dtype of this array. Out-of-bounds indices are mapped to nulls.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html#method.extend" class="fn">extend</a>(&mut self, other: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static), share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>)

Extends this builder with the contents of the given array. May panic if other does not match the dtype of this array.

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#impl-Default-for-ObjectChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
