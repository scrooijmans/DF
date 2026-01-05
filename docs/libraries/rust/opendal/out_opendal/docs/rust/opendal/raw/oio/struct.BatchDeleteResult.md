# Struct BatchDeleteResult Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/batch_delete.rs.html#51-56" class="src">Source</a>

``` rust
pub struct BatchDeleteResult {
    pub succeeded: Vec<(String, OpDelete)>,
    pub failed: Vec<(String, OpDelete, Error)>,
}
```

Expand description

BatchDeleteResult is the result of batch delete operation.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#structfield.succeeded" class="anchor field">Â§</a>`succeeded: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete"><code>OpDelete</code></a>`)>`

Collection of successful deletions, containing tuples of (path, args)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#structfield.failed" class="anchor field">Â§</a>`failed: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete"><code>OpDelete</code></a>`, `<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error"><code>Error</code></a>`)>`

Collection of failed deletions, containing tuples of (path, args, error)

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#impl-Default-for-BatchDeleteResult" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html" class="struct" title="struct opendal::raw::oio::BatchDeleteResult">BatchDeleteResult</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html" class="struct" title="struct opendal::raw::oio::BatchDeleteResult">BatchDeleteResult</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html#blanket-implementations" class="anchor">Â§</a>
