# Function visit_schema_with_partner Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/visitor.rs.html#269-282" class="src">Source</a>

``` rust
pub fn visit_schema_with_partner<P, V: SchemaWithPartnerVisitor<P>, A: PartnerAccessor<P>>(
    schema: &Schema,
    partner: &P,
    visitor: &mut V,
    accessor: &A,
) -> Result<V::T>
```

Expand description

Visit schema in post order.
