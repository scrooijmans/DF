# Enum ColumnOption Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/ddl.rs.html#1751" class="src">Source</a>

``` rust
pub enum ColumnOption {
Show 21 variants    Null,
    NotNull,
    Default(Expr),
    Materialized(Expr),
    Ephemeral(Option<Expr>),
    Alias(Expr),
    Unique {
        is_primary: bool,
        characteristics: Option<ConstraintCharacteristics>,
    },
    ForeignKey {
        foreign_table: ObjectName,
        referred_columns: Vec<Ident>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
        characteristics: Option<ConstraintCharacteristics>,
    },
    Check(Expr),
    DialectSpecific(Vec<Token>),
    CharacterSet(ObjectName),
    Collation(ObjectName),
    Comment(String),
    OnUpdate(Expr),
    Generated {
        generated_as: GeneratedAs,
        sequence_options: Option<Vec<SequenceOptions>>,
        generation_expr: Option<Expr>,
        generation_expr_mode: Option<GeneratedExpressionMode>,
        generated_keyword: bool,
    },
    Options(Vec<SqlOption>),
    Identity(IdentityPropertyKind),
    OnConflict(Keyword),
    Policy(ColumnPolicy),
    Tags(TagsColumnOption),
    Srid(Box<Expr>),
}
```

Expand description

`ColumnOption`s are modifiers that follow a column definition in a `CREATE TABLE` statement.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Null" class="anchor">§</a>

### Null

`NULL`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.NotNull" class="anchor">§</a>

### NotNull

`NOT NULL`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Default" class="anchor">§</a>

### Default(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>)

`DEFAULT <restricted-expr>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Materialized" class="anchor">§</a>

### Materialized(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>)

`MATERIALIZE <expr>` Syntax: `b INT MATERIALIZE (a + 1)`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/create/table#default_values)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Ephemeral" class="anchor">§</a>

### Ephemeral(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`EPHEMERAL [<expr>]`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/create/table#default_values)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Alias" class="anchor">§</a>

### Alias(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>)

`ALIAS <expr>`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/create/table#default_values)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Unique" class="anchor">§</a>

### Unique

`{ PRIMARY KEY | UNIQUE } [<constraint_characteristics>]`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Unique.field.is_primary" class="anchor field">§</a>`is_primary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Unique.field.characteristics" class="anchor field">§</a>`characteristics: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ConstraintCharacteristics.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ConstraintCharacteristics"><code>ConstraintCharacteristics</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.ForeignKey" class="anchor">§</a>

### ForeignKey

A referential integrity constraint (`[FOREIGN KEY REFERENCES <foreign_table> (<referred_columns>) { [ON DELETE <referential_action>] [ON UPDATE <referential_action>] | [ON UPDATE <referential_action>] [ON DELETE <referential_action>] } [<constraint_characteristics>] `).

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.ForeignKey.field.foreign_table" class="anchor field">§</a>`foreign_table: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.ForeignKey.field.referred_columns" class="anchor field">§</a>`referred_columns: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.ForeignKey.field.on_delete" class="anchor field">§</a>`on_delete: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ReferentialAction.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ReferentialAction"><code>ReferentialAction</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.ForeignKey.field.on_update" class="anchor field">§</a>`on_update: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ReferentialAction.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ReferentialAction"><code>ReferentialAction</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.ForeignKey.field.characteristics" class="anchor field">§</a>`characteristics: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ConstraintCharacteristics.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ConstraintCharacteristics"><code>ConstraintCharacteristics</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Check" class="anchor">§</a>

### Check(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>)

