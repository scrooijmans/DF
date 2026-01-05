# Module group_by Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/mod.rs.html#30" class="src">Source</a>

## Modules<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/index.html" class="mod" title="mod polars::frame::group_by::aggregations">aggregations</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/expr/index.html" class="mod" title="mod polars::frame::group_by::expr">expr</a>

## Structs<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html" class="struct" title="struct polars::frame::group_by::GroupBy">GroupBy</a>

Returned by a group_by operation on a DataFrame. This struct supports several aggregations.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupPositions.html" class="struct" title="struct polars::frame::group_by::GroupPositions">GroupPositions</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupsIdx.html" class="struct" title="struct polars::frame::group_by::GroupsIdx">GroupsIdx</a>

Indexes of the groups, the first index is stored separately. this make sorting fast.

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupsTypeIter.html" class="struct" title="struct polars::frame::group_by::GroupsTypeIter">GroupsTypeIter</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupsTypeParIter.html" class="struct" title="struct polars::frame::group_by::GroupsTypeParIter">GroupsTypeParIter</a>

## Enums<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupByMethod.html" class="enum" title="enum polars::frame::group_by::GroupByMethod">GroupByMethod</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsIndicator.html" class="enum" title="enum polars::frame::group_by::GroupsIndicator">GroupsIndicator</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/enum.GroupsType.html" class="enum" title="enum polars::frame::group_by::GroupsType">GroupsType</a>

## Traits<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/trait.IntoGroupsType.html" class="trait" title="trait polars::frame::group_by::IntoGroupsType">IntoGroupsType</a>  
Used to create the tuples for a group_by operation.

## Functions<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/fn.fmt_group_by_column.html" class="fn" title="fn polars::frame::group_by::fmt_group_by_column">fmt_group_by_column</a>

## Type Aliases<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/type.BorrowIdxItem.html" class="type" title="type polars::frame::group_by::BorrowIdxItem">BorrowIdxItem</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/type.GroupsSlice.html" class="type" title="type polars::frame::group_by::GroupsSlice">GroupsSlice</a>

Every group is indicated by an array where the

<a href="https://docs.rs/polars/latest/polars/frame/group_by/type.IdxItem.html" class="type" title="type polars::frame::group_by::IdxItem">IdxItem</a>
