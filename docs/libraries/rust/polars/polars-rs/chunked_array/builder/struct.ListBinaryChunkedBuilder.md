# Struct ListBinaryChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/list/binary.rs.html#93" class="src">Source</a>

``` rust
pub struct ListBinaryChunkedBuilder { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#impl-ListBinaryChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListBinaryChunkedBuilder.html" class="struct" title="struct polars::prelude::ListBinaryChunkedBuilder">ListBinaryChunkedBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.new" class="fn">new</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListBinaryChunkedBuilder.html" class="struct" title="struct polars::prelude::ListBinaryChunkedBuilder">ListBinaryChunkedBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.append_trusted_len_iter" class="fn">append_trusted_len_iter</a>\<'a, I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.append_values_iter" class="fn">append_values_iter</a>\<'a, I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#impl-ListBuilderTrait-for-ListBinaryChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListBinaryChunkedBuilder.html" class="struct" title="struct polars::prelude::ListBinaryChunkedBuilder">ListBinaryChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.append_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.append_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#tymethod.append_series" class="fn">append_series</a>(&mut self, s: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.field" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.field" class="fn">field</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.inner_array" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.inner_array" class="fn">inner_array</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.fast_explode" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.fast_explode" class="fn">fast_explode</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.append_opt_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.append_opt_series" class="fn">append_opt_series</a>( &mut self, opt_s: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.ListBinaryChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
