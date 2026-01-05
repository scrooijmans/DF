# Module kv Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/kv/mod.rs.html#18-31" class="src">Source</a>

Expand description

Providing Key Value Adapter for OpenDAL.

Any services that implement `Adapter` can be used an OpenDAL Service.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::kv::Backend">Backend</a>  
Backend of kv service. If the storage service is one k-v-like service, it should implement this kv [`Backend`](https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/struct.Backend.html "struct opendal::raw::adapters::kv::Backend") by right.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/struct.Info.html" class="struct" title="struct opendal::raw::adapters::kv::Info">Info</a>  
Info for this key value accessor.

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::kv::Adapter">Adapter</a>  
KvAdapter is the adapter to underlying kv services.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html" class="trait" title="trait opendal::raw::adapters::kv::Scan">Scan</a>  
Scan is the async iterator returned by `Adapter::scan`.

## Type Aliases<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/index.html#types" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/type.Scanner.html" class="type" title="type opendal::raw::adapters::kv::Scanner">Scanner</a>  
A type-erased wrapper of Scan
