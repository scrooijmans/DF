# Type Alias PlIdHashMap Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/aliases.rs.html#24" class="src">Source</a>

``` rust
pub type PlIdHashMap<K, V> = HashMap<K, V, BuildHasherDefault<IdHasher>>;
```

Expand description

This hashmap uses an IdHasher

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.PlIdHashMap.html#aliased-type" class="anchor">§</a>

``` rust
pub struct PlIdHashMap<K, V> { /* private fields */ }
```
