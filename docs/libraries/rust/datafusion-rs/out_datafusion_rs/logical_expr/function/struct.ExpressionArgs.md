# Struct ExpressionArgs Copy item path

<a href="https://docs.rs/datafusion-functions-window-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_window_common/expr.rs.html#24" class="src">Source</a>

``` rust
pub struct ExpressionArgs<'a> { /* private fields */ }
```

Expand description

Arguments passed to user-defined window function

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#impl-ExpressionArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#method.new" class="fn">new</a>( input_exprs: &'a \[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\], input_fields: &'a \[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\], ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'a\>

Create an instance of [`ExpressionArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html "struct datafusion::logical_expr::function::ExpressionArgs").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#arguments" class="doc-anchor">§</a>Arguments

- `input_exprs` - The expressions passed as arguments to the user-defined window function.
- `input_types` - The data types corresponding to the arguments to the user-defined window function.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#method.input_exprs" class="fn">input_exprs</a>(&self) -\> &'a \[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\]

Returns the expressions passed as arguments to the user-defined window function.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#method.input_fields" class="fn">input_fields</a>(&self) -\> &'a \[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\]

Returns the [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef")s corresponding to the input expressions to the user-defined window function.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#impl-Debug-for-ExpressionArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#impl-Default-for-ExpressionArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html#blanket-implementations" class="anchor">§</a>
