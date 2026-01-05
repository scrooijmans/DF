# Struct LazyGroupBy Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/mod.rs.html#2083" class="src">Source</a>

``` rust
pub struct LazyGroupBy {
    pub logical_plan: DslPlan,
    /* private fields */
}
```

Available on **crate feature `lazy`** only.

Expand description

Utility struct for lazy group_by operation.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#structfield.logical_plan" class="anchor field">§</a>`logical_plan: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#impl-LazyGroupBy" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.agg" class="fn">agg</a>\<E\>(self, aggs: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Group by and aggregate.

Select a column with [col](https://docs.rs/polars/latest/polars/prelude/fn.col.html "fn polars::prelude::col") and choose an aggregation. If you want to aggregate all columns use `col(PlSmallStr::from_static("*"))`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
       .group_by_stable([col("date")])
       .agg([
           col("rain").min().alias("min_rain"),
           col("rain").sum().alias("sum_rain"),
           col("rain").quantile(lit(0.5), QuantileMethod::Nearest).alias("median_rain"),
       ])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.head" class="fn">head</a>(self, n: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Return first n rows of each group

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.tail" class="fn">tail</a>(self, n: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Return last n rows of each group

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.apply" class="fn">apply</a>( self, f: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback">PlanCallback</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>, schema: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Apply a function over the groups as a new DataFrame.

**It is not recommended that you use this as materializing the DataFrame is very expensive.**

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#impl-Clone-for-LazyGroupBy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#impl-From%3CLazyGroupBy%3E-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(lgb: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html#blanket-implementations" class="anchor">§</a>
