# Crate opendal Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/lib.rs.html#18-203" class="src">Source</a>

Expand description

Apache OpenDALâ„¢ is an Open Data Access Layer that enables seamless interaction with diverse storage services.

OpenDALâ€™s development is guided by its vision of **One Layer, All Storage** and its core principles: **Open Community**, **Solid Foundation**, **Fast Access**, **Object Storage First**, and **Extensible Architecture**. Read the explained vision at [OpenDAL Vision](https://opendal.apache.org/vision).

## <a href="https://opendal.apache.org/docs/rust/opendal/index.html#quick-start" class="doc-anchor">Â§</a>Quick Start

OpenDALâ€™s API entry points are [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") and [`blocking::Operator`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html "struct opendal::blocking::Operator"). All public APIs are accessible through the operator. To utilize OpenDAL, you need to:

- [Init a service](https://opendal.apache.org/docs/rust/opendal/index.html#init-a-service)
- [Compose layers](https://opendal.apache.org/docs/rust/opendal/index.html#compose-layers)
- [Use operator](https://opendal.apache.org/docs/rust/opendal/index.html#use-operator)

### <a href="https://opendal.apache.org/docs/rust/opendal/index.html#init-a-service" class="doc-anchor">Â§</a>Init a service

The first step is to pick a service and init it with a builder. All supported services could be found at [`services`](https://opendal.apache.org/docs/rust/opendal/services/index.html "mod opendal::services").

Letâ€™s take [`services::S3`](https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html "struct opendal::services::S3") as an example:

``` rust
use opendal::services;
use opendal::Operator;
use opendal::Result;

fn main() -> Result<()> {
    // Pick a builder and configure it.
    let mut builder = services::S3::default().bucket("test");

    // Init an operator
    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/index.html#compose-layers" class="doc-anchor">Â§</a>Compose layers

The next setup is to compose layers. Layers are modules that provide extra features for every operation. All builtin layers could be found at [`layers`](https://opendal.apache.org/docs/rust/opendal/layers/index.html "mod opendal::layers").

Letâ€™s use [`layers::LoggingLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html "struct opendal::layers::LoggingLayer") as an example; this layer adds logging to every operation that OpenDAL performs.

``` rust
use opendal::layers::LoggingLayer;
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

    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/index.html#use-operator" class="doc-anchor">Â§</a>Use operator

The final step is to use the operator. OpenDAL supports both async [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") and blocking [`blocking::Operator`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html "struct opendal::blocking::Operator"). Please pick the one that fits your use case.

Every Operator API follows a consistent pattern. For example, consider the `read` operation:

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

## <a href="https://opendal.apache.org/docs/rust/opendal/index.html#useful-links" class="doc-anchor">Â§</a>Useful Links

- [Concept](https://opendal.apache.org/docs/rust/opendal/docs/concepts/index.html "mod opendal::docs::concepts")
- [Internals](https://opendal.apache.org/docs/rust/opendal/docs/internals/index.html "mod opendal::docs::internals")
- [Performance Guide](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance")

## Modules<a href="https://opendal.apache.org/docs/rust/opendal/index.html#modules" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/index.html" class="mod" title="mod opendal::blocking">blocking</a>`blocking`  
blocking module provides blocking APIs for OpenDAL.

<a href="https://opendal.apache.org/docs/rust/opendal/docs/index.html" class="mod" title="mod opendal::docs">docs</a>`docsrs`  
This module holds documentation for OpenDAL.

<a href="https://opendal.apache.org/docs/rust/opendal/executors/index.html" class="mod" title="mod opendal::executors">executors</a>  
executors module provides implementations for the [`Execute`](https://opendal.apache.org/docs/rust/opendal/trait.Execute.html "trait opendal::Execute") trait for widely used runtimes.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/index.html" class="mod" title="mod opendal::layers">layers</a>  
`Layer` is the mechanism to intercept operations.

<a href="https://opendal.apache.org/docs/rust/opendal/operator_futures/index.html" class="mod" title="mod opendal::operator_futures">operator_futures</a>  
Futures provides the futures generated by [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator")

<a href="https://opendal.apache.org/docs/rust/opendal/options/index.html" class="mod" title="mod opendal::options">options</a>  
Options module provides options definitions for operations.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html" class="mod" title="mod opendal::raw">raw</a>  
Raw modules provide raw APIs that used by underlying services

<a href="https://opendal.apache.org/docs/rust/opendal/services/index.html" class="mod" title="mod opendal::services">services</a>  
Services will provide builders to build underlying backends.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>  
Buffer is a wrapper of contiguous `Bytes` and non-contiguous `[Bytes]`.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferSink.html" class="struct" title="struct opendal::BufferSink">BufferSink</a>  
BufferSink is the adapter of \[`futures::Sink`\] generated by [`Writer::into_sink`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.into_sink "method opendal::Writer::into_sink")

<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html" class="struct" title="struct opendal::BufferStream">BufferStream</a>  
BufferStream is a stream of buffers, created by [`Reader::into_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_stream "method opendal::Reader::into_stream")

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>  
Capability defines the supported operations and their constraints for a storage Operator.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>  
DeleteInput is the input for delete operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html" class="struct" title="struct opendal::Deleter">Deleter</a>  
Deleter is designed to continuously remove content from storage.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>  
Entry returned by [`Lister`](https://opendal.apache.org/docs/rust/opendal/struct.Lister.html "struct opendal::Lister") or \[`BlockingLister`\] to represent a path and itâ€™s relative metadata.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>  
Error is the error struct returned by all opendal functions.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>  
Executor that runs futures in background.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncReader.html" class="struct" title="struct opendal::FuturesAsyncReader">FuturesAsyncReader</a>  
FuturesAsyncReader is the adapter of \[`AsyncRead`\], \[`AsyncBufRead`\] and \[`AsyncSeek`\] generated by [`Reader::into_futures_async_read`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_futures_async_read "method opendal::Reader::into_futures_async_read").

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesAsyncWriter.html" class="struct" title="struct opendal::FuturesAsyncWriter">FuturesAsyncWriter</a>  
FuturesIoAsyncWriter is the adapter of \[`AsyncWrite`\] for [`Writer`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html "struct opendal::Writer").

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesBytesSink.html" class="struct" title="struct opendal::FuturesBytesSink">FuturesBytesSink</a>  
FuturesBytesSink is the adapter of \[`futures::Sink`\] generated by [`Writer::into_bytes_sink`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html#method.into_bytes_sink "method opendal::Writer::into_bytes_sink").

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesBytesStream.html" class="struct" title="struct opendal::FuturesBytesStream">FuturesBytesStream</a>  
FuturesBytesStream is the adapter of \[`Stream`\] generated by [`Reader::into_bytes_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_bytes_stream "method opendal::Reader::into_bytes_stream").

<a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesDeleteSink.html" class="struct" title="struct opendal::FuturesDeleteSink">FuturesDeleteSink</a>  
FuturesDeleteSink is a sink that generated by [`Deleter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html "struct opendal::Deleter")

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html" class="struct" title="struct opendal::Lister">Lister</a>  
Lister is designed to list entries at given path in an asynchronous manner.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>  
Metadata contains all the information related to a specific path.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>  
The `Operator` serves as the entry point for all public asynchronous APIs.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>  
OperatorBuilder is a typed builder to build an Operator.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>  
Metadata for operator, users can use this metadata to get information of operator.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html" class="struct" title="struct opendal::OperatorRegistry">OperatorRegistry</a>  
Global registry that maps schemes to [`OperatorFactory`](https://opendal.apache.org/docs/rust/opendal/type.OperatorFactory.html "type opendal::OperatorFactory") functions.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>  
Parsed representation of an operator URI with normalized components.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>  
Reader is designed to read data from given path in an asynchronous manner.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Writer.html" class="struct" title="struct opendal::Writer">Writer</a>  
Writer is designed to write data into given path in an asynchronous manner.

## Enums<a href="https://opendal.apache.org/docs/rust/opendal/index.html#enums" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>  
EntryMode represents the mode.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>  
ErrorKind is all kinds of Error of opendal.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>  
Services that OpenDAL supports

## Statics<a href="https://opendal.apache.org/docs/rust/opendal/index.html#statics" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/static.DEFAULT_OPERATOR_REGISTRY.html" class="static" title="static opendal::DEFAULT_OPERATOR_REGISTRY">DEFAULT_OPERATOR_REGISTRY</a>  
Default registry initialized with builtin services.

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a>  
Builder is used to set up underlying services.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a>  
Configurator is used to configure the underlying service.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html" class="trait" title="trait opendal::Execute">Execute</a>  
Execute trait is used to execute task in background.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>  
IntoDeleteInput is a helper trait that makes it easier for users to play with `Deleter`.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a>  
Conversion trait that builds [`OperatorUri`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html "struct opendal::OperatorUri") from various inputs.

## Type Aliases<a href="https://opendal.apache.org/docs/rust/opendal/index.html#types" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/type.OperatorFactory.html" class="type" title="type opendal::OperatorFactory">OperatorFactory</a>  
Factory signature used to construct [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") from a URI and extra options.

<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>  
Result that is a wrapper of `Result<T, opendal::Error>`
