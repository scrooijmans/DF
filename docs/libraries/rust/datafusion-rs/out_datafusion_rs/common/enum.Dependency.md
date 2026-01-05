# Enum Dependency Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/functional_dependencies.rs.html#153" class="src">Source</a>

``` rust
pub enum Dependency {
    Single,
    Multi,
}
```

Expand description

Describes functional dependency mode.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#variant.Single" class="anchor">§</a>

### Single

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#variant.Multi" class="anchor">§</a>

### Multi

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#impl-Clone-for-Dependency" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#impl-Debug-for-Dependency" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#impl-PartialEq-for-Dependency" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#impl-Copy-for-Dependency" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#impl-Eq-for-Dependency" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#impl-StructuralPartialEq-for-Dependency" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html#blanket-implementations" class="anchor">§</a>
