# Struct ListPrimitiveChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/list/primitive.rs.html#3" class="src">Source</a>

``` rust
pub struct ListPrimitiveChunkedBuilder<T>where
    T: PolarsNumericType,{
    pub builder: MutableListArray<i64, MutablePrimitiveArray<<T as PolarsNumericType>::Native>>,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#structfield.builder" class="anchor field">§</a>`builder: `<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/mutable/struct.MutableListArray.html" class="struct" title="struct polars_arrow::array::list::mutable::MutableListArray"><code>MutableListArray</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`, `<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/mutable/struct.MutablePrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::mutable::MutablePrimitiveArray"><code>MutablePrimitiveArray</code></a>`<<T as `<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType"><code>PolarsNumericType</code></a>`>::`<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native"><code>Native</code></a>`>>`

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#impl-ListPrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::ListPrimitiveChunkedBuilder">ListPrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.new" class="fn">new</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, inner_type: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::ListPrimitiveChunkedBuilder">ListPrimitiveChunkedBuilder</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.new_with_values_type" class="fn">new_with_values_type</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, values_type: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, logical_type: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::ListPrimitiveChunkedBuilder">ListPrimitiveChunkedBuilder</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_slice" class="fn">append_slice</a>(&mut self, items: &\[\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\])

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_opt_slice" class="fn">append_opt_slice</a>( &mut self, opt_v: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\]\>, )

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_values_iter_trusted_len" class="fn">append_values_iter_trusted_len</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

Appends from an iterator over values

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_values_iter" class="fn">append_values_iter</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_iter" class="fn">append_iter</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

Appends from an iterator over values

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#impl-ListBuilderTrait-for-ListPrimitiveChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html" class="trait" title="trait polars::prelude::ListBuilderTrait">ListBuilderTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::ListPrimitiveChunkedBuilder">ListPrimitiveChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#tymethod.append_null" class="fn">append_null</a>(&mut self)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#tymethod.append_series" class="fn">append_series</a>(&mut self, s: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.field" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.field" class="fn">field</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.inner_array" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.inner_array" class="fn">inner_array</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.fast_explode" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.fast_explode" class="fn">fast_explode</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.append_opt_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.append_opt_series" class="fn">append_opt_series</a>( &mut self, opt_s: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ListBuilderTrait.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListPrimitiveChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
