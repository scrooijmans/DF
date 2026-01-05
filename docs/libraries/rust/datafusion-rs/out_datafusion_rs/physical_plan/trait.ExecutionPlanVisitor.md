# Trait ExecutionPlanVisitor Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/visitor.rs.html#62" class="src">Source</a>

``` rust
pub trait ExecutionPlanVisitor {
    type Error;

    // Required method
    fn pre_visit(
        &mut self,
        plan: &dyn ExecutionPlan,
    ) -> Result<bool, Self::Error>;

    // Provided method
    fn post_visit(
        &mut self,
        _plan: &dyn ExecutionPlan,
    ) -> Result<bool, Self::Error> { ... }
}
```

Expand description

Trait that implements the [Visitor pattern](https://en.wikipedia.org/wiki/Visitor_pattern) for a depth first walk of `ExecutionPlan` nodes. `pre_visit` is called before any children are visited, and then `post_visit` is called after all children have been visited.

To use, define a struct that implements this trait and then invoke \[‘accept’\].

For example, for an execution plan that looks like:

``` text
ProjectionExec: id
   FilterExec: state = CO
      DataSourceExec:
```

The sequence of visit operations would be:

``` text
visitor.pre_visit(ProjectionExec)
visitor.pre_visit(FilterExec)
visitor.pre_visit(DataSourceExec)
visitor.post_visit(DataSourceExec)
visitor.post_visit(FilterExec)
visitor.post_visit(ProjectionExec)
```

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#associatedtype.Error" class="associatedtype">Error</a>

The type of error returned by this visitor

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#tymethod.pre_visit" class="fn">pre_visit</a>(&mut self, plan: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#associatedtype.Error" class="associatedtype" title="type datafusion::physical_plan::ExecutionPlanVisitor::Error">Error</a>\>

Invoked on an `ExecutionPlan` plan before any of its child inputs have been visited. If Ok(true) is returned, the recursion continues. If Err(..) or Ok(false) are returned, the recursion stops immediately and the error, if any, is returned to `accept`

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#method.post_visit" class="fn">post_visit</a>(&mut self, \_plan: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#associatedtype.Error" class="associatedtype" title="type datafusion::physical_plan::ExecutionPlanVisitor::Error">Error</a>\>

Invoked on an `ExecutionPlan` plan *after* all of its child inputs have been visited. The return value is handled the same as the return value of `pre_visit`. The provided default implementation returns `Ok(true)`.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html#implementors" class="anchor">§</a>
