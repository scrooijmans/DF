# Struct HierarchyLister Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/hierarchy_list.rs.html#33-38" class="src">Source</a>

``` rust
pub struct HierarchyLister<P> { /* private fields */ }
```

Expand description

ToHierarchyLister will convert a flat list to hierarchy by filter not needed entries.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#notes" class="doc-anchor">Â§</a>Notes

ToHierarchyLister filter entries after fetch entries. So itâ€™s possible to return an empty vec. It doesnâ€™t mean the all pages have been returned.

Please keep calling next until we returned `Ok(None)`

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#impl-HierarchyLister%3CP%3E" class="anchor">Â§</a>

### impl\<P\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<P\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#method.new" class="fn">new</a>(lister: P, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, recursive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<P\>

Create a new hierarchy lister

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#impl-List-for-HierarchyLister%3CP%3E" class="anchor">Â§</a>

### impl\<P: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<P\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry") [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html#blanket-implementations" class="anchor">Â§</a>
