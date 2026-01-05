# Trait AnonymousScan Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/plans/anonymous_scan.rs.html#16" class="src">Source</a>

``` rust
pub trait AnonymousScan: Send + Sync {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn scan(
        &self,
        scan_opts: AnonymousScanArgs,
    ) -> Result<DataFrame, PolarsError>;

    // Provided methods
    fn next_batch(
        &self,
        scan_opts: AnonymousScanArgs,
    ) -> Result<Option<DataFrame>, PolarsError> { ... }
    fn schema(
        &self,
        _infer_schema_length: Option<usize>,
    ) -> Result<Arc<Schema<DataType>>, PolarsError> { ... }
    fn allows_predicate_pushdown(&self) -> bool { ... }
    fn allows_projection_pushdown(&self) -> bool { ... }
    fn allows_slice_pushdown(&self) -> bool { ... }
}
```

Available on **crate feature `lazy`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#tymethod.scan" class="fn">scan</a>(&self, scan_opts: <a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html" class="struct" title="struct polars::prelude::AnonymousScanArgs">AnonymousScanArgs</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Creates a DataFrame from the supplied function & scan options.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#method.next_batch" class="fn">next_batch</a>( &self, scan_opts: <a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html" class="struct" title="struct polars::prelude::AnonymousScanArgs">AnonymousScanArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Produce the next batch Polars can consume. Implement this method to get proper streaming support.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#method.schema" class="fn">schema</a>( &self, \_infer_schema_length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

function to supply the schema. Allows for an optional infer schema argument for data sources with dynamic schemas

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#method.allows_predicate_pushdown" class="fn">allows_predicate_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Specify if the scan provider should allow predicate pushdowns.

Defaults to `false`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#method.allows_projection_pushdown" class="fn">allows_projection_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Specify if the scan provider should allow projection pushdowns.

Defaults to `false`

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#method.allows_slice_pushdown" class="fn">allows_slice_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Specify if the scan provider should allow slice pushdowns.

Defaults to `false`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#impl-Debug-for-dyn+AnonymousScan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html" class="trait" title="trait polars::prelude::AnonymousScan">AnonymousScan</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html#implementors" class="anchor">§</a>
