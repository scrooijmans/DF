# Enum Expr Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/mod.rs.html#88" class="src">Source</a>

``` rust
pub enum Expr {
Show 24 variants    Alias(Arc<Expr>, PlSmallStr),
    Column(PlSmallStr),
    Selector(Selector),
    Literal(LiteralValue),
    DataTypeFunction(DataTypeFunction),
    BinaryExpr {
        left: Arc<Expr>,
        op: Operator,
        right: Arc<Expr>,
    },
    Cast {
        expr: Arc<Expr>,
        dtype: DataTypeExpr,
        options: CastOptions,
    },
    Sort {
        expr: Arc<Expr>,
        options: SortOptions,
    },
    Gather {
        expr: Arc<Expr>,
        idx: Arc<Expr>,
        returns_scalar: bool,
    },
    SortBy {
        expr: Arc<Expr>,
        by: Vec<Expr>,
        sort_options: SortMultipleOptions,
    },
    Agg(AggExpr),
    Ternary {
        predicate: Arc<Expr>,
        truthy: Arc<Expr>,
        falsy: Arc<Expr>,
    },
    Function {
        input: Vec<Expr>,
        function: FunctionExpr,
    },
    Explode {
        input: Arc<Expr>,
        skip_empty: bool,
    },
    Filter {
        input: Arc<Expr>,
        by: Arc<Expr>,
    },
    Window {
        function: Arc<Expr>,
        partition_by: Vec<Expr>,
        order_by: Option<(Arc<Expr>, SortOptions)>,
        options: WindowType,
    },
    Slice {
        input: Arc<Expr>,
        offset: Arc<Expr>,
        length: Arc<Expr>,
    },
    KeepName(Arc<Expr>),
    Len,
    Field(Arc<[PlSmallStr]>),
    AnonymousFunction {
        input: Vec<Expr>,
        function: LazySerde<SpecialEq<Arc<dyn AnonymousColumnsUdf>>>,
        options: FunctionOptions,
        fmt_str: Box<PlSmallStr>,
    },
    Eval {
        expr: Arc<Expr>,
        evaluation: Arc<Expr>,
        variant: EvalVariant,
    },
    SubPlan(SpecialEq<Arc<DslPlan>>, Vec<String>),
    RenameAlias {
        function: RenameAliasFn,
        expr: Arc<Expr>,
    },
}
```

Available on **crate feature `lazy`** only.

Expand description

Expressions that can be used in various contexts.

Queries consist of multiple expressions. When using the polars lazy API, don’t construct an `Expr` directly; instead, create one using the functions in the `polars_lazy::dsl` module. See that module’s docs for more info.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Alias" class="anchor">§</a>

### Alias(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Column" class="anchor">§</a>

### Column(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Selector" class="anchor">§</a>

### Selector(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Literal" class="anchor">§</a>

### Literal(<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.DataTypeFunction" class="anchor">§</a>

### DataTypeFunction(<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeFunction.html" class="enum" title="enum polars::prelude::DataTypeFunction">DataTypeFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.BinaryExpr" class="anchor">§</a>

### BinaryExpr

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.BinaryExpr.field.left" class="anchor field">§</a>`left: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.BinaryExpr.field.op" class="anchor field">§</a>`op: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator"><code>Operator</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.BinaryExpr.field.right" class="anchor field">§</a>`right: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Cast" class="anchor">§</a>

### Cast

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Cast.field.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Cast.field.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Cast.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions"><code>CastOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Sort" class="anchor">§</a>

### Sort

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Sort.field.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Sort.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions"><code>SortOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Gather" class="anchor">§</a>

### Gather

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Gather.field.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Gather.field.idx" class="anchor field">§</a>`idx: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Gather.field.returns_scalar" class="anchor field">§</a>`returns_scalar: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.SortBy" class="anchor">§</a>

### SortBy

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.SortBy.field.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.SortBy.field.by" class="anchor field">§</a>`by: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.SortBy.field.sort_options" class="anchor field">§</a>`sort_options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions"><code>SortMultipleOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Agg" class="anchor">§</a>

### Agg(<a href="https://docs.rs/polars/latest/polars/prelude/enum.AggExpr.html" class="enum" title="enum polars::prelude::AggExpr">AggExpr</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Ternary" class="anchor">§</a>

### Ternary

