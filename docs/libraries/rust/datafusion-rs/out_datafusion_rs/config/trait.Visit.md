# Trait Visit Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1430" class="src">Source</a>

``` rust
pub trait Visit {
    // Required methods
    fn some<V>(&mut self, key: &str, value: V, description: &'static str)
       where V: Display;
    fn none(&mut self, key: &str, description: &'static str);
}
```

Expand description

An implementation trait used to recursively walk configuration

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html#tymethod.some" class="fn">some</a>\<V\>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: V, description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>,

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html#tymethod.none" class="fn">none</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html#implementors" class="anchor">§</a>
