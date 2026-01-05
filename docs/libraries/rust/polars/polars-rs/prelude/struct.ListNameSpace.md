# Struct ListNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/list.rs.html#9" class="src">Source</a>

``` rust
pub struct ListNameSpace(pub Expr);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of [`DataType::List`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.List "variant polars::prelude::DataType::List").

## Tuple Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#impl-ListNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html" class="struct" title="struct polars::prelude::ListNameSpace">ListNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.len" class="fn">len</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Return the number of elements in each list.

Null values are treated like regular elements in this context.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.max" class="fn">max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the maximum of the items in every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.min" class="fn">min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the minimum of the items in every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.sum" class="fn">sum</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the sum the items in every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.mean" class="fn">mean</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the mean of every sublist and return a `Series` of dtype `Float64`

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.median" class="fn">median</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.std" class="fn">std</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.var" class="fn">var</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.sort" class="fn">sort</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Sort every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.reverse" class="fn">reverse</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Reverse every sublist

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.unique" class="fn">unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Keep only the unique values in every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.unique_stable" class="fn">unique_stable</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Keep only the unique values in every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.n_unique" class="fn">n_unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.get" class="fn">get</a>(self, index: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, null_on_oob: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get items in every sublist by index.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.first" class="fn">first</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get first item of every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.last" class="fn">last</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get last item of every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.join" class="fn">join</a>(self, separator: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, ignore_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Join all string items in a sublist and place a separator between them.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#error" class="doc-anchor">§</a>Error

This errors if inner type of list `!= DataType::String`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.arg_min" class="fn">arg_min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Return the index of the minimal value of every sublist

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.arg_max" class="fn">arg_max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Return the index of the maximum value of every sublist

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.diff" class="fn">diff</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, null_behavior: <a href="https://docs.rs/polars/latest/polars/series/ops/enum.NullBehavior.html" class="enum" title="enum polars::series::ops::NullBehavior">NullBehavior</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Diff every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.shift" class="fn">shift</a>(self, periods: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Shift every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.slice" class="fn">slice</a>(self, offset: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, length: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Slice every sublist.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.head" class="fn">head</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the head of every sublist

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.tail" class="fn">tail</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get the tail of every sublist

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.to_array" class="fn">to_array</a>(self, width: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Convert a List column into an Array column with the same inner data type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.contains" class="fn">contains</a>\<E\>(self, other: E, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if the list array contain an element

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#method.eval" class="fn">eval</a>\<E\>(self, other: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListNameSpace.html#blanket-implementations" class="anchor">§</a>
