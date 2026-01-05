# Struct Hdfs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/hdfs/backend.rs.html#38-40" class="src">Source</a>

``` rust
pub struct Hdfs { /* private fields */ }
```

Expand description

A distributed file system that provides high-throughput access to application data.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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
- ☒ blocking
- ☒ append

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#differences-with-webhdfs" class="doc-anchor">Â§</a>Differences with webhdfs

[Webhdfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html "struct opendal::services::Webhdfs") is powered by hdfsâ€™s RESTful HTTP API.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#features" class="doc-anchor">Â§</a>Features

HDFS support needs to enable feature `services-hdfs`.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `name_node`: Set the name node for backend.
- `kerberos_ticket_cache_path`: Set the kerberos ticket cache path for backend, this should be gotten by `klist` after `kinit`
- `user`: Set the user for backend
- `enable_append`: enable the append capacity. Default is false.

Refer to [`HdfsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html "struct opendal::services::Hdfs")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#environment" class="doc-anchor">Â§</a>Environment

HDFS needs some environment set correctly.

- `JAVA_HOME`: the path to java home, could be found via `java -XshowSettings:properties -version`
- `HADOOP_HOME`: the path to hadoop home, opendal relays on this env to discover hadoop jars and set `CLASSPATH` automatically.

Most of the time, setting `JAVA_HOME` and `HADOOP_HOME` is enough. But there are some edge cases:

- If meeting errors like the following:

``` shell
error while loading shared libraries: libjvm.so: cannot open shared object file: No such file or directory
```

Javaâ€™s lib are not including in pkg-config find path, please set `LD_LIBRARY_PATH`:

``` shell
export LD_LIBRARY_PATH=${JAVA_HOME}/lib/server:${LD_LIBRARY_PATH}
```

The path of `libjvm.so` could be different, please keep an eye on it.

- If meeting errors like the following:

``` shell
(unable to get stack trace for java.lang.NoClassDefFoundError exception: ExceptionUtils::getStackTrace error.)
```

`CLASSPATH` is not set correctly or your hadoop installation is incorrect.

To set `CLASSPATH`:

``` shell
export CLASSPATH=$(find $HADOOP_HOME -iname "*.jar" | xargs echo | tr ' ' ':'):${CLASSPATH}
```

- If HDFS has High Availability (HA) enabled with multiple available NameNodes, some configuration is required:

1.  Obtain the entire HDFS config folder (usually located at HADOOP_HOME/etc/hadoop).
2.  Set the environment variable HADOOP_CONF_DIR to the path of this folder.

``` shell
export HADOOP_CONF_DIR=<path of the config folder>
```

3.  Append the HADOOP_CONF_DIR to the `CLASSPATH`

``` shell
export CLASSPATH=$HADOOP_CONF_DIR:$HADOOP_CLASSPATH:$CLASSPATH
```

4.  Use the `cluster_name` specified in the `core-site.xml` file (located in the HADOOP_CONF_DIR folder) to replace namenode:port.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
builder.name_node("hdfs://cluster_name");
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#macos-specific-note" class="doc-anchor">Â§</a>macOS Specific Note

If you encounter an issue during the build process on macOS with an error message similar to:

``` shell
ld: unknown file type in $HADOOP_HOME/lib/native/libhdfs.so.0.0.0
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

This error is likely due to the fact that the official Hadoop build includes the libhdfs.so file for the x86-64 architecture, which is not compatible with aarch64 architecture required for MacOS.

To resolve this issue, you can add hdrs as a dependency in your Rust applicationâ€™s Cargo.toml file, and enable the vendored feature:

``` toml
[dependencies]
hdrs = { version = "<version_number>", features = ["vendored"] }
```

Enabling the vendored feature ensures that hdrs includes the necessary libhdfs.so library built for the correct architecture.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Hdfs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create fs backend builder.
    let mut builder = Hdfs::default()
        // Set the name node for hdfs.
        // If the string starts with a protocol type such as file://, hdfs://, or gs://, this protocol type will be used.
        .name_node("hdfs://127.0.0.1:9000")
        // Set the root for hdfs, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/tmp")
        
        // Enable the append capacity for hdfs. 
        // 
        // Note: HDFS run in non-distributed mode doesn't support append.
        .enable_append(true);

    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#impl-HdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-hdfs`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.name_node" class="fn">name_node</a>(self, name_node: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-hdfs`** only.

Set name_node of this backend.

Valid format including:

- `default`: using the default setting based on hadoop config.
- `hdfs://127.0.0.1:9000`: connect to hdfs cluster.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.kerberos_ticket_cache_path" class="fn">kerberos_ticket_cache_path</a>( self, kerberos_ticket_cache_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

Available on **crate feature `services-hdfs`** only.

Set kerberos_ticket_cache_path of this backend

This should be configured when kerberos is enabled.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.user" class="fn">user</a>(self, user: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-hdfs`** only.

Set user of this backend

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.enable_append" class="fn">enable_append</a>(self, enable_append: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-hdfs`** only.

Enable append capacity of this backend.

This should be disabled when HDFS runs in non-distributed mode.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.atomic_write_dir" class="fn">atomic_write_dir</a>(self, dir: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-hdfs`** only.

Set temp dir for atomic write.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#notes" class="doc-anchor">Â§</a>Notes

- When append is enabled, we will not use atomic write to avoid data loss and performance issue.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#impl-Builder-for-HdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

Available on **crate feature `services-hdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsConfig.html" class="struct" title="struct opendal::services::HdfsConfig">HdfsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#impl-Debug-for-HdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

Available on **crate feature `services-hdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#impl-Default-for-HdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

Available on **crate feature `services-hdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html#blanket-implementations" class="anchor">Â§</a>
