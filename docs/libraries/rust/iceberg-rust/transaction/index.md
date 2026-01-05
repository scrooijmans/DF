# Module transaction Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transaction/mod.rs.html#18-512" class="src">Source</a>

Expand description

This module contains transaction api.

The transaction API enables changes to be made to an existing table.

Note that this may also have side effects, such as producing new manifest files.

Below is a basic example using the “fast-append” action:

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/index.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
use iceberg::transaction::{ApplyTransactionAction, Transaction};
use iceberg::Catalog;

// Create a transaction.
let tx = Transaction::new(my_table);

// Create a `FastAppendAction` which will not rewrite or append
// to existing metadata. This will create a new manifest.
let action = tx.fast_append().add_data_files(my_data_files);

// Apply the fast-append action to the given transaction, returning
// the newly updated `Transaction`.
let tx = action.apply(tx).unwrap();


// End the transaction by committing to an `iceberg::Catalog`
// implementation. This will cause a table update to occur.
let table = tx
    .commit(&some_catalog_impl)
    .await
    .unwrap();
```

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html" class="struct" title="struct iceberg::transaction::ActionCommit">ActionCommit</a>  
The result of committing a `TransactionAction`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html" class="struct" title="struct iceberg::transaction::Transaction">Transaction</a>  
Table transaction.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html" class="trait" title="trait iceberg::transaction::ApplyTransactionAction">ApplyTransactionAction</a>  
A helper trait for applying a `TransactionAction` to a `Transaction`.
