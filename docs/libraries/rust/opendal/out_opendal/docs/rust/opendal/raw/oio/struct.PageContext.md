# Struct PageContext Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/page_list.rs.html#49-60" class="src">Source</a>

``` rust
pub struct PageContext {
    pub done: bool,
    pub token: String,
    pub entries: VecDeque<Entry>,
}
```

Expand description

PageContext is the context passing between `PageList`.

[`PageLister`](https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html "struct opendal::raw::oio::PageLister") will init the PageContext, and implementer of [`PageList`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html "trait opendal::raw::oio::PageList") should fill the `PageContext` based on their needs.

- Set `done` to `true` if all page have been fetched.
- Update `token` if there is more page to fetch. `token` is not exposed to users, itâ€™s internal used only.
- Push back into the entries for each entry fetched from underlying storage.

NOTE: `entries` is a `VecDeque` to avoid unnecessary memory allocation. Only `push_back` is allowed.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html#structfield.done" class="anchor field">Â§</a>`done: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

done is used to indicate whether the list operation is done.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html#structfield.token" class="anchor field">Â§</a>`token: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

token is used by underlying storage services to fetch next page.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html#structfield.entries" class="anchor field">Â§</a>`entries: `<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque"><code>VecDeque</code></a>`<`<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry"><code>Entry</code></a>`>`

entries are used to store entries fetched from underlying storage.

Please always reuse the same `VecDeque` to avoid unnecessary memory allocation. PageLister makes sure that entries is reset before calling `next_page`. Implementer can call `push_back` on `entries` directly.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html#blanket-implementations" class="anchor">Â§</a>
