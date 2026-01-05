# Module typed_kv Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/typed_kv/mod.rs.html#18-29" class="src">Source</a>

Expand description

Providing Typed Key Value Adapter for OpenDAL.

Any services that implement `Adapter` can be used an OpenDAL Service.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>  
The typed kv backend which implements Accessor for typed kv adapter.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Capability.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Capability">Capability</a>  
Capability is used to describe what operations are supported by Typed KV Operator.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Info">Info</a>  
Info for this key value accessor.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>  
Value is the typed value stored in adapter.

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::typed_kv::Adapter">Adapter</a>  
Adapter is the typed adapter to underlying kv services.
