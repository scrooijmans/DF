# Struct Capability Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/typed_kv/api.rs.html#100-111" class="src">Source</a>

``` rust
pub struct Capability {
    pub get: bool,
    pub set: bool,
    pub delete: bool,
    pub scan: bool,
    pub shared: bool,
}
```

Expand description

Capability is used to describe what operations are supported by Typed KV Operator.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#structfield.get" class="anchor field">Â§</a>`get: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If typed_kv operator supports get natively.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#structfield.set" class="anchor field">Â§</a>`set: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If typed_kv operator supports set natively.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#structfield.delete" class="anchor field">Â§</a>`delete: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If typed_kv operator supports delete natively.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#structfield.scan" class="anchor field">Â§</a>`scan: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If typed_kv operator supports scan natively.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#structfield.shared" class="anchor field">Â§</a>`shared: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If typed_kv operator supports shared access.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#impl-Clone-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#impl-Debug-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#impl-Default-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#impl-Copy-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html#blanket-implementations" class="anchor">Â§</a>
