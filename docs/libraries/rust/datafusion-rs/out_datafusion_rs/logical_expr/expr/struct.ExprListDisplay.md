# Struct ExprListDisplay Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr.rs.html#3230" class="src">Source</a>

``` rust
pub struct ExprListDisplay<'a> { /* private fields */ }
```

Expand description

Formats a list of `&Expr` with a custom separator using SQL display format

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#impl-ExprListDisplay%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html" class="struct" title="struct datafusion::logical_expr::expr::ExprListDisplay">ExprListDisplay</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#method.new" class="fn">new</a>(exprs: &'a \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], sep: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html" class="struct" title="struct datafusion::logical_expr::expr::ExprListDisplay">ExprListDisplay</a>\<'a\>

Create a new display struct with the given expressions and separator

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#method.comma_separated" class="fn">comma_separated</a>(exprs: &'a \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html" class="struct" title="struct datafusion::logical_expr::expr::ExprListDisplay">ExprListDisplay</a>\<'a\>

Create a new display struct with comma-space separator

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#impl-Display-for-ExprListDisplay%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html" class="struct" title="struct datafusion::logical_expr::expr::ExprListDisplay">ExprListDisplay</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html#blanket-implementations" class="anchor">§</a>
