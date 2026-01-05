# Enum Expr Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/mod.rs.html#731" class="src">Source</a>

``` rust
pub enum Expr {
Show 63 variants    Identifier(Ident),
    CompoundIdentifier(Vec<Ident>),
    CompoundFieldAccess {
        root: Box<Expr>,
        access_chain: Vec<AccessExpr>,
    },
    JsonAccess {
        value: Box<Expr>,
        path: JsonPath,
    },
    IsFalse(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsTrue(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    IsDistinctFrom(Box<Expr>, Box<Expr>),
    IsNotDistinctFrom(Box<Expr>, Box<Expr>),
    IsNormalized {
        expr: Box<Expr>,
        form: Option<NormalizationForm>,
        negated: bool,
    },
    InList {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },
    InSubquery {
        expr: Box<Expr>,
        subquery: Box<Query>,
        negated: bool,
    },
    InUnnest {
        expr: Box<Expr>,
        array_expr: Box<Expr>,
        negated: bool,
    },
    Between {
        expr: Box<Expr>,
        negated: bool,
        low: Box<Expr>,
        high: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    Like {
        negated: bool,
        any: bool,
        expr: Box<Expr>,
        pattern: Box<Expr>,
        escape_char: Option<Value>,
    },
    ILike {
        negated: bool,
        any: bool,
        expr: Box<Expr>,
        pattern: Box<Expr>,
        escape_char: Option<Value>,
    },
    SimilarTo {
        negated: bool,
        expr: Box<Expr>,
        pattern: Box<Expr>,
        escape_char: Option<Value>,
    },
    RLike {
        negated: bool,
        expr: Box<Expr>,
        pattern: Box<Expr>,
        regexp: bool,
    },
    AnyOp {
        left: Box<Expr>,
        compare_op: BinaryOperator,
        right: Box<Expr>,
        is_some: bool,
    },
    AllOp {
        left: Box<Expr>,
        compare_op: BinaryOperator,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expr>,
    },
    Convert {
        is_try: bool,
        expr: Box<Expr>,
        data_type: Option<DataType>,
        charset: Option<ObjectName>,
        target_before_value: bool,
        styles: Vec<Expr>,
    },
    Cast {
        kind: CastKind,
        expr: Box<Expr>,
        data_type: DataType,
        format: Option<CastFormat>,
    },
    AtTimeZone {
        timestamp: Box<Expr>,
        time_zone: Box<Expr>,
    },
    Extract {
        field: DateTimeField,
        syntax: ExtractSyntax,
        expr: Box<Expr>,
    },
    Ceil {
        expr: Box<Expr>,
        field: CeilFloorKind,
    },
    Floor {
        expr: Box<Expr>,
        field: CeilFloorKind,
    },
    Position {
        expr: Box<Expr>,
        in: Box<Expr>,
    },
    Substring {
        expr: Box<Expr>,
        substring_from: Option<Box<Expr>>,
        substring_for: Option<Box<Expr>>,
        special: bool,
        shorthand: bool,
    },
    Trim {
        expr: Box<Expr>,
        trim_where: Option<TrimWhereField>,
        trim_what: Option<Box<Expr>>,
        trim_characters: Option<Vec<Expr>>,
    },
    Overlay {
        expr: Box<Expr>,
        overlay_what: Box<Expr>,
        overlay_from: Box<Expr>,
        overlay_for: Option<Box<Expr>>,
    },
    Collate {
        expr: Box<Expr>,
        collation: ObjectName,
    },
    Nested(Box<Expr>),
    Value(ValueWithSpan),
    Prefixed {
        prefix: Ident,
        value: Box<Expr>,
    },
    TypedString {
        data_type: DataType,
        value: ValueWithSpan,
    },
    Function(Function),
    Case {
        case_token: AttachedToken,
        end_token: AttachedToken,
        operand: Option<Box<Expr>>,
        conditions: Vec<CaseWhen>,
        else_result: Option<Box<Expr>>,
    },
    Exists {
        subquery: Box<Query>,
        negated: bool,
    },
    Subquery(Box<Query>),
    GroupingSets(Vec<Vec<Expr>>),
    Cube(Vec<Vec<Expr>>),
    Rollup(Vec<Vec<Expr>>),
    Tuple(Vec<Expr>),
    Struct {
        values: Vec<Expr>,
        fields: Vec<StructField>,
    },
    Named {
        expr: Box<Expr>,
        name: Ident,
    },
    Dictionary(Vec<DictionaryField>),
    Map(Map),
    Array(Array),
    Interval(Interval),
    MatchAgainst {
        columns: Vec<ObjectName>,
        match_value: Value,
        opt_search_modifier: Option<SearchModifier>,
    },
    Wildcard(AttachedToken),
    QualifiedWildcard(ObjectName, AttachedToken),
    OuterJoin(Box<Expr>),
    Prior(Box<Expr>),
    Lambda(LambdaFunction),
    MemberOf(MemberOf),
}
```

