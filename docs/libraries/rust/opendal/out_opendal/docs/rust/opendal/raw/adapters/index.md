# Module adapters Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/mod.rs.html#18-50" class="src">Source</a>

Expand description

Providing adapters and its implementations.

Adapters in OpenDAL means services that shares similar behaviors. We use adapter to make those services been implemented more easily. For example, with [`kv::Adapter`](https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html "trait opendal::raw::adapters::kv::Adapter"), users only need to implement `get`, `set` for a service.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/index.html#notes" class="doc-anchor">Â§</a>Notes

Please import the module instead of its type.

For example, use the following:

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
use opendal::adapters::kv;

impl kv::Adapter for MyType {}
```

Instead of:

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
use opendal::adapters::kv::Adapter;

impl Adapter for MyType {}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/index.html#available-adapters" class="doc-anchor">Â§</a>Available Adapters

- [`kv::Adapter`](https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html "trait opendal::raw::adapters::kv::Adapter"): Adapter for Key Value Services like `redis`.
- [`typed_kv::Adapter`](https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html "trait opendal::raw::adapters::typed_kv::Adapter"): Adapter key value services that in-memory.

## Modules<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/index.html#modules" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/index.html" class="mod" title="mod opendal::raw::adapters::kv">kv</a>  
Providing Key Value Adapter for OpenDAL.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/index.html" class="mod" title="mod opendal::raw::adapters::typed_kv">typed_kv</a>  
Providing Typed Key Value Adapter for OpenDAL.
