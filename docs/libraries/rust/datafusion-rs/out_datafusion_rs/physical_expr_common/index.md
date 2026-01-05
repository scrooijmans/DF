# Module physical_expr_common Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#802" class="src">Source</a>

Expand description

re-export of [`datafusion_physical_expr`](https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_expr/index.html "mod datafusion_physical_expr") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/index.html" class="mod" title="mod datafusion::physical_expr_common::binary_map">binary_map</a>

[`ArrowBytesMap`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesMap.html "struct datafusion::physical_expr_common::binary_map::ArrowBytesMap") and [`ArrowBytesSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html "struct datafusion::physical_expr_common::binary_map::ArrowBytesSet") for storing maps/sets of values from StringArray / LargeStringArray / BinaryArray / LargeBinaryArray.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/index.html" class="mod" title="mod datafusion::physical_expr_common::binary_view_map">binary_view_map</a>

[`ArrowBytesViewMap`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html "struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap") and [`ArrowBytesViewSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html "struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet") for storing maps/sets of values from `StringViewArray`/`BinaryViewArray`. Much of the code is from `binary_map.rs`, but with simpler implementation because we directly use the [`GenericByteViewBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewBuilder.html "struct datafusion::common::arrow::array::GenericByteViewBuilder").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/index.html" class="mod" title="mod datafusion::physical_expr_common::datum">datum</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/index.html" class="mod" title="mod datafusion::physical_expr_common::physical_expr">physical_expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/index.html" class="mod" title="mod datafusion::physical_expr_common::sort_expr">sort_expr</a>

Sort expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/tree_node/index.html" class="mod" title="mod datafusion::physical_expr_common::tree_node">tree_node</a>

This module provides common traits for visiting or rewriting tree nodes easily.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/utils/index.html" class="mod" title="mod datafusion::physical_expr_common::utils">utils</a>
