# Constant PROPERTY_FORMAT_VERSION Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/table_metadata.rs.html#58" class="src">Source</a>

``` rust
pub const PROPERTY_FORMAT_VERSION: &str = "format-version";
```

Expand description

Reserved table property for table format version.

Iceberg will default a new table’s format version to the latest stable and recommended version. This reserved property keyword allows users to override the Iceberg format version of the table metadata.

If this table property exists when creating a table, the table will use the specified format version. If a table updates this property, it will try to upgrade to the specified format version.