`CHECK (<expr>)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.DialectSpecific" class="anchor">§</a>

### DialectSpecific(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>\>)

Dialect-specific options, such as:

- MySQL’s `AUTO_INCREMENT` or SQLite’s `AUTOINCREMENT`
- …

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.CharacterSet" class="anchor">§</a>

### CharacterSet(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName">ObjectName</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Collation" class="anchor">§</a>

### Collation(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName">ObjectName</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Comment" class="anchor">§</a>

### Comment(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.OnUpdate" class="anchor">§</a>

### OnUpdate(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Generated" class="anchor">§</a>

### Generated

`Generated`s are modifiers that follow a column definition in a `CREATE TABLE` statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Generated.field.generated_as" class="anchor field">§</a>`generated_as: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.GeneratedAs.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::GeneratedAs"><code>GeneratedAs</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Generated.field.sequence_options" class="anchor field">§</a>`sequence_options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SequenceOptions.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SequenceOptions"><code>SequenceOptions</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Generated.field.generation_expr" class="anchor field">§</a>`generation_expr: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Generated.field.generation_expr_mode" class="anchor field">§</a>`generation_expr_mode: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.GeneratedExpressionMode.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::GeneratedExpressionMode"><code>GeneratedExpressionMode</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Generated.field.generated_keyword" class="anchor field">§</a>`generated_keyword: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

false if ‘GENERATED ALWAYS’ is skipped (option starts with AS)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Options" class="anchor">§</a>

### Options(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption">SqlOption</a>\>)

BigQuery specific: Explicit column options in a view [1](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#view_column_option_list) or table [2](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#column_option_list) Syntax

``` sql
OPTIONS(description="field desc")
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Identity" class="anchor">§</a>

### Identity(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.IdentityPropertyKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::IdentityPropertyKind">IdentityPropertyKind</a>)

Creates an identity or an autoincrement column in a table. Syntax

``` sql
{ IDENTITY | AUTOINCREMENT } [ (seed , increment) | START num INCREMENT num ] [ ORDER | NOORDER ]
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.OnConflict" class="anchor">§</a>

### OnConflict(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>)

SQLite specific: ON CONFLICT option on column definition <https://www.sqlite.org/lang_conflict.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Policy" class="anchor">§</a>

### Policy(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnPolicy.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnPolicy">ColumnPolicy</a>)

Snowflake specific: an option of specifying security masking or projection policy to set on a column. Syntax:

``` sql
[ WITH ] MASKING POLICY <policy_name> [ USING ( <col_name> , <cond_col1> , ... ) ]
[ WITH ] PROJECTION POLICY <policy_name>
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Tags" class="anchor">§</a>

### Tags(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TagsColumnOption.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TagsColumnOption">TagsColumnOption</a>)

Snowflake specific: Specifies the tag name and the tag string value. Syntax:

``` sql
[ WITH ] TAG ( <tag_name> = '<tag_value>' [ , <tag_name> = '<tag_value>' , ... ] )
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#variant.Srid" class="anchor">§</a>

### Srid(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

MySQL specific: Spatial reference identifier Syntax:

``` sql
CREATE TABLE geom (g GEOMETRY NOT NULL SRID 4326);
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Clone-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Debug-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Display-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Hash-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Ord-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-PartialEq-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-PartialOrd-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Spanned-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Spanned">Spanned</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#partial-span" class="doc-anchor">§</a>partial span

Missing spans:

- [ColumnOption::Null](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html#variant.Null "variant datafusion::logical_expr::sqlparser::ast::ColumnOption::Null")
- [ColumnOption::NotNull](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html#variant.NotNull "variant datafusion::logical_expr::sqlparser::ast::ColumnOption::NotNull")
- [ColumnOption::Comment](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html#variant.Comment "variant datafusion::logical_expr::sqlparser::ast::ColumnOption::Comment")
- [ColumnOption::Unique](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html#variant.Unique "variant datafusion::logical_expr::sqlparser::ast::ColumnOption::Unique")¨
- [ColumnOption::DialectSpecific](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html#variant.DialectSpecific "variant datafusion::logical_expr::sqlparser::ast::ColumnOption::DialectSpecific")
- [ColumnOption::Generated](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html#variant.Generated "variant datafusion::logical_expr::sqlparser::ast::ColumnOption::Generated")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.span" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html#tymethod.span" class="fn">span</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Span">Span</a>

Return the [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html "struct datafusion::logical_expr::sqlparser::tokenizer::Span") (the minimum and maximum [`Location`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Location.html "struct datafusion::logical_expr::sqlparser::tokenizer::Location")) for this AST node, by recursively combining the spans of its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Visit-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-VisitMut-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-Eq-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#impl-StructuralPartialEq-for-ColumnOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.ColumnOption.html#blanket-implementations" class="anchor">§</a>
