# Struct ObjectRegistry Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/registry.rs.html#26" class="src">Source</a>

``` rust
pub struct ObjectRegistry {
    pub builder_constructor: Box<dyn Fn(PlSmallStr, usize) -> Box<dyn AnonymousObjectBuilder> + Send + Sync>,
    pub physical_dtype: ArrowDataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#structfield.builder_constructor" class="anchor field">§</a>`builder_constructor: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn"><code>Fn</code></a>`(`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`, `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`) -> `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/trait.AnonymousObjectBuilder.html" class="trait" title="trait polars::chunked_array::object::registry::AnonymousObjectBuilder"><code>AnonymousObjectBuilder</code></a>`> + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>`>`

A function that creates an object builder

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#structfield.physical_dtype" class="anchor field">§</a>`physical_dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType"><code>ArrowDataType</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#impl-Debug-for-ObjectRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html" class="struct" title="struct polars::chunked_array::object::registry::ObjectRegistry">ObjectRegistry</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/struct.ObjectRegistry.html#blanket-implementations" class="anchor">§</a>
