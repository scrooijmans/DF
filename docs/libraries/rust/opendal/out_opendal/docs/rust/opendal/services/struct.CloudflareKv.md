# Struct CloudflareKv Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/cloudflare_kv/backend.rs.html#39-44" class="src">Source</a>

``` rust
pub struct CloudflareKv { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `api_token`: Set the token of cloudflare api
- `account_id`: Set the account identifier of cloudflare
- `namespace_id`: Set the namespace identifier of d1

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#impl-CloudflareKvBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.api_token" class="fn">api_token</a>(self, api_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-cloudflare-kv`** only.

Set the token used to authenticate with CloudFlare.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.account_id" class="fn">account_id</a>(self, account_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-cloudflare-kv`** only.

Set the account ID used to authenticate with CloudFlare.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.namespace_id" class="fn">namespace_id</a>(self, namespace_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-cloudflare-kv`** only.

Set the namespace ID.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.default_ttl" class="fn">default_ttl</a>(self, ttl: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-cloudflare-kv`** only.

Set the default ttl for cloudflare_kv services.

If set, we will specify `EX` for write operations.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-cloudflare-kv`** only.

Set the root within this backend.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#impl-Builder-for-CloudflareKvBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

Available on **crate feature `services-cloudflare-kv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKvConfig.html" class="struct" title="struct opendal::services::CloudflareKvConfig">CloudflareKvConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#impl-Debug-for-CloudflareKvBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

Available on **crate feature `services-cloudflare-kv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#impl-Default-for-CloudflareKvBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

Available on **crate feature `services-cloudflare-kv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html#blanket-implementations" class="anchor">Â§</a>
