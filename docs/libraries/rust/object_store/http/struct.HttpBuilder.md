# Struct HttpBuilder Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/http/mod.rs.html#206-211" class="src">Source</a>

``` rust
pub struct HttpBuilder { /* private fields */ }
```

Available on **crate feature `http`** only.

Expand description

Configure a connection to a generic HTTP server

## Implementations<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#impl-HttpBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.new" class="fn">new</a>() -\> Self

Create a new [`HttpBuilder`](https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html "struct object_store::http::HttpBuilder") with default values.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.with_url" class="fn">with_url</a>(self, url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the URL

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.with_retry" class="fn">with_retry</a>(self, retry_config: <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>) -\> Self

Set the retry configuration

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.with_config" class="fn">with_config</a>(self, key: <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set individual client configuration without overriding the entire config

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.with_client_options" class="fn">with_client_options</a>(self, options: <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> Self

Sets the client options, overriding any already set

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.with_http_connector" class="fn">with_http_connector</a>\<C: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a>\>(self, connector: C) -\> Self

The [`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") to use

On non-WASM32 platforms uses [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") by default, on WASM32 platforms must be provided

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpStore.html" class="struct" title="struct object_store::http::HttpStore">HttpStore</a>\>

Build an [`HttpStore`](https://docs.rs/object_store/latest/object_store/http/struct.HttpStore.html "struct object_store::http::HttpStore") with the configured options

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#impl-Clone-for-HttpBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#impl-Debug-for-HttpBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#impl-Default-for-HttpBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html#blanket-implementations" class="anchor">§</a>