Expand description

An SQL expression of any type.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#semantics--type-checking" class="doc-anchor">§</a>Semantics / Type Checking

The parser does not distinguish between expressions of different types (e.g. boolean vs string). The caller is responsible for detecting and validating types as necessary (for example `WHERE 1` vs `SELECT 1=1`) See the [README.md](https://github.com/apache/datafusion-sqlparser-rs/blob/main/README.md#syntax-vs-semantics) for more details.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#equality-and-hashing-does-not-include-source-locations" class="doc-anchor">§</a>Equality and Hashing Does not Include Source Locations

The `Expr` type implements `PartialEq` and `Eq` based on the semantic value of the expression (not bitwise comparison). This means that `Expr` instances that are semantically equivalent but have different spans (locations in the source tree) will compare as equal.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Identifier" class="anchor">§</a>

### Identifier(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident">Ident</a>)

Identifier e.g. table name or column name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.CompoundIdentifier" class="anchor">§</a>

### CompoundIdentifier(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident">Ident</a>\>)

Multi-part identifier, e.g. `table_alias.column` or `schema.table.col`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.CompoundFieldAccess" class="anchor">§</a>

### CompoundFieldAccess

Multi-part expression access.

This structure represents an access chain in structured / nested types such as maps, arrays, and lists:

- Array
  - A 1-dim array `a[1]` will be represented like: `CompoundFieldAccess(Ident('a'), vec![Subscript(1)]`
  - A 2-dim array `a[1][2]` will be represented like: `CompoundFieldAccess(Ident('a'), vec![Subscript(1), Subscript(2)]`
- Map or Struct (Bracket-style)
  - A map `a['field1']` will be represented like: `CompoundFieldAccess(Ident('a'), vec![Subscript('field')]`
  - A 2-dim map `a['field1']['field2']` will be represented like: `CompoundFieldAccess(Ident('a'), vec![Subscript('field1'), Subscript('field2')]`
- Struct (Dot-style) (only effect when the chain contains both subscript and expr)
  - A struct access `a[field1].field2` will be represented like: `CompoundFieldAccess(Ident('a'), vec![Subscript('field1'), Ident('field2')]`
- If a struct access likes `a.field1.field2`, it will be represented by CompoundIdentifier(\[a, field1, field2\])

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.CompoundFieldAccess.field.root" class="anchor field">§</a>`root: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.CompoundFieldAccess.field.access_chain" class="anchor field">§</a>`access_chain: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AccessExpr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AccessExpr"><code>AccessExpr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.JsonAccess" class="anchor">§</a>

### JsonAccess

Access data nested in a value containing semi-structured data, such as the `VARIANT` type on Snowflake. for example `src:customer[0].name`.

See <https://docs.snowflake.com/en/user-guide/querying-semistructured>. See <https://docs.databricks.com/en/sql/language-manual/functions/colonsign.html>.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.JsonAccess.field.value" class="anchor field">§</a>`value: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

The value being queried.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.JsonAccess.field.path" class="anchor field">§</a>`path: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.JsonPath.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::JsonPath"><code>JsonPath</code></a>

The path to the data to extract.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsFalse" class="anchor">§</a>

### IsFalse(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS FALSE` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNotFalse" class="anchor">§</a>

### IsNotFalse(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS NOT FALSE` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsTrue" class="anchor">§</a>

### IsTrue(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS TRUE` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNotTrue" class="anchor">§</a>

### IsNotTrue(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS NOT TRUE` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNull" class="anchor">§</a>

### IsNull(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS NULL` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNotNull" class="anchor">§</a>

### IsNotNull(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS NOT NULL` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsUnknown" class="anchor">§</a>

### IsUnknown(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS UNKNOWN` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNotUnknown" class="anchor">§</a>

### IsNotUnknown(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS NOT UNKNOWN` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsDistinctFrom" class="anchor">§</a>

### IsDistinctFrom(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS DISTINCT FROM` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNotDistinctFrom" class="anchor">§</a>

### IsNotDistinctFrom(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

`IS NOT DISTINCT FROM` operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNormalized" class="anchor">§</a>

### IsNormalized

`<expr> IS [ NOT ] [ form ] NORMALIZED`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNormalized.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNormalized.field.form" class="anchor field">§</a>`form: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.NormalizationForm.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::NormalizationForm"><code>NormalizationForm</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.IsNormalized.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InList" class="anchor">§</a>

### InList

`[ NOT ] IN (val1, val2, ...)`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InList.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InList.field.list" class="anchor field">§</a>`list: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InList.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InSubquery" class="anchor">§</a>

### InSubquery

`[ NOT ] IN (SELECT ...)`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InSubquery.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InSubquery.field.subquery" class="anchor field">§</a>`subquery: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InSubquery.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InUnnest" class="anchor">§</a>

### InUnnest

`[ NOT ] IN UNNEST(array_expression)`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InUnnest.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InUnnest.field.array_expr" class="anchor field">§</a>`array_expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.InUnnest.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Between" class="anchor">§</a>

### Between

`<expr> [ NOT ] BETWEEN <low> AND <high>`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Between.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Between.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Between.field.low" class="anchor field">§</a>`low: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Between.field.high" class="anchor field">§</a>`high: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.BinaryOp" class="anchor">§</a>

### BinaryOp

Binary operation e.g. `1 + 1` or `foo > bar`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.BinaryOp.field.left" class="anchor field">§</a>`left: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.BinaryOp.field.op" class="anchor field">§</a>`op: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator"><code>BinaryOperator</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.BinaryOp.field.right" class="anchor field">§</a>`right: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Like" class="anchor">§</a>

### Like

`[NOT] LIKE <pattern> [ESCAPE <escape_character>]`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Like.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Like.field.any" class="anchor field">§</a>`any: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Like.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Like.field.pattern" class="anchor field">§</a>`pattern: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Like.field.escape_char" class="anchor field">§</a>`escape_char: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.ILike" class="anchor">§</a>

### ILike

`ILIKE` (case-insensitive `LIKE`)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.ILike.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.ILike.field.any" class="anchor field">§</a>`any: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.ILike.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.ILike.field.pattern" class="anchor field">§</a>`pattern: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.ILike.field.escape_char" class="anchor field">§</a>`escape_char: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.SimilarTo" class="anchor">§</a>

### SimilarTo

SIMILAR TO regex

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.SimilarTo.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.SimilarTo.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.SimilarTo.field.pattern" class="anchor field">§</a>`pattern: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.SimilarTo.field.escape_char" class="anchor field">§</a>`escape_char: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.RLike" class="anchor">§</a>

### RLike

MySQL: RLIKE regex or REGEXP regex

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.RLike.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.RLike.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.RLike.field.pattern" class="anchor field">§</a>`pattern: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.RLike.field.regexp" class="anchor field">§</a>`regexp: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AnyOp" class="anchor">§</a>

### AnyOp

`ANY` operation e.g. `foo > ANY(bar)`, comparison operator is one of `[=, >, <, =>, =<, !=]` <https://docs.snowflake.com/en/sql-reference/operators-subquery#all-any>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AnyOp.field.left" class="anchor field">§</a>`left: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AnyOp.field.compare_op" class="anchor field">§</a>`compare_op: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator"><code>BinaryOperator</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AnyOp.field.right" class="anchor field">§</a>`right: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AnyOp.field.is_some" class="anchor field">§</a>`is_some: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AllOp" class="anchor">§</a>

### AllOp

`ALL` operation e.g. `foo > ALL(bar)`, comparison operator is one of `[=, >, <, =>, =<, !=]` <https://docs.snowflake.com/en/sql-reference/operators-subquery#all-any>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AllOp.field.left" class="anchor field">§</a>`left: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AllOp.field.compare_op" class="anchor field">§</a>`compare_op: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator"><code>BinaryOperator</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AllOp.field.right" class="anchor field">§</a>`right: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.UnaryOp" class="anchor">§</a>

### UnaryOp

Unary operation e.g. `NOT foo`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.UnaryOp.field.op" class="anchor field">§</a>`op: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.UnaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::UnaryOperator"><code>UnaryOperator</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.UnaryOp.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert" class="anchor">§</a>

### Convert

CONVERT a value to a different data type or character encoding. e.g. `CONVERT(foo USING utf8mb4)`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert.field.is_try" class="anchor field">§</a>`is_try: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

CONVERT (false) or TRY_CONVERT (true) <https://learn.microsoft.com/en-us/sql/t-sql/functions/try-convert-transact-sql?view=sql-server-ver16>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

The expression to convert

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert.field.data_type" class="anchor field">§</a>`data_type: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType"><code>DataType</code></a>`>`

The target data type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert.field.charset" class="anchor field">§</a>`charset: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

The target character encoding

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert.field.target_before_value" class="anchor field">§</a>`target_before_value: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

whether the target comes before the expr (MSSQL syntax)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Convert.field.styles" class="anchor field">§</a>`styles: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

How to translate the expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Cast" class="anchor">§</a>

### Cast

`CAST` an expression to a different data type e.g. `CAST(foo AS VARCHAR(123))`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Cast.field.kind" class="anchor field">§</a>`kind: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CastKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CastKind"><code>CastKind</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Cast.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Cast.field.data_type" class="anchor field">§</a>`data_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType"><code>DataType</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Cast.field.format" class="anchor field">§</a>`format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CastFormat.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CastFormat"><code>CastFormat</code></a>`>`

Optional CAST(string_expression AS type FORMAT format_string_expression) as used by [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/format-elements#formatting_syntax)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AtTimeZone" class="anchor">§</a>

### AtTimeZone

AT a timestamp to a different timezone e.g. `FROM_UNIXTIME(0) AT TIME ZONE 'UTC-06:00'`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AtTimeZone.field.timestamp" class="anchor field">§</a>`timestamp: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.AtTimeZone.field.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Extract" class="anchor">§</a>

### Extract

Extract a field from a timestamp e.g. `EXTRACT(MONTH FROM foo)` Or `EXTRACT(MONTH, foo)`

Syntax:

``` sql
EXTRACT(DateTimeField FROM <expr>) | EXTRACT(DateTimeField, <expr>)
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Extract.field.field" class="anchor field">§</a>`field: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DateTimeField.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DateTimeField"><code>DateTimeField</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Extract.field.syntax" class="anchor field">§</a>`syntax: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ExtractSyntax.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ExtractSyntax"><code>ExtractSyntax</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Extract.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Ceil" class="anchor">§</a>

### Ceil

``` sql
CEIL(<expr> [TO DateTimeField])
```

``` sql
CEIL( <input_expr> [, <scale_expr> ] )
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Ceil.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Ceil.field.field" class="anchor field">§</a>`field: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CeilFloorKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CeilFloorKind"><code>CeilFloorKind</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Floor" class="anchor">§</a>

### Floor

``` sql
FLOOR(<expr> [TO DateTimeField])
```

``` sql
FLOOR( <input_expr> [, <scale_expr> ] )
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Floor.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Floor.field.field" class="anchor field">§</a>`field: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CeilFloorKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CeilFloorKind"><code>CeilFloorKind</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Position" class="anchor">§</a>

### Position

``` sql
POSITION(<expr> in <expr>)
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Position.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Position.field.in" class="anchor field">§</a>`in: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Substring" class="anchor">§</a>

### Substring

``` sql
SUBSTRING(<expr> [FROM <expr>] [FOR <expr>])
```

or

``` sql
SUBSTRING(<expr>, <expr>, <expr>)
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Substring.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Substring.field.substring_from" class="anchor field">§</a>`substring_from: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Substring.field.substring_for" class="anchor field">§</a>`substring_for: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Substring.field.special" class="anchor field">§</a>`special: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

false if the expression is represented using the `SUBSTRING(expr [FROM start] [FOR len])` syntax true if the expression is represented using the `SUBSTRING(expr, start, len)` syntax This flag is used for formatting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Substring.field.shorthand" class="anchor field">§</a>`shorthand: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

true if the expression is represented using the `SUBSTR` shorthand This flag is used for formatting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Trim" class="anchor">§</a>

### Trim

``` sql
TRIM([BOTH | LEADING | TRAILING] [<expr> FROM] <expr>)
TRIM(<expr>)
TRIM(<expr>, [, characters]) -- only Snowflake or Bigquery
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Trim.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Trim.field.trim_where" class="anchor field">§</a>`trim_where: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TrimWhereField.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TrimWhereField"><code>TrimWhereField</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Trim.field.trim_what" class="anchor field">§</a>`trim_what: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Trim.field.trim_characters" class="anchor field">§</a>`trim_characters: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Overlay" class="anchor">§</a>

### Overlay

``` sql
OVERLAY(<expr> PLACING <expr> FROM <expr>[ FOR <expr> ]
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Overlay.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Overlay.field.overlay_what" class="anchor field">§</a>`overlay_what: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Overlay.field.overlay_from" class="anchor field">§</a>`overlay_from: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Overlay.field.overlay_for" class="anchor field">§</a>`overlay_for: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Collate" class="anchor">§</a>

### Collate

`expr COLLATE collation`

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Collate.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Collate.field.collation" class="anchor field">§</a>`collation: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Nested" class="anchor">§</a>

### Nested(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

Nested expression e.g. `(foo > bar)` or `(1)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Value" class="anchor">§</a>

### Value(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>)

A literal value, such as string, number, date or NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Prefixed" class="anchor">§</a>

### Prefixed

Prefixed expression, e.g. introducer strings, projection prefix <https://dev.mysql.com/doc/refman/8.0/en/charset-introducer.html> <https://docs.snowflake.com/en/sql-reference/constructs/connect-by>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Prefixed.field.prefix" class="anchor field">§</a>`prefix: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Prefixed.field.value" class="anchor field">§</a>`value: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

The value of the constant. Hint: you can unwrap the string value using `value.into_string()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.TypedString" class="anchor">§</a>

### TypedString

A constant of form `<data_type> 'value'`. This can represent ANSI SQL `DATE`, `TIME`, and `TIMESTAMP` literals (such as `DATE '2020-01-01'`), as well as constants of other types (a non-standard PostgreSQL extension).

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.TypedString.field.data_type" class="anchor field">§</a>`data_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType"><code>DataType</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.TypedString.field.value" class="anchor field">§</a>`value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan"><code>ValueWithSpan</code></a>

The value of the constant. Hint: you can unwrap the string value using `value.into_string()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Function" class="anchor">§</a>

### Function(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Function.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Function">Function</a>)

Scalar function call e.g. `LEFT(foo, 5)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Case" class="anchor">§</a>

### Case

`CASE [<operand>] WHEN <condition> THEN <result> ... [ELSE <result>] END`

Note we only recognize a complete single expression as `<condition>`, not `< 0` nor `1, 2, 3` as allowed in a `<simple when clause>` per <https://jakewheat.github.io/sql-overview/sql-2011-foundation-grammar.html#simple-when-clause>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Case.field.case_token" class="anchor field">§</a>`case_token: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken"><code>AttachedToken</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Case.field.end_token" class="anchor field">§</a>`end_token: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken"><code>AttachedToken</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Case.field.operand" class="anchor field">§</a>`operand: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Case.field.conditions" class="anchor field">§</a>`conditions: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CaseWhen.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CaseWhen"><code>CaseWhen</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Case.field.else_result" class="anchor field">§</a>`else_result: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Exists" class="anchor">§</a>

### Exists

An exists expression `[ NOT ] EXISTS(SELECT ...)`, used in expressions like `WHERE [ NOT ] EXISTS (SELECT ...)`.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Exists.field.subquery" class="anchor field">§</a>`subquery: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Exists.field.negated" class="anchor field">§</a>`negated: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Subquery" class="anchor">§</a>

### Subquery(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query">Query</a>\>)

A parenthesized subquery `(SELECT ...)`, used in expression like `SELECT (subquery) AS x` or `WHERE (subquery) = x`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.GroupingSets" class="anchor">§</a>

### GroupingSets(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>\>)

The `GROUPING SETS` expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Cube" class="anchor">§</a>

### Cube(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>\>)

The `CUBE` expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Rollup" class="anchor">§</a>

### Rollup(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>\>)

The `ROLLUP` expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Tuple" class="anchor">§</a>

### Tuple(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

ROW / TUPLE a single value, such as `SELECT (1, 2)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Struct" class="anchor">§</a>

### Struct

`Struct` literal expression Syntax:

``` sql
STRUCT<[field_name] field_type, ...>( expr1 [, ... ])

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#struct_type)
[Databricks](https://docs.databricks.com/en/sql/language-manual/functions/struct.html)
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Struct.field.values" class="anchor field">§</a>`values: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

Struct values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Struct.field.fields" class="anchor field">§</a>`fields: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.StructField.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::StructField"><code>StructField</code></a>`>`

Struct field definitions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Named" class="anchor">§</a>

### Named

`BigQuery` specific: An named expression in a typeless struct [1](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#struct_type)

Syntax

``` sql
1 AS A
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Named.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Named.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Dictionary" class="anchor">§</a>

### Dictionary(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.DictionaryField.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::DictionaryField">DictionaryField</a>\>)

`DuckDB` specific `Struct` literal expression [1](https://duckdb.org/docs/sql/data_types/struct#creating-structs)

Syntax:

``` sql
syntax: {'field_name': expr1[, ... ]}
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Map" class="anchor">§</a>

### Map(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Map.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Map">Map</a>)

`DuckDB` specific `Map` literal expression [1](https://duckdb.org/docs/sql/data_types/map#creating-maps)

Syntax:

``` sql
syntax: Map {key1: value1[, ... ]}
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Array" class="anchor">§</a>

### Array(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Array.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Array">Array</a>)

An array expression e.g. `ARRAY[1, 2]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Interval" class="anchor">§</a>

### Interval(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Interval">Interval</a>)

An interval expression e.g. `INTERVAL '1' YEAR`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.MatchAgainst" class="anchor">§</a>

### MatchAgainst

`MySQL` specific text search function [(1)](https://dev.mysql.com/doc/refman/8.0/en/fulltext-search.html#function_match).

Syntax:

``` sql
MATCH (<col>, <col>, ...) AGAINST (<expr> [<search modifier>])

<col> = CompoundIdentifier
<expr> = String literal
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.MatchAgainst.field.columns" class="anchor field">§</a>`columns: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

`(<col>, <col>, ...)`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.MatchAgainst.field.match_value" class="anchor field">§</a>`match_value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a>

`<expr>`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.MatchAgainst.field.opt_search_modifier" class="anchor field">§</a>`opt_search_modifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SearchModifier.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SearchModifier"><code>SearchModifier</code></a>`>`

`<search modifier>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Wildcard" class="anchor">§</a>

### Wildcard(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.QualifiedWildcard" class="anchor">§</a>

### QualifiedWildcard(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName">ObjectName</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>)

Qualified wildcard, e.g. `alias.*` or `schema.table.*`. (Same caveats apply to `QualifiedWildcard` as to `Wildcard`.)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.OuterJoin" class="anchor">§</a>

### OuterJoin(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

Some dialects support an older syntax for outer joins where columns are marked with the `(+)` operator in the WHERE clause, for example:

``` sql
SELECT t1.c1, t2.c2 FROM t1, t2 WHERE t1.c1 = t2.c2 (+)
```

which is equivalent to

``` sql
SELECT t1.c1, t2.c2 FROM t1 LEFT OUTER JOIN t2 ON t1.c1 = t2.c2
```

See <https://docs.snowflake.com/en/sql-reference/constructs/where#joins-in-the-where-clause>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Prior" class="anchor">§</a>

### Prior(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>)

A reference to the prior level in a CONNECT BY clause.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Lambda" class="anchor">§</a>

### Lambda(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.LambdaFunction.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::LambdaFunction">LambdaFunction</a>)

A lambda function.

Syntax:

``` plaintext
param -> expr | (param1, ...) -> expr
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/functions#higher-order-functions---operator-and-lambdaparams-expr-function) [Databricks](https://docs.databricks.com/en/sql/language-manual/sql-ref-lambda-functions.html) [DuckDb](https://duckdb.org/docs/sql/functions/lambda.html)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.MemberOf" class="anchor">§</a>

### MemberOf(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.MemberOf.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::MemberOf">MemberOf</a>)

Checks membership of a value in a JSON array

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.value" class="fn">value</a>(value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

Creates a new [`Expr::Value`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Value "variant datafusion::logical_expr::sqlparser::ast::Expr::Value")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Clone-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Debug-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Display-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-From%3CExpr%3E-for-FunctionArgExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FunctionArgExpr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FunctionArgExpr">FunctionArgExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(wildcard_expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FunctionArgExpr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FunctionArgExpr">FunctionArgExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Hash-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Ord-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-PartialEq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-PartialOrd-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Spanned-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Spanned">Spanned</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#partial-span" class="doc-anchor">§</a>partial span

Most expressions are missing keywords in their spans. f.e. `IS NULL <expr>` reports as `<expr>::span`.

Missing spans:

- [Expr::MatchAgainst](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.MatchAgainst "variant datafusion::logical_expr::sqlparser::ast::Expr::MatchAgainst") \# MySQL specific
- [Expr::RLike](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.RLike "variant datafusion::logical_expr::sqlparser::ast::Expr::RLike") \# MySQL specific
- [Expr::Struct](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Struct "variant datafusion::logical_expr::sqlparser::ast::Expr::Struct") \# BigQuery specific
- [Expr::Named](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Named "variant datafusion::logical_expr::sqlparser::ast::Expr::Named") \# BigQuery specific
- [Expr::Dictionary](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Dictionary "variant datafusion::logical_expr::sqlparser::ast::Expr::Dictionary") \# DuckDB specific
- [Expr::Map](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Map "variant datafusion::logical_expr::sqlparser::ast::Expr::Map") \# DuckDB specific
- [Expr::Lambda](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#variant.Lambda "variant datafusion::logical_expr::sqlparser::ast::Expr::Lambda")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.span" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html#tymethod.span" class="fn">span</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Span">Span</a>

Return the [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html "struct datafusion::logical_expr::sqlparser::tokenizer::Span") (the minimum and maximum [`Location`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Location.html "struct datafusion::logical_expr::sqlparser::tokenizer::Location")) for this AST node, by recursively combining the spans of its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Visit-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-VisitMut-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-Eq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#impl-StructuralPartialEq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html#blanket-implementations" class="anchor">§</a>
