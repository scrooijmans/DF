# Struct OwnedObject Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/any_value.rs.html#16" class="src">Source</a>

``` rust
pub struct OwnedObject(pub Box<dyn PolarsObjectSafe>);
```

## Tuple Fields<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe"><code>PolarsObjectSafe</code></a>`>`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#impl-Clone-for-OwnedObject" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.OwnedObject.html" class="struct" title="struct polars::prelude::OwnedObject">OwnedObject</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.OwnedObject.html" class="struct" title="struct polars::prelude::OwnedObject">OwnedObject</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#impl-Debug-for-OwnedObject" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.OwnedObject.html" class="struct" title="struct polars::prelude::OwnedObject">OwnedObject</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.OwnedObject.html#blanket-implementations" class="anchor">§</a>
