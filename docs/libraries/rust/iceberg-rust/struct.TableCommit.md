# Struct TableCommit Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#303-312" class="src">Source</a>

``` rust
pub struct TableCommit { /* private fields */ }
```

Expand description

TableCommit represents the commit of a table in the catalog.

The builder is marked as private since it’s dangerous and error-prone to construct [`TableCommit`](https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html "struct iceberg::TableCommit") directly. Users are supposed to use [`crate::transaction::Transaction`](https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html "struct iceberg::transaction::Transaction") to update table.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#impl-TableCommit" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html" class="struct" title="struct iceberg::TableCommit">TableCommit</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#method.builder" class="fn">builder</a>() -\> TableCommitBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `TableCommit`. On the builder, call `.ident(...)`, `.requirements(...)`, `.updates(...)` to set the values of the fields. Finally, call `.build()` to create the instance of `TableCommit`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#impl-TableCommit-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html" class="struct" title="struct iceberg::TableCommit">TableCommit</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#method.identifier" class="fn">identifier</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>

Return the table identifier.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#method.take_requirements" class="fn">take_requirements</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>\>

Take all requirements.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#method.take_updates" class="fn">take_updates</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>\>

Take all updates.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#method.apply" class="fn">apply</a>(self, table: <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>

Applies this [`TableCommit`](https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html "struct iceberg::TableCommit") to the given [`Table`](https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html "struct iceberg::table::Table") as part of a catalog update. Typically used by [`Catalog::update_table`](https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html#tymethod.update_table "method iceberg::Catalog::update_table") to validate requirements and apply metadata updates.

Returns a new [`Table`](https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html "struct iceberg::table::Table") with updated metadata, or an error if validation or application fails.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#impl-Debug-for-TableCommit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html" class="struct" title="struct iceberg::TableCommit">TableCommit</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableCommit.html#blanket-implementations" class="anchor">§</a>
