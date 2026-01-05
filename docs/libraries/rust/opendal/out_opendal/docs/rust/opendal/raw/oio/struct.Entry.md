# Struct Entry Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/entry.rs.html#29-32" class="src">Source</a>

``` rust
pub struct Entry { /* private fields */ }
```

Expand description

Entry is returned by `Page` or `BlockingPage` during list operations.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#notes" class="doc-anchor">Â§</a>Notes

Differences between `crate::Entry` and `oio::Entry`:

- `crate::Entry` is the userâ€™s public API and have less public methods.
- `oio::Entry` is the raw API and doesnâ€™t expose to users.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#impl-Entry" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.new" class="fn">new</a>(path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, meta: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

Create a new entry by its corresponding underlying storage.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.with" class="fn">with</a>(path: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, meta: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

Create a new entry with given value.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.set_path" class="fn">set_path</a>(&mut self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set path for entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.path" class="fn">path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the path of entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.set_mode" class="fn">set_mode</a>(&mut self, mode: <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>) -\> &mut Self

Set mode for entry.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#note" class="doc-anchor">Â§</a>Note

Please use this function carefully.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.mode" class="fn">mode</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

Get entryâ€™s mode.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#impl-Clone-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#impl-Debug-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#impl-PartialEq-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#impl-Eq-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#impl-StructuralPartialEq-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html#blanket-implementations" class="anchor">Â§</a>
