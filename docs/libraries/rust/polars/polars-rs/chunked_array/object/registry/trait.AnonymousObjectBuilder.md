# Trait AnonymousObjectBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/registry.rs.html#47" class="src">Source</a>

``` rust
pub trait AnonymousObjectBuilder: ArrayBuilder {
    // Required methods
    fn as_array_builder(self: Box<Self>) -> Box<dyn ArrayBuilder>;
    unsafe fn from_chunks(
        self: Box<Self>,
        chunks: Vec<Box<dyn Array>>,
    ) -> Series;
    fn append_null(&mut self);
    fn append_value(&mut self, value: &(dyn Any + 'static));
    fn to_series(&mut self) -> Series;
    fn get_list_builder(
        &self,
        name: PlSmallStr,
        values_capacity: usize,
        list_capacity: usize,
    ) -> Box<dyn ListBuilderTrait>;

    // Provided method
    fn append_option(&mut self, value: Option<&(dyn Any + 'static)>) { ... }
}
```

Expand description

This trait can be registered, after which that global registration can be used to materialize object types

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.as_array_builder" class="fn">as_array_builder</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<Self\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/trait.ArrayBuilder.html" class="trait" title="trait polars_arrow::array::builder::ArrayBuilder">ArrayBuilder</a>\>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.from_chunks" class="fn">from_chunks</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<Self\>, chunks: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#safety" class="doc-anchor">§</a>Safety

Expect `ObjectArray<T>` arrays.

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

Append a `null` value.

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.append_value" class="fn">append_value</a>(&mut self, value: &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static))

Append a `T` of [`ObjectChunked<T>`](https://docs.rs/polars/latest/polars/prelude/type.ObjectChunked.html "type polars::prelude::ObjectChunked") made generic via the [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") trait.

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.to_series" class="fn">to_series</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take the current state and materialize as a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") the builder should not be used after that.

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#tymethod.get_list_builder" class="fn">get_list_builder</a>( &self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, values_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, list_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#method.append_option" class="fn">append_option</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)\>)

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html#impl-AnonymousObjectBuilder-for-ObjectChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html" class="trait" title="trait polars::chunked_array::object::registry::AnonymousObjectBuilder">AnonymousObjectBuilder</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/builder/struct.ObjectChunkedBuilder.html" class="struct" title="struct polars::chunked_array::object::builder::ObjectChunkedBuilder">ObjectChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,
