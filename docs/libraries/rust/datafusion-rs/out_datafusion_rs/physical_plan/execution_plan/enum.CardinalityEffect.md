# Enum CardinalityEffect Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1343" class="src">Source</a>

``` rust
pub enum CardinalityEffect {
    Unknown,
    Equal,
    LowerEqual,
    GreaterEqual,
}
```

Expand description

Indicates the effect an execution plan operator will have on the cardinality of its input stream

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#variant.Unknown" class="anchor">§</a>

### Unknown

Unknown effect. This is the default

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#variant.Equal" class="anchor">§</a>

### Equal

The operator is guaranteed to produce exactly one row for each input row

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#variant.LowerEqual" class="anchor">§</a>

### LowerEqual

The operator may produce fewer output rows than it receives input rows

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#variant.GreaterEqual" class="anchor">§</a>

### GreaterEqual

The operator may produce more output rows than it receives input rows

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html#blanket-implementations" class="anchor">§</a>
