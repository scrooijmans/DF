# Struct AliyunDrive Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/aliyun_drive/backend.rs.html#41-46" class="src">Source</a>

``` rust
pub struct AliyunDrive { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `access_token`: Set the access_token for backend.
- `client_id`: Set the client_id for backend.
- `client_secret`: Set the client_secret for backend.
- `refresh_token`: Set the refresh_token for backend.
- `drive_type`: Set the drive_type for backend.

Refer to [`AliyunDriveBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html "struct opendal::services::AliyunDrive")\`s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#basic-setup" class="doc-anchor">Â§</a>Basic Setup

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::AliyunDrive;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create aliyun-drive backend builder.
    let mut builder = AliyunDrive::default()
        // Set the root for aliyun-drive, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/path/to/dir")
        // Set the client_id. This is required.
        .client_id("client_id")
        // Set the client_secret. This is required.
        .client_secret("client_secret")
        // Set the refresh_token. This is required.
        .refresh_token("refresh_token")
        // Set the drive_type. This is required.
        //
        // Fallback to the default type if no other types found.
        .drive_type("resource");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#impl-AliyunDriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-aliyun-drive`** only.

Set the root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.access_token" class="fn">access_token</a>(self, access_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-aliyun-drive`** only.

Set access_token of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.client_id" class="fn">client_id</a>(self, client_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-aliyun-drive`** only.

Set client_id of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.client_secret" class="fn">client_secret</a>(self, client_secret: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-aliyun-drive`** only.

Set client_secret of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.refresh_token" class="fn">refresh_token</a>(self, refresh_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-aliyun-drive`** only.

Set refresh_token of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.drive_type" class="fn">drive_type</a>(self, drive_type: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-aliyun-drive`** only.

Set drive_type of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-aliyun-drive`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#impl-Builder-for-AliyunDriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

Available on **crate feature `services-aliyun-drive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDriveConfig.html" class="struct" title="struct opendal::services::AliyunDriveConfig">AliyunDriveConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#impl-Debug-for-AliyunDriveBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

Available on **crate feature `services-aliyun-drive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#impl-Default-for-AliyunDriveBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

Available on **crate feature `services-aliyun-drive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html#blanket-implementations" class="anchor">Â§</a>
