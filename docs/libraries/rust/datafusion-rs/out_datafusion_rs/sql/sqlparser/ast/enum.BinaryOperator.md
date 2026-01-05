# Enum BinaryOperator Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/operator.rs.html#99" class="src">Source</a>

``` rust
pub enum BinaryOperator {
Show 69 variants    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    StringConcat,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Spaceship,
    Eq,
    NotEq,
    And,
    Or,
    Xor,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    DuckIntegerDivide,
    MyIntegerDivide,
    Match,
    Regexp,
    Custom(String),
    PGBitwiseXor,
    PGBitwiseShiftLeft,
    PGBitwiseShiftRight,
    PGExp,
    PGOverlap,
    PGRegexMatch,
    PGRegexIMatch,
    PGRegexNotMatch,
    PGRegexNotIMatch,
    PGLikeMatch,
    PGILikeMatch,
    PGNotLikeMatch,
    PGNotILikeMatch,
    PGStartsWith,
    Arrow,
    LongArrow,
    HashArrow,
    HashLongArrow,
    AtAt,
    AtArrow,
    ArrowAt,
    HashMinus,
    AtQuestion,
    Question,
    QuestionAnd,
    QuestionPipe,
    PGCustomBinaryOperator(Vec<String>),
    Overlaps,
    DoubleHash,
    LtDashGt,
    AndLt,
    AndGt,
    LtLtPipe,
    PipeGtGt,
    AndLtPipe,
    PipeAndGt,
    LtCaret,
    GtCaret,
    QuestionHash,
    QuestionDash,
    QuestionDashPipe,
    QuestionDoublePipe,
    At,
    TildeEq,
    Assignment,
}
```

Expand description

Binary operators

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Plus" class="anchor">§</a>

### Plus

Plus, e.g. `a + b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Minus" class="anchor">§</a>

### Minus

Minus, e.g. `a - b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Multiply" class="anchor">§</a>

### Multiply

Multiply, e.g. `a * b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Divide" class="anchor">§</a>

### Divide

Divide, e.g. `a / b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Modulo" class="anchor">§</a>

### Modulo

Modulo, e.g. `a % b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.StringConcat" class="anchor">§</a>

### StringConcat

String/Array Concat operator, e.g. `a || b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Gt" class="anchor">§</a>

### Gt

Greater than, e.g. `a > b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Lt" class="anchor">§</a>

### Lt

Less than, e.g. `a < b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.GtEq" class="anchor">§</a>

### GtEq

Greater equal, e.g. `a >= b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.LtEq" class="anchor">§</a>

### LtEq

Less equal, e.g. `a <= b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Spaceship" class="anchor">§</a>

### Spaceship

Spaceship, e.g. `a <=> b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Eq" class="anchor">§</a>

### Eq

Equal, e.g. `a = b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.NotEq" class="anchor">§</a>

### NotEq

Not equal, e.g. `a <> b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.And" class="anchor">§</a>

### And

And, e.g. `a AND b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Or" class="anchor">§</a>

### Or

Or, e.g. `a OR b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Xor" class="anchor">§</a>

### Xor

XOR, e.g. `a XOR b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.BitwiseOr" class="anchor">§</a>

### BitwiseOr

Bitwise or, e.g. `a | b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.BitwiseAnd" class="anchor">§</a>

### BitwiseAnd

Bitwise and, e.g. `a & b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.BitwiseXor" class="anchor">§</a>

### BitwiseXor

Bitwise XOR, e.g. `a ^ b`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.DuckIntegerDivide" class="anchor">§</a>

### DuckIntegerDivide

Integer division operator `//` in DuckDB

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.MyIntegerDivide" class="anchor">§</a>

### MyIntegerDivide

