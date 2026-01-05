# Struct ArrayNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/array.rs.html#7" class="src">Source</a>

``` rust
pub struct ArrayNameSpace(pub Expr);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of [`DataType::Array`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Array "variant polars::prelude::DataType::Array").

## Tuple Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#impl-ArrayNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html" class="struct" title="struct polars::prelude::ArrayNameSpace">ArrayNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.len" class="fn">len</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the length of every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.max" class="fn">max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the maximum of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.min" class="fn">min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the minimum of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.sum" class="fn">sum</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the sum of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.std" class="fn">std</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the std of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.var" class="fn">var</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the var of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.mean" class="fn">mean</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the mean of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.median" class="fn">median</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Compute the median of the items in every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.unique" class="fn">unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Keep only the unique values in every sub-array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.unique_stable" class="fn">unique_stable</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Keep only the unique values in every sub-array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.n_unique" class="fn">n_unique</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.to_list" class="fn">to_list</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Cast the Array column to List column with the same inner data type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.sort" class="fn">sort</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.reverse" class="fn">reverse</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.arg_min" class="fn">arg_min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.arg_max" class="fn">arg_max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.get" class="fn">get</a>(self, index: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, null_on_oob: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Get items in every sub-array by index.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.join" class="fn">join</a>(self, separator: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, ignore_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Join all string items in a sub-array and place a separator between them.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#error" class="doc-anchor">§</a>Error

Raise if inner type of array is not `DataType::String`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.contains" class="fn">contains</a>\<E\>(self, other: E, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Check if the sub-array contains specific element

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.slice" class="fn">slice</a>( self, offset: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, length: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, as_array: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Slice every subarray.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.head" class="fn">head</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, as_array: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the head of every subarray

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.tail" class="fn">tail</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, as_array: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the tail of every subarray

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.shift" class="fn">shift</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Shift every sub-array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#method.explode" class="fn">explode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Returns a column with a separate row for every array element.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrayNameSpace.html#blanket-implementations" class="anchor">§</a>
