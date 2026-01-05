# Struct YandexDisk Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/yandex_disk/backend.rs.html#41-46" class="src">Source</a>

``` rust
pub struct YandexDisk { /* private fields */ }
```

Expand description

[YandexDisk](https://360.yandex.com/disk/) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `access_token` YandexDisk oauth access_token

You can refer to [`YandexDiskBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html "struct opendal::services::YandexDisk")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::YandexDisk;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = YandexDisk::default()
        // set the storage bucket for OpenDAL
        .root("/")
        // set the access_token for OpenDAL
        .access_token("test");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#impl-YandexDiskBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-yandex-disk`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#method.access_token" class="fn">access_token</a>(self, access_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-yandex-disk`** only.

yandex disk oauth access_token. The valid token will looks like `y0_XXXXXXqihqIWAADLWwAAAAD3IXXXXXX0gtVeSPeIKM0oITMGhXXXXXX`. We can fetch the debug token from <https://yandex.com/dev/disk/poligon>. To use it in production, please register an app at <https://oauth.yandex.com> instead.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-yandex-disk`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#impl-Builder-for-YandexDiskBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>

Available on **crate feature `services-yandex-disk`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Builds the backend and returns the result of YandexDiskBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDiskConfig.html" class="struct" title="struct opendal::services::YandexDiskConfig">YandexDiskConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#impl-Debug-for-YandexDiskBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>

Available on **crate feature `services-yandex-disk`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#impl-Default-for-YandexDiskBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>

Available on **crate feature `services-yandex-disk`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html#blanket-implementations" class="anchor">Â§</a>
