# Function visit_schema Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/visitor.rs.html#120-123" class="src">Source</a>

``` rust
pub fn visit_schema<V: SchemaVisitor>(
    schema: &Schema,
    visitor: &mut V,
) -> Result<V::T>
```

Expand description

Visit schema in post order.
