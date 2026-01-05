# Struct PrefixLister Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/prefix_list.rs.html#32-35" class="src">Source</a>

``` rust
pub struct PrefixLister<L> { /* private fields */ }
```

Expand description

PrefixLister is used to filter entries by prefix.

For example, if we have a lister that returns entries:

``` txt
.
âââ file_a
âââ file_b
```

We can use `PrefixLister` to filter entries with prefix `file_`.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#impl-PrefixLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html" class="struct" title="struct opendal::raw::oio::PrefixLister">PrefixLister</a>\<L\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#method.new" class="fn">new</a>(lister: L, prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html" class="struct" title="struct opendal::raw::oio::PrefixLister">PrefixLister</a>\<L\>

Create a new flat lister

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#impl-List-for-PrefixLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html" class="struct" title="struct opendal::raw::oio::PrefixLister">PrefixLister</a>\<L\>

where L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry") [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#impl-Sync-for-PrefixLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html" class="struct" title="struct opendal::raw::oio::PrefixLister">PrefixLister</a>\<L\>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#safety" class="doc-anchor">Â§</a>Safety

We will only take `&mut Self` reference for FsLister.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html#blanket-implementations" class="anchor">Â§</a>
