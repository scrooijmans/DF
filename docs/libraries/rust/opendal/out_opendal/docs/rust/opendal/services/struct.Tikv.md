# Struct Tikv Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/tikv/backend.rs.html#34-36" class="src">Source</a>

``` rust
pub struct Tikv { /* private fields */ }
```

Expand description

TiKV backend builder

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#configuration" class="doc-anchor">Â§</a>Configuration

- `endpoints`: Set the endpoints to the tikv cluster
- `insecure`: Set the insecure flag to the tikv cluster
- `ca_path`: Set the ca path to the tikv connection
- `cert_path`: Set the cert path to the tikv connection
- `key_path`: Set the key path to the tikv connection

You can refer to [`TikvBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html "struct opendal::services::Tikv")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Tikv;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Tikv::default()
        .endpoints(vec!["127.0.0.1:2379".to_string()]);

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#impl-TikvBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.endpoints" class="fn">endpoints</a>(self, endpoints: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Available on **crate feature `services-tikv`** only.

Set the network address of the TiKV service.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.insecure" class="fn">insecure</a>(self) -\> Self

Available on **crate feature `services-tikv`** only.

Set the insecure connection to TiKV.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.ca_path" class="fn">ca_path</a>(self, ca_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-tikv`** only.

Set the certificate authority file path.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.cert_path" class="fn">cert_path</a>(self, cert_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-tikv`** only.

Set the certificate file path.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.key_path" class="fn">key_path</a>(self, key_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-tikv`** only.

Set the key file path.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#impl-Builder-for-TikvBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Available on **crate feature `services-tikv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.TikvConfig.html" class="struct" title="struct opendal::services::TikvConfig">TikvConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#impl-Clone-for-TikvBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Available on **crate feature `services-tikv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#impl-Debug-for-TikvBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Available on **crate feature `services-tikv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#impl-Default-for-TikvBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Available on **crate feature `services-tikv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html#blanket-implementations" class="anchor">Â§</a>
