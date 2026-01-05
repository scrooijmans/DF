# Type Alias GroupsSlice Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/position.rs.html#250" class="src">Source</a>

``` rust
pub type GroupsSlice = Vec<[u32; 2]>;
```

Expand description

Every group is indicated by an array where the

- first value is an index to the start of the group
- second value is the length of the group

Only used when group values are stored together

This type should have the invariant that it is always sorted in ascending order.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/frame/group_by/type.GroupsSlice.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GroupsSlice { /* private fields */ }
```
