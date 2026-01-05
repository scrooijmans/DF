# Struct Dbfs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/dbfs/backend.rs.html#40-42" class="src">Source</a>

``` rust
pub struct Dbfs { /* private fields */ }
```

Expand description

[Dbfs](https://docs.databricks.com/api/azure/workspace/dbfs)â€™s REST API support. This service will visit the [DBFS API](https://docs.databricks.com/api/azure/workspace/dbfs) supported by [Databricks File System](https://docs.databricks.com/en/dbfs/index.html).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☒ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#configurations" class="doc-anchor">Â§</a>Configurations

- `root`: Set the work directory for backend.
- `endpoint`: Set the endpoint for backend.
- `token`: Databricks personal access token.

Refer to [`DbfsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html "struct opendal::services::Dbfs")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#examples" class="doc-anchor">Â§</a>Examples

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Dbfs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Dbfs::default()
        // set the root for Dbfs, all operations will happen under this root
        //
        // Note:
        // if the root is not exists, the builder will automatically create the
        // root directory for you
        // if the root exists and is a directory, the builder will continue working
        // if the root exists and is a folder, the builder will fail on building backend
        .root("/path/to/dir")
        // set the endpoint of Dbfs workspace
        .endpoint("https://adb-1234567890123456.78.azuredatabricks.net")
        // set the personal access token for builder
        .token("access_token");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#impl-DbfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dbfs`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dbfs`** only.

Set endpoint of this backend.

Endpoint must be full uri, e.g.

- Azure: `https://adb-1234567890123456.78.azuredatabricks.net`
- Aws: `https://dbc-123a5678-90bc.cloud.databricks.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dbfs`** only.

Set the token of this backend.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#impl-Builder-for-DbfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Available on **crate feature `services-dbfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Build a DbfsBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DbfsConfig.html" class="struct" title="struct opendal::services::DbfsConfig">DbfsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#impl-Clone-for-DbfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Available on **crate feature `services-dbfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#impl-Debug-for-DbfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Available on **crate feature `services-dbfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#impl-Default-for-DbfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Available on **crate feature `services-dbfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html#blanket-implementations" class="anchor">Â§</a>
