# Enum FunctionExpr Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/function_expr/mod.rs.html#75" class="src">Source</a>

``` rust
pub enum FunctionExpr {
Show 81 variants    ArrayExpr(ArrayFunction),
    BinaryExpr(BinaryFunction),
    Categorical(CategoricalFunction),
    ListExpr(ListFunction),
    StringExpr(StringFunction),
    StructExpr(StructFunction),
    TemporalExpr(TemporalFunction),
    Bitwise(BitwiseFunction),
    Boolean(BooleanFunction),
    Abs,
    Negate,
    NullCount,
    Pow(PowFunction),
    ArgWhere,
    Range(RangeFunction),
    FillNull,
    FillNullWithStrategy(FillNullStrategy),
    RollingExpr {
        function: RollingFunction,
        options: RollingOptionsFixedWindow,
    },
    RollingExprBy {
        function_by: RollingFunctionBy,
        options: RollingOptionsDynamicWindow,
    },
    Rechunk,
    Append {
        upcast: bool,
    },
    ShiftAndFill,
    Shift,
    DropNans,
    DropNulls,
    Mode,
    Skew(bool),
    Kurtosis(bool, bool),
    Reshape(Vec<ReshapeDimension>),
    RepeatBy,
    ArgUnique,
    ArgMin,
    ArgMax,
    ArgSort {
        descending: bool,
        nulls_last: bool,
    },
    Product,
    Rank {
        options: RankOptions,
        seed: Option<u64>,
    },
    Repeat,
    Clip {
        has_min: bool,
        has_max: bool,
    },
    AsStruct,
    CumCount {
        reverse: bool,
    },
    CumSum {
        reverse: bool,
    },
    CumProd {
        reverse: bool,
    },
    CumMin {
        reverse: bool,
    },
    CumMax {
        reverse: bool,
    },
    Reverse,
    ValueCounts {
        sort: bool,
        parallel: bool,
        name: PlSmallStr,
        normalize: bool,
    },
    UniqueCounts,
    ApproxNUnique,
    Coalesce,
    Diff(NullBehavior),
    Interpolate(InterpolationMethod),
    InterpolateBy,
    Unique(bool),
    Round {
        decimals: u32,
        mode: RoundMode,
    },
    RoundSF {
        digits: i32,
    },
    Floor,
    Ceil,
    UpperBound,
    LowerBound,
    ConcatExpr(bool),
    PeakMin,
    PeakMax,
    RLE,
    RLEID,
    ToPhysical,
    Random {
        method: RandomMethod,
        seed: Option<u64>,
    },
    SetSortedFlag(IsSorted),
    FoldHorizontal {
        callback: PlanCallback<(Series, Series), Series>,
        returns_scalar: bool,
        return_dtype: Option<DataTypeExpr>,
    },
    ReduceHorizontal {
        callback: PlanCallback<(Series, Series), Series>,
        returns_scalar: bool,
        return_dtype: Option<DataTypeExpr>,
    },
    CumReduceHorizontal {
        callback: PlanCallback<(Series, Series), Series>,
        returns_scalar: bool,
        return_dtype: Option<DataTypeExpr>,
    },
    CumFoldHorizontal {
        callback: PlanCallback<(Series, Series), Series>,
        returns_scalar: bool,
        return_dtype: Option<DataTypeExpr>,
        include_init: bool,
    },
    MaxHorizontal,
    MinHorizontal,
    SumHorizontal {
        ignore_nulls: bool,
    },
    MeanHorizontal {
        ignore_nulls: bool,
    },
    Replace,
    ReplaceStrict {
        return_dtype: Option<DataTypeExpr>,
    },
    GatherEvery {
        n: usize,
        offset: usize,
    },
    ExtendConstant,
    RowEncode(RowEncodingVariant),
    RowDecode(Vec<(PlSmallStr, DataTypeExpr)>, RowEncodingVariant),
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArrayExpr" class="anchor">§</a>

### ArrayExpr(<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.BinaryExpr" class="anchor">§</a>

### BinaryExpr(<a href="https://docs.rs/polars/latest/polars/prelude/enum.BinaryFunction.html" class="enum" title="enum polars::prelude::BinaryFunction">BinaryFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Categorical" class="anchor">§</a>

### Categorical(<a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalFunction.html" class="enum" title="enum polars::prelude::CategoricalFunction">CategoricalFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ListExpr" class="anchor">§</a>

### ListExpr(<a href="https://docs.rs/polars/latest/polars/prelude/enum.ListFunction.html" class="enum" title="enum polars::prelude::ListFunction">ListFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.StringExpr" class="anchor">§</a>

### StringExpr(<a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.StructExpr" class="anchor">§</a>

### StructExpr(<a href="https://docs.rs/polars/latest/polars/prelude/enum.StructFunction.html" class="enum" title="enum polars::prelude::StructFunction">StructFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.TemporalExpr" class="anchor">§</a>

### TemporalExpr(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Bitwise" class="anchor">§</a>

### Bitwise(<a href="https://docs.rs/polars/latest/polars/prelude/enum.BitwiseFunction.html" class="enum" title="enum polars::prelude::BitwiseFunction">BitwiseFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Boolean" class="anchor">§</a>

### Boolean(<a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Abs" class="anchor">§</a>

### Abs

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Negate" class="anchor">§</a>

### Negate

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.NullCount" class="anchor">§</a>

### NullCount

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Pow" class="anchor">§</a>

### Pow(<a href="https://docs.rs/polars/latest/polars/prelude/enum.PowFunction.html" class="enum" title="enum polars::prelude::PowFunction">PowFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgWhere" class="anchor">§</a>

### ArgWhere

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Range" class="anchor">§</a>

### Range(<a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.FillNull" class="anchor">§</a>

### FillNull

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.FillNullWithStrategy" class="anchor">§</a>

### FillNullWithStrategy(<a href="https://docs.rs/polars/latest/polars/prelude/enum.FillNullStrategy.html" class="enum" title="enum polars::prelude::FillNullStrategy">FillNullStrategy</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RollingExpr" class="anchor">§</a>

### RollingExpr

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RollingExpr.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.RollingFunction.html" class="enum" title="enum polars::prelude::RollingFunction"><code>RollingFunction</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RollingExpr.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow"><code>RollingOptionsFixedWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RollingExprBy" class="anchor">§</a>

### RollingExprBy

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RollingExprBy.field.function_by" class="anchor field">§</a>`function_by: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.RollingFunctionBy.html" class="enum" title="enum polars::prelude::RollingFunctionBy"><code>RollingFunctionBy</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RollingExprBy.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow"><code>RollingOptionsDynamicWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Rechunk" class="anchor">§</a>

