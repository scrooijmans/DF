# Struct ScalarFunctionArgs Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udf.rs.html#330" class="src">Source</a>

``` rust
pub struct ScalarFunctionArgs {
    pub args: Vec<ColumnarValue>,
    pub arg_fields: Vec<Arc<Field>>,
    pub number_rows: usize,
    pub return_field: Arc<Field>,
    pub config_options: Arc<ConfigOptions>,
}
```

Expand description

Arguments passed to [`ScalarUDFImpl::invoke_with_args`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#tymethod.invoke_with_args "method datafusion::logical_expr::ScalarUDFImpl::invoke_with_args") when invoking a scalar function.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#structfield.args" class="anchor field">§</a>`args: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue"><code>ColumnarValue</code></a>`>`

The evaluated arguments to the function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#structfield.arg_fields" class="anchor field">§</a>`arg_fields: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>>`

Field associated with each arg, if it exists

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#structfield.number_rows" class="anchor field">§</a>`number_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The number of rows in record batch being evaluated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#structfield.return_field" class="anchor field">§</a>`return_field: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>`

The return field of the scalar function returned (from `return_type` or `return_field_from_args`) when creating the physical expression from the logical expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#structfield.config_options" class="anchor field">§</a>`config_options: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions"><code>ConfigOptions</code></a>`>`

The config options at execution time

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#impl-ScalarFunctionArgs" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html" class="struct" title="struct datafusion::logical_expr::ScalarFunctionArgs">ScalarFunctionArgs</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#method.return_type" class="fn">return_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

The return type of the function. See [`Self::return_field`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#structfield.return_field "field datafusion::logical_expr::ScalarFunctionArgs::return_field") for more details.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#impl-Clone-for-ScalarFunctionArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html" class="struct" title="struct datafusion::logical_expr::ScalarFunctionArgs">ScalarFunctionArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html" class="struct" title="struct datafusion::logical_expr::ScalarFunctionArgs">ScalarFunctionArgs</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#impl-Debug-for-ScalarFunctionArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html" class="struct" title="struct datafusion::logical_expr::ScalarFunctionArgs">ScalarFunctionArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarFunctionArgs.html#blanket-implementations" class="anchor">§</a>
