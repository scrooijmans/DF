# Struct DisplayableExecutionPlan Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/display.rs.html#115" class="src">Source</a>

``` rust
pub struct DisplayableExecutionPlan<'a> { /* private fields */ }
```

Expand description

Wraps an `ExecutionPlan` with various methods for formatting

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#example" class="doc-anchor">§</a>Example

``` rust
// Get a one line description (Displayable)
let display_plan = displayable(plan.as_ref());

// you can use the returned objects to format plans
// where you can use `Display` such as  format! or println!
assert_eq!(
   &format!("The plan is: {}", display_plan.one_line()),
  "The plan is: FilterExec: i@0 = 1\n"
);
// You can also print out the plan and its children in indented mode
assert_eq!(display_plan.indent(false).to_string(),
  "FilterExec: i@0 = 1\
  \n  EmptyExec\
  \n"
);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#impl-DisplayableExecutionPlan%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.new" class="fn">new</a>(inner: &'a dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Create a wrapper around an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") which can be pretty printed in a variety of ways

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.with_metrics" class="fn">with_metrics</a>( inner: &'a dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Create a wrapper around an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") which can be pretty printed in a variety of ways that also shows aggregated metrics

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.with_full_metrics" class="fn">with_full_metrics</a>( inner: &'a dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Create a wrapper around an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") which can be pretty printed in a variety of ways that also shows all low level metrics

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.set_show_schema" class="fn">set_show_schema</a>(self, show_schema: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Enable display of schema

If true, plans will be displayed with schema information at the end of each line. The format is `schema=[[a:Int32;N, b:Int32;N, c:Int32;N]]`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.set_show_statistics" class="fn">set_show_statistics</a>( self, show_statistics: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Enable display of statistics

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.set_tree_maximum_render_width" class="fn">set_tree_maximum_render_width</a>( self, width: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Set the maximum render width for the tree format

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.indent" class="fn">indent</a>(&self, verbose: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> + 'a

Return a `format`able structure that produces a single line per node.

``` text
ProjectionExec: expr=[a]
  CoalesceBatchesExec: target_batch_size=8192
    FilterExec: a < 5
      RepartitionExec: partitioning=RoundRobinBatch(16)
        DataSourceExec: source=...",
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.graphviz" class="fn">graphviz</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> + 'a

Returns a `format`able structure that produces graphviz format for execution plan, which can be directly visualized [here](https://dreampuf.github.io/GraphvizOnline).

An example is

``` dot
strict digraph dot_plan {
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.tree_render" class="fn">tree_render</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> + 'a

Formats the plan using a ASCII art like tree

See [`DisplayFormatType::TreeRender`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html#variant.TreeRender "variant datafusion::physical_plan::DisplayFormatType::TreeRender") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.one_line" class="fn">one_line</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> + 'a

Return a single-line summary of the root of the plan Example: `ProjectionExec: expr=[a@0 as a]`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.to_stringified" class="fn">to_stringified</a>( &self, verbose: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, plan_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>, explain_format: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StringifiedPlan.html" class="struct" title="struct datafusion::logical_expr::StringifiedPlan">StringifiedPlan</a>

👎Deprecated since 47.0.0: indent() or tree_render() instead

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#impl-Clone-for-DisplayableExecutionPlan%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#impl-Debug-for-DisplayableExecutionPlan%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html#blanket-implementations" class="anchor">§</a>
