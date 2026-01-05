# Trait PolarsObject Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/mod.rs.html#49-50" class="src">Source</a>

``` rust
pub trait PolarsObject:
    Any
    + Debug
    + Clone
    + Send
    + Sync
    + Default
    + Display
    + Hash
    + TotalHash
    + PartialEq
    + Eq
    + TotalEq {
    // Required method
    fn type_name() -> &'static str;
}
```

Expand description

Values need to implement this so that they can be stored into a Series and DataFrame

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html#tymethod.type_name" class="fn">type_name</a>() -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

This should be used as type information. Consider this a part of the type system.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html#implementors" class="anchor">§</a>
