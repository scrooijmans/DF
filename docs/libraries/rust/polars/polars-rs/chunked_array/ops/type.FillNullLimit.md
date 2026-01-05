# Type Alias FillNullLimit Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#403" class="src">Source</a>

``` rust
pub type FillNullLimit = Option<u32>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/type.FillNullLimit.html#aliased-type" class="anchor">§</a>

``` rust
pub enum FillNullLimit {
    None,
    Some(u32),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/type.FillNullLimit.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/type.FillNullLimit.html#variant.None" class="anchor">§</a>1.0.0

### None

No value.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/type.FillNullLimit.html#variant.Some" class="anchor">§</a>1.0.0

### Some(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>)

Some value of type `T`.
