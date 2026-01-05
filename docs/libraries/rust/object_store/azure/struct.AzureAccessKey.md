# Struct AzureAccessKey Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/credential.rs.html#112" class="src">Source</a>

``` rust
pub struct AzureAccessKey(/* private fields */);
```

Available on **crate feature `azure`** only.

Expand description

A shared Azure Storage Account Key

## Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#impl-AzureAccessKey" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#method.try_new" class="fn">try_new</a>(key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Error\>

Create a new [`AzureAccessKey`](https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html "struct object_store::azure::AzureAccessKey"), checking it for validity

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#impl-Clone-for-AzureAccessKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#impl-Debug-for-AzureAccessKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#impl-PartialEq-for-AzureAccessKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#impl-Eq-for-AzureAccessKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#impl-StructuralPartialEq-for-AzureAccessKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html#blanket-implementations" class="anchor">§</a>
