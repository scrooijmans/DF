# Trait ExprSchema Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/dfschema.rs.html#1000" class="src">Source</a>

``` rust
pub trait ExprSchema: Debug {
    // Required method
    fn field_from_column(&self, col: &Column) -> Result<&Field, DataFusionError>;

    // Provided methods
    fn nullable(&self, col: &Column) -> Result<bool, DataFusionError> { ... }
    fn data_type(&self, col: &Column) -> Result<&DataType, DataFusionError> { ... }
    fn metadata(
        &self,
        col: &Column,
    ) -> Result<&HashMap<String, String>, DataFusionError> { ... }
    fn data_type_and_nullable(
        &self,
        col: &Column,
    ) -> Result<(&DataType, bool), DataFusionError> { ... }
}
```

Expand description

Provides schema information needed by certain methods of `Expr` (defined in the datafusion-common crate).

Note that this trait is implemented for &[DFSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema") which is widely used in the DataFusion codebase.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#tymethod.field_from_column" class="fn">field_from_column</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.nullable" class="fn">nullable</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Is this column reference nullable?

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.data_type" class="fn">data_type</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What is the datatype of this column?

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.metadata" class="fn">metadata</a>( &self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the column’s optional metadata.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.data_type_and_nullable" class="fn">data_type_and_nullable</a>( &self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the column’s datatype and nullability

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#impl-ExprSchema-for-DFSchema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#impl-ExprSchema-for-P" class="anchor">§</a>

### impl\<P\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a> for P

where P: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,
