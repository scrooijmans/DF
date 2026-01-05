# Struct CategoricalNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/cat.rs.html#4" class="src">Source</a>

``` rust
pub struct CategoricalNameSpace(/* private fields */);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for Categorical dtypes.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#impl-CategoricalNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalNameSpace.html" class="struct" title="struct polars::prelude::CategoricalNameSpace">CategoricalNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#method.get_categories" class="fn">get_categories</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#method.len_bytes" class="fn">len_bytes</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#method.len_chars" class="fn">len_chars</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#method.starts_with" class="fn">starts_with</a>(self, prefix: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#method.ends_with" class="fn">ends_with</a>(self, suffix: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#method.slice" class="fn">slice</a>(self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cat/struct.CategoricalNameSpace.html#blanket-implementations" class="anchor">§</a>
