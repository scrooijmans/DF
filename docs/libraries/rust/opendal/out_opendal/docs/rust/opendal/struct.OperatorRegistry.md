# Struct OperatorRegistry Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/registry.rs.html#37-39" class="src">Source</a>

``` rust
pub struct OperatorRegistry { /* private fields */ }
```

Expand description

Global registry that maps schemes to [`OperatorFactory`](https://opendal.apache.org/docs/rust/opendal/type.OperatorFactory.html "type opendal::OperatorFactory") functions.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#impl-OperatorRegistry" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html" class="struct" title="struct opendal::OperatorRegistry">OperatorRegistry</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#method.new" class="fn">new</a>() -\> Self

Create a new, empty registry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#method.register" class="fn">register</a>\<B: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a>\>(&self, scheme: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Register a builder for the given scheme.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#method.load" class="fn">load</a>(&self, uri: impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>\>

Load an [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") via the factory registered for the URIâ€™s scheme.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#impl-Debug-for-OperatorRegistry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html" class="struct" title="struct opendal::OperatorRegistry">OperatorRegistry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#impl-Default-for-OperatorRegistry" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html" class="struct" title="struct opendal::OperatorRegistry">OperatorRegistry</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html" class="struct" title="struct opendal::OperatorRegistry">OperatorRegistry</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorRegistry.html#blanket-implementations" class="anchor">Â§</a>
