# Trait ApplyTransactionAction Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transaction/action.rs.html#56-67" class="src">Source</a>

``` rust
pub trait ApplyTransactionAction {
    // Required method
    fn apply(self, tx: Transaction) -> Result<Transaction>;
}
```

Expand description

A helper trait for applying a `TransactionAction` to a `Transaction`.

This is implemented for all `TransactionAction` types to allow easy chaining of actions into a transaction context.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html#tymethod.apply" class="fn">apply</a>(self, tx: <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html" class="struct" title="struct iceberg::transaction::Transaction">Transaction</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.Transaction.html" class="struct" title="struct iceberg::transaction::Transaction">Transaction</a>\>

Adds this action to the given transaction.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html#arguments" class="doc-anchor">§</a>Arguments

- `tx` - The transaction to apply the action to.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html#returns" class="doc-anchor">§</a>Returns

The modified transaction containing this action, or an error if the operation fails.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html#impl-ApplyTransactionAction-for-T" class="anchor">§</a>

### impl\<T: TransactionAction + 'static\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/trait.ApplyTransactionAction.html" class="trait" title="trait iceberg::transaction::ApplyTransactionAction">ApplyTransactionAction</a> for T
