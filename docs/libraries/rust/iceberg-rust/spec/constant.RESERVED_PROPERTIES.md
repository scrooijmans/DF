# Constant RESERVED_PROPERTIES Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/table_metadata.rs.html#90-100" class="src">Source</a>

``` rust
pub const RESERVED_PROPERTIES: [&str; 9];
```

Expand description

Reserved Iceberg table properties list.

Reserved table properties are only used to control behaviors when creating or updating a table. The value of these properties are not persisted as a part of the table metadata.
