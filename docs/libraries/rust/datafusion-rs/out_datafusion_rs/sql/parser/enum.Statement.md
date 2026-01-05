# Enum Statement Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/parser.rs.html#254" class="src">Source</a>

``` rust
pub enum Statement {
    Statement(Box<Statement>),
    CreateExternalTable(CreateExternalTable),
    CopyTo(CopyToStatement),
    Explain(ExplainStatement),
}
```

Expand description

DataFusion SQL Statement.

This can either be a [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html "enum datafusion::logical_expr::sqlparser::ast::Statement") from [`sqlparser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/index.html "mod datafusion::logical_expr::sqlparser") from a standard SQL dialect, or a DataFusion extension such as `CREATE EXTERNAL TABLE`. See [`DFParser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html "struct datafusion::sql::parser::DFParser") for more information.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#variant.Statement" class="anchor">§</a>

### Statement(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>\>)

ANSI SQL AST node (from sqlparser-rs)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#variant.CreateExternalTable" class="anchor">§</a>

### CreateExternalTable(<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.CreateExternalTable.html" class="struct" title="struct datafusion::sql::parser::CreateExternalTable">CreateExternalTable</a>)

Extension: `CREATE EXTERNAL TABLE`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#variant.CopyTo" class="anchor">§</a>

### CopyTo(<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.CopyToStatement.html" class="struct" title="struct datafusion::sql::parser::CopyToStatement">CopyToStatement</a>)

Extension: `COPY TO`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#variant.Explain" class="anchor">§</a>

### Explain(<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>)

EXPLAIN for extensions

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#impl-Clone-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#impl-Debug-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#impl-Display-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#impl-PartialEq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#impl-Eq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#impl-StructuralPartialEq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html#blanket-implementations" class="anchor">§</a>
