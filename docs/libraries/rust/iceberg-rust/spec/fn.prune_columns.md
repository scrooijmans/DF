# Function prune_columns Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/prune_columns.rs.html#26-44" class="src">Source</a>

``` rust
pub fn prune_columns(
    schema: &Schema,
    selected: impl IntoIterator<Item = i32>,
    select_full_types: bool,
) -> Result<Type>
```

Expand description

Visit a schema and returns only the fields selected by id set
