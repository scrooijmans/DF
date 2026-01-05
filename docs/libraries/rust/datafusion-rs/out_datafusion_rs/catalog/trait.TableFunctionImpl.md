# Trait TableFunctionImpl Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#317" class="src">Source</a>

``` rust
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

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html#tymethod.call" class="fn">call</a>(&self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a table provider

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html#impl-TableFunctionImpl-for-GenerateSeriesFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.GenerateSeriesFunc.html" class="struct" title="struct datafusion::functions_table::generate_series::GenerateSeriesFunc">GenerateSeriesFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html#impl-TableFunctionImpl-for-RangeFunc" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.RangeFunc.html" class="struct" title="struct datafusion::functions_table::generate_series::RangeFunc">RangeFunc</a>
