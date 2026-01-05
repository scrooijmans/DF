# Struct Deleter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/delete/deleter.rs.html#84-89" class="src">Source</a>

``` rust
pub struct Deleter { /* private fields */ }
```

Expand description

Deleter is designed to continuously remove content from storage.

It leverages batch deletion capabilities provided by storage services for efficient removal.

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#usage" class="doc-anchor">Â§</a>Usage

[`Deleter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html "struct opendal::Deleter") provides several ways to delete files:

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#direct-deletion" class="doc-anchor">Â§</a>Direct Deletion

Use the `delete` method to remove a single file:

``` rust
use opendal::Operator;
use opendal::Result;

async fn example(op: Operator) -> Result<()> {
    let mut d = op.deleter().await?;
    d.delete("path/to/file").await?;
    d.close().await?;
    Ok(())
}
```

Delete multiple files via a stream:

``` rust
use futures::stream;
use opendal::Operator;
use opendal::Result;

async fn example(op: Operator) -> Result<()> {
    let mut d = op.deleter().await?;
    d.delete_stream(stream::iter(vec!["path/to/file"])).await?;
    d.close().await?;
    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#using-as-a-sink" class="doc-anchor">Â§</a>Using as a Sink

Deleter can be used as a Sink for file deletion:

``` rust
use futures::stream;
use futures::Sink;
use futures::SinkExt;
use opendal::Operator;
use opendal::Result;

async fn example(op: Operator) -> Result<()> {
    let mut sink = op.deleter().await?.into_sink();
    sink.send("path/to/file").await?;
    sink.close().await?;
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#impl-Deleter" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html" class="struct" title="struct opendal::Deleter">Deleter</a>

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete" class="fn">delete</a>(&mut self, input: impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Delete a path.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_iter" class="fn">delete_iter</a>\<I, D\>(&mut self, iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = D\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an infallible iterator of paths.

Also see:

- [`Deleter::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_iter "method opendal::Deleter::delete_try_iter"): delete a fallible iterator of paths.
- [`Deleter::delete_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_stream "method opendal::Deleter::delete_stream"): delete an infallible stream of paths.
- [`Deleter::delete_try_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_stream "method opendal::Deleter::delete_try_stream"): delete a fallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_iter" class="fn">delete_try_iter</a>\<I, D\>(&mut self, try_iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<D\>\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete a fallible iterator of paths.

Also see:

- [`Deleter::delete_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_iter "method opendal::Deleter::delete_iter"): delete an infallible iterator of paths.
- [`Deleter::delete_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_stream "method opendal::Deleter::delete_stream"): delete an infallible stream of paths.
- [`Deleter::delete_try_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_stream "method opendal::Deleter::delete_try_stream"): delete a fallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_stream" class="fn">delete_stream</a>\<S, D\>(&mut self, stream: S) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where S: Stream\<Item = D\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an infallible stream of paths.

Also see:

- [`Deleter::delete_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_iter "method opendal::Deleter::delete_iter"): delete an infallible iterator of paths.
- [`Deleter::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_iter "method opendal::Deleter::delete_try_iter"): delete a fallible iterator of paths.
- [`Deleter::delete_try_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_stream "method opendal::Deleter::delete_try_stream"): delete a fallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_stream" class="fn">delete_try_stream</a>\<S, D\>(&mut self, try_stream: S) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where S: Stream\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<D\>\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete a fallible stream of paths.

Also see:

- [`Deleter::delete_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_iter "method opendal::Deleter::delete_iter"): delete an infallible iterator of paths.
- [`Deleter::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_try_iter "method opendal::Deleter::delete_try_iter"): delete a fallible iterator of paths.
- [`Deleter::delete_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.delete_stream "method opendal::Deleter::delete_stream"): delete an infallible stream of paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Flush the deleter, returns the number of deleted paths.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Close the deleter, this will flush the deleter and wait until all paths are deleted.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#method.into_sink" class="fn">into_sink</a>\<T: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>\>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.FuturesDeleteSink.html" class="struct" title="struct opendal::FuturesDeleteSink">FuturesDeleteSink</a>\<T\>

Convert the deleter into a sink.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Deleter.html#blanket-implementations" class="anchor">Â§</a>
