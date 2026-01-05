# Type Alias BuilderConstructor Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/registry.rs.html#21" class="src">Source</a>

``` rust
pub type BuilderConstructor = Box<dyn Fn(PlSmallStr, usize) -> Box<dyn AnonymousObjectBuilder> + Send + Sync>;
```

Expand description

Takes a `name` and `capacity` and constructs a new builder.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/chunked_array/object/registry/type.BuilderConstructor.html#aliased-type" class="anchor">§</a>

``` rust
pub struct BuilderConstructor(/* private fields */);
```
