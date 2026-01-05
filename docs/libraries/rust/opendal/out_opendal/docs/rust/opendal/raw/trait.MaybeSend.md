# Trait MaybeSend Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/futures_util.rs.html#49" class="src">Source</a>

``` rust
pub trait MaybeSend: Send { }
```

Expand description

MaybeSend is a marker to determine whether a type is `Send` or not. We use this trait to wrap the `Send` requirement for wasm32 target.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html#safety" class="doc-anchor">Â§</a>Safety

[`MaybeSend`](https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html "trait opendal::raw::MaybeSend") is equivalent to `Send` on non-wasm32 target. And itâ€™s empty trait on wasm32 target to indicate that a type is not `Send`.

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html#impl-MaybeSend-for-T" class="anchor">Â§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a> for T

Available on **non-WebAssembly** only.
