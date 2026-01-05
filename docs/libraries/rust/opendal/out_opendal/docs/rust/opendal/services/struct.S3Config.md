# Struct S3Config Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/s3/config.rs.html#29-219" class="src">Source</a>

``` rust
#[non_exhaustive]pub struct S3Config {Show 29 fields
    pub root: Option<String>,
    pub bucket: String,
    pub enable_versioning: bool,
    pub endpoint: Option<String>,
    pub region: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub session_token: Option<String>,
    pub role_arn: Option<String>,
    pub external_id: Option<String>,
    pub role_session_name: Option<String>,
    pub disable_config_load: bool,
    pub disable_ec2_metadata: bool,
    pub allow_anonymous: bool,
    pub server_side_encryption: Option<String>,
    pub server_side_encryption_aws_kms_key_id: Option<String>,
    pub server_side_encryption_customer_algorithm: Option<String>,
    pub server_side_encryption_customer_key: Option<String>,
    pub server_side_encryption_customer_key_md5: Option<String>,
    pub default_storage_class: Option<String>,
    pub enable_virtual_host_style: bool,
    pub batch_max_operations: Option<usize>,
    pub delete_max_size: Option<usize>,
    pub disable_stat_with_override: bool,
    pub checksum_algorithm: Option<String>,
    pub disable_write_with_if_match: bool,
    pub enable_write_with_append: bool,
    pub disable_list_objects_v2: bool,
    pub enable_request_payer: bool,
}
```

Expand description

Config for Aws S3 and compatible services (including minio, digitalocean space, Tencent Cloud Object Storage(COS) and so on) support.

## Fields (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#fields" class="anchor">Â§</a>

This struct is marked as non-exhaustive

Non-exhaustive structs could have additional fields added in future. Therefore, non-exhaustive structs cannot be constructed in external crates using the traditional `Struct { .. }` syntax; cannot be matched against without a wildcard `..`; and struct update syntax will not work.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.root" class="anchor field">Â§</a>`root: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

root of this backend.

All operations will happen under this root.

default to `/` if not set.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.bucket" class="anchor field">Â§</a>`bucket: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

bucket name of this backend.

required.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.enable_versioning" class="anchor field">Â§</a>`enable_versioning: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

is bucket versioning enabled for this bucket

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.endpoint" class="anchor field">Â§</a>`endpoint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

endpoint of this backend.

Endpoint must be full uri, e.g.

- AWS S3: `https://s3.amazonaws.com` or `https://s3.{region}.amazonaws.com`
- Cloudflare R2: `https://<ACCOUNT_ID>.r2.cloudflarestorage.com`
- Aliyun OSS: `https://{region}.aliyuncs.com`
- Tencent COS: `https://cos.{region}.myqcloud.com`
- Minio: `http://127.0.0.1:9000`

If user inputs endpoint without scheme like â€œs3.amazonaws.comâ€?, we will prepend â€œhttps://â€? before it.

- If endpoint is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.
- If still not set, default to `https://s3.amazonaws.com`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.region" class="anchor field">Â§</a>`region: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Region represent the signing region of this endpoint. This is required if you are using the default AWS S3 endpoint.

If using a custom endpoint,

- If region is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.access_key_id" class="anchor field">Â§</a>`access_key_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

access_key_id of this backend.

- If access_key_id is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.secret_access_key" class="anchor field">Â§</a>`secret_access_key: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

secret_access_key of this backend.

- If secret_access_key is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.session_token" class="anchor field">Â§</a>`session_token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

session_token (aka, security token) of this backend.

This token will expire after sometime, itâ€™s recommended to set session_token by hand.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.role_arn" class="anchor field">Â§</a>`role_arn: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

role_arn for this backend.

If `role_arn` is set, we will use already known config as source credential to assume role with `role_arn`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.external_id" class="anchor field">Â§</a>`external_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

external_id for this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.role_session_name" class="anchor field">Â§</a>`role_session_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

role_session_name for this backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.disable_config_load" class="anchor field">Â§</a>`disable_config_load: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Disable config load so that opendal will not load config from environment.

For examples:

- envs like `AWS_ACCESS_KEY_ID`
- files like `~/.aws/config`

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.disable_ec2_metadata" class="anchor field">Â§</a>`disable_ec2_metadata: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Disable load credential from ec2 metadata.

This option is used to disable the default behavior of opendal to load credential from ec2 metadata, a.k.a, IMDSv2

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.allow_anonymous" class="anchor field">Â§</a>`allow_anonymous: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow anonymous will allow opendal to send request without signing when credential is not loaded.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.server_side_encryption" class="anchor field">Â§</a>`server_side_encryption: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

server_side_encryption for this backend.

Available values: `AES256`, `aws:kms`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.server_side_encryption_aws_kms_key_id" class="anchor field">Â§</a>`server_side_encryption_aws_kms_key_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

