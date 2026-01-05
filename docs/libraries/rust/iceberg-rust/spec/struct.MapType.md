# Struct MapType Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/datatypes.rs.html#807-812" class="src">Source</a>

``` rust
pub struct MapType {
    pub key_field: NestedFieldRef,
    pub value_field: NestedFieldRef,
}
```

Expand description

A map is a collection of key-value pairs with a key type and a value type. Both the key field and value field each have an integer id that is unique in the table schema. Map keys are required and map values can be either optional or required. Both map keys and map values may be any type, including nested types.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#structfield.key_field" class="anchor field">§</a>`key_field: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef"><code>NestedFieldRef</code></a>

Field for key.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#structfield.value_field" class="anchor field">§</a>`value_field: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef"><code>NestedFieldRef</code></a>

Field for value.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-MapType" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.new" class="fn">new</a>(key_field: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>, value_field: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> Self

Construct a map type with the given key and value fields.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-Clone-for-MapType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-Debug-for-MapType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-From%3CMapType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-PartialEq-for-MapType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-Eq-for-MapType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#impl-StructuralPartialEq-for-MapType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html#blanket-implementations" class="anchor">§</a>
