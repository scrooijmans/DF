# Struct Memory Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/memory/backend.rs.html#34-36" class="src">Source</a>

``` rust
pub struct Memory { /* private fields */ }
```

Expand description

In memory service support. (BTreeMap Based)

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☐ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Memory;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Memory::default().root("/tmp");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#impl-MemoryBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">MemoryBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#method.root" class="fn">root</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-memory`** only.

Set the root for BTreeMap.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#impl-Builder-for-MemoryBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">MemoryBuilder</a>

Available on **crate feature `services-memory`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemoryConfig.html" class="struct" title="struct opendal::services::MemoryConfig">MemoryConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#impl-Default-for-MemoryBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">MemoryBuilder</a>

Available on **crate feature `services-memory`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">MemoryBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html#blanket-implementations" class="anchor">Â§</a>
