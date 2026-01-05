# Enum DslPlan Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/plan.rs.html#27" class="src">Source</a>

``` rust
pub enum DslPlan {
Show 20 variants    Filter {
        input: Arc<DslPlan>,
        predicate: Expr,
    },
    Cache {
        input: Arc<DslPlan>,
        id: UniqueId,
    },
    Scan {
        sources: ScanSources,
        unified_scan_args: Box<UnifiedScanArgs>,
        scan_type: Box<FileScanDsl>,
        cached_ir: Arc<Mutex<Option<IR>>>,
    },
    DataFrameScan {
        df: Arc<DataFrame>,
        schema: Arc<Schema<DataType>>,
    },
    Select {
        expr: Vec<Expr>,
        input: Arc<DslPlan>,
        options: ProjectionOptions,
    },
    GroupBy {
        input: Arc<DslPlan>,
        keys: Vec<Expr>,
        aggs: Vec<Expr>,
        maintain_order: bool,
        options: Arc<GroupbyOptions>,
        apply: Option<(PlanCallback<DataFrame, DataFrame>, Arc<Schema<DataType>>)>,
    },
    Join {
        input_left: Arc<DslPlan>,
        input_right: Arc<DslPlan>,
        left_on: Vec<Expr>,
        right_on: Vec<Expr>,
        predicates: Vec<Expr>,
        options: Arc<JoinOptions>,
    },
    HStack {
        input: Arc<DslPlan>,
        exprs: Vec<Expr>,
        options: ProjectionOptions,
    },
    MatchToSchema {
        input: Arc<DslPlan>,
        match_schema: Arc<Schema<DataType>>,
        per_column: Arc<[MatchToSchemaPerColumn]>,
        extra_columns: ExtraColumnsPolicy,
    },
    PipeWithSchema {
        input: Arc<DslPlan>,
        callback: PlanCallback<(DslPlan, Schema<DataType>), DslPlan>,
    },
    Distinct {
        input: Arc<DslPlan>,
        options: DistinctOptionsDSL,
    },
    Sort {
        input: Arc<DslPlan>,
        by_column: Vec<Expr>,
        slice: Option<(i64, usize)>,
        sort_options: SortMultipleOptions,
    },
    Slice {
        input: Arc<DslPlan>,
        offset: i64,
        len: u32,
    },
    MapFunction {
        input: Arc<DslPlan>,
        function: DslFunction,
    },
    Union {
        inputs: Vec<DslPlan>,
        args: UnionArgs,
    },
    HConcat {
        inputs: Vec<DslPlan>,
        options: HConcatOptions,
    },
    ExtContext {
        input: Arc<DslPlan>,
        contexts: Vec<DslPlan>,
    },
    Sink {
        input: Arc<DslPlan>,
        payload: SinkType,
    },
    SinkMultiple {
        inputs: Vec<DslPlan>,
    },
    IR {
        dsl: Arc<DslPlan>,
        version: u32,
        node: Option<Node>,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Filter" class="anchor">§</a>

### Filter

Filter on a boolean mask

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Filter.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Filter.field.predicate" class="anchor field">§</a>`predicate: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Cache" class="anchor">§</a>

### Cache

Cache the input at this point in the LP

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Cache.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Cache.field.id" class="anchor field">§</a>`id: `<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/unique_id/struct.UniqueId.html" class="struct" title="struct polars_utils::unique_id::UniqueId"><code>UniqueId</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Scan" class="anchor">§</a>

### Scan

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Scan.field.sources" class="anchor field">§</a>`sources: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources"><code>ScanSources</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Scan.field.unified_scan_args" class="anchor field">§</a>`unified_scan_args: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs"><code>UnifiedScanArgs</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Scan.field.scan_type" class="anchor field">§</a>`scan_type: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl"><code>FileScanDsl</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Scan.field.cached_ir" class="anchor field">§</a>`cached_ir: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/sync/poison/mutex/struct.Mutex.html" class="struct" title="struct std::sync::poison::mutex::Mutex"><code>Mutex</code></a>`<`<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html" class="enum" title="enum polars_plan::plans::ir::IR"><code>IR</code></a>`>>>`

Local use cases often repeatedly collect the same `LazyFrame` (e.g. in interactive notebook use-cases), so we cache the IR conversion here, as the path expansion can be quite slow (especially for cloud paths). We don’t have the arena, as this is always a source node.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.DataFrameScan" class="anchor">§</a>

### DataFrameScan

In memory DataFrame

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.DataFrameScan.field.df" class="anchor field">§</a>`df: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame"><code>DataFrame</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.DataFrameScan.field.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Select" class="anchor">§</a>

### Select

Polars’ `select` operation, this can mean projection, but also full data access.

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Select.field.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Select.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Select.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/options/struct.ProjectionOptions.html" class="struct" title="struct polars_plan::plans::options::ProjectionOptions"><code>ProjectionOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy" class="anchor">§</a>

### GroupBy

Groupby aggregation

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy.field.keys" class="anchor field">§</a>`keys: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy.field.aggs" class="anchor field">§</a>`aggs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy.field.maintain_order" class="anchor field">§</a>`maintain_order: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupbyOptions.html" class="struct" title="struct polars::prelude::GroupbyOptions"><code>GroupbyOptions</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.GroupBy.field.apply" class="anchor field">§</a>`apply: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback"><code>PlanCallback</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame"><code>DataFrame</code></a>`, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame"><code>DataFrame</code></a>`>, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>)>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join" class="anchor">§</a>

### Join