MySQL [`DIV`](https://dev.mysql.com/doc/refman/8.0/en/arithmetic-functions.html) integer division

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Match" class="anchor">§</a>

### Match

MATCH operator, e.g. `a MATCH b` (SQLite-specific) See <https://www.sqlite.org/lang_expr.html#the_like_glob_regexp_match_and_extract_operators>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Regexp" class="anchor">§</a>

### Regexp

REGEXP operator, e.g. `a REGEXP b` (SQLite-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Custom" class="anchor">§</a>

### Custom(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Support for custom operators (such as Postgres custom operators)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGBitwiseXor" class="anchor">§</a>

### PGBitwiseXor

Bitwise XOR, e.g. `a # b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGBitwiseShiftLeft" class="anchor">§</a>

### PGBitwiseShiftLeft

Bitwise shift left, e.g. `a << b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGBitwiseShiftRight" class="anchor">§</a>

### PGBitwiseShiftRight

Bitwise shift right, e.g. `a >> b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGExp" class="anchor">§</a>

### PGExp

Exponent, e.g. `a ^ b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGOverlap" class="anchor">§</a>

### PGOverlap

Overlap operator, e.g. `a && b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGRegexMatch" class="anchor">§</a>

### PGRegexMatch

String matches regular expression (case sensitively), e.g. `a ~ b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGRegexIMatch" class="anchor">§</a>

### PGRegexIMatch

String matches regular expression (case insensitively), e.g. `a ~* b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGRegexNotMatch" class="anchor">§</a>

### PGRegexNotMatch

String does not match regular expression (case sensitively), e.g. `a !~ b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGRegexNotIMatch" class="anchor">§</a>

### PGRegexNotIMatch

String does not match regular expression (case insensitively), e.g. `a !~* b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGLikeMatch" class="anchor">§</a>

### PGLikeMatch

String matches pattern (case sensitively), e.g. `a ~~ b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGILikeMatch" class="anchor">§</a>

### PGILikeMatch

String matches pattern (case insensitively), e.g. `a ~~* b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGNotLikeMatch" class="anchor">§</a>

### PGNotLikeMatch

String does not match pattern (case sensitively), e.g. `a !~~ b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGNotILikeMatch" class="anchor">§</a>

### PGNotILikeMatch

String does not match pattern (case insensitively), e.g. `a !~~* b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGStartsWith" class="anchor">§</a>

### PGStartsWith

String “starts with”, eg: `a ^@ b` (PostgreSQL-specific)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Arrow" class="anchor">§</a>

### Arrow

The `->` operator.

On PostgreSQL, this operator extracts a JSON object field or array element, for example `'{"a":"b"}'::json -> 'a'` or `[1, 2, 3]'::json -> 2`.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.LongArrow" class="anchor">§</a>

### LongArrow

The `->>` operator.

On PostgreSQL, this operator extracts a JSON object field or JSON array element and converts it to text, for example `'{"a":"b"}'::json ->> 'a'` or `[1, 2, 3]'::json ->> 2`.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.HashArrow" class="anchor">§</a>

### HashArrow

The `#>` operator.

On PostgreSQL, this operator extracts a JSON sub-object at the specified path, for example:

``` notrust
'{"a": {"b": ["foo","bar"]}}'::json #> '{a,b,1}'
```

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.HashLongArrow" class="anchor">§</a>

### HashLongArrow

The `#>>` operator.

A PostgreSQL-specific operator that extracts JSON sub-object at the specified path, for example

``` notrust
'{"a": {"b": ["foo","bar"]}}'::json #>> '{a,b,1}'
```

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.AtAt" class="anchor">§</a>

### AtAt

The `@@` operator.

On PostgreSQL, this is used for JSON and text searches.

See <https://www.postgresql.org/docs/current/functions-json.html>. See <https://www.postgresql.org/docs/current/functions-textsearch.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.AtArrow" class="anchor">§</a>

### AtArrow

The `@>` operator.

On PostgreSQL, this is used for JSON and text searches.

See <https://www.postgresql.org/docs/current/functions-json.html>. See <https://www.postgresql.org/docs/current/functions-textsearch.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.ArrowAt" class="anchor">§</a>

### ArrowAt

The `<@` operator.

On PostgreSQL, this is used for JSON and text searches.

See <https://www.postgresql.org/docs/current/functions-json.html>. See <https://www.postgresql.org/docs/current/functions-textsearch.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.HashMinus" class="anchor">§</a>

### HashMinus

The `#-` operator.

On PostgreSQL, this operator is used to delete a field or array element at a specified path.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.AtQuestion" class="anchor">§</a>

### AtQuestion

The `@?` operator.

On PostgreSQL, this operator is used to check the given JSON path returns an item for the JSON value.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Question" class="anchor">§</a>

### Question

The `?` operator.

On PostgreSQL, this operator is used to check whether a string exists as a top-level key within the JSON value

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.QuestionAnd" class="anchor">§</a>

### QuestionAnd

The `?&` operator.

On PostgreSQL, this operator is used to check whether all of the the indicated array members exist as top-level keys.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.QuestionPipe" class="anchor">§</a>

### QuestionPipe

The `?|` operator.

On PostgreSQL, this operator is used to check whether any of the the indicated array members exist as top-level keys.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PGCustomBinaryOperator" class="anchor">§</a>

### PGCustomBinaryOperator(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

PostgreSQL-specific custom operator.

See [CREATE OPERATOR](https://www.postgresql.org/docs/current/sql-createoperator.html) for more information.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Overlaps" class="anchor">§</a>

### Overlaps

The `OVERLAPS` operator

Specifies a test for an overlap between two datetime periods: <https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#overlaps-predicate>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.DoubleHash" class="anchor">§</a>

### DoubleHash

`##` Point of closest proximity (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.LtDashGt" class="anchor">§</a>

### LtDashGt

`<->` Distance between (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.AndLt" class="anchor">§</a>

### AndLt

`&<` Overlaps to left? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.AndGt" class="anchor">§</a>

### AndGt

`&>` Overlaps to right? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.LtLtPipe" class="anchor">§</a>

### LtLtPipe

`<<|` Is strictly below? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PipeGtGt" class="anchor">§</a>

### PipeGtGt

`|>>` Is strictly above? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.AndLtPipe" class="anchor">§</a>

### AndLtPipe

`&<|` Does not extend above? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.PipeAndGt" class="anchor">§</a>

### PipeAndGt

`|&>` Does not extend below? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.LtCaret" class="anchor">§</a>

### LtCaret

`<^` Is below? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.GtCaret" class="anchor">§</a>

### GtCaret

`>^` Is above? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.QuestionHash" class="anchor">§</a>

### QuestionHash

`?#` Intersects? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.QuestionDash" class="anchor">§</a>

### QuestionDash

`?-` Is horizontal? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.QuestionDashPipe" class="anchor">§</a>

### QuestionDashPipe

`?-|` Is perpendicular? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.QuestionDoublePipe" class="anchor">§</a>

### QuestionDoublePipe

`?||` Are Parallel? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.At" class="anchor">§</a>

### At

`@` Contained or on? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.TildeEq" class="anchor">§</a>

### TildeEq

`~=` Same as? (PostgreSQL/Redshift geometric operator) See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#variant.Assignment" class="anchor">§</a>

### Assignment

‘:=’ Assignment Operator See <https://dev.mysql.com/doc/refman/8.4/en/assignment-operators.html#operator_assign-value>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Clone-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Debug-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Display-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Hash-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Ord-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-PartialEq-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-PartialOrd-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Visit-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-VisitMut-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-Eq-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#impl-StructuralPartialEq-for-BinaryOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/enum.BinaryOperator.html#blanket-implementations" class="anchor">§</a>
