# Crate iceberg Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/lib.rs.html#18-99" class="src">Source</a>

Expand description

Apache Iceberg Official Native Rust Implementation

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#examples" class="doc-anchor">§</a>Examples

### <a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#scan-a-table" class="doc-anchor">§</a>Scan A Table

``` rust
use std::collections::HashMap;

use futures::TryStreamExt;
use iceberg::io::{FileIO, FileIOBuilder};
use iceberg::memory::MemoryCatalogBuilder;
use iceberg::{Catalog, CatalogBuilder, MemoryCatalog, Result, TableIdent};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to a catalog.
    use iceberg::memory::MEMORY_CATALOG_WAREHOUSE;
    let catalog = MemoryCatalogBuilder::default()
        .load(
            "memory",
            HashMap::from([(
                MEMORY_CATALOG_WAREHOUSE.to_string(),
                "file:///path/to/warehouse".to_string(),
            )]),
        )
        .await?;
    // Load table from catalog.
    let table = catalog
        .load_table(&TableIdent::from_strs(["hello", "world"])?)
        .await?;
    // Build table scan.
    let stream = table
        .scan()
        .select(["name", "id"])
        .build()?
        .to_arrow()
        .await?;

    // Consume this stream like arrow record batch stream.
    let _data: Vec<_> = stream.try_collect().await?;
    Ok(())
}
```

## Modules<a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html" class="mod" title="mod iceberg::arrow">arrow</a>  
Conversion between Iceberg and Arrow schema

<a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/index.html" class="mod" title="mod iceberg::cache">cache</a>  
Cache management for Iceberg.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/index.html" class="mod" title="mod iceberg::expr">expr</a>  
This module contains expressions.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/index.html" class="mod" title="mod iceberg::inspect">inspect</a>  
Metadata table APIs.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/index.html" class="mod" title="mod iceberg::io">io</a>  
File io implementation.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/index.html" class="mod" title="mod iceberg::memory">memory</a>  
Memory catalog implementation.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/index.html" class="mod" title="mod iceberg::puffin">puffin</a>  
Iceberg Puffin implementation.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/index.html" class="mod" title="mod iceberg::scan">scan</a>  
Table scan api.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html" class="mod" title="mod iceberg::spec">spec</a>  
Spec for Iceberg.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/index.html" class="mod" title="mod iceberg::table">table</a>  
Table API for Apache Iceberg

<a href="https://docs.rs/iceberg/0.7.0/iceberg/test_utils/index.html" class="mod" title="mod iceberg::test_utils">test_utils</a>  
Test utilities. This module is pub just for internal testing. It is subject to change and is not intended to be used by external users.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/index.html" class="mod" title="mod iceberg::transaction">transaction</a>  
This module contains transaction api.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transform/index.html" class="mod" title="mod iceberg::transform">transform</a>  
Transform function used to compute partition values.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/index.html" class="mod" title="mod iceberg::writer">writer</a>  
Iceberg writer module.

## Macros<a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/macro.ensure_data_valid.html" class="macro" title="macro iceberg::ensure_data_valid">ensure_data_valid</a>  
Helper macro to check arguments.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>  
Error is the error struct returned by all iceberg functions.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MemoryCatalog.html" class="struct" title="struct iceberg::MemoryCatalog">MemoryCatalog</a>  
Memory catalog implementation.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MetadataLocation.html" class="struct" title="struct iceberg::MetadataLocation">MetadataLocation</a>  
Helper for parsing a location of the format: `<location>/metadata/<version>-<uuid>.metadata.json`

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>  
Namespace represents a namespace in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>  
NamespaceIdent represents the identifier of a namespace in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html" class="struct" title="struct iceberg::TableCommit">TableCommit</a>  
TableCommit represents the commit of a table in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html" class="struct" title="struct iceberg::TableCreation">TableCreation</a>  
TableCreation represents the creation of a table in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>  
TableIdent represents the identifier of a table in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html" class="struct" title="struct iceberg::ViewCreation">ViewCreation</a>  
ViewCreation represents the creation of a view in the catalog.

## Enums<a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>  
ErrorKind is all kinds of Error of iceberg.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>  
TableRequirement represents a requirement for a table in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>  
TableUpdate represents an update to a table in the catalog.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>  
ViewUpdate represents an update to a view in the catalog.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html" class="trait" title="trait iceberg::Catalog">Catalog</a>  
The catalog API for Iceberg Rust.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html" class="trait" title="trait iceberg::CatalogBuilder">CatalogBuilder</a>  
Common interface for all catalog builders.

## Type Aliases<a href="https://docs.rs/iceberg/0.7.0/iceberg/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>  
Result that is a wrapper of `Result<T, iceberg::Error>`
