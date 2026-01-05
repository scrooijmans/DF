# Enum PlanType Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/display/mod.rs.html#31" class="src">Source</a>

``` rust
pub enum PlanType {
Show 13 variants    InitialLogicalPlan,
    AnalyzedLogicalPlan {
        analyzer_name: String,
    },
    FinalAnalyzedLogicalPlan,
    OptimizedLogicalPlan {
        optimizer_name: String,
    },
    FinalLogicalPlan,
    InitialPhysicalPlan,
    InitialPhysicalPlanWithStats,
    InitialPhysicalPlanWithSchema,
    OptimizedPhysicalPlan {
        optimizer_name: String,
    },
    FinalPhysicalPlan,
    FinalPhysicalPlanWithStats,
    FinalPhysicalPlanWithSchema,
    PhysicalPlanError,
}
```

Expand description

Represents which type of plan, when storing multiple for use in EXPLAIN plans

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.InitialLogicalPlan" class="anchor">§</a>

### InitialLogicalPlan

The initial LogicalPlan provided to DataFusion

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.AnalyzedLogicalPlan" class="anchor">§</a>

### AnalyzedLogicalPlan

The LogicalPlan which results from applying an analyzer pass

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.AnalyzedLogicalPlan.field.analyzer_name" class="anchor field">§</a>`analyzer_name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The name of the analyzer which produced this plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.FinalAnalyzedLogicalPlan" class="anchor">§</a>

### FinalAnalyzedLogicalPlan

The LogicalPlan after all analyzer passes have been applied

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.OptimizedLogicalPlan" class="anchor">§</a>

### OptimizedLogicalPlan

The LogicalPlan which results from applying an optimizer pass

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.OptimizedLogicalPlan.field.optimizer_name" class="anchor field">§</a>`optimizer_name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The name of the optimizer which produced this plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.FinalLogicalPlan" class="anchor">§</a>

### FinalLogicalPlan

The final, fully optimized LogicalPlan that was converted to a physical plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.InitialPhysicalPlan" class="anchor">§</a>

### InitialPhysicalPlan

The initial physical plan, prepared for execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.InitialPhysicalPlanWithStats" class="anchor">§</a>

### InitialPhysicalPlanWithStats

The initial physical plan with stats, prepared for execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.InitialPhysicalPlanWithSchema" class="anchor">§</a>

### InitialPhysicalPlanWithSchema

The initial physical plan with schema, prepared for execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.OptimizedPhysicalPlan" class="anchor">§</a>

### OptimizedPhysicalPlan

The ExecutionPlan which results from applying an optimizer pass

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.OptimizedPhysicalPlan.field.optimizer_name" class="anchor field">§</a>`optimizer_name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The name of the optimizer which produced this plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.FinalPhysicalPlan" class="anchor">§</a>

### FinalPhysicalPlan

The final, fully optimized physical plan which would be executed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.FinalPhysicalPlanWithStats" class="anchor">§</a>

### FinalPhysicalPlanWithStats

The final with stats, fully optimized physical plan which would be executed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.FinalPhysicalPlanWithSchema" class="anchor">§</a>

### FinalPhysicalPlanWithSchema

The final with schema, fully optimized physical plan which would be executed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#variant.PhysicalPlanError" class="anchor">§</a>

### PhysicalPlanError

An error creating the physical plan

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-Clone-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-Debug-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-Display-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-Hash-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-PartialEq-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-PartialOrd-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-Eq-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#impl-StructuralPartialEq-for-PlanType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html#blanket-implementations" class="anchor">§</a>
