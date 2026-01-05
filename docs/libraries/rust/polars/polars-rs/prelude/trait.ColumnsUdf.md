# Trait ColumnsUdf Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/expr_dyn_fn.rs.html#19" class="src">Source</a>

``` rust
pub trait ColumnsUdf: Send + Sync {
    // Required method
    fn call_udf(&self, s: &mut [Column]) -> Result<Column, PolarsError>;

    // Provided method
    fn as_any(&self) -> &(dyn Any + 'static) { ... }
}
```

Available on **crate feature `lazy`** only.

Expand description

A wrapper trait for any closure `Fn(Vec<Series>) -> PolarsResult<Series>`

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#tymethod.call_udf" class="fn">call_udf</a>(&self, s: &mut \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#method.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#impl-Debug-for-dyn+ColumnsUdf" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html" class="trait" title="trait polars::prelude::ColumnsUdf">ColumnsUdf</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#impl-ColumnsUdf-for-F" class="anchor">§</a>

### impl\<F\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html" class="trait" title="trait polars::prelude::ColumnsUdf">ColumnsUdf</a> for F

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html#impl-ColumnsUdf-for-BaseColumnUdf%3CF,+DT%3E" class="anchor">§</a>

### impl\<F, DT\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html" class="trait" title="trait polars::prelude::ColumnsUdf">ColumnsUdf</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BaseColumnUdf.html" class="struct" title="struct polars::prelude::BaseColumnUdf">BaseColumnUdf</a>\<F, DT\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,
