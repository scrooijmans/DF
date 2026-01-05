# Trait ExprSchemable Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_schema.rs.html#40" class="src">Source</a>

``` rust
pub trait ExprSchemable {
    // Required methods
    fn get_type(
        &self,
        schema: &dyn ExprSchema,
    ) -> Result<DataType, DataFusionError>;
    fn nullable(
        &self,
        input_schema: &dyn ExprSchema,
    ) -> Result<bool, DataFusionError>;
    fn metadata(
        &self,
        schema: &dyn ExprSchema,
    ) -> Result<FieldMetadata, DataFusionError>;
    fn to_field(
        &self,
        input_schema: &dyn ExprSchema,
    ) -> Result<(Option<TableReference>, Arc<Field>), DataFusionError>;
    fn cast_to(
        self,
        cast_to_type: &DataType,
        schema: &dyn ExprSchema,
    ) -> Result<Expr, DataFusionError>;
    fn data_type_and_nullable(
        &self,
        schema: &dyn ExprSchema,
    ) -> Result<(DataType, bool), DataFusionError>;
}
```

Expand description

Trait to allow expr to typable with respect to a schema

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.get_type" class="fn">get_type</a>(&self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Given a schema, return the type of the expr

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.nullable" class="fn">nullable</a>( &self, input_schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Given a schema, return the nullability of the expr

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.metadata" class="fn">metadata</a>( &self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Given a schema, return the expr’s optional metadata

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.to_field" class="fn">to_field</a>( &self, input_schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert to a field with respect to a schema

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.cast_to" class="fn">cast_to</a>( self, cast_to_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Cast to a type with respect to a schema

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.data_type_and_nullable" class="fn">data_type_and_nullable</a>( &self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Given a schema, return the type and nullability of the expr

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#impl-ExprSchemable-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html" class="trait" title="trait datafusion::logical_expr::ExprSchemable">ExprSchemable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>
