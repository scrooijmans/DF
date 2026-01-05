# Struct SeriesBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/builder.rs.html#10" class="src">Source</a>

``` rust
pub struct SeriesBuilder { /* private fields */ }
```

Expand description

A type-erased wrapper around ArrayBuilder.

## Implementations<a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#impl-SeriesBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html" class="struct" title="struct polars::series::builder::SeriesBuilder">SeriesBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.new" class="fn">new</a>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html" class="struct" title="struct polars::series::builder::SeriesBuilder">SeriesBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.freeze" class="fn">freeze</a>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.freeze_reset" class="fn">freeze_reset</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.extend_nulls" class="fn">extend_nulls</a>(&mut self, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Extend this builder with the given number of null elements.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.extend" class="fn">extend</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>)

Extends this builder with the contents of the given series. May panic if other does not match the dtype of this builder.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.subslice_extend" class="fn">subslice_extend</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given series subslice. May panic if other does not match the dtype of this builder.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.subslice_extend_repeated" class="fn">subslice_extend_repeated</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repeats: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.subslice_extend_each_repeated" class="fn">subslice_extend_each_repeated</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repeats: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.gather_extend" class="fn">gather_extend</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, idxs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given series at the given indices. That is, `other[idxs[i]]` is appended to this builder in order, for each i=0..idxs.len(). May panic if other does not match the dtype of this builder, or if the other series is not rechunked.

##### <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#safety" class="doc-anchor">§</a>Safety

The indices must be in-bounds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.opt_gather_extend" class="fn">opt_gather_extend</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, idxs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#method.push_any_value" class="fn">push_any_value</a>(&mut self, value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/series/builder/struct.SeriesBuilder.html#blanket-implementations" class="anchor">§</a>
