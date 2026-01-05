# Struct UpdateVersion Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1153-1160" class="src">Source</a>

``` rust
pub struct UpdateVersion {
    pub e_tag: Option<String>,
    pub version: Option<String>,
}
```

Expand description

Uniquely identifies a version of an object to update

Stores will use differing combinations of `e_tag` and `version` to provide conditional updates, and it is therefore recommended applications preserve both

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#structfield.e_tag" class="anchor field">§</a>`e_tag: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The unique identifier for the newly created object

<https://datatracker.ietf.org/doc/html/rfc9110#name-etag>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#structfield.version" class="anchor field">§</a>`version: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

A version indicator for the newly created object

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#impl-Clone-for-UpdateVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#impl-Debug-for-UpdateVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#impl-From%3CPutResult%3E-for-UpdateVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#impl-PartialEq-for-UpdateVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#impl-Eq-for-UpdateVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#impl-StructuralPartialEq-for-UpdateVersion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html#blanket-implementations" class="anchor">§</a>
