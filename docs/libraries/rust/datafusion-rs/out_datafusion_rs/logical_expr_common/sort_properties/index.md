# Module sort_properties Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#43" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr_common::sort_properties::ExprProperties">ExprProperties</a>  
Represents the properties of a `PhysicalExpr`, including its sorting, range, and whether it preserves lexicographical ordering.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr_common::sort_properties::SortProperties">SortProperties</a>  
To propagate [`SortOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html "struct datafusion::common::arrow::compute::SortOptions") across the `PhysicalExpr`, it is insufficient to simply use `Option<SortOptions>`: There must be a differentiation between unordered columns and literal values, since literals may not break the ordering when they are used as a child of some binary expression when the other child has some ordering. On the other hand, unordered columns cannot maintain ordering when they take part in such operations.
