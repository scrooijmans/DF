# Struct ValueWithSpan Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/value.rs.html#70" class="src">Source</a>

``` rust
pub struct ValueWithSpan {
    pub value: Value,
    pub span: Span,
}
```

Expand description

Wraps a primitive SQL [`Value`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html "enum datafusion::logical_expr::sqlparser::ast::Value") with its [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html "struct datafusion::logical_expr::sqlparser::tokenizer::Span") location

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#example-create-a-valuewithspan-from-a-value" class="doc-anchor">§</a>Example: create a `ValueWithSpan` from a `Value`

``` rust
let value = Value::SingleQuotedString(String::from("endpoint"));
// from line 1, column 1 to line 1, column 7
let span = Span::new(Location::new(1, 1), Location::new(1, 7));
let value_with_span = value.with_span(span);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#example-create-a-valuewithspan-from-a-value-with-an-empty-span" class="doc-anchor">§</a>Example: create a `ValueWithSpan` from a `Value` with an empty span

You can call [`Value::with_empty_span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html#method.with_empty_span "method datafusion::logical_expr::sqlparser::ast::Value::with_empty_span") to create a `ValueWithSpan` with an empty span

``` rust
let value = Value::SingleQuotedString(String::from("endpoint"));
let value_with_span = value.with_empty_span();
assert_eq!(value_with_span.span, Span::empty());
```

You can also use the [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") trait to convert `ValueWithSpan` to/from `Value`s

``` rust
let value = Value::SingleQuotedString(String::from("endpoint"));
// converting `Value` to `ValueWithSpan` results in an empty span
let value_with_span: ValueWithSpan = value.into();
assert_eq!(value_with_span.span, Span::empty());
// convert back to `Value`
let value: Value = value_with_span.into();
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#structfield.value" class="anchor field">§</a>`value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#structfield.span" class="anchor field">§</a>`span: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Span"><code>Span</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.into_string" class="fn">into_string</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

If the underlying literal is a string, regardless of quote style, returns the associated string value

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Clone-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Debug-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Display-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-From%3CValue%3E-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value">Value</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value">Value</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-From%3CValueWithSpan%3E-for-Value" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value">Value</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value">Value</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Hash-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Ord-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-PartialEq-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-PartialOrd-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Spanned-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Spanned">Spanned</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.span" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html#tymethod.span" class="fn">span</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Span">Span</a>

Return the [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html "struct datafusion::logical_expr::sqlparser::tokenizer::Span") (the minimum and maximum [`Location`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Location.html "struct datafusion::logical_expr::sqlparser::tokenizer::Location")) for this AST node, by recursively combining the spans of its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Visit-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-VisitMut-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#impl-Eq-for-ValueWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ValueWithSpan">ValueWithSpan</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ValueWithSpan.html#blanket-implementations" class="anchor">§</a>
