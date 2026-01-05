# Struct Reference Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/term.rs.html#38-40" class="src">Source</a>

``` rust
pub struct Reference { /* private fields */ }
```

Expand description

A named reference in an unbound expression. For example, `a` in `a > 10`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Reference" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.new" class="fn">new</a>(name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Create a new unbound reference.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return the name of this reference.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Reference-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.less_than" class="fn">less_than</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an less than expression. For example, `a < 10`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").less_than(Datum::long(10));

assert_eq!(&format!("{expr}"), "a < 10");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.less_than_or_equal_to" class="fn">less_than_or_equal_to</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an less than or equal to expression. For example, `a <= 10`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-1" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").less_than_or_equal_to(Datum::long(10));

assert_eq!(&format!("{expr}"), "a <= 10");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.greater_than" class="fn">greater_than</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an greater than expression. For example, `a > 10`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-2" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").greater_than(Datum::long(10));

assert_eq!(&format!("{expr}"), "a > 10");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.greater_than_or_equal_to" class="fn">greater_than_or_equal_to</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates a greater-than-or-equal-to than expression. For example, `a >= 10`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-3" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").greater_than_or_equal_to(Datum::long(10));

assert_eq!(&format!("{expr}"), "a >= 10");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.equal_to" class="fn">equal_to</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an equal-to expression. For example, `a = 10`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-4" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").equal_to(Datum::long(10));

assert_eq!(&format!("{expr}"), "a = 10");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.not_equal_to" class="fn">not_equal_to</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates a not equal-to expression. For example, `a!= 10`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-5" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").not_equal_to(Datum::long(10));

assert_eq!(&format!("{expr}"), "a != 10");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.starts_with" class="fn">starts_with</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates a start-with expression. For example, `a STARTS WITH "foo"`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-6" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").starts_with(Datum::string("foo"));

assert_eq!(&format!("{expr}"), r#"a STARTS WITH "foo""#);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.not_starts_with" class="fn">not_starts_with</a>(self, datum: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates a not start-with expression. For example, `a NOT STARTS WITH 'foo'`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-7" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;

let expr = Reference::new("a").not_starts_with(Datum::string("foo"));

assert_eq!(&format!("{expr}"), r#"a NOT STARTS WITH "foo""#);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.is_nan" class="fn">is_nan</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an is-nan expression. For example, `a IS NAN`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-8" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").is_nan();

assert_eq!(&format!("{expr}"), "a IS NAN");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.is_not_nan" class="fn">is_not_nan</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an is-not-nan expression. For example, `a IS NOT NAN`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-9" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").is_not_nan();

assert_eq!(&format!("{expr}"), "a IS NOT NAN");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.is_null" class="fn">is_null</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an is-null expression. For example, `a IS NULL`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-10" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").is_null();

assert_eq!(&format!("{expr}"), "a IS NULL");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.is_not_null" class="fn">is_not_null</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an is-not-null expression. For example, `a IS NOT NULL`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-11" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").is_not_null();

assert_eq!(&format!("{expr}"), "a IS NOT NULL");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.is_in" class="fn">is_in</a>(self, literals: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an is-in expression. For example, `a IN (5, 6)`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-12" class="doc-anchor">§</a>Example

``` rust
use fnv::FnvHashSet;
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").is_in([Datum::long(5), Datum::long(6)]);

let as_string = format!("{expr}");
assert!(&as_string == "a IN (5, 6)" || &as_string == "a IN (6, 5)");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.is_not_in" class="fn">is_not_in</a>(self, literals: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Creates an is-not-in expression. For example, `a NOT IN (5, 6)`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#example-13" class="doc-anchor">§</a>Example

``` rust
use fnv::FnvHashSet;
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr = Reference::new("a").is_not_in([Datum::long(5), Datum::long(6)]);

let as_string = format!("{expr}");
assert!(&as_string == "a NOT IN (5, 6)" || &as_string == "a NOT IN (6, 5)");
```

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Bind-for-Reference" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#associatedtype.Bound" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BoundReference.html" class="struct" title="struct iceberg::expr::BoundReference">BoundReference</a>

The type of the bound result.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.bind" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#tymethod.bind" class="fn">bind</a>(&self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

Bind an expression to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Clone-for-Reference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Debug-for-Reference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Deserialize%3C&#39;de%3E-for-Reference" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Display-for-Reference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-PartialEq-for-Reference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-Serialize-for-Reference" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#impl-StructuralPartialEq-for-Reference" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html#blanket-implementations" class="anchor">§</a>
