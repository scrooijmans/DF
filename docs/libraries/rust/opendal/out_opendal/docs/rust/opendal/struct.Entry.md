# Struct Entry Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/entry.rs.html#23-29" class="src">Source</a>

``` rust
pub struct Entry { /* private fields */ }
```

Expand description

Entry returned by [`Lister`](https://opendal.apache.org/docs/rust/opendal/struct.Lister.html "struct opendal::Lister") or \[`BlockingLister`\] to represent a path and itâ€™s relative metadata.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#impl-Entry" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.path" class="fn">path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Path of entry. Path is relative to operatorâ€™s root.

Only valid in current operator.

If this entry is a dir, `path` MUST end with `/` Otherwise, `path` MUST NOT end with `/`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Name of entry. Name is the last segment of path.

If this entry is a dir, `name` MUST end with `/` Otherwise, `name` MUST NOT end with `/`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.metadata" class="fn">metadata</a>(&self) -\> &<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

Fetch metadata of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>)

Consume this entry to get its path and metadata.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#impl-Clone-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#impl-Debug-for-Entry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#impl-IntoDeleteInput-for-Entry" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>

Implement `IntoDeleteInput` for `Entry` so we can use `Lister` as a DeleteInput stream.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#method.into_delete_input" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#tymethod.into_delete_input" class="fn">into_delete_input</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

Convert `self` into a `DeleteInput`.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html#blanket-implementations" class="anchor">Â§</a>
