# Struct DataFrameBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/builder.rs.html#11" class="src">Source</a>

``` rust
pub struct DataFrameBuilder { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#impl-DataFrameBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html" class="struct" title="struct polars::frame::builder::DataFrameBuilder">DataFrameBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.new" class="fn">new</a>(schema: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html" class="struct" title="struct polars::frame::builder::DataFrameBuilder">DataFrameBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.freeze" class="fn">freeze</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.freeze_reset" class="fn">freeze_reset</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.extend" class="fn">extend</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>)

Extends this builder with the contents of the given dataframe. May panic if other does not match the schema of this builder.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.subslice_extend" class="fn">subslice_extend</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given dataframe subslice. May panic if other does not match the schema of this builder.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.subslice_extend_repeated" class="fn">subslice_extend_repeated</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repeats: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given dataframe subslice, repeating it `repeats` times. May panic if other does not match the schema of this builder.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.subslice_extend_each_repeated" class="fn">subslice_extend_each_repeated</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repeats: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given dataframe subslice. Each element is repeated repeats times. May panic if other does not match the schema of this builder.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.gather_extend" class="fn">gather_extend</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, idxs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given dataframe at the given indices. That is, `other[idxs[i]]` is appended to this builder in order, for each i=0..idxs.len(). May panic if other does not match the schema of this builder, or if the other dataframe is not rechunked.

##### <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#safety" class="doc-anchor">§</a>Safety

The indices must be in-bounds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#method.opt_gather_extend" class="fn">opt_gather_extend</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, idxs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], share: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/builder/enum.ShareStrategy.html" class="enum" title="enum polars_arrow::array::builder::ShareStrategy">ShareStrategy</a>, )

Extends this builder with the contents of the given dataframe at the given indices. That is, `other[idxs[i]]` is appended to this builder in order, for each i=0..idxs.len(). Out-of-bounds indices extend with nulls. May panic if other does not match the schema of this builder, or if the other dataframe is not rechunked.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/builder/struct.DataFrameBuilder.html#blanket-implementations" class="anchor">§</a>
