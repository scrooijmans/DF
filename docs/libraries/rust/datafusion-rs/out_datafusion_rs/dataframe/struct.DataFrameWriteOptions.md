# Struct DataFrameWriteOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/dataframe/mod.rs.html#77-90" class="src">Source</a>

``` rust
pub struct DataFrameWriteOptions { /* private fields */ }
```

Expand description

Contains options that control how data is written out from a DataFrame

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#impl-DataFrameWriteOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html" class="struct" title="struct datafusion::dataframe::DataFrameWriteOptions">DataFrameWriteOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#method.new" class="fn">new</a>() -\> Self

Create a new DataFrameWriteOptions with default values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#method.with_insert_operation" class="fn">with_insert_operation</a>(self, insert_op: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/enum.InsertOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::dml::InsertOp">InsertOp</a>) -\> Self

Set the insert operation

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#method.with_single_file_output" class="fn">with_single_file_output</a>(self, single_file_output: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set the single_file_output value to true or false

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#method.with_partition_by" class="fn">with_partition_by</a>(self, partition_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets the partition_by columns for output partitioning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#method.with_sort_by" class="fn">with_sort_by</a>(self, sort_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>) -\> Self

Sets the sort_by columns for output sorting

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#impl-Default-for-DataFrameWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html" class="struct" title="struct datafusion::dataframe::DataFrameWriteOptions">DataFrameWriteOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrameWriteOptions.html#blanket-implementations" class="anchor">§</a>
