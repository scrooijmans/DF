# Struct FileMetadataCacheEntry Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/cache/cache_manager.rs.html#96" class="src">Source</a>

``` rust
pub struct FileMetadataCacheEntry {
    pub object_meta: ObjectMeta,
    pub size_bytes: usize,
    pub hits: usize,
    pub extra: HashMap<String, String>,
}
```

Expand description

Represents information about a cached metadata entry. This is used to expose the metadata cache contents to outside modules.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#structfield.object_meta" class="anchor field">§</a>`object_meta: `<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta"><code>ObjectMeta</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#structfield.size_bytes" class="anchor field">§</a>`size_bytes: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Size of the cached metadata, in bytes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#structfield.hits" class="anchor field">§</a>`hits: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Number of times this entry was retrieved.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#structfield.extra" class="anchor field">§</a>`extra: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Additional object-specific information.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#impl-Clone-for-FileMetadataCacheEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#impl-Debug-for-FileMetadataCacheEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#impl-PartialEq-for-FileMetadataCacheEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#impl-Eq-for-FileMetadataCacheEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#impl-StructuralPartialEq-for-FileMetadataCacheEntry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html#blanket-implementations" class="anchor">§</a>
