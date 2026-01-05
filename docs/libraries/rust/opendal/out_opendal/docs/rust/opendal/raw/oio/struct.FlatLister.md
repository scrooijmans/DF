# Struct FlatLister Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/flat_list.rs.html#57-62" class="src">Source</a>

``` rust
pub struct FlatLister<A: Access, L> { /* private fields */ }
```

Expand description

FlatLister will walk dir in bottom up way:

- List nested dir first
- Go back into parent dirs one by one

Given the following file tree:

``` txt
.
âââ dir_x/
â   âââ dir_y/
â   â   âââ dir_z/
â   â   âââ file_c
â   âââ file_b
âââ file_a
```

ToFlatLister will output entries like:

``` txt
dir_x/dir_y/dir_z/file_c
dir_x/dir_y/dir_z/
dir_x/dir_y/file_b
dir_x/dir_y/
dir_x/file_a
dir_x/
```

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#note" class="doc-anchor">Â§</a>Note

There is no guarantee about the order between files and dirs at the same level. We only make sure the nested dirs will show up before parent dirs.

Especially, for storage services that canâ€™t return dirs first, ToFlatLister may output parent dirsâ€™ files before nested dirs, this is expected because files always output directly while listing.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#impl-FlatLister%3CA,+L%3E" class="anchor">Â§</a>

### impl\<A, L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>\<A, L\>

where A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>,

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#method.new" class="fn">new</a>(acc: A, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>\<A, L\>

Create a new flat lister

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#impl-List-for-FlatLister%3CA,+L%3E" class="anchor">Â§</a>

### impl\<A, L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>\<A, L\>

where A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\<Lister = L\>, L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry") [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#impl-Send-for-FlatLister%3CA,+L%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, L\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>\<A, L\>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#safety" class="doc-anchor">Â§</a>Safety

wasm32 is a special target that we only have one event-loop for this FlatLister.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#impl-Sync-for-FlatLister%3CA,+L%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, L\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>\<A, L\>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#safety-1" class="doc-anchor">Â§</a>Safety

We will only take `&mut Self` reference for FsLister.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html#blanket-implementations" class="anchor">Â§</a>
