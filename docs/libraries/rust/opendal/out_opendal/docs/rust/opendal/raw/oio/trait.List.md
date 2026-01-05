# Trait List Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/api.rs.html#29-35" class="src">Source</a>

``` rust
pub trait List:
    Unpin
    + Send
    + Sync {
    // Required method
    fn next(
        &mut self,
    ) -> impl Future<Output = Result<Option<Entry>>> + MaybeSend;
}
```

Expand description

Page trait is used by [`raw::Accessor`](https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html "type opendal::raw::Accessor") to implement `list` operation.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Fetch a new page of [`Entry`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html "struct opendal::raw::oio::Entry")

`Ok(None)` means all pages have been returned. Any following call to `next` will always get the same result.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-Option%3CP%3E" class="anchor">Â§</a>

### impl\<P: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#method.next-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-Box%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html" class="trait" title="trait opendal::raw::oio::ListDyn">ListDyn</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#method.next-2" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-FlatLister%3CA,+L%3E" class="anchor">Â§</a>

### impl\<A, L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>\<A, L\>

where A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\<Lister = L\>, L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-PageLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html" class="struct" title="struct opendal::raw::oio::PageLister">PageLister</a>\<L\>

where L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html" class="trait" title="trait opendal::raw::oio::PageList">PageList</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-PrefixLister%3CL%3E" class="anchor">Â§</a>

### impl\<L\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html" class="struct" title="struct opendal::raw::oio::PrefixLister">PrefixLister</a>\<L\>

where L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-FourWays%3CONE,+TWO,+THREE,+FOUR%3E" class="anchor">Â§</a>

### impl\<ONE, TWO, THREE, FOUR\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html" class="enum" title="enum opendal::raw::FourWays">FourWays</a>\<ONE, TWO, THREE, FOUR\>

where ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, FOUR: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-TwoWays%3CONE,+TWO%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<ONE, TWO\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-ThreeWays%3CONE,+TWO,+THREE%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.ThreeWays.html" class="enum" title="enum opendal::raw::ThreeWays">ThreeWays</a>\<ONE, TWO, THREE\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#impl-List-for-HierarchyLister%3CP%3E" class="anchor">Â§</a>

### impl\<P: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<P\>
