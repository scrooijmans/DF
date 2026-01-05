# Struct AttachedToken Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/helpers/attached_token.rs.html#83" class="src">Source</a>

``` rust
pub struct AttachedToken(pub TokenWithSpan);
```

Expand description

A wrapper over [`TokenWithSpan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html "struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan")s that ignores the token and source location in comparisons and hashing.

This type is used when the token and location is not relevant for semantics, but is still needed for accurate source location tracking, for example, in the nodes in the [ast](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/index.html "mod datafusion::logical_expr::sqlparser::ast") module.

Note: **All** `AttachedTokens` are equal.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#examples" class="doc-anchor">§</a>Examples

Same token, different location are equal

``` rust
// commas @ line 1, column 10
let tok1 = TokenWithLocation::new(
  Token::Comma,
  Span::new(Location::new(1, 10), Location::new(1, 11)),
);
// commas @ line 2, column 20
let tok2 = TokenWithLocation::new(
  Token::Comma,
  Span::new(Location::new(2, 20), Location::new(2, 21)),
);

assert_ne!(tok1, tok2); // token with locations are *not* equal
assert_eq!(AttachedToken(tok1), AttachedToken(tok2)); // attached tokens are
```

Different token, different location are equal 🤯

``` rust
// commas @ line 1, column 10
let tok1 = TokenWithLocation::new(
  Token::Comma,
  Span::new(Location::new(1, 10), Location::new(1, 11)),
);
// period @ line 2, column 20
let tok2 = TokenWithLocation::new(
 Token::Period,
  Span::new(Location::new(2, 10), Location::new(2, 21)),
);

assert_ne!(tok1, tok2); // token with locations are *not* equal
assert_eq!(AttachedToken(tok1), AttachedToken(tok2)); // attached tokens are
```

// period @ line 2, column 20

## Tuple Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan"><code>TokenWithSpan</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-AttachedToken" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

Return a new Empty AttachedToken

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-Clone-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-Debug-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-From%3CAttachedToken%3E-for-TokenWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-From%3CTokenWithSpan%3E-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-Hash-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, \_state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-Ord-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, \_: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-PartialEq-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, \_: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-PartialOrd-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-Visit-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-VisitMut-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#impl-Eq-for-AttachedToken" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::attached_token::AttachedToken">AttachedToken</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/attached_token/struct.AttachedToken.html#blanket-implementations" class="anchor">§</a>