### Rechunk

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Append" class="anchor">§</a>

### Append

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Append.field.upcast" class="anchor field">§</a>`upcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ShiftAndFill" class="anchor">§</a>

### ShiftAndFill

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Shift" class="anchor">§</a>

### Shift

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.DropNans" class="anchor">§</a>

### DropNans

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.DropNulls" class="anchor">§</a>

### DropNulls

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Mode" class="anchor">§</a>

### Mode

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Skew" class="anchor">§</a>

### Skew(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Kurtosis" class="anchor">§</a>

### Kurtosis(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Reshape" class="anchor">§</a>

### Reshape(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ReshapeDimension.html" class="enum" title="enum polars::prelude::ReshapeDimension">ReshapeDimension</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RepeatBy" class="anchor">§</a>

### RepeatBy

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgUnique" class="anchor">§</a>

### ArgUnique

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgMin" class="anchor">§</a>

### ArgMin

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgMax" class="anchor">§</a>

### ArgMax

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgSort" class="anchor">§</a>

### ArgSort

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgSort.field.descending" class="anchor field">§</a>`descending: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ArgSort.field.nulls_last" class="anchor field">§</a>`nulls_last: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Product" class="anchor">§</a>

### Product

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Rank" class="anchor">§</a>

### Rank

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Rank.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.RankOptions.html" class="struct" title="struct polars::prelude::RankOptions"><code>RankOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Rank.field.seed" class="anchor field">§</a>`seed: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Repeat" class="anchor">§</a>