Join operation

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join.field.input_left" class="anchor field">§</a>`input_left: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join.field.input_right" class="anchor field">§</a>`input_right: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join.field.left_on" class="anchor field">§</a>`left_on: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join.field.right_on" class="anchor field">§</a>`right_on: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join.field.predicates" class="anchor field">§</a>`predicates: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Join.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptions.html" class="struct" title="struct polars::prelude::JoinOptions"><code>JoinOptions</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HStack" class="anchor">§</a>

### HStack

Adding columns to the table without a Join

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HStack.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HStack.field.exprs" class="anchor field">§</a>`exprs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HStack.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/options/struct.ProjectionOptions.html" class="struct" title="struct polars_plan::plans::options::ProjectionOptions"><code>ProjectionOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MatchToSchema" class="anchor">§</a>

### MatchToSchema

Match / Evolve into a schema

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MatchToSchema.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MatchToSchema.field.match_schema" class="anchor field">§</a>`match_schema: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>`

The schema to match to.

This is also always the output schema.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MatchToSchema.field.per_column" class="anchor field">§</a>`per_column: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<[`<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn"><code>MatchToSchemaPerColumn</code></a>`]>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MatchToSchema.field.extra_columns" class="anchor field">§</a>`extra_columns: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ExtraColumnsPolicy.html" class="enum" title="enum polars::prelude::ExtraColumnsPolicy"><code>ExtraColumnsPolicy</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.PipeWithSchema" class="anchor">§</a>

### PipeWithSchema

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.PipeWithSchema.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.PipeWithSchema.field.callback" class="anchor field">§</a>`callback: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback"><code>PlanCallback</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`, `<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>), `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Distinct" class="anchor">§</a>

### Distinct

Remove duplicates from the table

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Distinct.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Distinct.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.DistinctOptionsDSL.html" class="struct" title="struct polars::prelude::DistinctOptionsDSL"><code>DistinctOptionsDSL</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sort" class="anchor">§</a>

### Sort

Sort the table

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sort.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sort.field.by_column" class="anchor field">§</a>`by_column: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sort.field.slice" class="anchor field">§</a>`slice: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`, `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`)>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sort.field.sort_options" class="anchor field">§</a>`sort_options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions"><code>SortMultipleOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Slice" class="anchor">§</a>

### Slice

Slice the table

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Slice.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Slice.field.offset" class="anchor field">§</a>`offset: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Slice.field.len" class="anchor field">§</a>`len: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MapFunction" class="anchor">§</a>

### MapFunction

A (User Defined) Function

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MapFunction.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.MapFunction.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/functions/dsl/enum.DslFunction.html" class="enum" title="enum polars_plan::plans::functions::dsl::DslFunction"><code>DslFunction</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Union" class="anchor">§</a>

### Union

Vertical concatenation

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Union.field.inputs" class="anchor field">§</a>`inputs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Union.field.args" class="anchor field">§</a>`args: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnionArgs.html" class="struct" title="struct polars::prelude::UnionArgs"><code>UnionArgs</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HConcat" class="anchor">§</a>

### HConcat

Horizontal concatenation of multiple plans

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HConcat.field.inputs" class="anchor field">§</a>`inputs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.HConcat.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.HConcatOptions.html" class="struct" title="struct polars::prelude::HConcatOptions"><code>HConcatOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.ExtContext" class="anchor">§</a>

### ExtContext

This allows expressions to access other tables

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.ExtContext.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.ExtContext.field.contexts" class="anchor field">§</a>`contexts: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sink" class="anchor">§</a>

### Sink

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sink.field.input" class="anchor field">§</a>`input: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.Sink.field.payload" class="anchor field">§</a>`payload: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkType.html" class="enum" title="enum polars::prelude::SinkType"><code>SinkType</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.SinkMultiple" class="anchor">§</a>

### SinkMultiple

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.SinkMultiple.field.inputs" class="anchor field">§</a>`inputs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.IR" class="anchor">§</a>

### IR

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.IR.field.dsl" class="anchor field">§</a>`dsl: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.IR.field.version" class="anchor field">§</a>`version: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#variant.IR.field.node" class="anchor field">§</a>`node: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Node.html" class="struct" title="struct polars_utils::arena::Node"><code>Node</code></a>`>`

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-DslPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.describe" class="fn">describe</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.describe_tree_format" class="fn">describe_tree_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.display" class="fn">display</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.to_alp" class="fn">to_alp</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/struct.IRPlan.html" class="struct" title="struct polars_plan::plans::ir::IRPlan">IRPlan</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.serialize_versioned" class="fn">serialize_versioned</a>\<W\>( &self, writer: W, ctx: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlanSerializationContext.html" class="struct" title="struct polars::prelude::PlanSerializationContext">PlanSerializationContext</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.deserialize_versioned" class="fn">deserialize_versioned</a>\<R\>(reader: R) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-DslPlan-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.compute_schema" class="fn">compute_schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the schema. This requires conversion to [`IR`](https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html "enum polars_plan::plans::ir::IR") and type-resolving.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-Clone-for-DslPlan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-Default-for-DslPlan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-Deserialize%3C&#39;de%3E-for-DslPlan" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-From%3CDslPlan%3E-for-DslBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DslBuilder.html" class="struct" title="struct polars::prelude::DslBuilder">DslBuilder</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(lp: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DslBuilder.html" class="struct" title="struct polars::prelude::DslBuilder">DslBuilder</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-From%3CDslPlan%3E-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(plan: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-IntoIterator-for-%26DslPlan" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = DslPlanIter\<'a\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#impl-Serialize-for-DslPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html#blanket-implementations" class="anchor">§</a>
