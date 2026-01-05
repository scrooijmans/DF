# Struct Span Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/spans.rs.html#53" class="src">Source</a>

``` rust
pub struct Span {
    pub start: Location,
    pub end: Location,
}
```

Expand description

Represents an interval of characters in the original SQL query.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#structfield.start" class="anchor field">§</a>`start: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Location.html" class="struct" title="struct datafusion::common::Location"><code>Location</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#structfield.end" class="anchor field">§</a>`end: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Location.html" class="struct" title="struct datafusion::common::Location"><code>Location</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Span" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.new" class="fn">new</a>(start: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Location.html" class="struct" title="struct datafusion::common::Location">Location</a>, end: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Location.html" class="struct" title="struct datafusion::common::Location">Location</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

Creates a new [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html "struct datafusion::common::Span") from a start and an end [`Location`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Location.html "struct datafusion::common::Location").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.try_from_sqlparser_span" class="fn">try_from_sqlparser_span</a>(span: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Span">Span</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>

Convert a [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html "struct datafusion::logical_expr::sqlparser::tokenizer::Span") from the parser, into a DataFusion [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html "struct datafusion::common::Span"). If the input span is empty (line 0 column 0, to line 0 column 0), then [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") is returned.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.union" class="fn">union</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

Returns the smallest Span that contains both `self` and `other`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#examples" class="doc-anchor">§</a>Examples

``` rust
// line 1, column1 -> line 2, column 5
let span1 = Span::new(Location::new(1, 1), Location::new(2, 5));
// line 2, column 3 -> line 3, column 7
let span2 = Span::new(Location::new(2, 3), Location::new(3, 7));
// Union of the two is the min/max of the two spans
// line 1, column 1 -> line 3, column 7
let union = span1.union(&span2);
assert_eq!(union, Span::new(Location::new(1, 1), Location::new(3, 7)));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.union_opt" class="fn">union_opt</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

Same as [Span::union](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.union "method datafusion::common::Span::union") for `Option<Span>`.

If `other` is `None`, `self` is returned.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.union_iter" class="fn">union_iter</a>\<I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>,

Return the [Span::union](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.union "method datafusion::common::Span::union") of all spans in the iterator.

If the iterator is empty, [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") is returned.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#example" class="doc-anchor">§</a>Example

``` rust
let spans = vec![
    Span::new(Location::new(1, 1), Location::new(2, 5)),
    Span::new(Location::new(2, 3), Location::new(3, 7)),
    Span::new(Location::new(3, 1), Location::new(4, 2)),
];
// line 1, column 1 -> line 4, column 2
assert_eq!(
  Span::union_iter(spans),
  Span::new(Location::new(1, 1), Location::new(4, 2))
);
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Clone-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Debug-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Hash-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Ord-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-PartialEq-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-PartialOrd-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Copy-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-Eq-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#impl-StructuralPartialEq-for-Span" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html#blanket-implementations" class="anchor">§</a>
