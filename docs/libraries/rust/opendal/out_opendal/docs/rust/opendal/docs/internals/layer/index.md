# Module layer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/internals/layer.rs.html#18-42" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

The internal implementation details of [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer").

[`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer") itself is quite simple:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/layer/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
pub trait Layer<A: Access> {
    type LayeredAccess: Access;

    fn layer(&self, inner: A) -> Self::LayeredAccess;
}
```

`XxxLayer` will wrap input [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") as inner and return a new [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"). So normally the implementation of [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer") will be split into two parts:

- `XxxLayer` will implement [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer") and return `XxxAccessor` as `Self::LayeredAccess`.
- `XxxAccess` will implement [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") and be built by `XxxLayer`.

Most layer only implements part of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"), so we provide [`LayeredAccess`](https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html "trait opendal::raw::LayeredAccess") which will forward all unimplemented methods to `inner`. Itâ€™s highly recommend to implement [`LayeredAccess`](https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html "trait opendal::raw::LayeredAccess") trait instead.