server_side_encryption_aws_kms_key_id for this backend

- If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id` is not set, S3 will use aws managed kms key to encrypt data.
- If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id` is a valid kms key id, S3 will use the provided kms key to encrypt data.
- If the `server_side_encryption_aws_kms_key_id` is invalid or not found, an error will be returned.
- If `server_side_encryption` is not `aws:kms`, setting `server_side_encryption_aws_kms_key_id` is a noop.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.server_side_encryption_customer_algorithm" class="anchor field">Â§</a>`server_side_encryption_customer_algorithm: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

server_side_encryption_customer_algorithm for this backend.

Available values: `AES256`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.server_side_encryption_customer_key" class="anchor field">Â§</a>`server_side_encryption_customer_key: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

server_side_encryption_customer_key for this backend.

Value: BASE64-encoded key that matches algorithm specified in `server_side_encryption_customer_algorithm`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.server_side_encryption_customer_key_md5" class="anchor field">Â§</a>`server_side_encryption_customer_key_md5: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set server_side_encryption_customer_key_md5 for this backend.

Value: MD5 digest of key specified in `server_side_encryption_customer_key`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.default_storage_class" class="anchor field">Â§</a>`default_storage_class: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

default storage_class for this backend.

Available values:

- `DEEP_ARCHIVE`
- `GLACIER`
- `GLACIER_IR`
- `INTELLIGENT_TIERING`
- `ONEZONE_IA`
- `EXPRESS_ONEZONE`
- `OUTPOSTS`
- `REDUCED_REDUNDANCY`
- `STANDARD`
- `STANDARD_IA`

S3 compatible services donâ€™t support all of them

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.enable_virtual_host_style" class="anchor field">Â§</a>`enable_virtual_host_style: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Enable virtual host style so that opendal will send API requests in virtual host style instead of path style.

- By default, opendal will send API to `https://s3.us-east-1.amazonaws.com/bucket_name`
- Enabled, opendal will send API to `https://bucket_name.s3.us-east-1.amazonaws.com`

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.batch_max_operations" class="anchor field">Â§</a>`batch_max_operations: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

ðŸ‘ŽDeprecated since 0.52.0: Please use `delete_max_size` instead of `batch_max_operations`

Set maximum batch operations of this backend.

Some compatible services have a limit on the number of operations in a batch request. For example, R2 could return `Internal Error` while batch delete 1000 files.

Please tune this value based on servicesâ€™ document.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.delete_max_size" class="anchor field">Â§</a>`delete_max_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Set the maximum delete size of this backend.

Some compatible services have a limit on the number of operations in a batch request. For example, R2 could return `Internal Error` while batch delete 1000 files.

Please tune this value based on servicesâ€™ document.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.disable_stat_with_override" class="anchor field">Â§</a>`disable_stat_with_override: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Disable stat with override so that opendal will not send stat request with override queries.

For example, R2 doesnâ€™t support stat with `response_content_type` query.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.checksum_algorithm" class="anchor field">Â§</a>`checksum_algorithm: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Checksum Algorithm to use when sending checksums in HTTP headers. This is necessary when writing to AWS S3 Buckets with Object Lock enabled for example.

Available options:

- â€œcrc32câ€?

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.disable_write_with_if_match" class="anchor field">Â§</a>`disable_write_with_if_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Disable write with if match so that opendal will not send write request with if match headers.

For example, Ceph RADOS S3 doesnâ€™t support write with if match.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.enable_write_with_append" class="anchor field">Â§</a>`enable_write_with_append: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Enable write with append so that opendal will send write request with append headers.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.disable_list_objects_v2" class="anchor field">Â§</a>`disable_list_objects_v2: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

OpenDAL uses List Objects V2 by default to list objects. However, some legacy services do not yet support V2. This option allows users to switch back to the older List Objects V1.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#structfield.enable_request_payer" class="anchor field">Â§</a>`enable_request_payer: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates whether the client agrees to pay for the requests made to the S3 bucket.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Clone-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Configurator-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#associatedtype.Builder" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

Associated builder for this configuration.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.from_uri" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_uri" class="fn">from_uri</a>(uri: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Build configuration from a parsed URI plus merged options.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.into_builder" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#tymethod.into_builder" class="fn">into_builder</a>(self) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype" title="type opendal::Configurator::Builder">Builder</a>

Convert this configuration into a service builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.from_iter" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter" class="fn">from_iter</a>(iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Deserialize from an iterator. [Read more](https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Debug-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Default-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Deserialize%3C&#39;de%3E-for-S3Config" class="anchor">Â§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

where <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.deserialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-PartialEq-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Serialize-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#method.serialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-Eq-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#impl-StructuralPartialEq-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html#blanket-implementations" class="anchor">Â§</a>
