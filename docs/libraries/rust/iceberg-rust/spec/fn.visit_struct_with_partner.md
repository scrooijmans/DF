# Function visit_struct_with_partner Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/visitor.rs.html#249-266" class="src">Source</a>

``` rust
pub fn visit_struct_with_partner<P, V: SchemaWithPartnerVisitor<P>, A: PartnerAccessor<P>>(
    s: &StructType,
    partner: &P,
    visitor: &mut V,
    accessor: &A,
) -> Result<V::T>
```

Expand description

Visit struct type in post order.
