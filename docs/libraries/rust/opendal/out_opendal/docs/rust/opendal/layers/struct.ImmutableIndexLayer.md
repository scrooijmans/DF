# Struct ImmutableIndexLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/immutable_index.rs.html#53-55" class="src">Source</a>

``` rust
pub struct ImmutableIndexLayer { /* private fields */ }
```

Expand description

Add an immutable in-memory index for underlying storage services.

Especially useful for services without list capability like HTTP.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust


let mut iil = ImmutableIndexLayer::default();

for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
    iil.insert(i.to_string())
}

let op = Operator::from_iter::<services::Memory>(HashMap::<_, _>::default())?
    .layer(iil)
    .finish();
Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#impl-ImmutableIndexLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.insert" class="fn">insert</a>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Insert a key into index.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.extend_iter" class="fn">extend_iter</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

Insert keys from iter.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#impl-Clone-for-ImmutableIndexLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#impl-Debug-for-ImmutableIndexLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#impl-Default-for-ImmutableIndexLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#impl-Layer%3CA%3E-for-ImmutableIndexLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ImmutableIndexAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html#blanket-implementations" class="anchor">Â§</a>
