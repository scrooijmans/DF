# Struct Azfile Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/azfile/backend.rs.html#55-60" class="src">Source</a>

``` rust
pub struct Azfile { /* private fields */ }
```

Expand description

Azure File services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☒ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `endpoint`: Set the endpoint for backend.
- `account_name`: Set the account_name for backend.
- `account_key`: Set the account_key for backend.
- `share_name`: Set the share_name for backend.

Refer to public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Azfile;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
  // Create azfile backend builder.
  let mut builder = Azfile::default()
      // Set the root for azfile, all operations will happen under this root.
      //
      // NOTE: the root must be absolute path.
      .root("/path/to/dir")
      // Set the filesystem name, this is required.
      .share_name("test")
      // Set the endpoint, this is required.
      //
      // For examples:
      // - "https://accountname.file.core.windows.net"
      .endpoint("https://accountname.file.core.windows.net")
      // Set the account_name and account_key.
      //
      // OpenDAL will try load credential from the env.
      // If credential not set and no valid credential in env, OpenDAL will
      // send request without signing like anonymous user.
      .account_name("account_name")
      .account_key("account_key");

  // `Accessor` provides the low level APIs, we will use `Operator` normally.
  let op: Operator = Operator::new(builder)?.finish();

  Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#impl-AzfileBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azfile`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azfile`** only.

Set endpoint of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.account_name" class="fn">account_name</a>(self, account_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azfile`** only.

Set account_name of this backend.

- If account_name is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.account_key" class="fn">account_key</a>(self, account_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azfile`** only.

Set account_key of this backend.

- If account_key is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.share_name" class="fn">share_name</a>(self, share_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azfile`** only.

Set file share name of this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#notes" class="doc-anchor">Â§</a>Notes

You can find more about from: <https://learn.microsoft.com/en-us/rest/api/storageservices/operations-on-shares--file-service>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-azfile`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.from_connection_string" class="fn">from_connection_string</a>(conn_str: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Available on **crate feature `services-azfile`** only.

Create a new `AfileBuilder` instance from an [Azure Storage connection string](https://learn.microsoft.com/en-us/azure/storage/common/storage-configure-connection-string).

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#example-1" class="doc-anchor">Â§</a>Example

``` rust
use opendal::Builder;
use opendal::services::Azfile;

let conn_str = "AccountName=example;DefaultEndpointsProtocol=https;EndpointSuffix=core.windows.net";

let mut config = Azfile::from_connection_string(&conn_str)
    .unwrap()
    // Add additional configuration if needed
    .share_name("myShare")
    .build()
    .unwrap();
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#impl-Builder-for-AzfileBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Available on **crate feature `services-azfile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzfileConfig.html" class="struct" title="struct opendal::services::AzfileConfig">AzfileConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#impl-Clone-for-AzfileBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Available on **crate feature `services-azfile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#impl-Debug-for-AzfileBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Available on **crate feature `services-azfile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#impl-Default-for-AzfileBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Available on **crate feature `services-azfile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html#blanket-implementations" class="anchor">Â§</a>
