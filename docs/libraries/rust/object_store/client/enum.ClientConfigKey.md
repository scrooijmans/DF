# Enum ClientConfigKey Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/mod.rs.html#83-135" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum ClientConfigKey {
Show 18 variants    AllowHttp,
    AllowInvalidCertificates,
    ConnectTimeout,
    DefaultContentType,
    Http1Only,
    Http2KeepAliveInterval,
    Http2KeepAliveTimeout,
    Http2KeepAliveWhileIdle,
    Http2MaxFrameSize,
    Http2Only,
    PoolIdleTimeout,
    PoolMaxIdlePerHost,
    ProxyUrl,
    ProxyCaCertificate,
    ProxyExcludes,
    RandomizeAddresses,
    Timeout,
    UserAgent,
}
```

Available on **crate feature `cloud`** only.

Expand description

Configuration keys for [`ClientOptions`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions")

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.AllowHttp" class="anchor">§</a>

### AllowHttp

Allow non-TLS, i.e. non-HTTPS connections

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.AllowInvalidCertificates" class="anchor">§</a>

### AllowInvalidCertificates

Skip certificate validation on https connections.

#### <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#warning" class="doc-anchor">§</a>Warning

You should think very carefully before using this method. If invalid certificates are trusted, *any* certificate for *any* site will be trusted for use. This includes expired certificates. This introduces significant vulnerabilities, and should only be used as a last resort or for testing

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.ConnectTimeout" class="anchor">§</a>

### ConnectTimeout

Timeout for only the connect phase of a Client

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.DefaultContentType" class="anchor">§</a>

### DefaultContentType

default CONTENT_TYPE for uploads

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http1Only" class="anchor">§</a>

### Http1Only

Only use http1 connections

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http2KeepAliveInterval" class="anchor">§</a>

### Http2KeepAliveInterval

Interval for HTTP2 Ping frames should be sent to keep a connection alive.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http2KeepAliveTimeout" class="anchor">§</a>

### Http2KeepAliveTimeout

Timeout for receiving an acknowledgement of the keep-alive ping.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http2KeepAliveWhileIdle" class="anchor">§</a>

### Http2KeepAliveWhileIdle

Enable HTTP2 keep alive pings for idle connections

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http2MaxFrameSize" class="anchor">§</a>

### Http2MaxFrameSize

Sets the maximum frame size to use for HTTP2.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http2Only" class="anchor">§</a>

### Http2Only

Only use http2 connections

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.PoolIdleTimeout" class="anchor">§</a>

### PoolIdleTimeout

The pool max idle timeout

This is the length of time an idle connection will be kept alive

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.PoolMaxIdlePerHost" class="anchor">§</a>

### PoolMaxIdlePerHost

maximum number of idle connections per host

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.ProxyUrl" class="anchor">§</a>

### ProxyUrl

HTTP proxy to use for requests

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.ProxyCaCertificate" class="anchor">§</a>

### ProxyCaCertificate

PEM-formatted CA certificate for proxy connections

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.ProxyExcludes" class="anchor">§</a>

### ProxyExcludes

List of hosts that bypass proxy

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.RandomizeAddresses" class="anchor">§</a>

### RandomizeAddresses

Randomize order addresses that the DNS resolution yields.

This will spread the connections across more servers.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Timeout" class="anchor">§</a>

### Timeout

Request timeout

The timeout is applied from when the request starts connecting until the response body has finished

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.UserAgent" class="anchor">§</a>

### UserAgent

User-Agent header to be used by this client

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-AsRef%3Cstr%3E-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Clone-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Debug-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Deserialize%3C&#39;de%3E-for-ClientConfigKey" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-FromStr-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Hash-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-PartialEq-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Serialize-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Copy-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-Eq-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#impl-StructuralPartialEq-for-ClientConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#blanket-implementations" class="anchor">§</a>
