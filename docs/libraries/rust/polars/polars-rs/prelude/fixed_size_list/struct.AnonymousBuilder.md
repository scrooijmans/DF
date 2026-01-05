# Struct AnonymousBuilder Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/fixed_size_list.rs.html#10" class="src">Source</a>

``` rust
pub struct AnonymousBuilder {
    pub width: usize,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#structfield.width" class="anchor field">§</a>`width: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#impl-AnonymousBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html" class="struct" title="struct polars::prelude::fixed_size_list::AnonymousBuilder">AnonymousBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#method.new" class="fn">new</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, width: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html" class="struct" title="struct polars::prelude::fixed_size_list::AnonymousBuilder">AnonymousBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#method.push" class="fn">push</a>(&mut self, arr: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#method.push_null" class="fn">push_null</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#method.finish" class="fn">finish</a>( self, inner_dtype: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/fixed_size_list/struct.FixedSizeListArray.html" class="struct" title="struct polars_arrow::array::fixed_size_list::FixedSizeListArray">FixedSizeListArray</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#impl-Default-for-AnonymousBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html" class="struct" title="struct polars::prelude::fixed_size_list::AnonymousBuilder">AnonymousBuilder</a>

<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html" class="struct" title="struct polars::prelude::fixed_size_list::AnonymousBuilder">AnonymousBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/fixed_size_list/struct.AnonymousBuilder.html#blanket-implementations" class="anchor">§</a>
