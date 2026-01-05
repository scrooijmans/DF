# Module default_table_source Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/lib.rs.html#35" class="src">Source</a>

Expand description

Default TableSource implementation used in DataFusion physical plans

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/default_table_source/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/default_table_source/struct.DefaultTableSource.html" class="struct" title="struct datafusion::catalog::default_table_source::DefaultTableSource">DefaultTableSource</a>  
Implements [`TableSource`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html "trait datafusion::logical_expr::TableSource") for a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/default_table_source/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/default_table_source/fn.provider_as_source.html" class="fn" title="fn datafusion::catalog::default_table_source::provider_as_source">provider_as_source</a>  
Wrap TableProvider in TableSource

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/default_table_source/fn.source_as_provider.html" class="fn" title="fn datafusion::catalog::default_table_source::source_as_provider">source_as_provider</a>  
Attempt to downcast a TableSource to DefaultTableSource and access the TableProvider. This will only work with a TableSource created by DataFusion.
