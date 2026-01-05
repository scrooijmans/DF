# Struct EquivalenceProperties Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/equivalence/properties/mod.rs.html#133" class="src">Source</a>

``` rust
pub struct EquivalenceProperties { /* private fields */ }
```

Expand description

`EquivalenceProperties` stores information about the output of a plan node that can be used to optimize the plan. Currently, it keeps track of:

- Sort expressions (orderings),
- Equivalent expressions; i.e. expressions known to have the same value.
- Constants expressions; i.e. expressions known to contain a single constant value.

Please see the [Using Ordering for Better Plans](https://datafusion.apache.org/blog/2025/03/11/ordering-analysis/) blog for more details.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#example-equivalent-sort-expressions" class="doc-anchor">§</a>Example equivalent sort expressions

Consider table below:

``` text
┌-------┐
| a | b |
|---|---|
| 1 | 9 |
| 2 | 8 |
| 3 | 7 |
| 5 | 5 |
└---┴---┘
```

In this case, both `a ASC` and `b DESC` can describe the table ordering. `EquivalenceProperties` tracks these different valid sort expressions and treat `a ASC` and `b DESC` on an equal footing. For example, if the query specifies the output sorted by EITHER `a ASC` or `b DESC`, the sort can be avoided.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#example-equivalent-expressions" class="doc-anchor">§</a>Example equivalent expressions

Similarly, consider the table below:

``` text
┌-------┐
| a | b |
|---|---|
| 1 | 1 |
| 2 | 2 |
| 3 | 3 |
| 5 | 5 |
└---┴---┘
```

In this case, columns `a` and `b` always have the same value. With this information, Datafusion can optimize various operations. For example, if the partition requirement is `Hash(a)` and output partitioning is `Hash(b)`, then DataFusion avoids repartitioning the data as the existing partitioning satisfies the requirement.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#code-example" class="doc-anchor">§</a>Code Example

``` rust
use datafusion_physical_expr_common::sort_expr::{LexOrdering, PhysicalSortExpr};
// This object represents data that is sorted by a ASC, c DESC
// with a single constant value of b
let mut eq_properties = EquivalenceProperties::new(schema);
eq_properties.add_constants(vec![ConstExpr::from(col_b)]);
eq_properties.add_ordering([
  PhysicalSortExpr::new_default(col_a).asc(),
  PhysicalSortExpr::new_default(col_c).desc(),
]);

assert_eq!(eq_properties.to_string(), "order: [[a@0 ASC, c@2 DESC]], eq: [{members: [b@1], constant: (heterogeneous)}]");
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#impl-EquivalenceProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.new" class="fn">new</a>(schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Creates an empty `EquivalenceProperties` object.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.with_constraints" class="fn">with_constraints</a>(self, constraints: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Adds constraints to the properties.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.new_with_orderings" class="fn">new_with_orderings</a>( schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, orderings: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Creates a new `EquivalenceProperties` object with the given orderings.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the associated schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.oeq_class" class="fn">oeq_class</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.OrderingEquivalenceClass.html" class="struct" title="struct datafusion::physical_expr::equivalence::OrderingEquivalenceClass">OrderingEquivalenceClass</a>

Returns a reference to the ordering equivalence class within.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.eq_group" class="fn">eq_group</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.EquivalenceGroup.html" class="struct" title="struct datafusion::physical_expr::equivalence::EquivalenceGroup">EquivalenceGroup</a>

Returns a reference to the equivalence group within.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.constraints" class="fn">constraints</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>

Returns a reference to the constraints within.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.constants" class="fn">constants</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>\>

Returns all the known constants expressions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.output_ordering" class="fn">output_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>

Returns the output ordering of the properties.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.extend" class="fn">extend</a>( self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extends this `EquivalenceProperties` with the `other` object.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.clear_orderings" class="fn">clear_orderings</a>(&mut self)

Clears (empties) the ordering equivalence class within this object. Call this method when existing orderings are invalidated.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.clear_per_partition_constants" class="fn">clear_per_partition_constants</a>(&mut self)

Removes constant expressions that may change across partitions. This method should be used when merging data from different partitions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.add_orderings" class="fn">add_orderings</a>( &mut self, orderings: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>\>, )

Adds new orderings into the existing ordering equivalence class.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.add_ordering" class="fn">add_ordering</a>( &mut self, ordering: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, )

Adds a single ordering to the existing ordering equivalence class.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.add_equivalence_group" class="fn">add_equivalence_group</a>( &mut self, other_eq_group: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.EquivalenceGroup.html" class="struct" title="struct datafusion::physical_expr::equivalence::EquivalenceGroup">EquivalenceGroup</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Incorporates the given equivalence group to into the existing equivalence group within.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.normalized_oeq_class" class="fn">normalized_oeq_class</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.OrderingEquivalenceClass.html" class="struct" title="struct datafusion::physical_expr::equivalence::OrderingEquivalenceClass">OrderingEquivalenceClass</a>

Returns the ordering equivalence class within in normal form. Normalization standardizes expressions according to the equivalence group within, and removes constants/duplicates.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.add_equal_conditions" class="fn">add_equal_conditions</a>( &mut self, left: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, right: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Adds a new equality condition into the existing equivalence group. If the given equality defines a new equivalence class, adds this new equivalence class to the equivalence group.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.add_constants" class="fn">add_constants</a>( &mut self, constants: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Track/register physical expressions with constant values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.reorder" class="fn">reorder</a>( &mut self, ordering: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the ordering equivalence class within assuming that the table is re-sorted according to the argument `ordering`, and returns whether this operation resulted in any change. Note that equivalence classes (and constants) do not change as they are unaffected by a re-sort. If the given ordering is already satisfied, the function does nothing.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.normalize_sort_exprs" class="fn">normalize_sort_exprs</a>( &self, sort_exprs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>

Normalizes the given sort expressions (i.e. `sort_exprs`) using the equivalence group within. Returns a `LexOrdering` instance if the expressions define a proper lexicographical ordering. For more details, see [`EquivalenceGroup::normalize_sort_exprs`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.EquivalenceGroup.html#method.normalize_sort_exprs "method datafusion::physical_expr::equivalence::EquivalenceGroup::normalize_sort_exprs").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.normalize_sort_requirements" class="fn">normalize_sort_requirements</a>( &self, sort_reqs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexRequirement.html" class="struct" title="struct datafusion::physical_expr::LexRequirement">LexRequirement</a>\>

Normalizes the given sort requirements (i.e. `sort_reqs`) using the equivalence group within. Returns a `LexRequirement` instance if the expressions define a proper lexicographical requirement. For more details, see [`EquivalenceGroup::normalize_sort_exprs`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.EquivalenceGroup.html#method.normalize_sort_exprs "method datafusion::physical_expr::equivalence::EquivalenceGroup::normalize_sort_exprs").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.ordering_satisfy" class="fn">ordering_satisfy</a>( &self, given: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Iteratively checks whether the given ordering is satisfied by any of the existing orderings. See [`Self::ordering_satisfy_requirement`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.ordering_satisfy_requirement "method datafusion::physical_expr::EquivalenceProperties::ordering_satisfy_requirement") for more details and examples.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.ordering_satisfy_requirement" class="fn">ordering_satisfy_requirement</a>( &self, given: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Iteratively checks whether the given sort requirement is satisfied by any of the existing orderings.

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#example-scenarios" class="doc-anchor">§</a>Example Scenarios

In these scenarios, assume that all expressions share the same sort properties.

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#case-1-sort-requirement-a-c" class="doc-anchor">§</a>Case 1: Sort Requirement `[a, c]`

**Existing orderings:** `[[a, b, c], [a, d]]`, **constants:** `[]`

1.  The function first checks the leading requirement `a`, which is satisfied by `[a, b, c].first()`.
2.  `a` is added as a constant for the next iteration.
3.  Normal orderings become `[[b, c], [d]]`.
4.  The function fails for `c` in the second iteration, as neither `[b, c]` nor `[d]` satisfies `c`.

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#case-2-sort-requirement-a-d" class="doc-anchor">§</a>Case 2: Sort Requirement `[a, d]`

**Existing orderings:** `[[a, b, c], [a, d]]`, **constants:** `[]`

1.  The function first checks the leading requirement `a`, which is satisfied by `[a, b, c].first()`.
2.  `a` is added as a constant for the next iteration.
3.  Normal orderings become `[[b, c], [d]]`.
4.  The function returns `true` as `[d]` satisfies `d`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.extract_common_sort_prefix" class="fn">extract_common_sort_prefix</a>( &self, ordering: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Determines the longest normal prefix of `ordering` satisfied by the existing ordering. Returns that prefix as a new `LexOrdering`, and a boolean indicating whether all the sort expressions are satisfied.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.requirements_compatible" class="fn">requirements_compatible</a>( &self, given: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexRequirement.html" class="struct" title="struct datafusion::physical_expr::LexRequirement">LexRequirement</a>, reference: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexRequirement.html" class="struct" title="struct datafusion::physical_expr::LexRequirement">LexRequirement</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks whether the `given` sort requirements are equal or more specific than the `reference` sort requirements.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.project_expr" class="fn">project_expr</a>( &self, expr: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, mapping: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.ProjectionMapping.html" class="struct" title="struct datafusion::physical_expr::equivalence::ProjectionMapping">ProjectionMapping</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Projects argument `expr` according to the projection described by `mapping`, taking equivalences into account.

For example, assume that columns `a` and `c` are always equal, and that the projection described by `mapping` encodes the following:

``` text
a -> a1
b -> b1
```

Then, this function projects `a + b` to `Some(a1 + b1)`, `c + b` to `Some(a1 + b1)` and `d` to `None`, meaning that it is not projectable.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.project_expressions" class="fn">project_expressions</a>\<'a\>( &'a self, expressions: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\> + 'a, mapping: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.ProjectionMapping.html" class="struct" title="struct datafusion::physical_expr::equivalence::ProjectionMapping">ProjectionMapping</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>\> + 'a

Projects the given `expressions` according to the projection described by `mapping`, taking equivalences into account. This function is similar to [`Self::project_expr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.project_expr "method datafusion::physical_expr::EquivalenceProperties::project_expr"), but projects multiple expressions at once more efficiently than calling `project_expr` for each expression.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.project" class="fn">project</a>( &self, mapping: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.ProjectionMapping.html" class="struct" title="struct datafusion::physical_expr::equivalence::ProjectionMapping">ProjectionMapping</a>, output_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Projects the equivalences within according to `mapping` and `output_schema`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.find_longest_permutation" class="fn">find_longest_permutation</a>( &self, exprs: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the longest (potentially partial) permutation satisfying the existing ordering. For example, if we have the equivalent orderings `[a ASC, b ASC]` and `[c DESC]`, with `exprs` containing `[c, b, a, d]`, then this function returns `([a ASC, b ASC, c DESC], [2, 1, 0])`. This means that the specification `[a ASC, b ASC, c DESC]` is satisfied by the existing ordering, and `[a, b, c]` resides at indices: `2, 1, 0` inside the argument `exprs` (respectively). For the mathematical definition of “partial permutation”, see:

