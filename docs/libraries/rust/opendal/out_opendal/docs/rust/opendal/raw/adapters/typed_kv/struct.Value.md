# Struct Value Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/typed_kv/api.rs.html#73-78" class="src">Source</a>

``` rust
pub struct Value {
    pub metadata: Metadata,
    pub value: Buffer,
}
```

Expand description

Value is the typed value stored in adapter.

Itâ€™s cheap to clone so that users can read data without extra copy.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#structfield.metadata" class="anchor field">Â§</a>`metadata: `<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata"><code>Metadata</code></a>

Metadata of this value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#structfield.value" class="anchor field">Â§</a>`value: `<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer"><code>Buffer</code></a>

The corresponding content of this value.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#impl-Value" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#method.new_dir" class="fn">new_dir</a>() -\> Self

Create a new dir of value.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Size returns the in-memory size of Value.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#impl-Clone-for-Value" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#impl-Debug-for-Value" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html#blanket-implementations" class="anchor">Â§</a>
