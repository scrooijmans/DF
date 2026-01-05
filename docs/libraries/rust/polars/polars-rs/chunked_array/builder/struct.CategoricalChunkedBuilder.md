# Struct CategoricalChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/categorical.rs.html#5" class="src">Source</a>

``` rust
pub struct CategoricalChunkedBuilder<T>where
    T: PolarsCategoricalType,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#impl-CategoricalChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::CategoricalChunkedBuilder">CategoricalChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::CategoricalChunkedBuilder">CategoricalChunkedBuilder</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.reserve" class="fn">reserve</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.append_cat" class="fn">append_cat</a>( &mut self, cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, mapping: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.append_str" class="fn">append_str</a>(&mut self, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.CategoricalChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
