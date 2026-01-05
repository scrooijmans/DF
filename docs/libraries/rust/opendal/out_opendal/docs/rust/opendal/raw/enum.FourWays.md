# Enum FourWays Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/enum_utils.rs.html#158-167" class="src">Source</a>

``` rust
pub enum FourWays<ONE, TWO, THREE, FOUR> {
    One(ONE),
    Two(TWO),
    Three(THREE),
    Four(FOUR),
}
```

Expand description

FourWays is used to implement traits that based on four ways.

Users can wrap four different trait types together.

## Variants<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#variants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#variant.One" class="anchor">Â§</a>

### One(ONE)

The first type for the [`FourWays`](https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html "enum opendal::raw::FourWays").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#variant.Two" class="anchor">Â§</a>

### Two(TWO)

The second type for the [`FourWays`](https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html "enum opendal::raw::FourWays").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#variant.Three" class="anchor">Â§</a>

### Three(THREE)

The third type for the [`FourWays`](https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html "enum opendal::raw::FourWays").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#variant.Four" class="anchor">Â§</a>

### Four(FOUR)

The fourth type for the [`FourWays`](https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html "enum opendal::raw::FourWays").

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#impl-List-for-FourWays%3CONE,+TWO,+THREE,+FOUR%3E" class="anchor">Â§</a>

### impl\<ONE, TWO, THREE, FOUR\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html" class="enum" title="enum opendal::raw::FourWays">FourWays</a>\<ONE, TWO, THREE, FOUR\>

where ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, FOUR: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry") [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#impl-Read-for-FourWays%3CONE,+TWO,+THREE,+FOUR%3E" class="anchor">Â§</a>

### impl\<ONE, TWO, THREE, FOUR\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html" class="enum" title="enum opendal::raw::FourWays">FourWays</a>\<ONE, TWO, THREE, FOUR\>

where ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, FOUR: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read at the given offset with the given size.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#method.read_all" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all" class="fn">read_all</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Read all data from the reader.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html#blanket-implementations" class="anchor">Â§</a>
