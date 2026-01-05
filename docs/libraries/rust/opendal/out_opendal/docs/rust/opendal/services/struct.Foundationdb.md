# Struct Foundationdb Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/foundationdb/backend.rs.html#32-34" class="src">Source</a>

``` rust
pub struct Foundationdb { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☐ create_dir
- ☒ stat
- ☒ read
- ☒ write
- ☒ delete
- ☐ copy
- ☐ rename
- ☐ list
- ☐ ~~presign~~

**Note**: As for [Known Limitations - FoundationDB](https://apple.github.io/foundationdb/known-limitations), keys cannot exceed 10,000 bytes in size, and values cannot exceed 100,000 bytes in size. Errors will be raised by OpenDAL if these limits are exceeded.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for this backend.
- `config_path`: Set the configuration path for foundationdb. If not provided, the default configuration path will be used.

You can refer to [`FoundationdbBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html "struct opendal::services::Foundationdb")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Foundationdb;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Foundationdb::default()
        .config_path("/etc/foundationdb/foundationdb.conf");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#impl-FoundationdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">FoundationdbBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#method.root" class="fn">root</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-foundationdb`** only.

Set the root for Foundationdb.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#method.config_path" class="fn">config_path</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-foundationdb`** only.

Set the config path for Foundationdb. If not set, will fallback to use default

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#impl-Builder-for-FoundationdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">FoundationdbBuilder</a>

Available on **crate feature `services-foundationdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FoundationdbConfig.html" class="struct" title="struct opendal::services::FoundationdbConfig">FoundationdbConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#impl-Default-for-FoundationdbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">FoundationdbBuilder</a>

Available on **crate feature `services-foundationdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">FoundationdbBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html#blanket-implementations" class="anchor">Â§</a>
