# Struct MiniMoka Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/mini_moka/backend.rs.html#40-42" class="src">Source</a>

``` rust
pub struct MiniMoka { /* private fields */ }
```

Expand description

[mini-moka](https://github.com/moka-rs/mini-moka) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#notes" class="doc-anchor">Â§</a>Notes

To better assist you in choosing the right cache for your use case, Hereâ€™s a comparison table with [moka](https://github.com/moka-rs/moka#choosing-the-right-cache-for-your-use-case)

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#impl-MiniMokaBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.new" class="fn">new</a>() -\> Self

Available on **crate feature `services-mini-moka`** only.

Create a [`MiniMokaBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html "struct opendal::services::MiniMoka") with default configuration.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.max_capacity" class="fn">max_capacity</a>(self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

Available on **crate feature `services-mini-moka`** only.

Sets the max capacity of the cache.

Refer to [`mini-moka::sync::CacheBuilder::max_capacity`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.max_capacity)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.time_to_live" class="fn">time_to_live</a>(self, v: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-mini-moka`** only.

Sets the time to live of the cache.

Refer to [`mini-moka::sync::CacheBuilder::time_to_live`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.time_to_live)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.time_to_idle" class="fn">time_to_idle</a>(self, v: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-mini-moka`** only.

Sets the time to idle of the cache.

Refer to [`mini-moka::sync::CacheBuilder::time_to_idle`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.time_to_idle)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.root" class="fn">root</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mini-moka`** only.

Set root path of this backend

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#impl-Builder-for-MiniMokaBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

Available on **crate feature `services-mini-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMokaConfig.html" class="struct" title="struct opendal::services::MiniMokaConfig">MiniMokaConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#impl-Debug-for-MiniMokaBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

Available on **crate feature `services-mini-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#impl-Default-for-MiniMokaBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

Available on **crate feature `services-mini-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html#blanket-implementations" class="anchor">Â§</a>
