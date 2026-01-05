# Trait ContextProvider Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/planner.rs.html#40" class="src">Source</a>

``` rust
pub trait ContextProvider {
Show 14 methods    // Required methods
    fn get_table_source(
        &self,
        name: TableReference,
    ) -> Result<Arc<dyn TableSource>, DataFusionError>;
    fn get_function_meta(&self, name: &str) -> Option<Arc<ScalarUDF>>;
    fn get_aggregate_meta(&self, name: &str) -> Option<Arc<AggregateUDF>>;
    fn get_window_meta(&self, name: &str) -> Option<Arc<WindowUDF>>;
    fn get_variable_type(&self, variable_names: &[String]) -> Option<DataType>;
    fn options(&self) -> &ConfigOptions;
    fn udf_names(&self) -> Vec<String>;
    fn udaf_names(&self) -> Vec<String>;
    fn udwf_names(&self) -> Vec<String>;

    // Provided methods
    fn get_file_type(
        &self,
        _ext: &str,
    ) -> Result<Arc<dyn FileType>, DataFusionError> { ... }
    fn get_table_function_source(
        &self,
        _name: &str,
        _args: Vec<Expr>,
    ) -> Result<Arc<dyn TableSource>, DataFusionError> { ... }
    fn create_cte_work_table(
        &self,
        _name: &str,
        _schema: Arc<Schema>,
    ) -> Result<Arc<dyn TableSource>, DataFusionError> { ... }
    fn get_expr_planners(&self) -> &[Arc<dyn ExprPlanner>] { ... }
    fn get_type_planner(&self) -> Option<Arc<dyn TypePlanner>> { ... }
}
```

Expand description

Provides the `SQL` query planner meta-data about tables and functions referenced in SQL statements, without a direct dependency on the `datafusion` Catalog structures such as [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html)

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.get_table_source" class="fn">get_table_source</a>( &self, name: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a table by reference, if it exists

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.get_function_meta" class="fn">get_function_meta</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

Return the scalar function with a given name, if any

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.get_aggregate_meta" class="fn">get_aggregate_meta</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

Return the aggregate function with a given name, if any

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.get_window_meta" class="fn">get_window_meta</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

Return the window function with a given name, if any

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.get_variable_type" class="fn">get_variable_type</a>(&self, variable_names: &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>

Return the system/user-defined variable type, if any

A user defined variable is typically accessed via `@var_name`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.options" class="fn">options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

Return overall configuration options

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.udf_names" class="fn">udf_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Return all scalar function names

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.udaf_names" class="fn">udaf_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Return all aggregate function names

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#tymethod.udwf_names" class="fn">udwf_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Return all window function names

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#method.get_file_type" class="fn">get_file_type</a>( &self, \_ext: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html" class="trait" title="trait datafusion::common::file_options::file_type::FileType">FileType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the type of a file based on its extension (e.g. `.parquet`)

This is used to plan `COPY` statements

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#method.get_table_function_source" class="fn">get_table_function_source</a>( &self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Getter for a table function

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#method.create_cte_work_table" class="fn">create_cte_work_table</a>( &self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Provides an intermediate table that is used to store the results of a CTE during execution

CTE stands for “Common Table Expression”

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#notes" class="doc-anchor">§</a>Notes

We don’t directly implement this in [`SqlToRel`](https://docs.rs/datafusion/latest/datafusion/sql/planner/struct.SqlToRel.html) as implementing this function often requires access to a table that contains execution-related types that can’t be a direct dependency of the sql crate (for example [`CteWorkTable`](https://docs.rs/datafusion/latest/datafusion/datasource/cte_worktable/struct.CteWorkTable.html)).

The [`ContextProvider`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html "trait datafusion::logical_expr::planner::ContextProvider") provides a way to “hide” this dependency.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#method.get_expr_planners" class="fn">get_expr_planners</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\]

Return [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") extensions for planning expressions

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#method.get_type_planner" class="fn">get_type_planner</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.TypePlanner.html" class="trait" title="trait datafusion::logical_expr::planner::TypePlanner">TypePlanner</a>\>\>

Return [`TypePlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.TypePlanner.html "trait datafusion::logical_expr::planner::TypePlanner") extensions for planning data types

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html#implementors" class="anchor">§</a>
