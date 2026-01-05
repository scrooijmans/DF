# Enum Operator Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/operator.rs.html#22" class="src">Source</a>

``` rust
pub enum Operator {
Show 42 variants    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    IsDistinctFrom,
    IsNotDistinctFrom,
    RegexMatch,
    RegexIMatch,
    RegexNotMatch,
    RegexNotIMatch,
    LikeMatch,
    ILikeMatch,
    NotLikeMatch,
    NotILikeMatch,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftRight,
    BitwiseShiftLeft,
    StringConcat,
    AtArrow,
    ArrowAt,
    Arrow,
    LongArrow,
    HashArrow,
    HashLongArrow,
    AtAt,
    IntegerDivide,
    HashMinus,
    AtQuestion,
    Question,
    QuestionAnd,
    QuestionPipe,
}
```

Expand description

Operators applied to expressions

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Eq" class="anchor">§</a>

### Eq

Expressions are equal

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.NotEq" class="anchor">§</a>

### NotEq

Expressions are not equal

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Lt" class="anchor">§</a>

### Lt

Left side is smaller than right side

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.LtEq" class="anchor">§</a>

### LtEq

Left side is smaller or equal to right side

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Gt" class="anchor">§</a>

### Gt

Left side is greater than right side

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.GtEq" class="anchor">§</a>

### GtEq

Left side is greater or equal to right side

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Plus" class="anchor">§</a>

### Plus

Addition

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Minus" class="anchor">§</a>

### Minus

Subtraction

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Multiply" class="anchor">§</a>

### Multiply

Multiplication operator, like `*`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Divide" class="anchor">§</a>

### Divide

Division operator, like `/`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Modulo" class="anchor">§</a>

### Modulo

Remainder operator, like `%`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.And" class="anchor">§</a>

### And

Logical AND, like `&&`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Or" class="anchor">§</a>

### Or

Logical OR, like `||`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.IsDistinctFrom" class="anchor">§</a>

### IsDistinctFrom

`IS DISTINCT FROM` (see [`distinct`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cmp/fn.distinct.html "fn datafusion::common::arrow::compute::kernels::cmp::distinct"))

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.IsNotDistinctFrom" class="anchor">§</a>

### IsNotDistinctFrom

`IS NOT DISTINCT FROM` (see [`not_distinct`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cmp/fn.not_distinct.html "fn datafusion::common::arrow::compute::kernels::cmp::not_distinct"))

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.RegexMatch" class="anchor">§</a>

### RegexMatch

Case sensitive regex match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.RegexIMatch" class="anchor">§</a>

### RegexIMatch

Case insensitive regex match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.RegexNotMatch" class="anchor">§</a>

### RegexNotMatch

Case sensitive regex not match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.RegexNotIMatch" class="anchor">§</a>

### RegexNotIMatch

Case insensitive regex not match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.LikeMatch" class="anchor">§</a>

### LikeMatch

Case sensitive pattern match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.ILikeMatch" class="anchor">§</a>

### ILikeMatch

Case insensitive pattern match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.NotLikeMatch" class="anchor">§</a>

### NotLikeMatch

Case sensitive pattern not match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.NotILikeMatch" class="anchor">§</a>

### NotILikeMatch

Case insensitive pattern not match

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.BitwiseAnd" class="anchor">§</a>

### BitwiseAnd

Bitwise and, like `&`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.BitwiseOr" class="anchor">§</a>

### BitwiseOr

Bitwise or, like `|`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.BitwiseXor" class="anchor">§</a>

### BitwiseXor

Bitwise xor, such as `^` in MySQL or `#` in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.BitwiseShiftRight" class="anchor">§</a>

### BitwiseShiftRight

Bitwise right, like `>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.BitwiseShiftLeft" class="anchor">§</a>

### BitwiseShiftLeft

Bitwise left, like `<<`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.StringConcat" class="anchor">§</a>

### StringConcat

String concat

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.AtArrow" class="anchor">§</a>

### AtArrow

At arrow, like `@>`.

Currently only supported to be used with lists:

``` sql
select [1,3] <@ [1,2,3]
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.ArrowAt" class="anchor">§</a>

### ArrowAt

Arrow at, like `<@`.

Currently only supported to be used with lists:

``` sql
select [1,2,3] @> [1,3]
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Arrow" class="anchor">§</a>

### Arrow

Arrow, like `->`.

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.LongArrow" class="anchor">§</a>

### LongArrow

Long arrow, like `->>`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.HashArrow" class="anchor">§</a>

### HashArrow

Hash arrow, like `#>`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.HashLongArrow" class="anchor">§</a>

### HashLongArrow

Hash long arrow, like `#>>`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.AtAt" class="anchor">§</a>

### AtAt

At at, like `@@`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.IntegerDivide" class="anchor">§</a>

### IntegerDivide

Integer division operator, like `DIV` from MySQL or `//` from DuckDB

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.HashMinus" class="anchor">§</a>

### HashMinus

Hash Minis, like `#-`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.AtQuestion" class="anchor">§</a>

### AtQuestion

At question, like `@?`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.Question" class="anchor">§</a>

### Question

Question, like `?`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.QuestionAnd" class="anchor">§</a>

### QuestionAnd

Question and, like `?&`

Not implemented in DataFusion yet.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#variant.QuestionPipe" class="anchor">§</a>

### QuestionPipe

Question pipe, like `?|`

Not implemented in DataFusion yet.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Operator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.negate" class="fn">negate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>\>

If the operator can be negated, return the negated operator otherwise return None

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.is_numerical_operators" class="fn">is_numerical_operators</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the operator is a numerical operator.

For example, ‘Binary(a, +, b)’ would be a numerical expression. PostgresSQL concept: <https://www.postgresql.org/docs/7.0/operators2198.htm>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.supports_propagation" class="fn">supports_propagation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the comparison operator can be used in interval arithmetic and constraint propagation

For example, ‘Binary(a, \>, b)’ expression supports propagation.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.is_logic_operator" class="fn">is_logic_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the operator is a logic operator.

For example, ‘Binary(Binary(a, \>, b), AND, Binary(a, \<, b + 3))’ would be a logical expression.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.swap" class="fn">swap</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>\>

Return the operator where swapping lhs and rhs wouldn’t change the result.

For example `Binary(50, >=, a)` could also be represented as `Binary(a, <=, 50)`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.precedence" class="fn">precedence</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the operator precedence use <https://www.postgresql.org/docs/7.2/sql-precedence.html> as a reference

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.returns_null_on_null" class="fn">returns_null_on_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the `Expr::BinaryOperator` with this operator is guaranteed to return null if either side is null.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Clone-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Debug-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Display-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Hash-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-PartialEq-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-PartialOrd-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Copy-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-Eq-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#impl-StructuralPartialEq-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html#blanket-implementations" class="anchor">§</a>
