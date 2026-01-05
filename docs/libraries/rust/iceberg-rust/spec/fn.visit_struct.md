# Function visit_struct Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/visitor.rs.html#106-117" class="src">Source</a>

``` rust
pub fn visit_struct<V: SchemaVisitor>(
    s: &StructType,
    visitor: &mut V,
) -> Result<V::T>
```

Expand description

Visit struct type in post order.