<https://en.wikipedia.org/wiki/Permutation#k-permutations_of_n>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.is_expr_constant" class="fn">is_expr_constant</a>( &self, expr: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>\>

This function determines whether the provided expression is constant based on the known constants. For example, if columns `a` and `b` are constant, then expressions `a`, `b` and `a + b` will all return `true` whereas expression `c` will return `false`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#parameters" class="doc-anchor">§</a>Parameters

- `expr`: A reference to a `Arc<dyn PhysicalExpr>` representing the expression to be checked.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#returns" class="doc-anchor">§</a>Returns

Returns a `Some` value if the expression is constant according to equivalence group, and `None` otherwise. The `Some` variant contains an `AcrossPartitions` value indicating whether the expression is constant across partitions, and its actual value (if available).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.get_expr_properties" class="fn">get_expr_properties</a>(&self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

Retrieves the properties for a given physical expression.

This function constructs an [`ExprProperties`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html "struct datafusion::logical_expr::sort_properties::ExprProperties") object for the given expression, which encapsulates information about the expression’s properties, including its [`SortProperties`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html "enum datafusion::logical_expr::sort_properties::SortProperties") and [`Interval`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#parameters-1" class="doc-anchor">§</a>Parameters

- `expr`: An `Arc<dyn PhysicalExpr>` representing the physical expression for which ordering information is sought.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#returns-1" class="doc-anchor">§</a>Returns

Returns an [`ExprProperties`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html "struct datafusion::logical_expr::sort_properties::ExprProperties") object containing the ordering and range information for the given expression.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.with_new_schema" class="fn">with_new_schema</a>( self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Transforms this `EquivalenceProperties` by mapping columns in the original schema to columns in the new schema by index.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#impl-Clone-for-EquivalenceProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#impl-Debug-for-EquivalenceProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#impl-Display-for-EquivalenceProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

More readable display version of the `EquivalenceProperties`.

Format:

``` text
order: [[b@1 ASC NULLS LAST]], eq: [{members: [a@0], constant: (heterogeneous)}]
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#impl-From%3CEquivalenceProperties%3E-for-OrderingEquivalenceClass" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.OrderingEquivalenceClass.html" class="struct" title="struct datafusion::physical_expr::equivalence::OrderingEquivalenceClass">OrderingEquivalenceClass</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(eq_properties: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.OrderingEquivalenceClass.html" class="struct" title="struct datafusion::physical_expr::equivalence::OrderingEquivalenceClass">OrderingEquivalenceClass</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html#blanket-implementations" class="anchor">§</a>
