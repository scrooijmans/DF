# MemorySchemaProvider in datafusion::catalog - Rust

## Struct MemorySchemaProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#29)

```
pub struct MemorySchemaProvider { /* private fields */ }
```

Expand description

Simple in-memory implementation of a schema.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#33)
[§](#impl-MemorySchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#35)

Instantiates a new MemorySchemaProvider with an empty collection of tables.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#28)
[§](#impl-Debug-for-MemorySchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#42)
[§](#impl-Default-for-MemorySchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#49)
[§](#impl-SchemaProvider-for-MemorySchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#50)
[§](#method.as_any)

Returns this `SchemaProvider` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#54)
[§](#method.table_names)

Retrieves the list of available table names in this schema.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#48)
[§](#method.table)

Retrieves a specific table from the schema by name, if it exists, otherwise returns `None`.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#68-72)
[§](#method.register_table)

If supported by the implementation, adds a new table named `name` to this schema. [Read more](about:blank/trait.SchemaProvider.html#method.register_table)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#79-82)
[§](#method.deregister_table)

If supported by the implementation, removes the `name` table from this schema and returns the previously registered [`TableProvider`](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"), if any. [Read more](about:blank/trait.SchemaProvider.html#method.deregister_table)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#86)
[§](#method.table_exist)

Returns true if table exist in the schema provider, false otherwise.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#40)
[§](#method.owner_name)

Returns the owner of the Schema, default is None. This value is reported as part of \`information_tables.schemata

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#36)
[§](#method.table_type)

Retrieves the type of a specific table from the schema by name, if it exists, otherwise returns `None`. Implementations for which this operation is cheap but [Self::table](about:blank/trait.SchemaProvider.html#tymethod.table "method datafusion_catalog::schema::SchemaProvider::table::table") is expensive can override this to improve operations that only need the type, e.g. `SELECT * FROM information_schema.tables`.

[§](#impl-Freeze-for-MemorySchemaProvider)

[§](#impl-RefUnwindSafe-for-MemorySchemaProvider)

[§](#impl-Send-for-MemorySchemaProvider)

[§](#impl-Sync-for-MemorySchemaProvider)

[§](#impl-Unpin-for-MemorySchemaProvider)

[§](#impl-UnwindSafe-for-MemorySchemaProvider)
