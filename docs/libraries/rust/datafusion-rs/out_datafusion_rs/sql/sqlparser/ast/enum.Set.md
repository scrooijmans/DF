# Enum Set Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/mod.rs.html#2884" class="src">Source</a>

``` rust
pub enum Set {
    SingleAssignment {
        scope: Option<ContextModifier>,
        hivevar: bool,
        variable: ObjectName,
        values: Vec<Expr>,
    },
    ParenthesizedAssignments {
        variables: Vec<ObjectName>,
        values: Vec<Expr>,
    },
    MultipleAssignments {
        assignments: Vec<SetAssignment>,
    },
    SetSessionParam(SetSessionParamKind),
    SetRole {
        context_modifier: Option<ContextModifier>,
        role_name: Option<Ident>,
    },
    SetTimeZone {
        local: bool,
        value: Expr,
    },
    SetNames {
        charset_name: Ident,
        collation_name: Option<String>,
    },
    SetNamesDefault {},
    SetTransaction {
        modes: Vec<TransactionMode>,
        snapshot: Option<Value>,
        session: bool,
    },
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SingleAssignment" class="anchor">§</a>

### SingleAssignment

SQL Standard-style SET a = 1;

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SingleAssignment.field.scope" class="anchor field">§</a>`scope: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ContextModifier.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ContextModifier"><code>ContextModifier</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SingleAssignment.field.hivevar" class="anchor field">§</a>`hivevar: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SingleAssignment.field.variable" class="anchor field">§</a>`variable: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SingleAssignment.field.values" class="anchor field">§</a>`values: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.ParenthesizedAssignments" class="anchor">§</a>

### ParenthesizedAssignments

Snowflake-style SET (a, b, ..) = (1, 2, ..);

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.ParenthesizedAssignments.field.variables" class="anchor field">§</a>`variables: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.ParenthesizedAssignments.field.values" class="anchor field">§</a>`values: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.MultipleAssignments" class="anchor">§</a>

### MultipleAssignments

MySQL-style SET a = 1, b = 2, ..;

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.MultipleAssignments.field.assignments" class="anchor field">§</a>`assignments: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.SetAssignment.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::SetAssignment"><code>SetAssignment</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetSessionParam" class="anchor">§</a>

### SetSessionParam(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SetSessionParamKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SetSessionParamKind">SetSessionParamKind</a>)

MS-SQL session

See <https://learn.microsoft.com/en-us/sql/t-sql/statements/set-statements-transact-sql>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetRole" class="anchor">§</a>

### SetRole

``` sql
SET [ SESSION | LOCAL ] ROLE role_name
```

Sets session state. Examples: [ANSI](https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#set-role-statement), [Postgresql](https://www.postgresql.org/docs/14/sql-set-role.html), [MySQL](https://dev.mysql.com/doc/refman/8.0/en/set-role.html), and [Oracle](https://docs.oracle.com/cd/B19306_01/server.102/b14200/statements_10004.htm)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetRole.field.context_modifier" class="anchor field">§</a>`context_modifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ContextModifier.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ContextModifier"><code>ContextModifier</code></a>`>`

Non-ANSI optional identifier to inform if the role is defined inside the current session (`SESSION`) or transaction (`LOCAL`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetRole.field.role_name" class="anchor field">§</a>`role_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

Role name. If NONE is specified, then the current role name is removed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTimeZone" class="anchor">§</a>

### SetTimeZone

``` sql
SET TIME ZONE <value>
```

Note: this is a PostgreSQL-specific statements `SET TIME ZONE <value>` is an alias for `SET timezone TO <value>` in PostgreSQL However, we allow it for all dialects.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTimeZone.field.local" class="anchor field">§</a>`local: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTimeZone.field.value" class="anchor field">§</a>`value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetNames" class="anchor">§</a>

### SetNames

``` sql
SET NAMES 'charset_name' [COLLATE 'collation_name']
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetNames.field.charset_name" class="anchor field">§</a>`charset_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetNames.field.collation_name" class="anchor field">§</a>`collation_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetNamesDefault" class="anchor">§</a>

### SetNamesDefault

``` sql
SET NAMES DEFAULT
```

Note: this is a MySQL-specific statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTransaction" class="anchor">§</a>

### SetTransaction

``` sql
SET TRANSACTION ...
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTransaction.field.modes" class="anchor field">§</a>`modes: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TransactionMode.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TransactionMode"><code>TransactionMode</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTransaction.field.snapshot" class="anchor field">§</a>`snapshot: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#variant.SetTransaction.field.session" class="anchor field">§</a>`session: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Clone-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Debug-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Display-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-From%3CSet%3E-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

Convert a `Set` into a `Statement`. Convenience function, instead of writing `Statement::Set(Set::Set...{...})`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(set: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Hash-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Ord-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-PartialEq-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-PartialOrd-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Visit-for-Set" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-VisitMut-for-Set" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-Eq-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#impl-StructuralPartialEq-for-Set" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.Set.html#blanket-implementations" class="anchor">§</a>
