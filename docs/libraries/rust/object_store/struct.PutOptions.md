# Struct PutOptions Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1173-1191" class="src">Source</a>

``` rust
pub struct PutOptions {
    pub mode: PutMode,
    pub tags: TagSet,
    pub attributes: Attributes,
    pub extensions: Extensions,
}
```

Expand description

Options for a put request

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#structfield.mode" class="anchor field">§</a>`mode: `<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode"><code>PutMode</code></a>

Configure the [`PutMode`](https://docs.rs/object_store/latest/object_store/enum.PutMode.html "enum object_store::PutMode") for this operation

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#structfield.tags" class="anchor field">§</a>`tags: `<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet"><code>TagSet</code></a>

Provide a [`TagSet`](https://docs.rs/object_store/latest/object_store/struct.TagSet.html "struct object_store::TagSet") for this object

Implementations that don’t support object tagging should ignore this

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#structfield.attributes" class="anchor field">§</a>`attributes: `<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes"><code>Attributes</code></a>

Provide a set of [`Attributes`](https://docs.rs/object_store/latest/object_store/struct.Attributes.html "struct object_store::Attributes")

Implementations that don’t support an attribute should return an error

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#structfield.extensions" class="anchor field">§</a>`extensions: `<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions"><code>Extensions</code></a>

Implementation-specific extensions. Intended for use by [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementations that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

They are also eclused from [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") and [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq").

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-Clone-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-Debug-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-Default-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-From%3CAttributes%3E-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(attributes: <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-From%3CPutMode%3E-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(mode: <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-From%3CTagSet%3E-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(tags: <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-PartialEq-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#impl-Eq-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html#blanket-implementations" class="anchor">§</a>
