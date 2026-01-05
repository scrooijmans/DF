# Struct Namespace Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#194-197" class="src">Source</a>

``` rust
pub struct Namespace { /* private fields */ }
```

Expand description

Namespace represents a namespace in the catalog.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#impl-Namespace" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>) -\> Self

Create a new namespace.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.with_properties" class="fn">with_properties</a>( name: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Create a new namespace with properties.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>

Get the name of the namespace.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.properties" class="fn">properties</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get the properties of the namespace.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#impl-Clone-for-Namespace" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#impl-Debug-for-Namespace" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#impl-PartialEq-for-Namespace" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#impl-Eq-for-Namespace" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#impl-StructuralPartialEq-for-Namespace" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html#blanket-implementations" class="anchor">§</a>