A ternary operation if true then “foo” else “bar”

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Ternary.field.predicate" class="anchor field">§</a>`predicate: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Ternary.field.truthy" class="anchor field">§</a>`truthy: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Ternary.field.falsy" class="anchor field">§</a>`falsy: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Function" class="anchor">§</a>

### Function

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Function.field.input" class="anchor field">§</a>`input: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

function arguments

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Function.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr"><code>FunctionExpr</code></a>

function to apply

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Explode" class="anchor">§</a>

### Explode

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Explode.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Explode.field.skip_empty" class="anchor field">§</a>`skip_empty: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Filter" class="anchor">§</a>

### Filter

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Filter.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Filter.field.by" class="anchor field">§</a>`by: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Window" class="anchor">§</a>

### Window

Polars flavored window functions.

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Window.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

Also has the input. i.e. avg(“foo”)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Window.field.partition_by" class="anchor field">§</a>`partition_by: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Window.field.order_by" class="anchor field">§</a>`order_by: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions"><code>SortOptions</code></a>`)>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Window.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.WindowType.html" class="enum" title="enum polars::prelude::WindowType"><code>WindowType</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Slice" class="anchor">§</a>

### Slice

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Slice.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Slice.field.offset" class="anchor field">§</a>`offset: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

length is not yet known so we accept negative offsets

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Slice.field.length" class="anchor field">§</a>`length: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.KeepName" class="anchor">§</a>

### KeepName(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>)

Set root name as Alias

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Len" class="anchor">§</a>

### Len

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Field" class="anchor">§</a>

### Field(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\]\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.AnonymousFunction" class="anchor">§</a>

### AnonymousFunction

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.AnonymousFunction.field.input" class="anchor field">§</a>`input: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

function arguments

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.AnonymousFunction.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.LazySerde.html" class="enum" title="enum polars::prelude::LazySerde"><code>LazySerde</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq"><code>SpecialEq</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf"><code>AnonymousColumnsUdf</code></a>`>>>`

function to apply

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.AnonymousFunction.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/options/struct.FunctionOptions.html" class="struct" title="struct polars_plan::plans::options::FunctionOptions"><code>FunctionOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.AnonymousFunction.field.fmt_str" class="anchor field">§</a>`fmt_str: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`>`

used for formatting

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Eval" class="anchor">§</a>

### Eval

Evaluates the `evaluation` expression on the output of the `expr`.

Consequently, `expr` is an input and `evaluation` is not and needs a different schema.

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Eval.field.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Eval.field.evaluation" class="anchor field">§</a>`evaluation: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Eval.field.variant" class="anchor field">§</a>`variant: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.EvalVariant.html" class="enum" title="enum polars::prelude::EvalVariant"><code>EvalVariant</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.SubPlan" class="anchor">§</a>

### SubPlan(<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq">SpecialEq</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.RenameAlias" class="anchor">§</a>