### Repeat

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Clip" class="anchor">§</a>

### Clip

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Clip.field.has_min" class="anchor field">§</a>`has_min: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Clip.field.has_max" class="anchor field">§</a>`has_max: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.AsStruct" class="anchor">§</a>

### AsStruct

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumCount" class="anchor">§</a>

### CumCount

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumCount.field.reverse" class="anchor field">§</a>`reverse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumSum" class="anchor">§</a>

### CumSum

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumSum.field.reverse" class="anchor field">§</a>`reverse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumProd" class="anchor">§</a>

### CumProd

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumProd.field.reverse" class="anchor field">§</a>`reverse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumMin" class="anchor">§</a>

### CumMin

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumMin.field.reverse" class="anchor field">§</a>`reverse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumMax" class="anchor">§</a>

### CumMax

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumMax.field.reverse" class="anchor field">§</a>`reverse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Reverse" class="anchor">§</a>

### Reverse

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ValueCounts" class="anchor">§</a>

### ValueCounts

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ValueCounts.field.sort" class="anchor field">§</a>`sort: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ValueCounts.field.parallel" class="anchor field">§</a>`parallel: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ValueCounts.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ValueCounts.field.normalize" class="anchor field">§</a>`normalize: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.UniqueCounts" class="anchor">§</a>

### UniqueCounts

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ApproxNUnique" class="anchor">§</a>

### ApproxNUnique

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Coalesce" class="anchor">§</a>

### Coalesce

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Diff" class="anchor">§</a>

### Diff(<a href="https://docs.rs/polars/latest/polars/series/ops/enum.NullBehavior.html" class="enum" title="enum polars::series::ops::NullBehavior">NullBehavior</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Interpolate" class="anchor">§</a>

### Interpolate(<a href="https://docs.rs/polars/latest/polars/prelude/enum.InterpolationMethod.html" class="enum" title="enum polars::prelude::InterpolationMethod">InterpolationMethod</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.InterpolateBy" class="anchor">§</a>

### InterpolateBy

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Unique" class="anchor">§</a>

### Unique(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Round" class="anchor">§</a>

### Round

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Round.field.decimals" class="anchor field">§</a>`decimals: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Round.field.mode" class="anchor field">§</a>`mode: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.RoundMode.html" class="enum" title="enum polars::prelude::RoundMode"><code>RoundMode</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RoundSF" class="anchor">§</a>

### RoundSF

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RoundSF.field.digits" class="anchor field">§</a>`digits: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Floor" class="anchor">§</a>

### Floor

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Ceil" class="anchor">§</a>

### Ceil

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.UpperBound" class="anchor">§</a>

### UpperBound

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.LowerBound" class="anchor">§</a>

### LowerBound

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ConcatExpr" class="anchor">§</a>

### ConcatExpr(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.PeakMin" class="anchor">§</a>

### PeakMin

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.PeakMax" class="anchor">§</a>

### PeakMax

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RLE" class="anchor">§</a>

### RLE

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RLEID" class="anchor">§</a>

### RLEID

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ToPhysical" class="anchor">§</a>

### ToPhysical

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Random" class="anchor">§</a>

### Random

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Random.field.method" class="anchor field">§</a>`method: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.RandomMethod.html" class="enum" title="enum polars::prelude::RandomMethod"><code>RandomMethod</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Random.field.seed" class="anchor field">§</a>`seed: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.SetSortedFlag" class="anchor">§</a>

### SetSortedFlag(<a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.FoldHorizontal" class="anchor">§</a>

### FoldHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.FoldHorizontal.field.callback" class="anchor field">§</a>`callback: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback"><code>PlanCallback</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`), `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.FoldHorizontal.field.returns_scalar" class="anchor field">§</a>`returns_scalar: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.FoldHorizontal.field.return_dtype" class="anchor field">§</a>`return_dtype: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ReduceHorizontal" class="anchor">§</a>

### ReduceHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ReduceHorizontal.field.callback" class="anchor field">§</a>`callback: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback"><code>PlanCallback</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`), `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ReduceHorizontal.field.returns_scalar" class="anchor field">§</a>`returns_scalar: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ReduceHorizontal.field.return_dtype" class="anchor field">§</a>`return_dtype: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumReduceHorizontal" class="anchor">§</a>

### CumReduceHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumReduceHorizontal.field.callback" class="anchor field">§</a>`callback: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback"><code>PlanCallback</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`), `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumReduceHorizontal.field.returns_scalar" class="anchor field">§</a>`returns_scalar: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumReduceHorizontal.field.return_dtype" class="anchor field">§</a>`return_dtype: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumFoldHorizontal" class="anchor">§</a>

### CumFoldHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumFoldHorizontal.field.callback" class="anchor field">§</a>`callback: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback"><code>PlanCallback</code></a>`<(`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`, `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`), `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series"><code>Series</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumFoldHorizontal.field.returns_scalar" class="anchor field">§</a>`returns_scalar: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumFoldHorizontal.field.return_dtype" class="anchor field">§</a>`return_dtype: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.CumFoldHorizontal.field.include_init" class="anchor field">§</a>`include_init: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.MaxHorizontal" class="anchor">§</a>

### MaxHorizontal

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.MinHorizontal" class="anchor">§</a>

### MinHorizontal

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.SumHorizontal" class="anchor">§</a>

### SumHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.SumHorizontal.field.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.MeanHorizontal" class="anchor">§</a>

### MeanHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.MeanHorizontal.field.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.Replace" class="anchor">§</a>

### Replace

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ReplaceStrict" class="anchor">§</a>

### ReplaceStrict

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ReplaceStrict.field.return_dtype" class="anchor field">§</a>`return_dtype: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.GatherEvery" class="anchor">§</a>

### GatherEvery

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.GatherEvery.field.n" class="anchor field">§</a>`n: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.GatherEvery.field.offset" class="anchor field">§</a>`offset: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.ExtendConstant" class="anchor">§</a>

### ExtendConstant

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RowEncode" class="anchor">§</a>

### RowEncode(<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/function_expr/row_encode/enum.RowEncodingVariant.html" class="enum" title="enum polars_plan::plans::aexpr::function_expr::row_encode::RowEncodingVariant">RowEncodingVariant</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#variant.RowDecode" class="anchor">§</a>

### RowDecode(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>)\>, <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/function_expr/row_encode/enum.RowEncodingVariant.html" class="enum" title="enum polars_plan::plans::aexpr::function_expr::row_encode::RowEncodingVariant">RowEncodingVariant</a>)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-Clone-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-Debug-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-Deserialize%3C&#39;de%3E-for-FunctionExpr" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-Display-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CArrayFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CBooleanFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CCategoricalFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalFunction.html" class="enum" title="enum polars::prelude::CategoricalFunction">CategoricalFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(func: <a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalFunction.html" class="enum" title="enum polars::prelude::CategoricalFunction">CategoricalFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CListFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ListFunction.html" class="enum" title="enum polars::prelude::ListFunction">ListFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ListFunction.html" class="enum" title="enum polars::prelude::ListFunction">ListFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CPowFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PowFunction.html" class="enum" title="enum polars::prelude::PowFunction">PowFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PowFunction.html" class="enum" title="enum polars::prelude::PowFunction">PowFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CRangeFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-From%3CStringFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-Hash-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-PartialEq-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-Serialize-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#impl-StructuralPartialEq-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.FunctionExpr.html#blanket-implementations" class="anchor">§</a>
