# Struct HdfsNative Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/hdfs_native/backend.rs.html#39-41" class="src">Source</a>

``` rust
pub struct HdfsNative { /* private fields */ }
```

Expand description

[Hadoop Distributed File System (HDFSâ„¢)](https://hadoop.apache.org/) support. Using [Native Rust HDFS client](https://github.com/Kimahriman/hdfs-native). A distributed file system that provides high-throughput access to application data. Using [Native Rust HDFS client](https://github.com/Kimahriman/hdfs-native).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ rename
- ☒ list
- ☒ blocking
- ☒ append

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#differences-with-webhdfs" class="doc-anchor">Â§</a>Differences with webhdfs

[Webhdfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html "struct opendal::services::Webhdfs") is powered by hdfsâ€™s RESTful HTTP API.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#differences-with-hdfs" class="doc-anchor">Â§</a>Differences with hdfs

[hdfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html "struct opendal::services::Hdfs") is powered by libhdfs and require the Java dependencies

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#features" class="doc-anchor">Â§</a>Features

HDFS-native support needs to enable feature `services-hdfs-native`.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `name_node`: Set the name node for backend.
- `enable_append`: enable the append capacity. Default is false.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#impl-HdfsNativeBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-hdfs-native`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#method.name_node" class="fn">name_node</a>(self, name_node: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-hdfs-native`** only.

Set name_node of this backend.

Valid format including:

- `default`: using the default setting based on hadoop config.
- `hdfs://127.0.0.1:9000`: connect to hdfs cluster.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#method.enable_append" class="fn">enable_append</a>(self, enable_append: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-hdfs-native`** only.

Enable append capacity of this backend.

This should be disabled when HDFS runs in non-distributed mode.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#impl-Builder-for-HdfsNativeBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

Available on **crate feature `services-hdfs-native`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNativeConfig.html" class="struct" title="struct opendal::services::HdfsNativeConfig">HdfsNativeConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#impl-Debug-for-HdfsNativeBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

Available on **crate feature `services-hdfs-native`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#impl-Default-for-HdfsNativeBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

Available on **crate feature `services-hdfs-native`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html#blanket-implementations" class="anchor">Â§</a>
