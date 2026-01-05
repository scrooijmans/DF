# Struct OssConfig Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/oss/config.rs.html#29-102" class="src">Source</a>

``` rust
#[non_exhaustive]pub struct OssConfig {Show 20 fields
    pub root: Option<String>,
    pub endpoint: Option<String>,
    pub presign_endpoint: Option<String>,
    pub bucket: String,
    pub addressing_style: Option<String>,
    pub presign_addressing_style: Option<String>,
    pub enable_versioning: bool,
    pub server_side_encryption: Option<String>,
    pub server_side_encryption_key_id: Option<String>,
    pub allow_anonymous: bool,
    pub access_key_id: Option<String>,
    pub access_key_secret: Option<String>,
    pub security_token: Option<String>,
    pub batch_max_operations: Option<usize>,
    pub delete_max_size: Option<usize>,
    pub role_arn: Option<String>,
    pub role_session_name: Option<String>,
    pub oidc_provider_arn: Option<String>,
    pub oidc_token_file: Option<String>,
    pub sts_endpoint: Option<String>,
}
```

Expand description

Config for Aliyun Object Storage Service (OSS) support.

## Fields (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#fields" class="anchor">Â§</a>

This struct is marked as non-exhaustive

Non-exhaustive structs could have additional fields added in future. Therefore, non-exhaustive structs cannot be constructed in external crates using the traditional `Struct { .. }` syntax; cannot be matched against without a wildcard `..`; and struct update syntax will not work.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.root" class="anchor field">Â§</a>`root: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Root for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.endpoint" class="anchor field">Â§</a>`endpoint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Endpoint for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.presign_endpoint" class="anchor field">Â§</a>`presign_endpoint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Presign endpoint for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.bucket" class="anchor field">Â§</a>`bucket: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Bucket for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.addressing_style" class="anchor field">Â§</a>`addressing_style: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Addressing style for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.presign_addressing_style" class="anchor field">Â§</a>`presign_addressing_style: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Pre sign addressing style for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.enable_versioning" class="anchor field">Â§</a>`enable_versioning: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

is bucket versioning enabled for this bucket

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.server_side_encryption" class="anchor field">Â§</a>`server_side_encryption: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Server side encryption for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.server_side_encryption_key_id" class="anchor field">Â§</a>`server_side_encryption_key_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Server side encryption key id for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.allow_anonymous" class="anchor field">Â§</a>`allow_anonymous: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow anonymous for oss.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.access_key_id" class="anchor field">Â§</a>`access_key_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Access key id for oss.

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_ACCESS_KEY_ID`\]

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.access_key_secret" class="anchor field">Â§</a>`access_key_secret: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Access key secret for oss.

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_ACCESS_KEY_SECRET`\]

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.security_token" class="anchor field">Â§</a>`security_token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

`security_token` will be loaded from

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_SECURITY_TOKEN`\]

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.batch_max_operations" class="anchor field">Â§</a>`batch_max_operations: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

ðŸ‘ŽDeprecated since 0.52.0: Please use `delete_max_size` instead of `batch_max_operations`

The size of max batch operations.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.delete_max_size" class="anchor field">Â§</a>`delete_max_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

The size of max delete operations.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.role_arn" class="anchor field">Â§</a>`role_arn: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

If `role_arn` is set, we will use already known config as source credential to assume role with `role_arn`.

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_ROLE_ARN`\]

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.role_session_name" class="anchor field">Â§</a>`role_session_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

role_session_name for this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.oidc_provider_arn" class="anchor field">Â§</a>`oidc_provider_arn: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

`oidc_provider_arn` will be loaded from

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_OIDC_PROVIDER_ARN`\]

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.oidc_token_file" class="anchor field">Â§</a>`oidc_token_file: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

`oidc_token_file` will be loaded from

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_OIDC_TOKEN_FILE`\]

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#structfield.sts_endpoint" class="anchor field">Â§</a>`sts_endpoint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

`sts_endpoint` will be loaded from

- this field if itâ€™s `is_some`
- env value: \[`ALIBABA_CLOUD_STS_ENDPOINT`\]

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Clone-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Configurator-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#associatedtype.Builder" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

Associated builder for this configuration.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.from_uri" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_uri" class="fn">from_uri</a>(uri: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Build configuration from a parsed URI plus merged options.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.into_builder" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#tymethod.into_builder" class="fn">into_builder</a>(self) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype" title="type opendal::Configurator::Builder">Builder</a>

Convert this configuration into a service builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.from_iter" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter" class="fn">from_iter</a>(iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Deserialize from an iterator. [Read more](https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Debug-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Default-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Deserialize%3C&#39;de%3E-for-OssConfig" class="anchor">Â§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

where <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.deserialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-PartialEq-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Serialize-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#method.serialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-Eq-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#impl-StructuralPartialEq-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html#blanket-implementations" class="anchor">Â§</a>
