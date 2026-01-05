# Struct OpList Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#91-123" class="src">Source</a>

``` rust
pub struct OpList { /* private fields */ }
```

Expand description

Args for `list` operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#impl-OpList" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.new" class="fn">new</a>() -\> Self

Create a new `OpList`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_limit" class="fn">with_limit</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Change the limit of this list operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.limit" class="fn">limit</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the limit of list operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_start_after" class="fn">with_start_after</a>(self, start_after: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Change the start_after of this list operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.start_after" class="fn">start_after</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the start_after of list operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_recursive" class="fn">with_recursive</a>(self, recursive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

The recursive is used to control whether the list operation is recursive.

- If `false`, list operation will only list the entries under the given path.
- If `true`, list operation will list all entries that starts with given path.

Default to `false`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.recursive" class="fn">recursive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get the current recursive.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_concurrent" class="fn">with_concurrent</a>(self, concurrent: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.2: concurrent in list is no-op

Change the concurrent of this list operation.

The default concurrent is 1.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.concurrent" class="fn">concurrent</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

ðŸ‘ŽDeprecated since 0.53.2: concurrent in list is no-op

Get the concurrent of list operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_version" class="fn">with_version</a>(self, version: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

ðŸ‘ŽDeprecated since 0.51.1: use with_versions instead

Change the version of this list operation

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_versions" class="fn">with_versions</a>(self, versions: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Change the version of this list operation

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.version" class="fn">version</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

ðŸ‘ŽDeprecated since 0.51.1: use versions instead

Get the version of this list operation

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.versions" class="fn">versions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get the version of this list operation

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.with_deleted" class="fn">with_deleted</a>(self, deleted: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Change the deleted of this list operation

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.deleted" class="fn">deleted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get the deleted of this list operation

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#impl-Clone-for-OpList" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#impl-Debug-for-OpList" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#impl-Default-for-OpList" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#impl-From%3CListOptions%3E-for-OpList" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html#blanket-implementations" class="anchor">Â§</a>
