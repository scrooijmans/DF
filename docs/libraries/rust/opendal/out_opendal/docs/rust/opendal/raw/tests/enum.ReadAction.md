# Enum ReadAction Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/tests/read.rs.html#28-35" class="src">Source</a>

``` rust
pub enum ReadAction {
    Read(usize, usize),
}
```

Available on **crate feature `tests`** only.

Expand description

ReadAction represents a read action.

## Variants<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#variants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#variant.Read" class="anchor">Â§</a>

### Read(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Read represents a read action with given input buf size.

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#note" class="doc-anchor">Â§</a>NOTE

The size is the input buf size, itâ€™s possible that the actual read size is smaller.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#impl-Clone-for-ReadAction" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#impl-Debug-for-ReadAction" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#impl-PartialEq-for-ReadAction" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#impl-Copy-for-ReadAction" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#impl-Eq-for-ReadAction" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#impl-StructuralPartialEq-for-ReadAction" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html#blanket-implementations" class="anchor">Â§</a>
