# Trait Catalog Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#52-111" class="src">Source</a>

``` rust
pub trait Catalog:
    Debug
    + Sync
    + Send {
Show 14 methods    // Required methods
    fn list_namespaces<'life0, 'life1, 'async_trait>(
        &'life0 self,
        parent: Option<&'life1 NamespaceIdent>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<NamespaceIdent>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn create_namespace<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
        properties: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<Namespace>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn get_namespace<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
    ) -> Pin<Box<dyn Future<Output = Result<Namespace>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn namespace_exists<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn update_namespace<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
        properties: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn drop_namespace<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn list_tables<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<TableIdent>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn create_table<'life0, 'life1, 'async_trait>(
        &'life0 self,
        namespace: &'life1 NamespaceIdent,
        creation: TableCreation,
    ) -> Pin<Box<dyn Future<Output = Result<Table>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn load_table<'life0, 'life1, 'async_trait>(
        &'life0 self,
        table: &'life1 TableIdent,
    ) -> Pin<Box<dyn Future<Output = Result<Table>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn drop_table<'life0, 'life1, 'async_trait>(
        &'life0 self,
        table: &'life1 TableIdent,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn table_exists<'life0, 'life1, 'async_trait>(
        &'life0 self,
        table: &'life1 TableIdent,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn rename_table<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        src: &'life1 TableIdent,
        dest: &'life2 TableIdent,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait;
    fn register_table<'life0, 'life1, 'async_trait>(
        &'life0 self,
        table: &'life1 TableIdent,
        metadata_location: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn update_table<'life0, 'async_trait>(
        &'life0 self,
        commit: TableCommit,
    ) -> Pin<Box<dyn Future<Output = Result<Table>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Expand description

The catalog API for Iceberg Rust.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.list_namespaces" class="fn">list_namespaces</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, parent: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

List namespaces inside the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.create_namespace" class="fn">create_namespace</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Create a new namespace inside the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.get_namespace" class="fn">get_namespace</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Namespace.html" class="struct" title="struct iceberg::Namespace">Namespace</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Get a namespace information from the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.namespace_exists" class="fn">namespace_exists</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Check if namespace exists in catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.update_namespace" class="fn">update_namespace</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Update a namespace inside the catalog.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#behavior" class="doc-anchor">§</a>Behavior

The properties must be the full set of namespace.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.drop_namespace" class="fn">drop_namespace</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Drop a namespace from the catalog, or returns error if it doesn’t exist.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.list_tables" class="fn">list_tables</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

List tables from namespace.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.create_table" class="fn">create_table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, namespace: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent">NamespaceIdent</a>, creation: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html" class="struct" title="struct iceberg::TableCreation">TableCreation</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Create a new table inside the namespace.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.load_table" class="fn">load_table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, table: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Load table from the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.drop_table" class="fn">drop_table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, table: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Drop a table from the catalog, or returns error if it doesn’t exist.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.table_exists" class="fn">table_exists</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, table: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Check if a table exists in the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.rename_table" class="fn">rename_table</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, src: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, dest: &'life2 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Rename a table in the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.register_table" class="fn">register_table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, table: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, metadata_location: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Register an existing table to the catalog.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.update_table" class="fn">update_table</a>\<'life0, 'async_trait\>( &'life0 self, commit: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html" class="struct" title="struct iceberg::TableCommit">TableCommit</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Update a table to the catalog.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#impl-Catalog-for-MemoryCatalog" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html" class="trait" title="trait iceberg::Catalog">Catalog</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MemoryCatalog.html" class="struct" title="struct iceberg::MemoryCatalog">MemoryCatalog</a>
