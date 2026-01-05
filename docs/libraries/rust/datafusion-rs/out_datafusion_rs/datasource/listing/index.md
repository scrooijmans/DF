# Module listing Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/listing/mod.rs.html#18-26" class="src">Source</a>

Expand description

A table that uses the `ObjectStore` listing capability to get the list of files to process.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/helpers/index.html" class="mod" title="mod datafusion::datasource::listing::helpers">helpers</a>  
Helper functions for the table implementation

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.FileRange.html" class="struct" title="struct datafusion::datasource::listing::FileRange">FileRange</a>  
Only scan a subset of Row Groups from the Parquet file whose data “midpoint” lies within the \[start, end) byte offsets. This option can be used to scan non-overlapping sections of a Parquet file in parallel.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>  
Options for creating a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>  
Built in [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") that reads data from one or more files as a single table.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>  
Configuration for creating a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>  
A parsed URL identifying files for a listing table, see [`ListingTableUrl::parse`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.parse "associated function datafusion::datasource::listing::ListingTableUrl::parse") for more information on the supported expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.PartitionedFile.html" class="struct" title="struct datafusion::datasource::listing::PartitionedFile">PartitionedFile</a>  
A single file or part of a file that should be read, along with its schema, statistics and partition column values that need to be appended to each row.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/type.PartitionedFileStream.html" class="type" title="type datafusion::datasource::listing::PartitionedFileStream">PartitionedFileStream</a>  
Stream of files get listed from object store
