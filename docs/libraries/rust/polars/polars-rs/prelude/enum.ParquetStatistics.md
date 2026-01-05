# Enum ParquetStatistics Copy item path

<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/src/polars_parquet/arrow/read/statistics.rs.html#24" class="src">Source</a>

``` rust
pub enum ParquetStatistics {
    Column(Box<ColumnStatistics>),
    List(Option<Box<Statistics>>),
    FixedSizeList(Option<Box<Statistics>>, usize),
    Struct(Box<[Option<Statistics>]>),
    Dictionary(IntegerType, Option<Box<Statistics>>, bool),
}
```

Available on **crate feature `polars-io`** only.

Expand description

Parquet statistics for a nesting level

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#variant.Column" class="anchor">§</a>

### Column(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/arrow/read/statistics/struct.ColumnStatistics.html" class="struct" title="struct polars_parquet::arrow::read::statistics::ColumnStatistics">ColumnStatistics</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>\>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#variant.FixedSizeList" class="anchor">§</a>

### FixedSizeList(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>\>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>\>\]\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#variant.Dictionary" class="anchor">§</a>

### Dictionary(<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.IntegerType.html" class="enum" title="enum polars_arrow::datatypes::physical_type::IntegerType">IntegerType</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>\>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#impl-Debug-for-ParquetStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#impl-PartialEq-for-ParquetStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#impl-StructuralPartialEq-for-ParquetStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html" class="enum" title="enum polars::prelude::ParquetStatistics">Statistics</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html#blanket-implementations" class="anchor">§</a>
