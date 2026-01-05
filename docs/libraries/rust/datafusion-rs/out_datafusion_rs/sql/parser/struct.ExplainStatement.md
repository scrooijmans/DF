# Struct ExplainStatement Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/parser.rs.html#63" class="src">Source</a>

``` rust
pub struct ExplainStatement {
    pub analyze: bool,
    pub verbose: bool,
    pub format: Option<String>,
    pub statement: Box<Statement>,
}
```

Expand description

DataFusion specific `EXPLAIN`

Syntax:

``` sql
EXPLAIN <ANALYZE> <VERBOSE> [FORMAT format] statement
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#structfield.analyze" class="anchor field">§</a>`analyze: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

`EXPLAIN ANALYZE ..`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#structfield.verbose" class="anchor field">§</a>`verbose: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

`EXPLAIN .. VERBOSE ..`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#structfield.format" class="anchor field">§</a>`format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

`EXPLAIN .. FORMAT `

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#structfield.statement" class="anchor field">§</a>`statement: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement"><code>Statement</code></a>`>`

The statement to analyze. Note this is a DataFusion [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html "enum datafusion::sql::parser::Statement") (not a [`sqlparser::ast::Statement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html "enum datafusion::logical_expr::sqlparser::ast::Statement") so that we can use `EXPLAIN`, `COPY`, and other DataFusion specific statements

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#impl-Clone-for-ExplainStatement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#impl-Debug-for-ExplainStatement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#impl-Display-for-ExplainStatement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#impl-PartialEq-for-ExplainStatement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#impl-Eq-for-ExplainStatement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#impl-StructuralPartialEq-for-ExplainStatement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html#blanket-implementations" class="anchor">§</a>
