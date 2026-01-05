# Enum SelectExpr Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/select_expr.rs.html#56" class="src">Source</a>

``` rust
pub enum SelectExpr {
    Wildcard(WildcardOptions),
    QualifiedWildcard(TableReference, WildcardOptions),
    Expression(Expr),
}
```

Expand description

Represents a SELECT expression in a SQL query.

`SelectExpr` supports three types of expressions commonly found in the SELECT clause:

- Wildcard (`*`) - Selects all columns
- Qualified wildcard (`table.*`) - Selects all columns from a specific table
- Regular expression - Any other expression like columns, functions, literals etc.

This enum is typically used when you need to handle wildcards. After expanding `*` in the query, you can use `Expr` for all other expressions.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#examples" class="doc-anchor">§</a>Examples

``` rust
use datafusion_expr::col;
use datafusion_expr::expr::WildcardOptions;
use datafusion_expr::select_expr::SelectExpr;

// SELECT *
let wildcard = SelectExpr::Wildcard(WildcardOptions::default());

// SELECT mytable.*
let qualified = SelectExpr::QualifiedWildcard(
    "mytable".into(),
    WildcardOptions::default()
);

// SELECT col1
let expr = SelectExpr::Expression(col("col1").into());
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variant.Wildcard" class="anchor">§</a>

### Wildcard(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>)

Represents a wildcard (`*`) that selects all columns from all tables. The `WildcardOptions` control additional behavior like exclusions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variant.QualifiedWildcard" class="anchor">§</a>

### QualifiedWildcard(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>)

Represents a qualified wildcard (`table.*`) that selects all columns from a specific table. The `TableReference` specifies the table and `WildcardOptions` control additional behavior.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variant.Expression" class="anchor">§</a>

### Expression(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>)

Represents any other valid SELECT expression like column references, function calls, literals, etc.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#impl-Clone-for-SelectExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#impl-Debug-for-SelectExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#impl-Display-for-SelectExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#impl-From%3C(Option%3C%26TableReference%3E,+%26Arc%3CField%3E)%3E-for-SelectExpr" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &'a <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Create an [`SelectExpr::Expression`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variant.Expression "variant datafusion::logical_expr::select_expr::SelectExpr::Expression") from an optional qualifier and a [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef"). This is useful for creating [`SelectExpr::Expression`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variant.Expression "variant datafusion::logical_expr::select_expr::SelectExpr::Expression") from a `DFSchema`.

See example on [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &'a <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#impl-From%3CColumn%3E-for-SelectExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Create an [`SelectExpr::Expression`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#variant.Expression "variant datafusion::logical_expr::select_expr::SelectExpr::Expression") from a [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#impl-From%3CExpr%3E-for-SelectExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html#blanket-implementations" class="anchor">§</a>
