# Module binary_view_map Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/lib.rs.html#33" class="src">Source</a>

Expand description

[`ArrowBytesViewMap`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html "struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap") and [`ArrowBytesViewSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html "struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet") for storing maps/sets of values from `StringViewArray`/`BinaryViewArray`. Much of the code is from `binary_map.rs`, but with simpler implementation because we directly use the [`GenericByteViewBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewBuilder.html "struct datafusion::common::arrow::array::GenericByteViewBuilder").

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap">ArrowBytesViewMap</a>  
Optimized map for storing Arrow “byte view” types (`StringView`, `BinaryView`) values that can produce the set of keys on output as `GenericBinaryViewArray` without copies.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet">ArrowBytesViewSet</a>  
HashSet optimized for storing string or binary values that can produce that the final set as a `GenericBinaryViewArray` with minimal copies.
