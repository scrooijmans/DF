# Module sort_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/lib.rs.html#36" class="src">Source</a>

Expand description

Sort expressions

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr_common::sort_expr::LexOrdering">LexOrdering</a>  
This object represents a lexicographical ordering and contains a vector of `PhysicalSortExpr` objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/struct.LexRequirement.html" class="struct" title="struct datafusion::physical_expr_common::sort_expr::LexRequirement">LexRequirement</a>  
This object represents a lexicographical ordering requirement and contains a vector of `PhysicalSortRequirement` objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr_common::sort_expr::PhysicalSortExpr">PhysicalSortExpr</a>  
Represents Sort operation for a column in a RecordBatch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr_common::sort_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>  
Represents sort requirement associated with a plan

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/enum.OrderingRequirements.html" class="enum" title="enum datafusion::physical_expr_common::sort_expr::OrderingRequirements">OrderingRequirements</a>  
Represents a plan’s input ordering requirements. Vector elements represent alternative ordering requirements in the order of preference. The list of alternatives can be either hard or soft, depending on whether the operator can work without an input ordering.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/fn.format_physical_sort_requirement_list.html" class="fn" title="fn datafusion::physical_expr_common::sort_expr::format_physical_sort_requirement_list">format_physical_sort_requirement_list</a>  
Writes a list of [`PhysicalSortRequirement`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html "struct datafusion::physical_expr::PhysicalSortRequirement")s to a `std::fmt::Formatter`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/sort_expr/fn.options_compatible.html" class="fn" title="fn datafusion::physical_expr_common::sort_expr::options_compatible">options_compatible</a>  
Returns whether the given two [`SortOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html "struct datafusion::common::arrow::compute::SortOptions") are compatible. Here, compatibility means that they are either exactly equal, or they differ only in whether NULL values come in first/last, which is immaterial because the column in question is not nullable (specified by the `nullable` parameter).
