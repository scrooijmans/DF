# Enum PutMode Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1136-1146" class="src">Source</a>

``` rust
pub enum PutMode {
    Overwrite,
    Create,
    Update(UpdateVersion),
}
```

Expand description

Configure preconditions for the put operation

## Variants<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#variants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#variant.Overwrite" class="anchor">§</a>

### Overwrite

Perform an atomic write operation, overwriting any object present at the provided path

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#variant.Create" class="anchor">§</a>

### Create

Perform an atomic write operation, returning [`Error::AlreadyExists`](https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.AlreadyExists "variant object_store::Error::AlreadyExists") if an object already exists at the provided path

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#variant.Update" class="anchor">§</a>

### Update(<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>)

Perform an atomic write operation if the current version of the object matches the provided [`UpdateVersion`](https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html "struct object_store::UpdateVersion"), returning [`Error::Precondition`](https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Precondition "variant object_store::Error::Precondition") otherwise

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-Clone-for-PutMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-Debug-for-PutMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-Default-for-PutMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-From%3CPutMode%3E-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(mode: <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-PartialEq-for-PutMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-Eq-for-PutMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#impl-StructuralPartialEq-for-PutMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html#blanket-implementations" class="anchor">§</a>
