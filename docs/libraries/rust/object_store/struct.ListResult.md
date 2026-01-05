# Struct ListResult Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#911-916" class="src">Source</a>

``` rust
pub struct ListResult {
    pub common_prefixes: Vec<Path>,
    pub objects: Vec<ObjectMeta>,
}
```

Expand description

Result of a list call that includes objects, prefixes (directories) and a token for the next set of results. Individual result sets may be limited to 1,000 objects based on the underlying object storage’s limitations.

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#structfield.common_prefixes" class="anchor field">§</a>`common_prefixes: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path"><code>Path</code></a>`>`

Prefixes that are common (like directories)

<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#structfield.objects" class="anchor field">§</a>`objects: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta"><code>ObjectMeta</code></a>`>`

Object metadata for the listing

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#impl-Debug-for-ListResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html" class="struct" title="struct object_store::ListResult">ListResult</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html#blanket-implementations" class="anchor">§</a>
