# Struct ListOptions Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/options.rs.html#33-65" class="src">Source</a>

``` rust
pub struct ListOptions {
    pub limit: Option<usize>,
    pub start_after: Option<String>,
    pub recursive: bool,
    pub versions: bool,
    pub deleted: bool,
}
```

Expand description

Options for list operations.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#structfield.limit" class="anchor field">Â§</a>`limit: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

The limit passed to underlying service to specify the max results that could return per-request.

Users could use this to control the memory usage of list operation.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#structfield.start_after" class="anchor field">Â§</a>`start_after: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The start_after passes to underlying service to specify the specified key to start listing from.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#structfield.recursive" class="anchor field">Â§</a>`recursive: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

The recursive is used to control whether the list operation is recursive.

- If `false`, list operation will only list the entries under the given path.
- If `true`, list operation will list all entries that starts with given path.

Default to `false`.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#structfield.versions" class="anchor field">Â§</a>`versions: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

The version is used to control whether the object versions should be returned.

- If `false`, list operation will not return with object versions
- If `true`, list operation will return with object versions if object versioning is supported by the underlying service

Default to `false`

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#structfield.deleted" class="anchor field">Â§</a>`deleted: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

The deleted is used to control whether the deleted objects should be returned.

- If `false`, list operation will not return with deleted objects
- If `true`, list operation will return with deleted objects if object versioning is supported by the underlying service

Default to `false`

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-Clone-for-ListOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-Debug-for-ListOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-Default-for-ListOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-From%3CListOptions%3E-for-OpList" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-PartialEq-for-ListOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-Eq-for-ListOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#impl-StructuralPartialEq-for-ListOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html#blanket-implementations" class="anchor">Â§</a>
