# Module memory Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/lib.rs.html#38" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html" class="struct" title="struct datafusion::datasource::memory::DataSourceExec">DataSourceExec</a>  
[`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") that reads one or more files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemTable.html" class="struct" title="struct datafusion::datasource::memory::MemTable">MemTable</a>  
In-memory data source for presenting a `Vec<RecordBatch>` as a data source that can be queried by DataFusion. This allows data to be pre-loaded into memory and then repeatedly queried without incurring additional file I/O overhead.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::datasource::memory::MemoryCatalogProvider">MemoryCatalogProvider</a>  
Simple in-memory implementation of a catalog.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::datasource::memory::MemoryCatalogProviderList">MemoryCatalogProviderList</a>  
Simple in-memory list of catalogs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemorySchemaProvider.html" class="struct" title="struct datafusion::datasource::memory::MemorySchemaProvider">MemorySchemaProvider</a>  
Simple in-memory implementation of a schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemorySourceConfig.html" class="struct" title="struct datafusion::datasource::memory::MemorySourceConfig">MemorySourceConfig</a>  
Data source configuration for reading in-memory batches of data

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/type.PartitionData.html" class="type" title="type datafusion::datasource::memory::PartitionData">PartitionData</a>  
Type alias for partition data
