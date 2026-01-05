# opendal - Rust

Expand description

Apache OpenDAL™ is an Open Data Access Layer that enables seamless interaction with diverse storage services.

OpenDAL’s development is guided by its vision of **One Layer, All Storage** and its core principles: **Open Community**, **Solid Foundation**, **Fast Access**, **Object Storage First**, and **Extensible Architecture**. Read the explained vision at [OpenDAL Vision](https://opendal.apache.org/vision).

## [§](#quick-start)Quick Start

OpenDAL’s API entry points are [`Operator`](struct.Operator.html "struct opendal::Operator") and [`blocking::Operator`](blocking/struct.Operator.html "struct opendal::blocking::Operator"). All public APIs are accessible through the operator. To utilize OpenDAL, you need to:

- [Init a service](#init-a-service)
- [Compose layers](#compose-layers)
- [Use operator](#use-operator)

### [§](#init-a-service)Init a service

The first step is to pick a service and init it with a builder. All supported services could be found at [`services`](services/index.html "mod opendal::services").

Let’s take [`services::S3`](services/struct.S3.html "struct opendal::services::S3") as an example:

```
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

### [§](#compose-layers)Compose layers

The next setup is to compose layers. Layers are modules that provide extra features for every operation. All builtin layers could be found at [`layers`](layers/index.html "mod opendal::layers").

Let’s use [`layers::LoggingLayer`](layers/struct.LoggingLayer.html "struct opendal::layers::LoggingLayer") as an example; this layer adds logging to every operation that OpenDAL performs.

```
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

### [§](#use-operator)Use operator

The final step is to use the operator. OpenDAL supports both async [`Operator`](struct.Operator.html "struct opendal::Operator") and blocking [`blocking::Operator`](blocking/struct.Operator.html "struct opendal::blocking::Operator"). Please pick the one that fits your use case.

Every Operator API follows a consistent pattern. For example, consider the `read` operation:

- [`Operator::read`](about:blank/struct.Operator.html#method.read "method opendal::Operator::read"): Executes a read operation.
- [`Operator::read_with`](about:blank/struct.Operator.html#method.read_with "method opendal::Operator::read_with"): Executes a read operation with additional options using the builder pattern.
- [`Operator::read_options`](about:blank/struct.Operator.html#method.read_options "method opendal::Operator::read_options"): Executes a read operation with extra options provided via a [`options::ReadOptions`](options/struct.ReadOptions.html "struct opendal::options::ReadOptions") struct.
- [`Operator::reader`](about:blank/struct.Operator.html#method.reader "method opendal::Operator::reader"): Creates a reader for streaming data, allowing for flexible access.
- [`Operator::reader_with`](about:blank/struct.Operator.html#method.reader_with "method opendal::Operator::reader_with"): Creates a reader with advanced options using the builder pattern.
- [`Operator::reader_options`](about:blank/struct.Operator.html#method.reader_options "method opendal::Operator::reader_options"): Creates a reader with extra options provided via a [`options::ReadOptions`](options/struct.ReadOptions.html "struct opendal::options::ReadOptions") struct.

The [`Reader`](struct.Reader.html "struct opendal::Reader") created by [`Operator`](struct.Operator.html "struct opendal::Operator") supports custom read control methods and can be converted into \[`futures::AsyncRead`\] or \[`futures::Stream`\] for broader ecosystem compatibility.

```
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

## [§](#useful-links)Useful Links

- [Concept](docs/concepts/index.html "mod opendal::docs::concepts")
- [Internals](docs/internals/index.html "mod opendal::docs::internals")
- [Performance Guide](docs/performance/index.html "mod opendal::docs::performance")

[blocking](blocking/index.html "mod opendal::blocking")`blocking`

blocking module provides blocking APIs for OpenDAL.

[docs](docs/index.html "mod opendal::docs")`docsrs`

This module holds documentation for OpenDAL.

[executors](executors/index.html "mod opendal::executors")

executors module provides implementations for the [`Execute`](trait.Execute.html "trait opendal::Execute") trait for widely used runtimes.

[layers](layers/index.html "mod opendal::layers")

`Layer` is the mechanism to intercept operations.

[operator_futures](operator_futures/index.html "mod opendal::operator_futures")

Futures provides the futures generated by [`Operator`](struct.Operator.html "struct opendal::Operator")

[options](options/index.html "mod opendal::options")

Options module provides options definitions for operations.

[raw](raw/index.html "mod opendal::raw")

Raw modules provide raw APIs that used by underlying services

[services](services/index.html "mod opendal::services")

Services will provide builders to build underlying backends.

[Buffer](struct.Buffer.html "struct opendal::Buffer")

Buffer is a wrapper of contiguous `Bytes` and non-contiguous `[Bytes]`.

[BufferSink](struct.BufferSink.html "struct opendal::BufferSink")

BufferSink is the adapter of \[`futures::Sink`\] generated by [`Writer::into_sink`](about:blank/struct.Writer.html#method.into_sink "method opendal::Writer::into_sink")

[BufferStream](struct.BufferStream.html "struct opendal::BufferStream")

BufferStream is a stream of buffers, created by [`Reader::into_stream`](about:blank/struct.Reader.html#method.into_stream "method opendal::Reader::into_stream")

[Capability](struct.Capability.html "struct opendal::Capability")

Capability defines the supported operations and their constraints for a storage Operator.

[DeleteInput](struct.DeleteInput.html "struct opendal::DeleteInput")

DeleteInput is the input for delete operations.

[Deleter](struct.Deleter.html "struct opendal::Deleter")

Deleter is designed to continuously remove content from storage.

[Entry](struct.Entry.html "struct opendal::Entry")

Entry returned by [`Lister`](struct.Lister.html "struct opendal::Lister") or \[`BlockingLister`\] to represent a path and it’s relative metadata.

[Error](struct.Error.html "struct opendal::Error")

Error is the error struct returned by all opendal functions.

[Executor](struct.Executor.html "struct opendal::Executor")

Executor that runs futures in background.

[FuturesAsyncReader](struct.FuturesAsyncReader.html "struct opendal::FuturesAsyncReader")

FuturesAsyncReader is the adapter of \[`AsyncRead`\], \[`AsyncBufRead`\] and \[`AsyncSeek`\] generated by [`Reader::into_futures_async_read`](about:blank/struct.Reader.html#method.into_futures_async_read "method opendal::Reader::into_futures_async_read").

[FuturesAsyncWriter](struct.FuturesAsyncWriter.html "struct opendal::FuturesAsyncWriter")

FuturesIoAsyncWriter is the adapter of \[`AsyncWrite`\] for [`Writer`](struct.Writer.html "struct opendal::Writer").

[FuturesBytesSink](struct.FuturesBytesSink.html "struct opendal::FuturesBytesSink")

FuturesBytesSink is the adapter of \[`futures::Sink`\] generated by [`Writer::into_bytes_sink`](about:blank/struct.Writer.html#method.into_bytes_sink "method opendal::Writer::into_bytes_sink").

[FuturesBytesStream](struct.FuturesBytesStream.html "struct opendal::FuturesBytesStream")

FuturesBytesStream is the adapter of \[`Stream`\] generated by [`Reader::into_bytes_stream`](about:blank/struct.Reader.html#method.into_bytes_stream "method opendal::Reader::into_bytes_stream").

[FuturesDeleteSink](struct.FuturesDeleteSink.html "struct opendal::FuturesDeleteSink")

FuturesDeleteSink is a sink that generated by [`Deleter`](struct.Deleter.html "struct opendal::Deleter")

[Lister](struct.Lister.html "struct opendal::Lister")

Lister is designed to list entries at given path in an asynchronous manner.

[Metadata](struct.Metadata.html "struct opendal::Metadata")

Metadata contains all the information related to a specific path.

[Operator](struct.Operator.html "struct opendal::Operator")

The `Operator` serves as the entry point for all public asynchronous APIs.

[OperatorBuilder](struct.OperatorBuilder.html "struct opendal::OperatorBuilder")

OperatorBuilder is a typed builder to build an Operator.

[OperatorInfo](struct.OperatorInfo.html "struct opendal::OperatorInfo")

Metadata for operator, users can use this metadata to get information of operator.

[OperatorRegistry](struct.OperatorRegistry.html "struct opendal::OperatorRegistry")

Global registry that maps schemes to [`OperatorFactory`](type.OperatorFactory.html "type opendal::OperatorFactory") functions.

[Reader](struct.Reader.html "struct opendal::Reader")

Reader is designed to read data from given path in an asynchronous manner.

[Writer](struct.Writer.html "struct opendal::Writer")

Writer is designed to write data into given path in an asynchronous manner.

[EntryMode](enum.EntryMode.html "enum opendal::EntryMode")

EntryMode represents the mode.

[ErrorKind](enum.ErrorKind.html "enum opendal::ErrorKind")

ErrorKind is all kinds of Error of opendal.

[Scheme](enum.Scheme.html "enum opendal::Scheme")

Services that OpenDAL supports

[DEFAULT_OPERATOR_REGISTRY](static.DEFAULT_OPERATOR_REGISTRY.html "static opendal::DEFAULT_OPERATOR_REGISTRY")

Default registry initialized with builtin services.

[Builder](trait.Builder.html "trait opendal::Builder")

Builder is used to set up underlying services.

[Configurator](trait.Configurator.html "trait opendal::Configurator")

Configurator is used to configure the underlying service.

[Execute](trait.Execute.html "trait opendal::Execute")

Execute trait is used to execute task in background.

[IntoDeleteInput](trait.IntoDeleteInput.html "trait opendal::IntoDeleteInput")

IntoDeleteInput is a helper trait that makes it easier for users to play with `Deleter`.

[OperatorFactory](type.OperatorFactory.html "type opendal::OperatorFactory")

Factory signature used to construct [`Operator`](struct.Operator.html "struct opendal::Operator") from a URI and extra options.

[Result](type.Result.html "type opendal::Result")

Result that is a wrapper of `Result<T, opendal::Error>`
