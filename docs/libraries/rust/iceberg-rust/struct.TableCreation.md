# Struct TableCreation Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#275-294" class="src">Source</a>

``` rust
pub struct TableCreation {
    pub name: String,
    pub location: Option<String>,
    pub schema: Schema,
    pub partition_spec: Option<UnboundPartitionSpec>,
    pub sort_order: Option<SortOrder>,
    pub properties: HashMap<String, String>,
}
```

Expand description

TableCreation represents the creation of a table in the catalog.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The name of the table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#structfield.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The location of the table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema"><code>Schema</code></a>

The schema of the table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#structfield.partition_spec" class="anchor field">§</a>`partition_spec: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec"><code>UnboundPartitionSpec</code></a>`>`

The partition spec of the table, could be None.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#structfield.sort_order" class="anchor field">§</a>`sort_order: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder"><code>SortOrder</code></a>`>`

The sort order of the table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#structfield.properties" class="anchor field">§</a>`properties: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The properties of the table.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#impl-TableCreation" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html" class="struct" title="struct iceberg::TableCreation">TableCreation</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#method.builder" class="fn">builder</a>() -\> TableCreationBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `TableCreation`. On the builder, call `.name(...)`, `.location(...)`(optional), `.schema(...)`, `.partition_spec(...)`(optional), `.sort_order(...)`(optional), `.properties(...)`(optional) to set the values of the fields. Finally, call `.build()` to create the instance of `TableCreation`.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#impl-Debug-for-TableCreation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html" class="struct" title="struct iceberg::TableCreation">TableCreation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCreation.html#blanket-implementations" class="anchor">§</a>
