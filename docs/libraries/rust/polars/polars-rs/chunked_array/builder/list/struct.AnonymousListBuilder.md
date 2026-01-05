# Struct AnonymousListBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/list/anonymous.rs.html#3" class="src">Source</a>

``` rust
pub struct AnonymousListBuilder<'a> { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#impl-AnonymousListBuilder%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.AnonymousListBuilder.html" class="struct" title="struct polars::chunked_array::builder::AnonymousListBuilder">AnonymousListBuilder</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.new" class="fn">new</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, inner_dtype: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.AnonymousListBuilder.html" class="struct" title="struct polars::chunked_array::builder::AnonymousListBuilder">AnonymousListBuilder</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.append_opt_series" class="fn">append_opt_series</a>( &mut self, opt_s: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.append_opt_array" class="fn">append_opt_array</a>(&mut self, opt_s: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a (dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static)\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.append_array" class="fn">append_array</a>(&mut self, arr: &'a (dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static))

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.append_empty" class="fn">append_empty</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.append_series" class="fn">append_series</a>(&mut self, s: &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#impl-Default-for-AnonymousListBuilder%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.AnonymousListBuilder.html" class="struct" title="struct polars::chunked_array::builder::AnonymousListBuilder">AnonymousListBuilder</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.AnonymousListBuilder.html" class="struct" title="struct polars::chunked_array::builder::AnonymousListBuilder">AnonymousListBuilder</a>\<'\_\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/list/struct.AnonymousListBuilder.html#blanket-implementations" class="anchor">§</a>
