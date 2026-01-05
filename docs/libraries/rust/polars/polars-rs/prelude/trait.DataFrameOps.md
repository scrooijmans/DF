# Trait DataFrameOps Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/mod.rs.html#26" class="src">Source</a>

``` rust
pub trait DataFrameOps: IntoDf {
    // Provided methods
    fn to_dummies(
        &self,
        separator: Option<&str>,
        drop_first: bool,
        drop_nulls: bool,
    ) -> Result<DataFrame, PolarsError> { ... }
    fn columns_to_dummies(
        &self,
        columns: Vec<&str>,
        separator: Option<&str>,
        drop_first: bool,
        drop_nulls: bool,
    ) -> Result<DataFrame, PolarsError> { ... }
    fn _to_dummies(
        &self,
        columns: Option<Vec<&str>>,
        separator: Option<&str>,
        drop_first: bool,
        drop_nulls: bool,
    ) -> Result<DataFrame, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#method.to_dummies" class="fn">to_dummies</a>( &self, separator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, drop_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, drop_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create dummy variables.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#example" class="doc-anchor">§</a>Example

<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust


 use polars_core::prelude::*;

 let df = df! {
      "id" => &[1, 2, 3, 1, 2, 3, 1, 1],
      "type" => &["A", "B", "B", "B", "C", "C", "C", "B"],
      "code" => &["X1", "X2", "X3", "X3", "X2", "X2", "X1", "X1"]
  }.unwrap();

  let dummies = df.to_dummies(None, false, false).unwrap();
  println!("{}", dummies);
```

Outputs:

``` text
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | id_1 | id_3 | id_2 | type_A | type_B | type_C | code_X1 | code_X2 | code_X3 |
 | ---  | ---  | ---  | ---    | ---    | ---    | ---     | ---     | ---     |
 | u8   | u8   | u8   | u8     | u8     | u8     | u8      | u8      | u8      |
 +======+======+======+========+========+========+=========+=========+=========+
 | 1    | 0    | 0    | 1      | 0      | 0      | 1       | 0       | 0       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 0    | 0    | 1    | 0      | 1      | 0      | 0       | 1       | 0       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 0    | 1    | 0    | 0      | 1      | 0      | 0       | 0       | 1       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 1    | 0    | 0    | 0      | 1      | 0      | 0       | 0       | 1       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 0    | 0    | 1    | 0      | 0      | 1      | 0       | 1       | 0       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 0    | 1    | 0    | 0      | 0      | 1      | 0       | 1       | 0       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 1    | 0    | 0    | 0      | 0      | 1      | 1       | 0       | 0       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
 | 1    | 0    | 0    | 0      | 1      | 0      | 1       | 0       | 0       |
 +------+------+------+--------+--------+--------+---------+---------+---------+
```

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#method.columns_to_dummies" class="fn">columns_to_dummies</a>( &self, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, separator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, drop_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, drop_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#method._to_dummies" class="fn">_to_dummies</a>( &self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, separator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, drop_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, drop_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html#impl-DataFrameOps-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameOps.html" class="trait" title="trait polars::prelude::DataFrameOps">DataFrameOps</a> for T

where T: <a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/polars_ops/frame/trait.IntoDf.html" class="trait" title="trait polars_ops::frame::IntoDf">IntoDf</a>,
