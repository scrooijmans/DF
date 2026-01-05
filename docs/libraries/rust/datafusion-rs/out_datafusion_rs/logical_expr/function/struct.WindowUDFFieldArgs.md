# Struct WindowUDFFieldArgs Copy item path

<a href="https://docs.rs/datafusion-functions-window-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_window_common/field.rs.html#22" class="src">Source</a>

``` rust
pub struct WindowUDFFieldArgs<'a> { /* private fields */ }
```

Expand description

Metadata for defining the result field from evaluating a user-defined window function.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#impl-WindowUDFFieldArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html" class="struct" title="struct datafusion::logical_expr::function::WindowUDFFieldArgs">WindowUDFFieldArgs</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#method.new" class="fn">new</a>( input_fields: &'a \[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\], display_name: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html" class="struct" title="struct datafusion::logical_expr::function::WindowUDFFieldArgs">WindowUDFFieldArgs</a>\<'a\>

Create an instance of [`WindowUDFFieldArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html "struct datafusion::logical_expr::function::WindowUDFFieldArgs").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#arguments" class="doc-anchor">§</a>Arguments

- `input_fields` - The fields corresponding to the arguments to the user-defined window function.
- `function_name` - The qualified schema name of the user-defined window function expression.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#method.input_fields" class="fn">input_fields</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\]

Returns the field of input expressions passed as arguments to the user-defined window function.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the name for the field of the final result of evaluating the user-defined window function.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#method.get_input_field" class="fn">get_input_field</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>

Returns `Some(Field)` of input expression at index, otherwise returns `None` if the index is out of bounds.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html#blanket-implementations" class="anchor">§</a>
