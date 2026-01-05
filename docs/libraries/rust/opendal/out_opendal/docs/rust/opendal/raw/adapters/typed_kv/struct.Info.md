# Struct Info Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/typed_kv/api.rs.html#138-142" class="src">Source</a>

``` rust
pub struct Info { /* private fields */ }
```

Expand description

Info for this key value accessor.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#impl-Info" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Info">Info</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#method.new" class="fn">new</a>(scheme: <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, capabilities: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>) -\> Self

Create a new KeyValueAccessorInfo.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#method.scheme" class="fn">scheme</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

Get the scheme.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the name.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#method.capabilities" class="fn">capabilities</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>

Get the capabilities.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html#blanket-implementations" class="anchor">Â§</a>
