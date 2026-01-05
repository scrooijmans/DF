# Struct StateFieldsArgs Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/accumulator.rs.html#85" class="src">Source</a>

``` rust
pub struct StateFieldsArgs<'a> {
    pub name: &'a str,
    pub input_fields: &'a [Arc<Field>],
    pub return_field: Arc<Field>,
    pub ordering_fields: &'a [Arc<Field>],
    pub is_distinct: bool,
}
```

Expand description

[`StateFieldsArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html "struct datafusion::logical_expr::function::StateFieldsArgs") contains information about the fields that an aggregate function’s accumulator should have. Used for `AggregateUDFImpl::state_fields`.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#structfield.name" class="anchor field">§</a>`name: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

The name of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#structfield.input_fields" class="anchor field">§</a>`input_fields: &'a [`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>]`

The input fields of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#structfield.return_field" class="anchor field">§</a>`return_field: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>`

The return fields of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#structfield.ordering_fields" class="anchor field">§</a>`ordering_fields: &'a [`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>]`

The ordering fields of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#structfield.is_distinct" class="anchor field">§</a>`is_distinct: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the aggregate function is distinct.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#impl-StateFieldsArgs%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html" class="struct" title="struct datafusion::logical_expr::function::StateFieldsArgs">StateFieldsArgs</a>\<'\_\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#method.return_type" class="fn">return_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

The return type of the aggregate function.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html#blanket-implementations" class="anchor">§</a>
