# Struct ActionCommit Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/transaction/action.rs.html#81-84" class="src">Source</a>

``` rust
pub struct ActionCommit { /* private fields */ }
```

Expand description

The result of committing a `TransactionAction`.

This struct contains the updates to apply to the table’s metadata and any preconditions that must be satisfied before the update can be committed.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#impl-ActionCommit" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html" class="struct" title="struct iceberg::transaction::ActionCommit">ActionCommit</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#method.new" class="fn">new</a>( updates: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>\>, requirements: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>\>, ) -\> Self

Creates a new `ActionCommit` from the given updates and requirements.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#method.take_updates" class="fn">take_updates</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableUpdate.html" class="enum" title="enum iceberg::TableUpdate">TableUpdate</a>\>

Consumes and returns the list of table updates.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#method.take_requirements" class="fn">take_requirements</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.TableRequirement.html" class="enum" title="enum iceberg::TableRequirement">TableRequirement</a>\>

Consumes and returns the list of table requirements.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/transaction/struct.ActionCommit.html#blanket-implementations" class="anchor">§</a>
