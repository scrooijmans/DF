# Struct FilterPushdownPropagation Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/filter_pushdown.rs.html#225" class="src">Source</a>

``` rust
pub struct FilterPushdownPropagation<T> {
    pub filters: Vec<PushedDown>,
    pub updated_node: Option<T>,
}
```

Expand description

The result of pushing down filters into a node.

Returned from [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") to communicate to the optimizer:

1.  What to do with any parent filters that were could not be pushed down into the children.
2.  If the node needs to be replaced in the execution plan with a new node or not.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#structfield.filters" class="anchor field">§</a>`filters: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.PushedDown.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::PushedDown"><code>PushedDown</code></a>`>`

What filters were pushed into the parent node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#structfield.updated_node" class="anchor field">§</a>`updated_node: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<T>`

The updated node, if it was updated during pushdown

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#impl-FilterPushdownPropagation%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.if_all" class="fn">if_all</a>( child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

Create a new [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation") that tells the parent node that each parent filter is supported if it was supported by *all* children.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.if_any" class="fn">if_any</a>( child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

Create a new [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation") that tells the parent node that each parent filter is supported if it was supported by *any* child.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.all_unsupported" class="fn">all_unsupported</a>( child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

Create a new [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation") that tells the parent node that no filters were pushed down regardless of the child results.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.with_parent_pushdown_result" class="fn">with_parent_pushdown_result</a>( filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.PushedDown.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::PushedDown">PushedDown</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

Create a new [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation") with the specified filter support. This transmits up to our parent node what the result of pushing down the filters into our node and possibly our subtree was.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.with_updated_node" class="fn">with_updated_node</a>(self, updated_node: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

Bind an updated node to the [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation"). Use this when the current node wants to update itself in the tree or replace itself with a new node (e.g. one of it’s children). You do not need to call this if one of the children of the current node may have updated itself, that is handled by the optimizer.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#impl-Clone-for-FilterPushdownPropagation%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#impl-Debug-for-FilterPushdownPropagation%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#blanket-implementations" class="anchor">§</a>
