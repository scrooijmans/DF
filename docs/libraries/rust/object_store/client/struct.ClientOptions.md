# Struct ClientOptions Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/mod.rs.html#246-269" class="src">Source</a>

``` rust
pub struct ClientOptions { /* private fields */ }
```

Available on **crate feature `cloud`** only.

Expand description

HTTP client configuration for remote object stores

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#impl-ClientOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.new" class="fn">new</a>() -\> Self

Create a new [`ClientOptions`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions") with default values

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_config" class="fn">with_config</a>(self, key: <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set an option by key

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.get_config_value" class="fn">get_config_value</a>(&self, key: &<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get an option by key

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_user_agent" class="fn">with_user_agent</a>(self, agent: <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> Self

Sets the User-Agent header to be used by this client

Default is based on the version of this crate

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_root_certificate" class="fn">with_root_certificate</a>(self, certificate: <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate">Certificate</a>) -\> Self

Available on **non-WebAssembly** only.

Add a custom root certificate.

This can be used to connect to a server that has a self-signed certificate for example.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_default_content_type" class="fn">with_default_content_type</a>(self, mime: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the default CONTENT_TYPE for uploads

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_content_type_for_suffix" class="fn">with_content_type_for_suffix</a>( self, extension: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, mime: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set the CONTENT_TYPE for a given file extension

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_default_headers" class="fn">with_default_headers</a>(self, headers: <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>) -\> Self

Sets the default headers for every request

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_allow_http" class="fn">with_allow_http</a>(self, allow_http: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Sets what protocol is allowed. If `allow_http` is :

- false (default): Only HTTPS are allowed
- true: HTTP and HTTPS are allowed

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_allow_invalid_certificates" class="fn">with_allow_invalid_certificates</a>(self, allow_insecure: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Allows connections to invalid SSL certificates

- false (default): Only valid HTTPS certificates are allowed
- true: All HTTPS certificates are allowed

##### <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#warning" class="doc-anchor">§</a>Warning

You should think very carefully before using this method. If invalid certificates are trusted, *any* certificate for *any* site will be trusted for use. This includes expired certificates. This introduces significant vulnerabilities, and should only be used as a last resort or for testing

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_http1_only" class="fn">with_http1_only</a>(self) -\> Self

Only use http1 connections

This is on by default, since http2 is known to be significantly slower than http1.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_http2_only" class="fn">with_http2_only</a>(self) -\> Self

Only use http2 connections

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_allow_http2" class="fn">with_allow_http2</a>(self) -\> Self

Use http2 if supported, otherwise use http1.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_proxy_url" class="fn">with_proxy_url</a>(self, proxy_url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set a proxy URL to use for requests

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_proxy_ca_certificate" class="fn">with_proxy_ca_certificate</a>( self, proxy_ca_certificate: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set a trusted proxy CA certificate

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_proxy_excludes" class="fn">with_proxy_excludes</a>(self, proxy_excludes: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set a list of hosts to exclude from proxy connections

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_timeout" class="fn">with_timeout</a>(self, timeout: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set timeout for the overall request

The timeout starts from when the request starts connecting until the response body has finished. If the request does not complete within the timeout, the client returns a timeout error.

Timeout errors are retried, subject to the [`RetryConfig`](https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html "struct object_store::RetryConfig")

Default is 30 seconds

##### <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#see-also" class="doc-anchor">§</a>See Also

- [`Self::with_timeout_disabled`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_timeout_disabled "method object_store::client::ClientOptions::with_timeout_disabled") to disable the timeout
- [`Self::with_connect_timeout`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_connect_timeout "method object_store::client::ClientOptions::with_connect_timeout") to set a timeout for the connect phase

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_timeout_disabled" class="fn">with_timeout_disabled</a>(self) -\> Self

Disables the request timeout

##### <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#see-also-1" class="doc-anchor">§</a>See Also

- [`Self::with_timeout`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_timeout "method object_store::client::ClientOptions::with_timeout")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_connect_timeout" class="fn">with_connect_timeout</a>(self, timeout: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set a timeout for only the connect phase of a Client

This is the time allowed for the client to establish a connection and if the connection is not established within this time, the client returns a timeout error.

Timeout errors are retried, subject to the [`RetryConfig`](https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html "struct object_store::RetryConfig")

Default is 5 seconds

##### <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#see-also-2" class="doc-anchor">§</a>See Also

- [`Self::with_timeout`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_timeout "method object_store::client::ClientOptions::with_timeout") to set a timeout for the overall request
- [`Self::with_connect_timeout_disabled`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_connect_timeout_disabled "method object_store::client::ClientOptions::with_connect_timeout_disabled") to disable the connect timeout

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_connect_timeout_disabled" class="fn">with_connect_timeout_disabled</a>(self) -\> Self

Disables the connection timeout

##### <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#see-also-3" class="doc-anchor">§</a>See Also

- [`Self::with_connect_timeout`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_connect_timeout "method object_store::client::ClientOptions::with_connect_timeout")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_pool_idle_timeout" class="fn">with_pool_idle_timeout</a>(self, timeout: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set the pool max idle timeout

This is the length of time an idle connection will be kept alive

Default is 90 seconds enforced by reqwest

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_pool_max_idle_per_host" class="fn">with_pool_max_idle_per_host</a>(self, max: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the maximum number of idle connections per host

Default is no limit enforced by reqwest

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_http2_keep_alive_interval" class="fn">with_http2_keep_alive_interval</a>(self, interval: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Sets an interval for HTTP2 Ping frames should be sent to keep a connection alive.

Default is disabled enforced by reqwest

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_http2_keep_alive_timeout" class="fn">with_http2_keep_alive_timeout</a>(self, interval: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Sets a timeout for receiving an acknowledgement of the keep-alive ping.

If the ping is not acknowledged within the timeout, the connection will be closed. Does nothing if http2_keep_alive_interval is disabled.

Default is disabled enforced by reqwest

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_http2_keep_alive_while_idle" class="fn">with_http2_keep_alive_while_idle</a>(self) -\> Self

Enable HTTP2 keep alive pings for idle connections

If disabled, keep-alive pings are only sent while there are open request/response streams. If enabled, pings are also sent when no streams are active

Default is disabled enforced by reqwest

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.with_http2_max_frame_size" class="fn">with_http2_max_frame_size</a>(self, sz: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> Self

Sets the maximum frame size to use for HTTP2.

Default is currently 16,384 but may change internally to optimize for common uses.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.get_content_type" class="fn">get_content_type</a>(&self, path: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the mime type for the file in `path` to be uploaded

Gets the file extension from `path`, and returns the mime type if it was defined initially through `ClientOptions::with_content_type_for_suffix`

Otherwise, returns the default mime type if it was defined earlier through `ClientOptions::with_default_content_type`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#impl-Clone-for-ClientOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#impl-Debug-for-ClientOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#impl-Default-for-ClientOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html#blanket-implementations" class="anchor">§</a>
