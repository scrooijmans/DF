# Type Alias SchemaFieldMetadata Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr.rs.html#665" class="src">Source</a>

``` rust
pub type SchemaFieldMetadata = HashMap<String, String>;
```

Expand description

The metadata used in [`Field::metadata`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.metadata "method datafusion::common::arrow::datatypes::Field::metadata").

This represents the metadata associated with an Arrow [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field"). The metadata consists of key-value pairs.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/type.SchemaFieldMetadata.html#common-use-cases" class="doc-anchor">§</a>Common Use Cases

Field metadata is commonly used to store:

- Default values for columns when data is missing
- Column descriptions or documentation
- Data lineage information
- Custom application-specific annotations
- Encoding hints or display formatting preferences

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/type.SchemaFieldMetadata.html#example-storing-default-values" class="doc-anchor">§</a>Example: Storing Default Values

A practical example of using field metadata is storing default values for columns that may be missing in the physical data but present in the logical schema. See the [default_column_values.rs](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/default_column_values.rs) example implementation.

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/type.SchemaFieldMetadata.html#aliased-type" class="anchor">§</a>

``` rust
pub struct SchemaFieldMetadata { /* private fields */ }
```
