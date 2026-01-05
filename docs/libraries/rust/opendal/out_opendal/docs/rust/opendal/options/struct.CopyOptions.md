# Struct CopyOptions Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/options.rs.html#528-544" class="src">Source</a>

``` rust
pub struct CopyOptions {
    pub if_not_exists: bool,
}
```

Expand description

Options for copy operations.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#structfield.if_not_exists" class="anchor field">Â§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Sets the condition that copy operation will succeed only if target does not exist.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#capability" class="doc-anchor">Â§</a>Capability

Check \[`Capability::copy_with_if_not_exists`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#behavior" class="doc-anchor">Â§</a>Behavior

- If supported, the copy operation will only succeed if the target path does not exist
- Will return error if target already exists
- If not supported, the value will be ignored

This operation provides a way to ensure copy operations only create new resources without overwriting existing ones, useful for implementing â€œcopy if not existsâ€? logic.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#impl-Clone-for-CopyOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#impl-Debug-for-CopyOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#impl-Default-for-CopyOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#impl-PartialEq-for-CopyOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#impl-Eq-for-CopyOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#impl-StructuralPartialEq-for-CopyOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html#blanket-implementations" class="anchor">Â§</a>
