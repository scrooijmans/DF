# TableFunctionImpl in datafusion::catalog - Rust

[![logo](https://raw.githubusercontent.com/apache/datafusion/19fe44cf2f30cbdd63d4a4f52c74055163c6cc38/docs/logos/standalone_logo/logo_original.svg)](../../datafusion/index.html)

## [datafusion](../../datafusion/index.html)50.2.0

## [TableFunctionImpl](#)

### [Required Methods](#required-methods)

- [call](#tymethod.call "call")

### [Implementors](#implementors)

## [In datafusion::catalog](index.html)

[datafusion](../index.html)::[catalog](index.html)

## Trait TableFunctionImpl 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#317)

```
pub trait TableFunctionImpl:
    Debug
    + Sync
    + Send {
    // Required method
    fn call(
        &self,
        args: &[Expr],
    ) -> Result<Arc<dyn TableProvider>, DataFusionError>;
}
```

Expand description

A trait for table function implementations

## Required Methods[§](#required-methods)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#319)

#### fn [call](#tymethod.call)(&self, args: &\[[Expr](../prelude/enum.Expr.html "enum datafusion::prelude::Expr")\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arc](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")<dyn [TableProvider](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")\>, [DataFusionError](../error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")\>

Create a table provider

## Implementors[§](#implementors)

[Source](https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_table/generate_series.rs.html#758)
[§](#impl-TableFunctionImpl-for-GenerateSeriesFunc)

### impl [TableFunctionImpl](trait.TableFunctionImpl.html "trait datafusion::catalog::TableFunctionImpl") for [GenerateSeriesFunc](../functions_table/generate_series/struct.GenerateSeriesFunc.html "struct datafusion::functions_table::generate_series::GenerateSeriesFunc")

[Source](https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_table/generate_series.rs.html#771)
[§](#impl-TableFunctionImpl-for-RangeFunc)

### impl [TableFunctionImpl](trait.TableFunctionImpl.html "trait datafusion::catalog::TableFunctionImpl") for [RangeFunc](../functions_table/generate_series/struct.RangeFunc.html "struct datafusion::functions_table::generate_series::RangeFunc")
