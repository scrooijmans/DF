# Struct ExprNameNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/name.rs.html#7" class="src">Source</a>

``` rust
pub struct ExprNameNameSpace(/* private fields */);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for modifying the name of existing expressions.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#impl-ExprNameNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html" class="struct" title="struct polars::prelude::ExprNameNameSpace">ExprNameNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.keep" class="fn">keep</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Keep the original root name

``` rust
fn example(df: LazyFrame) -> LazyFrame {
    df.select([
// even thought the alias yields a different column name,
// `keep` will make sure that the original column name is used
        col("*").alias("foo").name().keep()
])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.map" class="fn">map</a>(self, function: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback">PlanCallback</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Define an alias by mapping a function over the original root column name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.prefix" class="fn">prefix</a>(self, prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Add a prefix to the root column name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.suffix" class="fn">suffix</a>(self, suffix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Add a suffix to the root column name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.to_lowercase" class="fn">to_lowercase</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Update the root column name to use lowercase characters.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.to_uppercase" class="fn">to_uppercase</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Update the root column name to use uppercase characters.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.map_fields" class="fn">map_fields</a>(self, function: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback">PlanCallback</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.prefix_fields" class="fn">prefix_fields</a>(self, prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#method.suffix_fields" class="fn">suffix_fields</a>(self, suffix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ExprNameNameSpace.html#blanket-implementations" class="anchor">§</a>
