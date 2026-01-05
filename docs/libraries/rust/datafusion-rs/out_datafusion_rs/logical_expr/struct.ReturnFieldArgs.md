# Struct ReturnFieldArgs Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udf.rs.html#361" class="src">Source</a>

``` rust
pub struct ReturnFieldArgs<'a> {
    pub arg_fields: &'a [Arc<Field>],
    pub scalar_arguments: &'a [Option<&'a ScalarValue>],
}
```

Expand description

Information about arguments passed to the function

This structure contains metadata about how the function was called such as the type of the arguments, any scalar arguments and if the arguments can (ever) be null

See [`ScalarUDFImpl::return_field_from_args`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.return_field_from_args "method datafusion::logical_expr::ScalarUDFImpl::return_field_from_args") for more information

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#structfield.arg_fields" class="anchor field">§</a>`arg_fields: &'a [`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>]`

The data types of the arguments to the function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#structfield.scalar_arguments" class="anchor field">§</a>`scalar_arguments: &'a [`<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<&'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue"><code>ScalarValue</code></a>`>]`

Is argument `i` to the function a scalar (constant)?

If the argument `i` is not a scalar, it will be None

For example, if a function is called like `my_function(column_a, 5)` this field will be `[None, Some(ScalarValue::Int32(Some(5)))]`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#impl-Debug-for-ReturnFieldArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html" class="struct" title="struct datafusion::logical_expr::ReturnFieldArgs">ReturnFieldArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ReturnFieldArgs.html#blanket-implementations" class="anchor">§</a>
