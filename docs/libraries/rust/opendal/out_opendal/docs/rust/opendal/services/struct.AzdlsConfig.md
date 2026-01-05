# Struct AzdlsConfig Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/azdls/config.rs.html#27-60" class="src">Source</a>

``` rust
pub struct AzdlsConfig {
    pub root: Option<String>,
    pub filesystem: String,
    pub endpoint: Option<String>,
    pub account_name: Option<String>,
    pub account_key: Option<String>,
    pub client_secret: Option<String>,
    pub tenant_id: Option<String>,
    pub client_id: Option<String>,
    pub sas_token: Option<String>,
    pub authority_host: Option<String>,
}
```

Expand description

Azure Data Lake Storage Gen2 Support.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.root" class="anchor field">Â§</a>`root: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Root of this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.filesystem" class="anchor field">Â§</a>`filesystem: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Filesystem name of this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.endpoint" class="anchor field">Â§</a>`endpoint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Endpoint of this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.account_name" class="anchor field">Â§</a>`account_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Account name of this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.account_key" class="anchor field">Â§</a>`account_key: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Account key of this backend.

- required for shared_key authentication

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.client_secret" class="anchor field">Â§</a>`client_secret: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

client_secret The client secret of the service principal.

- required for client_credentials authentication

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.tenant_id" class="anchor field">Â§</a>`tenant_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

tenant_id The tenant id of the service principal.

- required for client_credentials authentication

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.client_id" class="anchor field">Â§</a>`client_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

client_id The client id of the service principal.

- required for client_credentials authentication

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.sas_token" class="anchor field">Â§</a>`sas_token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

sas_token The shared access signature token.

- required for sas authentication

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#structfield.authority_host" class="anchor field">Â§</a>`authority_host: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

authority_host The authority host of the service principal.

- required for client_credentials authentication
- default value: `https://login.microsoftonline.com`

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Clone-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Configurator-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#associatedtype.Builder" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Associated builder for this configuration.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.from_uri" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_uri" class="fn">from_uri</a>(uri: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Build configuration from a parsed URI plus merged options.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.into_builder" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#tymethod.into_builder" class="fn">into_builder</a>(self) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype" title="type opendal::Configurator::Builder">Builder</a>

Convert this configuration into a service builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.from_iter" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter" class="fn">from_iter</a>(iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Deserialize from an iterator. [Read more](https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Debug-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Default-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Deserialize%3C&#39;de%3E-for-AzdlsConfig" class="anchor">Â§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.deserialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-From%3CAzureStorageConfig%3E-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Config\> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(config: AzureStorageConfig) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-PartialEq-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Serialize-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#method.serialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-Eq-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#impl-StructuralPartialEq-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html#blanket-implementations" class="anchor">Â§</a>
