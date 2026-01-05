# Struct PageLister Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/page_list.rs.html#63-66" class="src">Source</a>

``` rust
pub struct PageLister<L: PageList> { /* private fields */ }
```

Expand description

PageLister implements [`oio::List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List") based on [`PageList`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html "trait opendal::raw::oio::PageList").

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#impl-PageLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html" class="struct" title="struct opendal::raw::oio::PageLister">PageLister</a>\<L\>

where L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html" class="trait" title="trait opendal::raw::oio::PageList">PageList</a>,

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#method.new" class="fn">new</a>(l: L) -\> Self

Create a new PageLister.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#impl-List-for-PageLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html" class="struct" title="struct opendal::raw::oio::PageLister">PageLister</a>\<L\>

where L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html" class="trait" title="trait opendal::raw::oio::PageList">PageList</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry") [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html#blanket-implementations" class="anchor">Â§</a>
