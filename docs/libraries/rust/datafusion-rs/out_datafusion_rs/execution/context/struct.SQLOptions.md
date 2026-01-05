# Struct SQLOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/context/mod.rs.html#1826-1833" class="src">Source</a>

``` rust
pub struct SQLOptions { /* private fields */ }
```

Expand description

Describes which SQL statements can be run.

See [`SessionContext::sql_with_options`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql_with_options "method datafusion::execution::context::SessionContext::sql_with_options") for more details.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#impl-SQLOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.new" class="fn">new</a>() -\> Self

Create a new `SQLOptions` with default values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.with_allow_ddl" class="fn">with_allow_ddl</a>(self, allow: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Should DDL data definition commands (e.g. `CREATE TABLE`) be run? Defaults to `true`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.with_allow_dml" class="fn">with_allow_dml</a>(self, allow: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Should DML data modification commands (e.g. `INSERT` and `COPY`) be run? Defaults to `true`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.with_allow_statements" class="fn">with_allow_statements</a>(self, allow: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Should Statements such as (e.g. `SET VARIABLE and `BEGIN TRANSACTION` ...`) be run?. Defaults to `true`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.verify_plan" class="fn">verify_plan</a>(&self, plan: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Return an error if the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") has any nodes that are incompatible with this [`SQLOptions`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html "struct datafusion::execution::context::SQLOptions").

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#impl-Clone-for-SQLOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#impl-Debug-for-SQLOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#impl-Default-for-SQLOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#impl-Copy-for-SQLOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html#blanket-implementations" class="anchor">§</a>
