# Trait ListBuilderTrait Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/list/mod.rs.html#19" class="src">Source</a>

``` rust
pub trait ListBuilderTrait {
    // Required methods
    fn append_series(&mut self, s: &Series) -> Result<(), PolarsError>;
    fn append_null(&mut self);

    // Provided methods
    fn append_opt_series(
        &mut self,
        opt_s: Option<&Series>,
    ) -> Result<(), PolarsError> { ... }
    fn field(&self) -> &Field { ... }
    fn inner_array(&mut self) -> Box<dyn Array> { ... }
    fn fast_explode(&self) -> bool { ... }
    fn finish(&mut self) -> ChunkedArray<ListType> { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#tymethod.append_series" class="fn">append_series</a>(&mut self, s: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

## Provided Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.append_opt_series" class="fn">append_opt_series</a>( &mut self, opt_s: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.field" class="fn">field</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.inner_array" class="fn">inner_array</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.fast_explode" class="fn">fast_explode</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-Box%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<S\>

where S: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.append_opt_series-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.append_opt_series" class="fn">append_opt_series</a>( &mut self, opt_s: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.append_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#tymethod.append_series" class="fn">append_series</a>(&mut self, s: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.append_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-ListBinaryChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListBinaryChunkedBuilder.html" class="struct" title="struct polars::prelude::ListBinaryChunkedBuilder">ListBinaryChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-ListBooleanChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListBooleanChunkedBuilder.html" class="struct" title="struct polars::prelude::ListBooleanChunkedBuilder">ListBooleanChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-ListStringChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListStringChunkedBuilder.html" class="struct" title="struct polars::prelude::ListStringChunkedBuilder">ListStringChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-AnonymousOwnedListBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.AnonymousOwnedListBuilder.html" class="struct" title="struct polars::chunked_array::builder::AnonymousOwnedListBuilder">AnonymousOwnedListBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-ListNullChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListNullChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::ListNullChunkedBuilder">ListNullChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/trait.ListBuilderTrait.html#impl-ListBuilderTrait-for-ListPrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::ListPrimitiveChunkedBuilder">ListPrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
