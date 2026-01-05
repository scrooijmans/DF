# Struct Cacache Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/cacache/backend.rs.html#32-34" class="src">Source</a>

``` rust
pub struct Cacache { /* private fields */ }
```

Expand description

cacache service support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☐ list
- ☐ ~~presign~~
- ☒ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#configuration" class="doc-anchor">Â§</a>Configuration

- `datadir`: Set the path to the cacache data directory

You can refer to [`CacacheBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html "struct opendal::services::Cacache")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Cacache;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Cacache::default().datadir("/tmp/opendal/cacache");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#impl-CacacheBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">CacacheBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#method.datadir" class="fn">datadir</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-cacache`** only.

Set the path to the cacache data directory. Will create if not exists.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#impl-Builder-for-CacacheBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">CacacheBuilder</a>

Available on **crate feature `services-cacache`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CacacheConfig.html" class="struct" title="struct opendal::services::CacacheConfig">CacacheConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#impl-Default-for-CacacheBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">CacacheBuilder</a>

Available on **crate feature `services-cacache`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">CacacheBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html#blanket-implementations" class="anchor">Â§</a>
