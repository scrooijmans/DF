# Struct ListType Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/datatypes.rs.html#685-688" class="src">Source</a>

``` rust
pub struct ListType {
    pub element_field: NestedFieldRef,
}
```

Expand description

A list is a collection of values with some element type. The element field has an integer id that is unique in the table schema. Elements can be either optional or required. Element types may be any type.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#structfield.element_field" class="anchor field">§</a>`element_field: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef"><code>NestedFieldRef</code></a>

Element field of list type.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-ListType" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.new" class="fn">new</a>(element_field: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> Self

Construct a list type with the given element field.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-Clone-for-ListType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-Debug-for-ListType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-From%3CListType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-PartialEq-for-ListType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-Eq-for-ListType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#impl-StructuralPartialEq-for-ListType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html#blanket-implementations" class="anchor">§</a>
