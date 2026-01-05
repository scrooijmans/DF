# Trait PageList Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/page_list.rs.html#34-37" class="src">Source</a>

``` rust
pub trait PageList:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required method
    fn next_page(
        &self,
        ctx: &mut PageContext,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
}
```

Expand description

PageList is used to implement [`oio::List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List") based on API supporting pagination. By implementing PageList, services donâ€™t need to care about the details of page list.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html#architecture" class="doc-anchor">Â§</a>Architecture

The architecture after adopting [`PageList`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html "trait opendal::raw::oio::PageList"):

- Services impl `PageList`
- `PageLister` impl `List`
- Expose `PageLister` as `Accessor::Lister`

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html#tymethod.next_page" class="fn">next_page</a>( &self, ctx: &mut <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html" class="struct" title="struct opendal::raw::oio::PageContext">PageContext</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

next_page is used to fetch next page of entries from underlying storage.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html#implementors" class="anchor">Â§</a>
