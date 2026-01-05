# Struct Transaction Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transaction/mod.rs.html#90-93" class="src">Source</a>

``` rust
pub struct Transaction { /* private fields */ }
```

Expand description

Table transaction.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#impl-Transaction" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html" class="struct" title="struct iceberg::transaction::Transaction">Transaction</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.new" class="fn">new</a>(table: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>) -\> Self

Creates a new transaction.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.upgrade_table_version" class="fn">upgrade_table_version</a>(&self) -\> UpgradeFormatVersionAction

Sets table to a new version.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.update_table_properties" class="fn">update_table_properties</a>(&self) -\> UpdatePropertiesAction

Update table’s property.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.fast_append" class="fn">fast_append</a>(&self) -\> FastAppendAction

Creates a fast append action.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.replace_sort_order" class="fn">replace_sort_order</a>(&self) -\> ReplaceSortOrderAction

Creates replace sort order action.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.update_location" class="fn">update_location</a>(&self) -\> UpdateLocationAction

Set the location of table

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.update_statistics" class="fn">update_statistics</a>(&self) -\> UpdateStatisticsAction

Update the statistics of table

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.commit" class="fn">commit</a>(self, catalog: &dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html" class="trait" title="trait iceberg::Catalog">Catalog</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>

Commit transaction.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#impl-Clone-for-Transaction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html" class="struct" title="struct iceberg::transaction::Transaction">Transaction</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html" class="struct" title="struct iceberg::transaction::Transaction">Transaction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html#blanket-implementations" class="anchor">§</a>
