# Struct Operator Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/operator.rs.html#148-151" class="src">Source</a>

``` rust
pub struct Operator { /* private fields */ }
```

Expand description

The `Operator` serves as the entry point for all public asynchronous APIs.

For more details about the `Operator`, refer to the [`concepts`](https://opendal.apache.org/docs/rust/opendal/docs/concepts/index.html "mod opendal::docs::concepts") section.

All cloned `Operator` instances share the same internal state, such as `HttpClient` and `Runtime`. Some layers may modify the internal state of the `Operator` too like inject logging and metrics for `HttpClient`.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#build" class="doc-anchor">Â§</a>Build

Users can initialize an `Operator` through the following methods:

- [`Operator::new`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.new "associated function opendal::Operator::new"): Creates an operator using a [`services`](https://opendal.apache.org/docs/rust/opendal/services/index.html "mod opendal::services") builder, such as [`services::S3`](https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html "struct opendal::services::S3").
- [`Operator::from_config`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_config "associated function opendal::Operator::from_config"): Creates an operator using a [`services`](https://opendal.apache.org/docs/rust/opendal/services/index.html "mod opendal::services") configuration, such as [`services::S3Config`](https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html "struct opendal::services::S3Config").
- [`Operator::from_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_iter "associated function opendal::Operator::from_iter"): Creates an operator from an iterator of configuration key-value pairs.

``` rust
use opendal::services::Memory;
use opendal::Operator;
async fn test() -> Result<()> {
    // Build an `Operator` to start operating the storage.
    let _: Operator = Operator::new(Memory::default())?.finish();

    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#layer" class="doc-anchor">Â§</a>Layer

After the operator is built, users can add the layers they need on top of it.

OpenDAL offers various layers for users to choose from, such as `RetryLayer`, `LoggingLayer`, and more. Visit [`layers`](https://opendal.apache.org/docs/rust/opendal/layers/index.html "mod opendal::layers") for further details.

Please note that `Layer` can modify internal contexts such as `HttpClient` and `Runtime` for all clones of given operator. Therefore, it is recommended to add layers before interacting with the storage. Adding or duplicating layers after accessing the storage may result in unexpected behavior.

``` rust
use opendal::layers::RetryLayer;
use opendal::services::Memory;
use opendal::Operator;
async fn test() -> Result<()> {
    let op: Operator = Operator::new(Memory::default())?.finish();

    // OpenDAL will retry failed operations now.
    let op = op.layer(RetryLayer::default());

    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#operate" class="doc-anchor">Â§</a>Operate

After the operator is built and the layers are added, users can start operating the storage.

The operator is `Send`, `Sync`, and `Clone`. It has no internal state, and all APIs only take a `&self` reference, making it safe to share the operator across threads.

Operator provides a consistent API pattern for data operations. For reading operations, it exposes:

- [`Operator::read`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read "method opendal::Operator::read"): Executes a read operation.
- [`Operator::read_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read_with "method opendal::Operator::read_with"): Executes a read operation with additional options using the builder pattern.
- [`Operator::read_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read_options "method opendal::Operator::read_options"): Executes a read operation with extra options provided via a [`options::ReadOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html "struct opendal::options::ReadOptions") struct.
- [`Operator::reader`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader "method opendal::Operator::reader"): Creates a reader for streaming data, allowing for flexible access.
- [`Operator::reader_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader_with "method opendal::Operator::reader_with"): Creates a reader with advanced options using the builder pattern.
- [`Operator::reader_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader_options "method opendal::Operator::reader_options"): Creates a reader with extra options provided via a [`options::ReadOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html "struct opendal::options::ReadOptions") struct.

The [`Reader`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html "struct opendal::Reader") created by [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") supports custom read control methods and can be converted into \[`futures::AsyncRead`\] or \[`futures::Stream`\] for broader ecosystem compatibility.

``` rust
use opendal::layers::LoggingLayer;
use opendal::options;
use opendal::services;
use opendal::Operator;
use opendal::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Pick a builder and configure it.
    let mut builder = services::S3::default().bucket("test");

    // Init an operator
    let op = Operator::new(builder)?
        // Init with logging layer enabled.
        .layer(LoggingLayer::default())
        .finish();

    // Fetch this file's metadata
    let meta = op.stat("hello.txt").await?;
    let length = meta.content_length();

    // Read data from `hello.txt` with options.
    let bs = op
        .read_with("hello.txt")
        .range(0..8 * 1024 * 1024)
        .chunk(1024 * 1024)
        .concurrent(4)
        .await?;

    // The same to:
    let bs = op
        .read_options("hello.txt", options::ReadOptions {
            range: (0..8 * 1024 * 1024).into(),
            chunk: Some(1024 * 1024),
            concurrent: 4,
            ..Default::default()
        })
        .await?;

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-Operator" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#operator-basic-api" class="doc-anchor">Â§</a>Operator basic API.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html" class="type" title="type opendal::raw::Accessor">Accessor</a>

Fetch the internal accessor.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_inner" class="fn">from_inner</a>(accessor: <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html" class="type" title="type opendal::raw::Accessor">Accessor</a>) -\> Self

Convert inner accessor into operator.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html" class="type" title="type opendal::raw::Accessor">Accessor</a>

Convert operator into inner accessor.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.info" class="fn">info</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

Get information of underlying accessor.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::Operator;

let info = op.info();
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.executor" class="fn">executor</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

Get the executor used by current operator.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.update_executor" class="fn">update_executor</a>(&self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>)

Update executor for the context.

All cloned `Operator` instances share the same internal state, such as `HttpClient` and `Runtime`. Some layers may modify the internal state of the `Operator` too like inject logging and metrics for `HttpClient`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#note" class="doc-anchor">Â§</a>Note

Tasks must be forwarded to the old executor after the update. Otherwise, features such as retry, timeout, and metrics may not function properly.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.http_client" class="fn">http_client</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

ðŸ‘ŽDeprecated since 0.54.0: Use HttpClientLayer instead. This method will be removed in next version.

Get the http client used by current operator.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.update_http_client" class="fn">update_http_client</a>(&self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>)

ðŸ‘ŽDeprecated since 0.54.0: Use HttpClientLayer instead. This method will be removed in next version

Update http client for the context.

All cloned `Operator` instances share the same internal state, such as `HttpClient` and `Runtime`. Some layers may modify the internal state of the `Operator` too like inject logging and metrics for `HttpClient`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#note-1" class="doc-anchor">Â§</a>Note

Tasks must be forwarded to the old executor after the update. Otherwise, features such as retry, timeout, and metrics may not function properly.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#deprecated" class="doc-anchor">Â§</a>Deprecated

This method is deprecated since v0.54.0. Use [`HttpClientLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html "struct opendal::layers::HttpClientLayer") instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#migration-example" class="doc-anchor">Â§</a>Migration Example

Instead of:

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
let operator = Operator::new(service)?;
operator.update_http_client(|_| custom_client);
```

Use:

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
use opendal::layers::HttpClientLayer;

let operator = Operator::new(service)?
    .layer(HttpClientLayer::new(custom_client))
    .finish();
```

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-Operator-1" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#operator-async-api" class="doc-anchor">Â§</a>Operator async API.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.check" class="fn">check</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Check if this operator can work correctly.

We will send a `list` request to path and return any errors we met.

``` rust
use opendal::Operator;

op.check().await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.stat" class="fn">stat</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Retrieve the metadata for the specified path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options" class="doc-anchor">Â§</a>Extra Options

[`Operator::stat`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.stat "method opendal::Operator::stat") is a wrapper around [`Operator::stat_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.stat_with "method opendal::Operator::stat_with") that uses no additional options. To specify extra options such as `if_match` and `if_none_match`, please use [`Operator::stat_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.stat_with "method opendal::Operator::stat_with") instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-1" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#check-if-file-exists" class="doc-anchor">Â§</a>Check if file exists

``` rust
use opendal::ErrorKind;
if let Err(e) = op.stat("test").await {
    if e.kind() == ErrorKind::NotFound {
        println!("file not exist")
    }
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.stat_with" class="fn">stat_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureStat.html" class="type" title="type opendal::operator_futures::FutureStat">FutureStat</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\>\>

Retrieve the metadata of the specified path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options" class="doc-anchor">Â§</a>Options

Check [`options::StatOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.StatOptions.html "struct opendal::options::StatOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-2" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#get-metadata-while-etag-matches" class="doc-anchor">Â§</a>Get metadata while `ETag` matches

`stat_with` will

- return `Ok(metadata)` if `ETag` matches
- return `Err(error)` and `error.kind() == ErrorKind::ConditionNotMatch` if file exists but `ETag` mismatch
- return `Err(err)` if other errors occur, for example, `NotFound`.

``` rust
use opendal::ErrorKind;
if let Err(e) = op.stat_with("test").if_match("<etag>").await {
    if e.kind() == ErrorKind::ConditionNotMatch {
        println!("file exists, but etag mismatch")
    }
    if e.kind() == ErrorKind::NotFound {
        println!("file not exist")
    }
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.stat_options" class="fn">stat_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.StatOptions.html" class="struct" title="struct opendal::options::StatOptions">StatOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Retrieve the metadata of the specified path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-3" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#get-metadata-while-etag-matches-1" class="doc-anchor">Â§</a>Get metadata while `ETag` matches

`stat_with` will

- return `Ok(metadata)` if `ETag` matches
- return `Err(error)` and `error.kind() == ErrorKind::ConditionNotMatch` if file exists but `ETag` mismatch
- return `Err(err)` if other errors occur, for example, `NotFound`.

``` rust
use opendal::options;
use opendal::ErrorKind;
let res = op
    .stat_options("test", options::StatOptions {
        if_match: Some("<etag>".to_string()),
        ..Default::default()
    })
    .await;
if let Err(e) = res {
    if e.kind() == ErrorKind::ConditionNotMatch {
        println!("file exists, but etag mismatch")
    }
    if e.kind() == ErrorKind::NotFound {
        println!("file not exist")
    }
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.exists" class="fn">exists</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Check whether this path exists.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use futures::io;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let _ = op.exists("test").await?;

    Ok(())
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.create_dir" class="fn">create_dir</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Create a directory at the specified path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-1" class="doc-anchor">Â§</a>Notes

To specify that a path is a directory, you must include a trailing slash (/). Omitting the trailing slash may cause OpenDAL to return a `NotADirectory` error.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#behavior" class="doc-anchor">Â§</a>Behavior

- Creating a directory that already exists will succeed.
- Directory creation is always recursive, functioning like `mkdir -p`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-4" class="doc-anchor">Â§</a>Examples

``` rust
op.create_dir("path/to/dir/").await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read" class="fn">read</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read the entire file into bytes from given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-2" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#additional-options" class="doc-anchor">Â§</a>Additional Options

[`Operator::read`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read "method opendal::Operator::read") is a simplified method that does not support additional options. To access features like `range` and `if_match`, please use [`Operator::read_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read_with "method opendal::Operator::read_with") or [`Operator::read_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read_options "method opendal::Operator::read_options") instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-read" class="doc-anchor">Â§</a>Streaming Read

This function reads all content into memory at once. For more precise memory management or to read big file lazily, please use [`Operator::reader`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader "method opendal::Operator::reader").

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-5" class="doc-anchor">Â§</a>Examples

``` rust
let bs = op.read("path/to/file").await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read_with" class="fn">read_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureRead.html" class="type" title="type opendal::operator_futures::FutureRead">FutureRead</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>\>

Read the entire file into bytes from given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-3" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-read-1" class="doc-anchor">Â§</a>Streaming Read

This function reads all content into memory at once. For more precise memory management or to read big file lazily, please use [`Operator::reader`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader "method opendal::Operator::reader").

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-1" class="doc-anchor">Â§</a>Options

Visit [`options::ReadOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html "struct opendal::options::ReadOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-6" class="doc-anchor">Â§</a>Examples

Read the first 10 bytes of a file:

``` rust
let bs = op.read_with("path/to/file").range(0..10).await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.read_options" class="fn">read_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read the entire file into bytes from given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-4" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-read-2" class="doc-anchor">Â§</a>Streaming Read

This function reads all content into memory at once. For more precise memory management or to read big file lazily, please use [`Operator::reader`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader "method opendal::Operator::reader").

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-7" class="doc-anchor">Â§</a>Examples

Read the first 10 bytes of a file:

``` rust
use opendal::options;
let bs = op
    .read_options("path/to/file", options::ReadOptions {
        range: (0..10).into(),
        ..Default::default()
    })
    .await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader" class="fn">reader</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>\>

Create a new reader of given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-5" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options-1" class="doc-anchor">Â§</a>Extra Options

[`Operator::reader`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader "method opendal::Operator::reader") is a simplified method without any options. To use additional options such as `concurrent` or `if_match`, please use [`Operator::reader_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader_with "method opendal::Operator::reader_with") or [`Operator::reader_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader_options "method opendal::Operator::reader_options") instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-8" class="doc-anchor">Â§</a>Examples

``` rust
let r = op.reader("path/to/file").await?;
// Read the first 10 bytes of the file
let data = r.read(0..10).await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader_with" class="fn">reader_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureReader.html" class="type" title="type opendal::operator_futures::FutureReader">FutureReader</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>\>\>\>

Create a new reader of given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-2" class="doc-anchor">Â§</a>Options

Visit [`options::ReaderOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html "struct opendal::options::ReaderOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-9" class="doc-anchor">Â§</a>Examples

Create a reader with a specific version ID:

``` rust
let r = op.reader_with("path/to/file").version("version_id").await?;
// Read the first 10 bytes of the file
let data = r.read(0..10).await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.reader_options" class="fn">reader_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>\>

Create a new reader of given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-10" class="doc-anchor">Â§</a>Examples

Create a reader with a specific version ID:

``` rust
use opendal::options;
let r = op
    .reader_options("path/to/file", options::ReaderOptions {
        version: Some("version_id".to_string()),
        ..Default::default()
    })
    .await?;
// Read the first 10 bytes of the file
let data = r.read(0..10).await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.write" class="fn">write</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Write all data to the specified path at once.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-6" class="doc-anchor">Â§</a>Notes

Visit [`performance::concurrent_write`](https://opendal.apache.org/docs/rust/opendal/docs/performance/concurrent_write/index.html "mod opendal::docs::performance::concurrent_write") for more details on concurrent writes.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options-2" class="doc-anchor">Â§</a>Extra Options

[`Operator::write`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.write "method opendal::Operator::write") is a simplified method that does not include additional options. For advanced features such as `chunk` and `concurrent`, use [`Operator::write_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.write_with "method opendal::Operator::write_with") or [`Operator::write_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.write_options "method opendal::Operator::write_options") instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-write" class="doc-anchor">Â§</a>Streaming Write

This method executes a single bulk write operation. For more precise memory management or to write data in a streaming fashion, use [`Operator::writer`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer "method opendal::Operator::writer") instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#multipart-uploads" class="doc-anchor">Â§</a>Multipart Uploads

OpenDAL offers multipart upload capabilities through the [`Writer`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html "struct opendal::Writer") abstraction, automatically managing all upload details for you. You can fine-tune the upload process by adjusting the `chunk` size and the number of `concurrent` operations using [`Operator::writer_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_with "method opendal::Operator::writer_with").

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-11" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;

op.write("path/to/file", vec![0; 4096]).await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.write_with" class="fn">write_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureWrite.html" class="type" title="type opendal::operator_futures::FutureWrite">FutureWrite</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\>\>

Write all data to the specified path at once with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-7" class="doc-anchor">Â§</a>Notes

Visit [`performance::concurrent_write`](https://opendal.apache.org/docs/rust/opendal/docs/performance/concurrent_write/index.html "mod opendal::docs::performance::concurrent_write") for more details on concurrent writes.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-write-1" class="doc-anchor">Â§</a>Streaming Write

This method executes a single bulk write operation. For more precise memory management or to write data in a streaming fashion, use [`Operator::writer`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer "method opendal::Operator::writer") instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#multipart-uploads-1" class="doc-anchor">Â§</a>Multipart Uploads

OpenDAL offers multipart upload capabilities through the [`Writer`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html "struct opendal::Writer") abstraction, automatically managing all upload details for you. You can fine-tune the upload process by adjusting the `chunk` size and the number of `concurrent` operations using [`Operator::writer_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_with "method opendal::Operator::writer_with").

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-3" class="doc-anchor">Â§</a>Options

Visit [`options::WriteOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html "struct opendal::options::WriteOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-12" class="doc-anchor">Â§</a>Examples

Write data to a file only when it does not already exist:

``` rust
use bytes::Bytes;

let _ = op
    .write_with("path/to/file", vec![0; 4096])
    .if_not_exists(true)
    .await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.write_options" class="fn">write_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Write all data to the specified path at once with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-8" class="doc-anchor">Â§</a>Notes

Visit [`performance::concurrent_write`](https://opendal.apache.org/docs/rust/opendal/docs/performance/concurrent_write/index.html "mod opendal::docs::performance::concurrent_write") for more details on concurrent writes.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-write-2" class="doc-anchor">Â§</a>Streaming Write

This method executes a single bulk write operation. For more precise memory management or to write data in a streaming fashion, use [`Operator::writer`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer "method opendal::Operator::writer") instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#multipart-uploads-2" class="doc-anchor">Â§</a>Multipart Uploads

OpenDAL offers multipart upload capabilities through the [`Writer`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html "struct opendal::Writer") abstraction, automatically managing all upload details for you. You can fine-tune the upload process by adjusting the `chunk` size and the number of `concurrent` operations using [`Operator::writer_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_with "method opendal::Operator::writer_with").

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-13" class="doc-anchor">Â§</a>Examples

Write data to a file only when it does not already exist:

``` rust
use opendal::options;

let _ = op
    .write_options("path/to/file", vec![0; 4096], options::WriteOptions {
        if_not_exists: true,
        ..Default::default()
    })
    .await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer" class="fn">writer</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html" class="struct" title="struct opendal::Writer">Writer</a>\>

Create a new writer of given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-9" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#writer-features" class="doc-anchor">Â§</a>Writer Features

The writer provides several powerful capabilities:

- Streaming writes for continuous data transfer
- Automatic multipart upload handling
- Memory-efficient chunk-based writing

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options-3" class="doc-anchor">Â§</a>Extra Options

[`Operator::writer`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer "method opendal::Operator::writer") is a simplified version that does not include additional options. For advanced features such as `chunk` and `concurrent`, use [`Operator::writer_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_with "method opendal::Operator::writer_with") or [`Operator::writer_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_options "method opendal::Operator::writer_options") instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-14" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;

let mut w = op.writer("path/to/file").await?;
w.write(vec![0; 4096]).await?;
w.write(vec![1; 4096]).await?;
w.close().await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_with" class="fn">writer_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureWriter.html" class="type" title="type opendal::operator_futures::FutureWriter">FutureWriter</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html" class="struct" title="struct opendal::Writer">Writer</a>\>\>\>

Create a new writer of given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-10" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#writer-features-1" class="doc-anchor">Â§</a>Writer Features

The writer provides several powerful capabilities:

- Streaming writes for continuous data transfer
- Automatic multipart upload handling
- Memory-efficient chunk-based writing

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#chunk-size-handling" class="doc-anchor">Â§</a>Chunk Size Handling

Storage services often have specific requirements for chunk sizes:

- Services like `s3` may return `EntityTooSmall` errors for undersized chunks
- Using small chunks in cloud storage services can lead to increased costs

OpenDAL automatically determines optimal chunk sizes based on the serviceâ€™s [Capability](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html "struct opendal::Capability"). However, you can override this by explicitly setting the `chunk` parameter.

Visit [`performance::concurrent_write`](https://opendal.apache.org/docs/rust/opendal/docs/performance/concurrent_write/index.html "mod opendal::docs::performance::concurrent_write") for more details on concurrent writes.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-15" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;

let mut w = op
    .writer_with("path/to/file")
    .chunk(4 * 1024 * 1024)
    .concurrent(8)
    .await?;
w.write(vec![0; 4096]).await?;
w.write(vec![1; 4096]).await?;
w.close().await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.writer_options" class="fn">writer_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html" class="struct" title="struct opendal::Writer">Writer</a>\>

Create a new writer of given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-11" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#writer-features-2" class="doc-anchor">Â§</a>Writer Features

The writer provides several powerful capabilities:

- Streaming writes for continuous data transfer
- Automatic multipart upload handling
- Memory-efficient chunk-based writing

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#chunk-size-handling-1" class="doc-anchor">Â§</a>Chunk Size Handling

Storage services often have specific requirements for chunk sizes:

- Services like `s3` may return `EntityTooSmall` errors for undersized chunks
- Using small chunks in cloud storage services can lead to increased costs

OpenDAL automatically determines optimal chunk sizes based on the serviceâ€™s [Capability](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html "struct opendal::Capability"). However, you can override this by explicitly setting the `chunk` parameter.

Visit [`performance::concurrent_write`](https://opendal.apache.org/docs/rust/opendal/docs/performance/concurrent_write/index.html "mod opendal::docs::performance::concurrent_write") for more details on concurrent writes.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-16" class="doc-anchor">Â§</a>Examples

Write data to a file in 4MiB chunk size and at 8 concurrency:

``` rust
use bytes::Bytes;

let mut w = op
    .writer_with("path/to/file")
    .chunk(4 * 1024 * 1024)
    .concurrent(8)
    .await?;
w.write(vec![0; 4096]).await?;
w.write(vec![1; 4096]).await?;
w.close().await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.copy" class="fn">copy</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Copy a file from `from` to `to`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-12" class="doc-anchor">Â§</a>Notes

- `from` and `to` must be a file.
- `to` will be overwritten if it exists.
- If `from` and `to` are the same, an `IsSameFile` error will occur.
- `copy` is idempotent. For same `from` and `to` input, the result will be the same.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-17" class="doc-anchor">Â§</a>Examples

``` rust

op.copy("path/to/file", "path/to/file2").await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.copy_with" class="fn">copy_with</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureCopy.html" class="type" title="type opendal::operator_futures::FutureCopy">FutureCopy</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>\>

Copy a file from `from` to `to` with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-13" class="doc-anchor">Â§</a>Notes

- `from` and `to` must be a file.
- If `from` and `to` are the same, an `IsSameFile` error will occur.
- `copy` is idempotent. For same `from` and `to` input, the result will be the same.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-4" class="doc-anchor">Â§</a>Options

Visit [`options::CopyOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html "struct opendal::options::CopyOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-18" class="doc-anchor">Â§</a>Examples

Copy a file only if the destination doesnâ€™t exist:

``` rust

op.copy_with("path/to/file", "path/to/file2")
    .if_not_exists(true)
    .await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.copy_options" class="fn">copy_options</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html" class="struct" title="struct opendal::options::CopyOptions">CopyOptions</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Copy a file from `from` to `to` with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-14" class="doc-anchor">Â§</a>Notes

- `from` and `to` must be a file.
- If `from` and `to` are the same, an `IsSameFile` error will occur.
- `copy` is idempotent. For same `from` and `to` input, the result will be the same.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-5" class="doc-anchor">Â§</a>Options

Check [`options::CopyOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.CopyOptions.html "struct opendal::options::CopyOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-19" class="doc-anchor">Â§</a>Examples

Copy a file only if the destination doesnâ€™t exist:

``` rust

let mut opts = CopyOptions::default();
opts.if_not_exists = true;
op.copy_options("path/to/file", "path/to/file2", opts).await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.rename" class="fn">rename</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Rename a file from `from` to `to`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-15" class="doc-anchor">Â§</a>Notes

- `from` and `to` must be a file.
- `to` will be overwritten if it exists.
- If `from` and `to` are the same, an `IsSameFile` error will occur.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-20" class="doc-anchor">Â§</a>Examples

``` rust

op.rename("path/to/file", "path/to/file2").await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete" class="fn">delete</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Delete the given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-16" class="doc-anchor">Â§</a>Notes

- Deleting a file that does not exist wonâ€™t return errors.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-21" class="doc-anchor">Â§</a>Examples

``` rust
op.delete("test").await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_with" class="fn">delete_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureDelete.html" class="type" title="type opendal::operator_futures::FutureDelete">FutureDelete</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>\>

Delete the given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-17" class="doc-anchor">Â§</a>Notes

- Deleting a file that does not exist wonâ€™t return errors.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-6" class="doc-anchor">Â§</a>Options

Visit [`options::DeleteOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.DeleteOptions.html "struct opendal::options::DeleteOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-22" class="doc-anchor">Â§</a>Examples

Delete a specific version of a file:

``` rust

op.delete_with("path/to/file").version(version).await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_options" class="fn">delete_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.DeleteOptions.html" class="struct" title="struct opendal::options::DeleteOptions">DeleteOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Delete the given path with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-18" class="doc-anchor">Â§</a>Notes

- Deleting a file that does not exist wonâ€™t return errors.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-23" class="doc-anchor">Â§</a>Examples

Delete a specific version of a file:

``` rust
use opendal::options;

op.delete_options("path/to/file", options::DeleteOptions {
    version: Some(version.to_string()),
    ..Default::default()
})
.await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_iter" class="fn">delete_iter</a>\<I, D\>(&self, iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = D\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an infallible iterator of paths.

Also see:

- [`Operator::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_iter "method opendal::Operator::delete_try_iter"): delete a fallible iterator of paths.
- [`Operator::delete_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_stream "method opendal::Operator::delete_stream"): delete an infallible stream of paths.
- [`Operator::delete_try_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_stream "method opendal::Operator::delete_try_stream"): delete a fallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_iter" class="fn">delete_try_iter</a>\<I, D\>(&self, try_iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<D\>\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete a fallible iterator of paths.

Also see:

- [`Operator::delete_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_iter "method opendal::Operator::delete_iter"): delete an infallible iterator of paths.
- [`Operator::delete_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_stream "method opendal::Operator::delete_stream"): delete an infallible stream of paths.
- [`Operator::delete_try_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_stream "method opendal::Operator::delete_try_stream"): delete a fallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_stream" class="fn">delete_stream</a>\<S, D\>(&self, stream: S) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where S: Stream\<Item = D\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an infallible stream of paths.

Also see:

- [`Operator::delete_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_iter "method opendal::Operator::delete_iter"): delete an infallible iterator of paths.
- [`Operator::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_iter "method opendal::Operator::delete_try_iter"): delete a fallible iterator of paths.
- [`Operator::delete_try_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_stream "method opendal::Operator::delete_try_stream"): delete a fallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_stream" class="fn">delete_try_stream</a>\<S, D\>(&self, try_stream: S) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where S: Stream\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<D\>\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete a fallible stream of paths.

Also see:

- [`Operator::delete_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_iter "method opendal::Operator::delete_iter"): delete an infallible iterator of paths.
- [`Operator::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_try_iter "method opendal::Operator::delete_try_iter"): delete a fallible iterator of paths.
- [`Operator::delete_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.delete_stream "method opendal::Operator::delete_stream"): delete an infallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.deleter" class="fn">deleter</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html" class="struct" title="struct opendal::Deleter">Deleter</a>\>

Create a [`Deleter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html "struct opendal::Deleter") to continuously remove content from storage.

It leverages batch deletion capabilities provided by storage services for efficient removal.

Users can have more control over the deletion process by using [`Deleter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html "struct opendal::Deleter") directly.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.remove_all" class="fn">remove_all</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Remove the path and all nested dirs and files recursively.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-19" class="doc-anchor">Â§</a>Notes

If underlying services support delete in batch, we will use batch delete instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-24" class="doc-anchor">Â§</a>Examples

``` rust
op.remove_all("path/to/dir").await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.list" class="fn">list</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>\>\>

List entries in the parent directory that start with the specified `path`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-20" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#recursively-list" class="doc-anchor">Â§</a>Recursively List

This function only reads the immediate children of the specified directory. To list all entries recursively, use `Operator::list_with("path").recursive(true)` instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-list" class="doc-anchor">Â§</a>Streaming List

This function reads all entries in the specified directory. If the directory contains many entries, this process may take a long time and use significant memory.

To prevent this, consider using [`Operator::lister`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister "method opendal::Operator::lister") to stream the entries instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-25" class="doc-anchor">Â§</a>Examples

This example will list all entries under the dir `path/to/dir/`.

``` rust
use opendal::EntryMode;
use opendal::Operator;
let mut entries = op.list("path/to/dir/").await?;
for entry in entries {
    match entry.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file")
        }
        EntryMode::DIR => {
            println!("Handling dir {}", entry.path())
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.list_with" class="fn">list_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureList.html" class="type" title="type opendal::operator_futures::FutureList">FutureList</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>\>\>\>\>

List entries in the parent directory that start with the specified `path` with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-21" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-list-1" class="doc-anchor">Â§</a>Streaming List

This function reads all entries in the specified directory. If the directory contains many entries, this process may take a long time and use significant memory.

To prevent this, consider using [`Operator::lister`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister "method opendal::Operator::lister") to stream the entries instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-7" class="doc-anchor">Â§</a>Options

Visit [`options::ListOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html "struct opendal::options::ListOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-26" class="doc-anchor">Â§</a>Examples

This example will list all entries recursively under the prefix `path/to/prefix`.

``` rust
use opendal::EntryMode;
use opendal::Operator;
let mut entries = op.list_with("path/to/prefix").recursive(true).await?;
for entry in entries {
    match entry.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file")
        }
        EntryMode::DIR => {
            println!("Handling dir like start a new list via meta.path()")
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.list_options" class="fn">list_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>\>\>

List entries in the parent directory that start with the specified `path` with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-22" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#streaming-list-2" class="doc-anchor">Â§</a>Streaming List

This function reads all entries in the specified directory. If the directory contains many entries, this process may take a long time and use significant memory.

To prevent this, consider using [`Operator::lister`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister "method opendal::Operator::lister") to stream the entries instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-8" class="doc-anchor">Â§</a>Options

Visit [`options::ListOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html "struct opendal::options::ListOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-27" class="doc-anchor">Â§</a>Examples

This example will list all entries recursively under the prefix `path/to/prefix`.

``` rust
use opendal::options;
use opendal::EntryMode;
use opendal::Operator;
let mut entries = op
    .list_options("path/to/prefix", options::ListOptions {
        recursive: true,
        ..Default::default()
    })
    .await?;
for entry in entries {
    match entry.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file")
        }
        EntryMode::DIR => {
            println!("Handling dir like start a new list via meta.path()")
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister" class="fn">lister</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html" class="struct" title="struct opendal::Lister">Lister</a>\>

Create a new lister to list entries that starts with given `path` in parent dir.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-23" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#recursively-list-1" class="doc-anchor">Â§</a>Recursively list

This function only reads the immediate children of the specified directory. To retrieve all entries recursively, use [`Operator::lister_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister_with "method opendal::Operator::lister_with") with `recursive(true)` instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-28" class="doc-anchor">Â§</a>Examples

``` rust
use futures::TryStreamExt;
use opendal::EntryMode;
use opendal::Operator;
let mut ds = op.lister("path/to/dir/").await?;
while let Some(mut de) = ds.try_next().await? {
    match de.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file")
        }
        EntryMode::DIR => {
            println!("Handling dir like start a new list via meta.path()")
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister_with" class="fn">lister_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FutureLister.html" class="type" title="type opendal::operator_futures::FutureLister">FutureLister</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html" class="struct" title="struct opendal::Lister">Lister</a>\>\>\>

Create a new lister to list entries that starts with given `path` in parent dir with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-9" class="doc-anchor">Â§</a>Options

Visit [`options::ListOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html "struct opendal::options::ListOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-29" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#list-all-files-recursively" class="doc-anchor">Â§</a>List all files recursively

``` rust
use futures::TryStreamExt;
use opendal::EntryMode;
use opendal::Operator;
let mut lister = op.lister_with("path/to/dir/").recursive(true).await?;
while let Some(mut entry) = lister.try_next().await? {
    match entry.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file {}", entry.path())
        }
        EntryMode::DIR => {
            println!("Handling dir {}", entry.path())
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.lister_options" class="fn">lister_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html" class="struct" title="struct opendal::Lister">Lister</a>\>

Create a new lister to list entries that starts with given `path` in parent dir with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-30" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#list-all-files-recursively-1" class="doc-anchor">Â§</a>List all files recursively

``` rust
use futures::TryStreamExt;
use opendal::options;
use opendal::EntryMode;
use opendal::Operator;
let mut lister = op
    .lister_options("path/to/dir/", options::ListOptions {
        recursive: true,
        ..Default::default()
    })
    .await?;
while let Some(mut entry) = lister.try_next().await? {
    match entry.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file {}", entry.path())
        }
        EntryMode::DIR => {
            println!("Handling dir {}", entry.path())
        }
        EntryMode::Unknown => continue,
    }
}
```

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-Operator-2" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

Operator presign API.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_stat" class="fn">presign_stat</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for stat(head).

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-1" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use futures::io;
use opendal::Operator;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_stat("test",Duration::from_secs(3600)).await?;
    let req = http::Request::builder()
        .method(signed_req.method())
        .uri(signed_req.uri())
        .body(())?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_stat_with" class="fn">presign_stat_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FuturePresignStat.html" class="type" title="type opendal::operator_futures::FuturePresignStat">FuturePresignStat</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>\>\>

Presign an operation for stat(head).

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-2" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use futures::io;
use opendal::Operator;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_stat_with("test",Duration::from_secs(3600)).override_content_disposition("attachment; filename=\"othertext.txt\"").await?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_stat_options" class="fn">presign_stat_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.StatOptions.html" class="struct" title="struct opendal::options::StatOptions">StatOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for stat(head) with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-10" class="doc-anchor">Â§</a>Options

Visit [`options::StatOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.StatOptions.html "struct opendal::options::StatOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-3" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use opendal::Operator;
use opendal::options;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_stat_options(
        "test",
        Duration::from_secs(3600),
        options::StatOptions {
            if_match: Some("<etag>".to_string()),
            ..Default::default()
        }
    ).await?;
    let req = http::Request::builder()
        .method(signed_req.method())
        .uri(signed_req.uri())
        .body(())?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_read" class="fn">presign_read</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for read.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-24" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options-4" class="doc-anchor">Â§</a>Extra Options

`presign_read` is a wrapper of [`Self::presign_read_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_read_with "method opendal::Operator::presign_read_with") without any options. To use extra options like `override_content_disposition`, please use [`Self::presign_read_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_read_with "method opendal::Operator::presign_read_with") or [\`Self::presign_read_options](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_read_options "method opendal::Operator::presign_read_options") instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-4" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use futures::io;
use opendal::Operator;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_read("test.txt", Duration::from_secs(3600)).await?;
```

- `signed_req.method()`: `GET`
- `signed_req.uri()`: `https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>`
- `signed_req.headers()`: `{ "host": "s3.amazonaws.com" }`

We can download this file via `curl` or other tools without credentials:

``` shell
curl "https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>" -O /tmp/test.txt
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_read_with" class="fn">presign_read_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FuturePresignRead.html" class="type" title="type opendal::operator_futures::FuturePresignRead">FuturePresignRead</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>\>\>

Presign an operation for read with extra options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-11" class="doc-anchor">Â§</a>Options

Visit [`options::ReadOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html "struct opendal::options::ReadOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-5" class="doc-anchor">Â§</a>Example

``` rust
use std::time::Duration;

use anyhow::Result;
use futures::io;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op
        .presign_read_with("test.txt", Duration::from_secs(3600))
        .override_content_type("text/plain")
        .await?;
    Ok(())
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_read_options" class="fn">presign_read_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for read with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-12" class="doc-anchor">Â§</a>Options

Visit [`options::ReadOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html "struct opendal::options::ReadOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-6" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use opendal::Operator;
use opendal::options;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_read_options(
        "file",
        Duration::from_secs(3600),
        options::ReadOptions {
            override_content_disposition: Some("attachment; filename=\"othertext.txt\"".to_string()),
            ..Default::default()
        }
    ).await?;
    let req = http::Request::builder()
        .method(signed_req.method())
        .uri(signed_req.uri())
        .body(())?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_write" class="fn">presign_write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for write.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-25" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options-5" class="doc-anchor">Â§</a>Extra Options

`presign_write` is a wrapper of [`Self::presign_write_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_write_with "method opendal::Operator::presign_write_with") without any options. To use extra options like `content_type`, please use [`Self::presign_write_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_write_with "method opendal::Operator::presign_write_with") or [`Self::presign_write_options`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_write_options "method opendal::Operator::presign_write_options") instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-7" class="doc-anchor">Â§</a>Example

``` rust
use std::time::Duration;

use anyhow::Result;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op
        .presign_write("test.txt", Duration::from_secs(3600))
        .await?;
    Ok(())
}
```

- `signed_req.method()`: `PUT`
- `signed_req.uri()`: `https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>`
- `signed_req.headers()`: `{ "host": "s3.amazonaws.com" }`

We can upload file as this file via `curl` or other tools without credential:

``` shell
curl -X PUT "https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>" -d "Hello, World!"
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_write_with" class="fn">presign_write_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FuturePresignWrite.html" class="type" title="type opendal::operator_futures::FuturePresignWrite">FuturePresignWrite</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>\>\>

Presign an operation for write with extra options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-13" class="doc-anchor">Â§</a>Options

Visit [`options::WriteOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html "struct opendal::options::WriteOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-8" class="doc-anchor">Â§</a>Example

``` rust
use std::time::Duration;

use anyhow::Result;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op
        .presign_write_with("test", Duration::from_secs(3600))
        .cache_control("no-store")
        .await?;
    let req = http::Request::builder()
        .method(signed_req.method())
        .uri(signed_req.uri())
        .body(())?;

    Ok(())
}
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_write_options" class="fn">presign_write_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for write with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-14" class="doc-anchor">Â§</a>Options

Check [`options::WriteOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html "struct opendal::options::WriteOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-9" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use opendal::Operator;
use opendal::options;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_write_options(
        "file",
        Duration::from_secs(3600),
        options::WriteOptions {
            content_type: Some("application/json".to_string()),
            cache_control: Some("max-age=3600".to_string()),
            if_not_exists: true,
            ..Default::default()
        }
    ).await?;
    let req = http::Request::builder()
        .method(signed_req.method())
        .uri(signed_req.uri())
        .body(())?;
```

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_delete" class="fn">presign_delete</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for delete.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-26" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#extra-options-6" class="doc-anchor">Â§</a>Extra Options

`presign_delete` is a wrapper of [`Self::presign_delete_with`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_delete_with "method opendal::Operator::presign_delete_with") without any options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-10" class="doc-anchor">Â§</a>Example

``` rust
use std::time::Duration;

use anyhow::Result;
use opendal::Operator;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op
        .presign_delete("test.txt", Duration::from_secs(3600))
        .await?;
    Ok(())
}
```

- `signed_req.method()`: `DELETE`
- `signed_req.uri()`: `https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>`
- `signed_req.headers()`: `{ "host": "s3.amazonaws.com" }`

We can delete file as this file via `curl` or other tools without credential:

``` shell
curl -X DELETE "https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>"
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_delete_with" class="fn">presign_delete_with</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/type.FuturePresignDelete.html" class="type" title="type opendal::operator_futures::FuturePresignDelete">FuturePresignDelete</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>\>\>

Presign an operation for delete without extra options.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.presign_delete_options" class="fn">presign_delete_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.DeleteOptions.html" class="struct" title="struct opendal::options::DeleteOptions">DeleteOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\>

Presign an operation for delete with additional options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#options-15" class="doc-anchor">Â§</a>Options

Visit [`options::DeleteOptions`](https://opendal.apache.org/docs/rust/opendal/options/struct.DeleteOptions.html "struct opendal::options::DeleteOptions") for all available options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#example-11" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use opendal::Operator;
use opendal::options;
use std::time::Duration;

async fn test(op: Operator) -> Result<()> {
    let signed_req = op.presign_delete_options(
        "path/to/file",
        Duration::from_secs(3600),
        options::DeleteOptions {
            ..Default::default()
        }
    ).await?;
    let req = http::Request::builder()
        .method(signed_req.method())
        .uri(signed_req.uri())
        .body(())?;
```

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-Operator-3" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#operator-build-api" class="doc-anchor">Â§</a>Operator build API

Operator should be built via [`OperatorBuilder`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html "struct opendal::OperatorBuilder"). We recommend to use [`Operator::new`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.new "associated function opendal::Operator::new") to get started:

``` rust
use opendal::services::Fs;
use opendal::Operator;
async fn test() -> Result<()> {
    // Create fs backend builder.
    let builder = Fs::default().root("/tmp");

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.new" class="fn">new</a>\<B: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a>\>(ab: B) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>\>

Create a new operator with input builder.

OpenDAL will call `builder.build()` internally, so we donâ€™t need to import `opendal::Builder` trait.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-31" class="doc-anchor">Â§</a>Examples

Read more backend init examples in [examples](https://github.com/apache/opendal/tree/main/examples).

``` rust
use opendal::services::Fs;
use opendal::Operator;
async fn test() -> Result<()> {
    // Create fs backend builder.
    let builder = Fs::default().root("/tmp");

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_config" class="fn">from_config</a>\<C: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a>\>( cfg: C, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>\>

Create a new operator from given config.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-32" class="doc-anchor">Â§</a>Examples

``` rust
use std::collections::HashMap;

use opendal::services::MemoryConfig;
use opendal::Operator;
async fn test() -> Result<()> {
    let cfg = MemoryConfig::default();

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::from_config(cfg)?.finish();

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_iter" class="fn">from_iter</a>\<B: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a>\>( iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>\>

Create a new operator from given iterator in static dispatch.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-27" class="doc-anchor">Â§</a>Notes

`from_iter` generates a `OperatorBuilder` which allows adding layer in zero-cost way.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-33" class="doc-anchor">Â§</a>Examples

``` rust
use std::collections::HashMap;

use opendal::services::Fs;
use opendal::Operator;
async fn test() -> Result<()> {
    let map = HashMap::from([
        // Set the root for fs, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        ("root".to_string(), "/tmp".to_string()),
    ]);

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::from_iter::<Fs>(map)?.finish();

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_uri" class="fn">from_uri</a>(uri: impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>\>

Create a new operator by parsing configuration from a URI.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-34" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::Operator;

let op = Operator::from_uri("memory://localhost/")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.via_iter" class="fn">via_iter</a>( scheme: <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>, iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>\>

Create a new operator via given scheme and iterator of config value in dynamic dispatch.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-28" class="doc-anchor">Â§</a>Notes

`via_iter` generates a `Operator` which allows building operator without generic type.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-35" class="doc-anchor">Â§</a>Examples

``` rust
use std::collections::HashMap;

use opendal::Operator;
use opendal::Scheme;
async fn test() -> Result<()> {
    let map = [
        // Set the root for fs, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        ("root".to_string(), "/tmp".to_string()),
    ];

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::via_iter(Scheme::Fs, map)?;

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from_map" class="fn">from_map</a>\<B: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a>\>( map: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>\>

ðŸ‘ŽDeprecated: use from_iter instead

Create a new operator from given map.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-29" class="doc-anchor">Â§</a>Notes

from_map is using static dispatch layers which is zero cost. via_map is using dynamic dispatch layers which has a bit runtime overhead with an extra vtable lookup and unable to inline. But from_map requires generic type parameter which is not always easy to be used.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-36" class="doc-anchor">Â§</a>Examples

``` rust
use std::collections::HashMap;

use opendal::services::Memory;
use opendal::Operator;
async fn test() -> Result<()> {
    let map = HashMap::new();

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::from_map::<Memory>(map)?.finish();

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.via_map" class="fn">via_map</a>(scheme: <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>, map: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>\>

ðŸ‘ŽDeprecated: use via_iter instead

Create a new operator from given scheme and map.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-30" class="doc-anchor">Â§</a>Notes

from_map is using static dispatch layers which is zero cost. via_map is using dynamic dispatch layers which has a bit runtime overhead with an extra vtable lookup and unable to inline. But from_map requires generic type parameter which is not always easy to be used.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-37" class="doc-anchor">Â§</a>Examples

``` rust
use std::collections::HashMap;

use opendal::Operator;
use opendal::Scheme;
async fn test() -> Result<()> {
    let map = HashMap::new();

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::via_map(Scheme::Memory, map)?;

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.layer" class="fn">layer</a>\<L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html" class="type" title="type opendal::raw::Accessor">Accessor</a>\>\>(self, layer: L) -\> Self

Create a new layer with dynamic dispatch.

Please note that `Layer` can modify internal contexts such as `HttpClient` and `Runtime` for the operator. Therefore, it is recommended to add layers before interacting with the storage. Adding or duplicating layers after accessing the storage may result in unexpected behavior.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#notes-31" class="doc-anchor">Â§</a>Notes

`OperatorBuilder::layer()` is using static dispatch which is zero cost. `Operator::layer()` is using dynamic dispatch which has a bit runtime overhead with an extra vtable lookup and unable to inline.

Itâ€™s always recommended to use `OperatorBuilder::layer()` instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#examples-38" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::LoggingLayer;
use opendal::services::Memory;
use opendal::Operator;

let op = Operator::new(Memory::default())?.finish();
let op = op.layer(LoggingLayer::default());
// All operations will go through the new_layer
let _ = op.read("test_file").await?;
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-Clone-for-Operator" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-Debug-for-Operator" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#impl-From%3COperator%3E-for-Operator" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

Available on **crate feature `blocking`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html#blanket-implementations" class="anchor">Â§</a>
