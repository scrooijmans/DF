# Struct Webhdfs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/webhdfs/backend.rs.html#46-48" class="src">Source</a>

``` rust
pub struct Webhdfs { /* private fields */ }
```

Expand description

[WebHDFS](https://hadoop.apache.org/docs/stable/hadoop-project-dist/hadoop-hdfs/WebHDFS.html)â€™s REST API support. There two implementations of WebHDFS REST API:

- Native via HDFS Namenode and Datanode, data are transferred between nodes directly.
- [HttpFS](https://hadoop.apache.org/docs/stable/hadoop-hdfs-httpfs/index.html) is a gateway before hdfs nodes, data are proxied.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#differences-with-hdfs" class="doc-anchor">Â§</a>Differences with HDFS

[Hdfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html "struct opendal::services::Hdfs") is powered by HDFSâ€™s native java client. Users need to set up the HDFS services correctly. But webhdfs can access from HTTP API and no extra setup needed.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#webhdfs-compatibility-guidelines" class="doc-anchor">Â§</a>WebHDFS Compatibility Guidelines

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#file-creation-and-write" class="doc-anchor">Â§</a>File Creation and Write

For [File creation and write](https://hadoop.apache.org/docs/r3.1.3/hadoop-project-dist/hadoop-hdfs/WebHDFS.html#Create_and_Write_to_a_File) operations, OpenDAL WebHDFS is optimized for Hadoop Distributed File System (HDFS) versions 2.9 and later. This involves two API calls in webhdfs, where the initial `put` call to the namenode is redirected to the datanode handling the file data. The optional `noredirect` flag can be set to prevent redirection. If used, the API response body contains the datanode URL, which is then utilized for the subsequent `put` call with the actual file data. OpenDAL automatically sets the `noredirect` flag with the first `put` call. This feature is supported starting from HDFS version 2.9.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#multi-write-support" class="doc-anchor">Â§</a>Multi-Write Support

OpenDAL WebHDFS supports multi-write operations by creating temporary files in the specified `atomic_write_dir`. The final concatenation of these temporary files occurs when the writer is closed. However, itâ€™s essential to be aware of HDFS concat restrictions for earlier versions, where the target file must not be empty, and its last block must be full. Due to these constraints, the concat operation might fail for HDFS 2.6. This issue, identified as [HDFS-6641](https://issues.apache.org/jira/browse/HDFS-6641), has been addressed in later versions of HDFS.

In summary, OpenDAL WebHDFS is designed for optimal compatibility with HDFS, specifically versions 2.9 and later.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#configurations" class="doc-anchor">Â§</a>Configurations

- `root`: The root path of the WebHDFS service.
- `endpoint`: The endpoint of the WebHDFS service.
- `delegation`: The delegation token for WebHDFS.
- `atomic_write_dir`: The tmp write dir of multi write for WebHDFS.Needs to be configured for multi write support.

Refer to [`Builder`](https://opendal.apache.org/docs/rust/opendal/trait.Builder.html "trait opendal::Builder")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#examples" class="doc-anchor">Â§</a>Examples

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Webhdfs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Webhdfs::default()
        // set the root for WebHDFS, all operations will happen under this root
        //
        // Note:
        // if the root exists, the builder will automatically create the
        // root directory for you
        // if the root exists and is a directory, the builder will continue working
        // if the root exists and is a file, the builder will fail on building backend
        .root("/path/to/dir")
        // set the endpoint of webhdfs namenode, controlled by dfs.namenode.http-address
        // default is http://127.0.0.1:9870
        .endpoint("http://127.0.0.1:9870")
        // set the delegation_token for builder
        .delegation("delegation_token")
        // set atomic_write_dir for builder
        .atomic_write_dir(".opendal_tmp/");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#impl-WebhdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webhdfs`** only.

Set the working directory of this backend

All operations will happen under this root

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#note" class="doc-anchor">Â§</a>Note

The root will be automatically created if not exists.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webhdfs`** only.

Set the remote address of this backend default to `http://127.0.0.1:9870`

Endpoints should be full uri, e.g.

- `https://webhdfs.example.com:9870`
- `http://192.168.66.88:9870`

If user inputs endpoint without scheme, we will prepend `http://` to it.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.user_name" class="fn">user_name</a>(self, user_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webhdfs`** only.

Set the username of this backend, used for authentication

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.delegation" class="fn">delegation</a>(self, delegation: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webhdfs`** only.

Set the delegation token of this backend, used for authentication

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#note-1" class="doc-anchor">Â§</a>Note

The builder prefers using delegation token over username. If both are set, delegation token will be used.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.disable_list_batch" class="fn">disable_list_batch</a>(self) -\> Self

Available on **crate feature `services-webhdfs`** only.

Disable batch listing

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#note-2" class="doc-anchor">Â§</a>Note

When listing a directory, the backend will default to use batch listing. If disabled, the backend will list all files/directories in one request.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.atomic_write_dir" class="fn">atomic_write_dir</a>(self, dir: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webhdfs`** only.

Set temp dir for atomic write.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#notes" class="doc-anchor">Â§</a>Notes

If not set, write multi not support, eg: `.opendal_tmp/`.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#impl-Builder-for-WebhdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Available on **crate feature `services-webhdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

build the backend

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#note-3" class="doc-anchor">Â§</a>Note

when building backend, the built backend will check if the root directory exits. if the directory does not exit, the directory will be automatically created

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebhdfsConfig.html" class="struct" title="struct opendal::services::WebhdfsConfig">WebhdfsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#impl-Clone-for-WebhdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Available on **crate feature `services-webhdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#impl-Debug-for-WebhdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Available on **crate feature `services-webhdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#impl-Default-for-WebhdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Available on **crate feature `services-webhdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html#blanket-implementations" class="anchor">Â§</a>
