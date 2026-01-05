# Struct StructNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/struct_.rs.html#5" class="src">Source</a>

``` rust
pub struct StructNameSpace(/* private fields */);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for Struct dtypes.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#impl-StructNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html" class="struct" title="struct polars::prelude::StructNameSpace">StructNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method.field_by_index" class="fn">field_by_index</a>(self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method.field_by_names" class="fn">field_by_names</a>\<I, S\>(self, names: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Retrieve one or multiple of the fields of this [`StructChunked`](https://docs.rs/polars/latest/polars/prelude/type.StructChunked.html "type polars::prelude::StructChunked") as a new Series. This expression also expands the `"*"` wildcard column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method.field_by_name" class="fn">field_by_name</a>(self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Retrieve one of the fields of this [`StructChunked`](https://docs.rs/polars/latest/polars/prelude/type.StructChunked.html "type polars::prelude::StructChunked") as a new Series. This expression also supports wildcard “\*” and regex expansion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method.rename_fields" class="fn">rename_fields</a>\<I, S\>(self, names: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Rename the fields of the [`StructChunked`](https://docs.rs/polars/latest/polars/prelude/type.StructChunked.html "type polars::prelude::StructChunked").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method._rename_fields_impl" class="fn">_rename_fields_impl</a>(self, names: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\]\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method.json_encode" class="fn">json_encode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#method.with_fields" class="fn">with_fields</a>(self, fields: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructNameSpace.html#blanket-implementations" class="anchor">§</a>