### RenameAlias

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.RenameAlias.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.RenameAliasFn.html" class="enum" title="enum polars::prelude::RenameAliasFn"><code>RenameAliasFn</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.RenameAlias.field.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.floor_div" class="fn">floor_div</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Floor divide `self` by `rhs`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.pow" class="fn">pow</a>\<E\>(self, exponent: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Raise expression to the power `exponent`

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sqrt" class="fn">sqrt</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the square root of the given expression

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cbrt" class="fn">cbrt</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the cube root of the given expression

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_count_ones" class="fn">bitwise_count_ones</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Evaluate the number of set bits.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_count_zeros" class="fn">bitwise_count_zeros</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Evaluate the number of unset bits.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_leading_ones" class="fn">bitwise_leading_ones</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Evaluate the number most-significant set bits before seeing an unset bit.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_leading_zeros" class="fn">bitwise_leading_zeros</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Evaluate the number most-significant unset bits before seeing an set bit.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_trailing_ones" class="fn">bitwise_trailing_ones</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Evaluate the number least-significant set bits before seeing an unset bit.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_trailing_zeros" class="fn">bitwise_trailing_zeros</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Evaluate the number least-significant unset bits before seeing an set bit.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_and" class="fn">bitwise_and</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Perform an aggregation of bitwise ANDs

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_or" class="fn">bitwise_or</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Perform an aggregation of bitwise ORs

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.bitwise_xor" class="fn">bitwise_xor</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Perform an aggregation of bitwise XORs

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.to_field" class="fn">to_field</a>(&self, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get Field result of the expression. The schema is the input data.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.into_selector" class="fn">into_selector</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.try_into_selector" class="fn">try_into_selector</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.extract_usize" class="fn">extract_usize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract a constant usize from an expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.extract_i64" class="fn">extract_i64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_unary" class="fn">map_unary</a>(self, function: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_binary" class="fn">map_binary</a>(self, function: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>\>, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_ternary" class="fn">map_ternary</a>( self, function: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>\>, arg1: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, arg2: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.try_map_n_ary" class="fn">try_map_n_ary</a>( self, function: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>\>, exprs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_n_ary" class="fn">map_n_ary</a>( self, function: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>\>, exprs: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.n_ary" class="fn">n_ary</a>(function: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>\>, input: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr-3" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.shuffle" class="fn">shuffle</a>(self, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sample_n" class="fn">sample_n</a>( self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sample_frac" class="fn">sample_frac</a>( self, frac: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr-4" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.std" class="fn">std</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Standard deviation of the values of the Series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.var" class="fn">var</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Variance of the values of the Series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.min" class="fn">min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to minimal value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.max" class="fn">max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to maximum value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.nan_min" class="fn">nan_min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to minimal value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.nan_max" class="fn">nan_max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to maximum value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.mean" class="fn">mean</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to the mean value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.median" class="fn">median</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to the median value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sum" class="fn">sum</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reduce groups to the sum of all the values.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr-5" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.eq" class="fn">eq</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Compare `Expr` with other `Expr` on equality.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.eq_missing" class="fn">eq_missing</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Compare `Expr` with other `Expr` on equality where `None == None`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.neq" class="fn">neq</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Compare `Expr` with other `Expr` on non-equality.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.neq_missing" class="fn">neq_missing</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Compare `Expr` with other `Expr` on non-equality where `None == None`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.lt" class="fn">lt</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if `Expr` \< `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.gt" class="fn">gt</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if `Expr` \> `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.gt_eq" class="fn">gt_eq</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if `Expr` \>= `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.lt_eq" class="fn">lt_eq</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if `Expr` \<= `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.not" class="fn">not</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Negate `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.alias" class="fn">alias</a>\<S\>(self, name: S) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Rename Column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_null" class="fn">is_null</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Run is_null operation on `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_not_null" class="fn">is_not_null</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Run is_not_null operation on `Expr`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.drop_nulls" class="fn">drop_nulls</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Drop null values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.drop_nans" class="fn">drop_nans</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Drop NaN values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.n_unique" class="fn">n_unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the number of unique values in the groups.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.first" class="fn">first</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the first value in the group.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.last" class="fn">last</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the last value in the group.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.implode" class="fn">implode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

GroupBy the group to a Series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.quantile" class="fn">quantile</a>(self, quantile: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the quantile per group.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.agg_groups" class="fn">agg_groups</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the group indexes of the group by operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.flatten" class="fn">flatten</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Alias for `explode`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.explode" class="fn">explode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Explode the String/List column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.slice" class="fn">slice</a>\<E, F\>(self, offset: E, length: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>, F: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Slice the Series. `offset` may be negative.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.append" class="fn">append</a>\<E\>(self, other: E, upcast: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Append expressions. This is done by adding the chunks of `other` to this [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rechunk" class="fn">rechunk</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Collect all chunks into a single chunk before continuing.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.head" class="fn">head</a>(self, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the first `n` elements of the Expr result.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.tail" class="fn">tail</a>(self, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the last `n` elements of the Expr result.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.unique" class="fn">unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get unique values of this expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.unique_stable" class="fn">unique_stable</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get unique values of this expression, while maintaining order. This requires more work than [`Expr::unique`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.unique "method polars::prelude::Expr::unique").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.arg_unique" class="fn">arg_unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the first index of unique values of this expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.arg_min" class="fn">arg_min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the index value that has the minimum value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.arg_max" class="fn">arg_max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the index value that has the maximum value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.arg_sort" class="fn">arg_sort</a>(self, descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_last: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the index values that would sort this expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.strict_cast" class="fn">strict_cast</a>(self, dtype: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Cast expression to another data type. Throws an error if conversion had overflows. Returns an Error if cast is invalid on rows after predicates are pushed down.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cast" class="fn">cast</a>(self, dtype: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Cast expression to another data type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cast_with_options" class="fn">cast_with_options</a>( self, dtype: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>\>, cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Cast expression to another data type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.gather" class="fn">gather</a>\<E\>(self, idx: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Take the values by idx.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.get" class="fn">get</a>\<E\>(self, idx: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Take the values by a single index.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sort" class="fn">sort</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Sort with given options.

##### <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#example" class="doc-anchor">§</a>Example

``` rust
let lf = df! {
   "a" => [Some(5), Some(4), Some(3), Some(2), None]
}?
.lazy();

let sorted = lf
    .select(
        vec![col("a").sort(SortOptions::default())],
    )
    .collect()?;

assert_eq!(
    sorted,
    df! {
        "a" => [None, Some(2), Some(3), Some(4), Some(5)]
    }?
);
```

See [`SortOptions`](https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html "struct polars::prelude::SortOptions") for more options.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.reverse" class="fn">reverse</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reverse column

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map" class="fn">map</a>\<F, DT\>(self, function: F, output_type: DT) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Apply a function/closure once the logical plan get executed.

This function is very similar to [`Expr::apply`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply "method polars::prelude::Expr::apply"), but differs in how it handles aggregations.

- `map` should be used for operations that are independent of groups, e.g. `multiply * 2`, or `raise to the power`
- `apply` should be used for operations that work on a group of data. e.g. `sum`, `count`, etc.

It is the responsibility of the caller that the schema is correct by giving the correct output_type. If None given the output type of the input expr is used.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_with_fmt_str" class="fn">map_with_fmt_str</a>\<F, DT\>( self, function: F, output_type: DT, fmt_str: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.agg_with_fmt_str" class="fn">agg_with_fmt_str</a>\<F, DT\>( self, function: F, output_type: DT, fmt_str: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply_with_fmt_str" class="fn">apply_with_fmt_str</a>\<F, DT\>( self, function: F, output_type: DT, fmt_str: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_many" class="fn">map_many</a>\<F, DT\>( self, function: F, arguments: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\], output_type: DT, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Apply a function/closure once the logical plan get executed with many arguments.

See the [`Expr::map`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map "method polars::prelude::Expr::map") function for the differences between [`map`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map "method polars::prelude::Expr::map") and [`apply`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply "method polars::prelude::Expr::apply").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply" class="fn">apply</a>\<F, DT\>(self, function: F, output_type: DT) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Apply a function/closure over the groups. This should only be used in a group_by aggregation.

It is the responsibility of the caller that the schema is correct by giving the correct output_type. If None given the output type of the input expr is used.

This difference with [map](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map "method polars::prelude::Expr::map") is that `apply` will create a separate `Series` per group.

- `map` should be used for operations that are independent of groups, e.g. `multiply * 2`, or `raise to the power`
- `apply` should be used for operations that work on a group of data. e.g. `sum`, `count`, etc.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply_many" class="fn">apply_many</a>\<F, DT\>( self, function: F, arguments: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\], output_type: DT, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, DT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Apply a function/closure over the groups with many arguments. This should only be used in a group_by aggregation.

See the [`Expr::apply`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply "method polars::prelude::Expr::apply") function for the differences between [`map`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map "method polars::prelude::Expr::map") and [`apply`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply "method polars::prelude::Expr::apply").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_finite" class="fn">is_finite</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get mask of finite values if dtype is Float.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_infinite" class="fn">is_infinite</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get mask of infinite values if dtype is Float.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_nan" class="fn">is_nan</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get mask of NaN values if dtype is Float.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_not_nan" class="fn">is_not_nan</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get inverse mask of NaN values if dtype is Float.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.shift" class="fn">shift</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Shift the values in the array by some period. See [the eager implementation](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift "method polars::prelude::SeriesTrait::shift").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.shift_and_fill" class="fn">shift_and_fill</a>\<E, IE\>(self, n: E, fill_value: IE) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Shift the values in the array by some period and fill the resulting empty values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cumulative_eval" class="fn">cumulative_eval</a>(self, evaluation: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, min_samples: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Cumulatively count values from 0 to len.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cum_count" class="fn">cum_count</a>(self, reverse: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Cumulatively count values from 0 to len.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cum_sum" class="fn">cum_sum</a>(self, reverse: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get an array with the cumulative sum computed at every element.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cum_prod" class="fn">cum_prod</a>(self, reverse: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get an array with the cumulative product computed at every element.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cum_min" class="fn">cum_min</a>(self, reverse: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get an array with the cumulative min computed at every element.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cum_max" class="fn">cum_max</a>(self, reverse: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get an array with the cumulative max computed at every element.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.product" class="fn">product</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the product aggregation of an expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.round" class="fn">round</a>(self, decimals: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, mode: <a href="https://docs.rs/polars/latest/polars/prelude/enum.RoundMode.html" class="enum" title="enum polars::prelude::RoundMode">RoundMode</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Round underlying floating point array to given decimal numbers.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.round_sig_figs" class="fn">round_sig_figs</a>(self, digits: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Round to a number of significant figures.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.floor" class="fn">floor</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Floor underlying floating point array to the lowest integers smaller or equal to the float value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.pi" class="fn">pi</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Constant Pi

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.ceil" class="fn">ceil</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Ceil underlying floating point array to the highest integers smaller or equal to the float value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.clip" class="fn">clip</a>(self, min: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, max: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Clip underlying values to a set boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.clip_max" class="fn">clip_max</a>(self, max: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Clip underlying values to a set boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.clip_min" class="fn">clip_min</a>(self, min: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Clip underlying values to a set boundary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.abs" class="fn">abs</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Convert all values to their absolute/positive value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.over" class="fn">over</a>\<E, IE\>(self, partition_by: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[IE]</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Apply window function over a subgroup. This is similar to a group_by + aggregation + self join. Or similar to [window functions in Postgres](https://www.postgresql.org/docs/9.1/tutorial-window.html).

##### <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#example-1" class="doc-anchor">§</a>Example

``` rust
#[macro_use] extern crate polars_core;
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example() -> PolarsResult<()> {
    let df = df! {
            "groups" => &[1, 1, 2, 2, 1, 2, 3, 3, 1],
            "values" => &[1, 2, 3, 4, 5, 6, 7, 8, 8]
        }?;

    let out = df
     .lazy()
     .select(&[
         col("groups"),
         sum("values").over([col("groups")]),
     ])
     .collect()?;
    println!("{}", &out);
    Ok(())
}
```

Outputs:

``` text
╭────────┬────────╮
│ groups ┆ values │
│ ---    ┆ ---    │
│ i32    ┆ i32    │
╞════════╪════════╡
│ 1      ┆ 16     │
│ 1      ┆ 16     │
│ 2      ┆ 13     │
│ 2      ┆ 13     │
│ …      ┆ …      │
│ 1      ┆ 16     │
│ 2      ┆ 13     │
│ 3      ┆ 15     │
│ 3      ┆ 15     │
│ 1      ┆ 16     │
╰────────┴────────╯
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.over_with_options" class="fn">over_with_options</a>\<E, IE\>( self, partition_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>, order_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(E, <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>)\>, options: <a href="https://docs.rs/polars/latest/polars/prelude/enum.WindowMapping.html" class="enum" title="enum polars::prelude::WindowMapping">WindowMapping</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[IE]</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling" class="fn">rolling</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingGroupOptions.html" class="struct" title="struct polars::prelude::RollingGroupOptions">RollingGroupOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.fill_null" class="fn">fill_null</a>\<E\>(self, fill_value: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Replace the null values by a value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.fill_null_with_strategy" class="fn">fill_null_with_strategy</a>(self, strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.FillNullStrategy.html" class="enum" title="enum polars::prelude::FillNullStrategy">FillNullStrategy</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.fill_nan" class="fn">fill_nan</a>\<E\>(self, fill_value: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Replace the floating point `NaN` values by a value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.count" class="fn">count</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Count the values of the Series or Get counts of the group by operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.len" class="fn">len</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_between" class="fn">is_between</a>\<E\>(self, lower: E, upper: E, closed: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedInterval.html" class="enum" title="enum polars::prelude::ClosedInterval">ClosedInterval</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_close" class="fn">is_close</a>\<E\>( self, expr: E, abs_tol: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, rel_tol: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, nans_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check whether floating point values are close to each other.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.approx_n_unique" class="fn">approx_n_unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the approximate count of unique values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.and" class="fn">and</a>\<E\>(self, expr: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Bitwise “and” operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.xor" class="fn">xor</a>\<E\>(self, expr: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Bitwise “xor” operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.or" class="fn">or</a>\<E\>(self, expr: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Bitwise “or” operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.logical_or" class="fn">logical_or</a>\<E\>(self, expr: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Logical “or” operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.logical_and" class="fn">logical_and</a>\<E\>(self, expr: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Logical “and” operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.filter" class="fn">filter</a>\<E\>(self, predicate: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Filter a single column.

Should be used in aggregation context. If you want to filter on a DataFrame level, use `LazyFrame::filter`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_in" class="fn">is_in</a>\<E\>(self, other: E, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if the values of the left expression are in the lists of the right expr.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sort_by" class="fn">sort_by</a>\<E, IE\>(self, by: E, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[IE]</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Sort this column by the ordering of another column evaluated from given expr. Can also be used in a group_by context to sort the groups.

##### <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#example-2" class="doc-anchor">§</a>Example

``` rust
let lf = df! {
    "a" => [1, 2, 3, 4, 5],
    "b" => [5, 4, 3, 2, 1]
}?.lazy();

let sorted = lf
    .select(
        vec![col("a").sort_by(col("b"), SortOptions::default())],
    )
    .collect()?;

assert_eq!(
    sorted,
    df! { "a" => [5, 4, 3, 2, 1] }?
);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.repeat_by" class="fn">repeat_by</a>\<E\>(self, by: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Repeat the column `n` times, where `n` is determined by the values in `by`. This yields an `Expr` of dtype `List`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_first_distinct" class="fn">is_first_distinct</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get a mask of the first unique value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.is_last_distinct" class="fn">is_last_distinct</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get a mask of the last unique value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.dot" class="fn">dot</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Compute the dot/inner product between two expressions.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.mode" class="fn">mode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the mode(s) of this column. This is the most occurring value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.interpolate" class="fn">interpolate</a>(self, method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.InterpolationMethod.html" class="enum" title="enum polars::prelude::InterpolationMethod">InterpolationMethod</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Interpolate intermediate values. Nulls at the beginning and end of the series remain null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.interpolate_by" class="fn">interpolate_by</a>(self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Interpolate intermediate values. Nulls at the beginning and end of the series remain null. The `by` column provides the x-coordinates for interpolation and must not contain nulls.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_min_by" class="fn">rolling_min_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling minimum based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_max_by" class="fn">rolling_max_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling maximum based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_mean_by" class="fn">rolling_mean_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling mean based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_sum_by" class="fn">rolling_sum_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling sum based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_quantile_by" class="fn">rolling_quantile_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling quantile based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_var_by" class="fn">rolling_var_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling variance based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_std_by" class="fn">rolling_std_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling std-dev based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_median_by" class="fn">rolling_median_by</a>( self, by: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling median based on another column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_min" class="fn">rolling_min</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling minimum.

See: \[`RollingAgg::rolling_min`\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_max" class="fn">rolling_max</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling maximum.

See: \[`RollingAgg::rolling_max`\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_mean" class="fn">rolling_mean</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling mean.

See: \[`RollingAgg::rolling_mean`\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_sum" class="fn">rolling_sum</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling sum.

See: \[`RollingAgg::rolling_sum`\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_median" class="fn">rolling_median</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling median.

See: \[`RollingAgg::rolling_median`\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_quantile" class="fn">rolling_quantile</a>( self, method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling quantile.

See: \[`RollingAgg::rolling_quantile`\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_var" class="fn">rolling_var</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling variance.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_std" class="fn">rolling_std</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling std-dev.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_skew" class="fn">rolling_skew</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling skew.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_kurtosis" class="fn">rolling_kurtosis</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a rolling skew.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rolling_map" class="fn">rolling_map</a>( self, f: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback">PlanCallback</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Apply a custom function over a rolling/ moving window of the array. This has quite some dynamic dispatch, so prefer rolling_min, max, mean, sum over this.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.peak_min" class="fn">peak_min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.peak_max" class="fn">peak_max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rank" class="fn">rank</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RankOptions.html" class="struct" title="struct polars::prelude::RankOptions">RankOptions</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Assign ranks to data, dealing with ties appropriately.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.replace" class="fn">replace</a>\<E\>(self, old: E, new: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Replace the given values with other values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.replace_strict" class="fn">replace_strict</a>\<E\>( self, old: E, new: E, default: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>, return_dtype: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Replace the given values with other values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rle" class="fn">rle</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the lengths of runs of identical values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rle_id" class="fn">rle_id</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Similar to `rle`, but maps values to run IDs.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.diff" class="fn">diff</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, null_behavior: <a href="https://docs.rs/polars/latest/polars/series/ops/enum.NullBehavior.html" class="enum" title="enum polars::series::ops::NullBehavior">NullBehavior</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Calculate the n-th discrete difference between values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.skew" class="fn">skew</a>(self, bias: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the sample skewness of a data set.

For normally distributed data, the skewness should be about zero. For uni-modal continuous distributions, a skewness value greater than zero means that there is more weight in the right tail of the distribution. The function `skewtest` can be used to determine if the skewness value is close enough to zero, statistically speaking.

see: [scipy](https://github.com/scipy/scipy/blob/47bb6febaa10658c72962b9615d5d5aa2513fa3a/scipy/stats/stats.py#L1024)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.kurtosis" class="fn">kurtosis</a>(self, fisher: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, bias: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the kurtosis (Fisher or Pearson).

Kurtosis is the fourth central moment divided by the square of the variance. If Fisher’s definition is used, then 3.0 is subtracted from the result to give 0.0 for a normal distribution. If bias is False then the kurtosis is calculated using k statistics to eliminate bias coming from biased moment estimators.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.upper_bound" class="fn">upper_bound</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get maximal value that could be hold by this dtype.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.lower_bound" class="fn">lower_bound</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get minimal value that could be hold by this dtype.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.reshape" class="fn">reshape</a>(self, dimensions: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.any" class="fn">any</a>(self, ignore_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Returns whether any of the values in the column are `true`.

If `ignore_nulls` is `False`, [Kleene logic](https://en.wikipedia.org/wiki/Three-valued_logic) is used to deal with nulls: if the column contains any null values and no `true` values, the output is null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.all" class="fn">all</a>(self, ignore_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Returns whether all values in the column are `true`.

If `ignore_nulls` is `False`, [Kleene logic](https://en.wikipedia.org/wiki/Three-valued_logic) is used to deal with nulls: if the column contains any null values and no `false` values, the output is null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.value_counts" class="fn">value_counts</a>( self, sort: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, parallel: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, normalize: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Count all unique values and create a struct mapping value to count. (Note that it is better to turn parallel off in the aggregation context). The name of the struct field with the counts is given by the parameter `name`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.unique_counts" class="fn">unique_counts</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Returns a count of the unique values in the order of appearance. This method differs from [`Expr::value_counts`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.value_counts "method polars::prelude::Expr::value_counts") in that it does not return the values, only the counts and might be faster.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.null_count" class="fn">null_count</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the null count of the column/group.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.set_sorted_flag" class="fn">set_sorted_flag</a>(self, sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Set this `Series` as `sorted` so that downstream code can use fast paths for sorted arrays.

##### <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#warning" class="doc-anchor">§</a>Warning

This can lead to incorrect results if this `Series` is not sorted!! Use with care!

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.to_physical" class="fn">to_physical</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.gather_every" class="fn">gather_every</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.extend_constant" class="fn">extend_constant</a>(self, value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.str" class="fn">str</a>(self) -\> <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/dsl/string/struct.StringNameSpace.html" class="struct" title="struct polars_plan::dsl::string::StringNameSpace">StringNameSpace</a>

Get the [`string::StringNameSpace`](https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/dsl/string/struct.StringNameSpace.html "struct polars_plan::dsl::string::StringNameSpace")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.binary" class="fn">binary</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html" class="struct" title="struct polars::prelude::binary::BinaryNameSpace">BinaryNameSpace</a>

Get the [`binary::BinaryNameSpace`](https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html "struct polars::prelude::binary::BinaryNameSpace")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.dt" class="fn">dt</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html" class="struct" title="struct polars::prelude::dt::DateLikeNameSpace">DateLikeNameSpace</a>

Get the [`dt::DateLikeNameSpace`](https://docs.rs/polars/latest/polars/prelude/dt/struct.DateLikeNameSpace.html "struct polars::prelude::dt::DateLikeNameSpace")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.list" class="fn">list</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html" class="struct" title="struct polars::prelude::ListNameSpace">ListNameSpace</a>

Get the [`list::ListNameSpace`](https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html "struct polars::prelude::ListNameSpace")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.name" class="fn">name</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html" class="struct" title="struct polars::prelude::ExprNameNameSpace">ExprNameNameSpace</a>

Get the [`name::ExprNameNameSpace`](https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html "struct polars::prelude::ExprNameNameSpace")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.arr" class="fn">arr</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html" class="struct" title="struct polars::prelude::ArrayNameSpace">ArrayNameSpace</a>

Get the [`array::ArrayNameSpace`](https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html "struct polars::prelude::ArrayNameSpace").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.cat" class="fn">cat</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalNameSpace.html" class="struct" title="struct polars::prelude::CategoricalNameSpace">CategoricalNameSpace</a>

Get the [`CategoricalNameSpace`](https://docs.rs/polars/latest/polars/prelude/struct.CategoricalNameSpace.html "struct polars::prelude::CategoricalNameSpace").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.struct_" class="fn">struct_</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html" class="struct" title="struct polars::prelude::StructNameSpace">StructNameSpace</a>

Get the [`struct_::StructNameSpace`](https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html "struct polars::prelude::StructNameSpace").

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Expr-6" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.nodes" class="fn">nodes</a>\<'a\>(&'a self, container: &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.nodes_owned" class="fn">nodes_owned</a>(self, container: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_expr" class="fn">map_expr</a>\<F\>(self, f: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.try_map_expr" class="fn">try_map_expr</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Add-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.add" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-AsRef%3CExpr%3E-for-AggExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AggExpr.html" class="enum" title="enum polars::prelude::AggExpr">AggExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Clone-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Debug-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Default-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Deserialize%3C&#39;de%3E-for-Expr" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Display-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Div-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.div" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3C%26str%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3CAggExpr%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AggExpr.html" class="enum" title="enum polars::prelude::AggExpr">AggExpr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(agg: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AggExpr.html" class="enum" title="enum polars::prelude::AggExpr">AggExpr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3CSelector%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cbool%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cf32%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cf64%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Ci16%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Ci32%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Ci64%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Ci8%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cu16%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cu32%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cu64%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-From%3Cu8%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Hash-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-IntoIterator-for-%26Expr" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/iterator/struct.ExprIter.html" class="struct" title="struct polars_plan::plans::iterator::ExprIter">ExprIter</a>\<'a\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Mul-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.mul" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Neg-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html" class="trait" title="trait core::ops::arith::Neg">Neg</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.neg" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg" class="fn">neg</a>(self) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html" class="trait" title="trait core::ops::arith::Neg">Neg</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Neg::Output">Output</a>

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-PartialEq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Rem-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rem" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Serialize-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Sub-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-TreeWalker-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html" class="trait" title="trait polars_plan::plans::visitor::visitors::TreeWalker">TreeWalker</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#associatedtype.Arena" class="anchor">§</a>

#### type <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype">Arena</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.apply_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#tymethod.apply_children" class="fn">apply_children</a>\<F\>( &self, op: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut F</a>, arena: &\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html" class="trait" title="trait polars_plan::plans::visitor::visitors::TreeWalker">TreeWalker</a>\>::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/enum.VisitRecursion.html" class="enum" title="enum polars_plan::plans::visitor::VisitRecursion">VisitRecursion</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, &\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html" class="trait" title="trait polars_plan::plans::visitor::visitors::TreeWalker">TreeWalker</a>\>::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/enum.VisitRecursion.html" class="enum" title="enum polars_plan::plans::visitor::VisitRecursion">VisitRecursion</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#tymethod.map_children" class="fn">map_children</a>\<F\>( self, f: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut F</a>, \_arena: &mut \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html" class="trait" title="trait polars_plan::plans::visitor::visitors::TreeWalker">TreeWalker</a>\>::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, &mut \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a> as <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html" class="trait" title="trait polars_plan::plans::visitor::visitors::TreeWalker">TreeWalker</a>\>::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#method.visit" class="fn">visit</a>\<V\>( &self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, arena: &Self::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/enum.VisitRecursion.html" class="enum" title="enum polars_plan::plans::visitor::VisitRecursion">VisitRecursion</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where V: <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.Visitor.html" class="trait" title="trait polars_plan::plans::visitor::visitors::Visitor">Visitor</a>\<Node = Self, Arena = Self::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>\>,

Walks all nodes in depth-first-order.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#method.rewrite" class="fn">rewrite</a>\<R\>( self, rewriter: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>, arena: &mut Self::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where R: <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.RewritingVisitor.html" class="trait" title="trait polars_plan::plans::visitor::visitors::RewritingVisitor">RewritingVisitor</a>\<Node = Self, Arena = Self::<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/visitor/visitors/trait.TreeWalker.html#associatedtype.Arena" class="associatedtype" title="type polars_plan::plans::visitor::visitors::TreeWalker::Arena">Arena</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-Eq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#impl-StructuralPartialEq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#blanket-implementations" class="anchor">§</a>
