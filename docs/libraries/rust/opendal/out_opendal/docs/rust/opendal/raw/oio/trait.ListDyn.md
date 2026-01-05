# Trait ListDyn Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/list/api.rs.html#54-57" class="src">Source</a>

``` rust
pub trait ListDyn:
    Unpin
    + Send
    + Sync {
    // Required method
    fn next_dyn(&mut self) -> BoxedFuture<'_, Result<Option<Entry>>>;
}
```

Expand description

ListDyn is the dyn version of [`List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List"). Makes it possible to use as `Box<dyn ListDyn>`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html#tymethod.next_dyn" class="fn">next_dyn</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>\>\>\>

The dyn version of [`List::next`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html#tymethod.next "method opendal::raw::oio::List::next").

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html#impl-ListDyn-for-T" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html" class="trait" title="trait opendal::raw::oio::ListDyn">ListDyn</a> for T
