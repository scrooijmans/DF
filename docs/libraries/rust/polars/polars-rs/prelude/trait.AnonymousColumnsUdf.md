# Trait AnonymousColumnsUdf Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/expr_dyn_fn.rs.html#7" class="src">Source</a>

``` rust
pub trait AnonymousColumnsUdf: ColumnsUdf {
    // Required methods
    fn as_column_udf(self: Arc<Self>) -> Arc<dyn ColumnsUdf>;
    fn deep_clone(self: Arc<Self>) -> Arc<dyn AnonymousColumnsUdf>;
    fn get_field(
        &self,
        input_schema: &Schema<DataType>,
        fields: &[Field],
    ) -> Result<Field, PolarsError>;

    // Provided method
    fn try_serialize(&self, _buf: &mut Vec<u8>) -> Result<(), PolarsError> { ... }
}
```

Available on **crate feature `lazy`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#tymethod.as_column_udf" class="fn">as_column_udf</a>(self: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<Self\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ColumnsUdf.html" class="trait" title="trait polars::prelude::ColumnsUdf">ColumnsUdf</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#tymethod.deep_clone" class="fn">deep_clone</a>(self: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<Self\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf">AnonymousColumnsUdf</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#tymethod.get_field" class="fn">get_field</a>( &self, input_schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, fields: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#method.try_serialize" class="fn">try_serialize</a>(&self, \_buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html#impl-AnonymousColumnsUdf-for-BaseColumnUdf%3CF,+DT%3E" class="anchor">§</a>

### impl\<F, DT\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf">AnonymousColumnsUdf</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.BaseColumnUdf.html" class="struct" title="struct polars::prelude::BaseColumnUdf">BaseColumnUdf</a>\<F, DT\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,
