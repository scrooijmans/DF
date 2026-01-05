# Struct MimeGuessLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/mime_guess.rs.html#63" class="src">Source</a>

``` rust
#[non_exhaustive]pub struct MimeGuessLayer {}
```

Available on **crate feature `layers-mime-guess`** only.

Expand description

A layer that can automatically set `Content-Type` based on the file extension in the path.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#mimeguess" class="doc-anchor">Â§</a>MimeGuess

This layer uses [mime_guess](https://crates.io/crates/mime_guess) to automatically set `Content-Type` based on the file extension in the operation path.

However, please note that this layer will not overwrite the `content_type` you manually set, nor will it overwrite the `content_type` provided by backend services.

A simple example is that for object storage backends, when you call `stat`, the backend will provide `content_type` information, and `mime_guess` will not be called, but will use the `content_type` provided by the backend.

But if you use the [Fs](https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html) backend to call `stat`, the backend will not provide `content_type` information, and our `mime_guess` will be called to provide you with appropriate `content_type` information.

Another thing to note is that using this layer does not necessarily mean that the result will 100% contain `content_type` information. If the extension of your path is custom or an uncommon type, the returned result will still not contain `content_type` information (the specific condition here is when [mime_guess::from_path::first_raw](https://docs.rs/mime_guess/latest/mime_guess/struct.MimeGuess.html#method.first_raw) returns `None`).

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(MimeGuessLayer::default())
    .finish();
Ok(())
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#impl-Clone-for-MimeGuessLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#impl-Debug-for-MimeGuessLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#impl-Default-for-MimeGuessLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#impl-Layer%3CA%3E-for-MimeGuessLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = MimeGuessAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html#blanket-implementations" class="anchor">Â§</a>
