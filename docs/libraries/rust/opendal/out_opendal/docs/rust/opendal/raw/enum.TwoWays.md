# Enum TwoWays Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/enum_utils.rs.html#47-52" class="src">Source</a>

``` rust
pub enum TwoWays<ONE, TWO> {
    One(ONE),
    Two(TWO),
}
```

Expand description

TwoWays is used to implement traits that based on two ways.

Users can wrap two different trait types together.

## Variants<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#variants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#variant.One" class="anchor">Â§</a>

### One(ONE)

The first type for the [`TwoWays`](https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html "enum opendal::raw::TwoWays").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#variant.Two" class="anchor">Â§</a>

### Two(TWO)

The second type for the [`TwoWays`](https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html "enum opendal::raw::TwoWays").

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#impl-TwoWays%3CGhacWriterV1,+GhacWriterV2%3E" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<GhacWriterV1, GhacWriterV2\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.new" class="fn">new</a>(core: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<GhacCore\>, write_path: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, url: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Available on **crate feature `services-ghac`** only.

TODO: maybe we can move the signed url logic to azblob service instead.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#impl-List-for-TwoWays%3CONE,+TWO%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<ONE, TWO\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry") [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#impl-Read-for-TwoWays%3CONE,+TWO%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<ONE, TWO\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read at the given offset with the given size.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.read_all" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all" class="fn">read_all</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Read all data from the reader.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#impl-Write-for-TwoWays%3CONE,+TWO%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<ONE, TWO\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write given bytes into writer. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.close" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data has been flushed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#method.abort" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort the pending writer.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html#blanket-implementations" class="anchor">Â§</a>
