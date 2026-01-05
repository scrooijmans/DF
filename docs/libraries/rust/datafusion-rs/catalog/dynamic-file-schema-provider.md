# DynamicFileSchemaProvider in datafusion::catalog - Rust

## Struct DynamicFileSchemaProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#121)

```
pub struct DynamicFileSchemaProvider { /* private fields */ }
```

Expand description

Implements the [DynamicFileSchemaProvider](struct.DynamicFileSchemaProvider.html "struct datafusion::catalog::DynamicFileSchemaProvider") that can create tables provider from the file path.

The provider will try to create a table provider from the file path if the table provider isn’t exist in the inner schema provider.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#120)
[§](#impl-Debug-for-DynamicFileSchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#139)
[§](#impl-SchemaProvider-for-DynamicFileSchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#140)
[§](#method.as_any)

Returns this `SchemaProvider` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#144)
[§](#method.table_names)

Retrieves the list of available table names in this schema.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#138)
[§](#method.table)

Retrieves a specific table from the schema by name, if it exists, otherwise returns `None`.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#159-163)
[§](#method.register_table)

If supported by the implementation, adds a new table named `name` to this schema. [Read more](about:blank/trait.SchemaProvider.html#method.register_table)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#167-170)
[§](#method.deregister_table)

If supported by the implementation, removes the `name` table from this schema and returns the previously registered [`TableProvider`](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"), if any. [Read more](about:blank/trait.SchemaProvider.html#method.deregister_table)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#174)
[§](#method.table_exist)

Returns true if table exist in the schema provider, false otherwise.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#40)
[§](#method.owner_name)

Returns the owner of the Schema, default is None. This value is reported as part of \`information_tables.schemata

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#36)
[§](#method.table_type)

Retrieves the type of a specific table from the schema by name, if it exists, otherwise returns `None`. Implementations for which this operation is cheap but [Self::table](about:blank/trait.SchemaProvider.html#tymethod.table "method datafusion_catalog::schema::SchemaProvider::table::table") is expensive can override this to improve operations that only need the type, e.g. `SELECT * FROM information_schema.tables`.

[§](#impl-Freeze-for-DynamicFileSchemaProvider)

[§](#impl-RefUnwindSafe-for-DynamicFileSchemaProvider)

[§](#impl-Send-for-DynamicFileSchemaProvider)

[§](#impl-Sync-for-DynamicFileSchemaProvider)

[§](#impl-Unpin-for-DynamicFileSchemaProvider)

[§](#impl-UnwindSafe-for-DynamicFileSchemaProvider)
