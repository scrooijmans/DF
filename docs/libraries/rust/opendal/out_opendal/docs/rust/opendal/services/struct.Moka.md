# Struct Moka Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/moka/backend.rs.html#44-47" class="src">Source</a>

``` rust
pub struct Moka { /* private fields */ }
```

Expand description

[moka](https://github.com/moka-rs/moka) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☐ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#configuration" class="doc-anchor">Â§</a>Configuration

- `name`: Set the name for this cache instance.
- `max_capacity`: Set the max capacity of the cache.
- `time_to_live`: Set the time to live of the cache.
- `time_to_idle`: Set the time to idle of the cache.

You can refer to [`MokaBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html "struct opendal::services::Moka")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Moka;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Moka::default()
        .name("opendal");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#impl-MokaBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.new" class="fn">new</a>(builder: <a href="https://opendal.apache.org/docs/rust/opendal/services/type.MokaCacheBuilder.html" class="type" title="type opendal::services::MokaCacheBuilder">MokaCacheBuilder</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html" class="struct" title="struct opendal::services::MokaValue">MokaValue</a>\>) -\> Self

Available on **crate feature `services-moka`** only.

Create a [`MokaBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html "struct opendal::services::Moka") with the given \[`moka::future::CacheBuilder`\].

Refer to [`moka::future::CacheBuilder`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html)

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#example-1" class="doc-anchor">Â§</a>Example

``` rust
let moka = Moka::new(
    MokaCacheBuilder::<String, MokaValue>::default()
        .name("demo")
        .max_capacity(1000)
        .time_to_live(Duration::from_secs(300))
        .weigher(|k, v| (k.len() + v.content.len()) as u32)
        .eviction_listener(|k: Arc<String>, v: MokaValue, cause: RemovalCause| {
            debug!(
                "moka cache eviction listener, key = {}, value = {:?}, cause = {:?}",
                k.as_str(), v.content.to_vec(), cause
            );
        })
);
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.name" class="fn">name</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-moka`** only.

Sets the name of the cache.

Refer to [`moka::future::CacheBuilder::name`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.name)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.max_capacity" class="fn">max_capacity</a>(self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

Available on **crate feature `services-moka`** only.

Sets the max capacity of the cache.

Refer to [`moka::future::CacheBuilder::max_capacity`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.max_capacity)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.time_to_live" class="fn">time_to_live</a>(self, v: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-moka`** only.

Sets the time to live of the cache.

Refer to [`moka::future::CacheBuilder::time_to_live`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.time_to_live)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.time_to_idle" class="fn">time_to_idle</a>(self, v: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-moka`** only.

Sets the time to idle of the cache.

Refer to [`moka::future::CacheBuilder::time_to_idle`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.time_to_idle)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.root" class="fn">root</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-moka`** only.

Set the root path of this backend

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#impl-Builder-for-MokaBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

Available on **crate feature `services-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaConfig.html" class="struct" title="struct opendal::services::MokaConfig">MokaConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#impl-Debug-for-MokaBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

Available on **crate feature `services-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#impl-Default-for-MokaBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

Available on **crate feature `services-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html#blanket-implementations" class="anchor">Â§</a>
