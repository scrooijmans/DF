# Struct JoinBuilder Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/mod.rs.html#2198" class="src">Source</a>

``` rust
pub struct JoinBuilder { /* private fields */ }
```

Available on **crate feature `lazy`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#impl-JoinBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.new" class="fn">new</a>(lf: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Create the `JoinBuilder` with the provided `LazyFrame` as the left table.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.with" class="fn">with</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

The right table in the join.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.how" class="fn">how</a>(self, how: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinType.html" class="enum" title="enum polars::prelude::JoinType">JoinType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Select the join type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.validate" class="fn">validate</a>(self, validation: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinValidation.html" class="enum" title="enum polars::prelude::JoinValidation">JoinValidation</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.on" class="fn">on</a>\<E\>(self, on: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

The expressions you want to join both tables on.

The passed expressions must be valid in both `LazyFrame`s in the join.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.left_on" class="fn">left_on</a>\<E\>(self, on: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

The expressions you want to join the left table on.

The passed expressions must be valid in the left table.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.right_on" class="fn">right_on</a>\<E\>(self, on: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

The expressions you want to join the right table on.

The passed expressions must be valid in the right table.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.allow_parallel" class="fn">allow_parallel</a>(self, allow: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Allow parallel table evaluation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.force_parallel" class="fn">force_parallel</a>(self, force: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Force parallel table evaluation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.join_nulls" class="fn">join_nulls</a>(self, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Join on null values. By default null values will never produce matches.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.suffix" class="fn">suffix</a>\<S\>(self, suffix: S) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Suffix to add duplicate column names in join. Defaults to `"_right"` if this method is never called.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.coalesce" class="fn">coalesce</a>(self, coalesce: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinCoalesce.html" class="enum" title="enum polars::prelude::JoinCoalesce">JoinCoalesce</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Whether to coalesce join columns.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.maintain_order" class="fn">maintain_order</a>(self, maintain_order: <a href="https://docs.rs/polars/latest/polars/prelude/enum.MaintainOrderJoin.html" class="enum" title="enum polars::prelude::MaintainOrderJoin">MaintainOrderJoin</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Whether to preserve the row order.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Finish builder

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.join_where" class="fn">join_where</a>(self, predicates: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#blanket-implementations" class="anchor">§</a>
