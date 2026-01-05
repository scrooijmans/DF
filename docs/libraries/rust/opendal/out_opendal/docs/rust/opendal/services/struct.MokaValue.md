# Struct MokaValue Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/moka/core.rs.html#27-32" class="src">Source</a>

``` rust
pub struct MokaValue {
    pub metadata: Metadata,
    pub content: Buffer,
}
```

Expand description

Value stored in moka cache containing both metadata and content

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#structfield.metadata" class="anchor field">Â§</a>`metadata: `<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata"><code>Metadata</code></a>

Stored metadata in moka cache.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#structfield.content" class="anchor field">Â§</a>`content: `<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer"><code>Buffer</code></a>

Stored content in moka cache.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#impl-Clone-for-MokaValue" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html" class="struct" title="struct opendal::services::MokaValue">MokaValue</a>

Available on **crate feature `services-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html" class="struct" title="struct opendal::services::MokaValue">MokaValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html#blanket-implementations" class="anchor">Â§</a>
