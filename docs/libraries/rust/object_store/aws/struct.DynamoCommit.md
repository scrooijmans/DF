# Struct DynamoCommit Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/dynamo.rs.html#112-125" class="src">Source</a>

``` rust
pub struct DynamoCommit { /* private fields */ }
```

Available on **crate feature `aws`** only.

Expand description

A DynamoDB-based commit protocol, used to provide conditional write support for S3

### <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#limitations" class="doc-anchor">§</a>Limitations

Only conditional operations, e.g. `copy_if_not_exists` will be synchronized, and can therefore race with non-conditional operations, e.g. `put`, `copy`, `delete`, or conditional operations performed by writers not configured to synchronize with DynamoDB.

Workloads making use of this mechanism **must** ensure:

- Conditional and non-conditional operations are not performed on the same paths
- Conditional operations are only performed via similarly configured clients

Additionally as the locking mechanism relies on timeouts to detect stale locks, performance will be poor for systems that frequently delete and then create objects at the same path, instead being optimised for systems that primarily create files with paths never used before, or perform conditional updates to existing files

### <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#commit-protocol" class="doc-anchor">§</a>Commit Protocol

The DynamoDB schema is as follows:

- A string partition key named `"path"`
- A string sort key named `"etag"`
- A numeric [TTL](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/howitworks-ttl.html) attribute named `"ttl"`
- A numeric attribute named `"generation"`
- A numeric attribute named `"timeout"`

An appropriate DynamoDB table can be created with the CLI as follows:

``` bash
$ aws dynamodb create-table --table-name <TABLE_NAME> --key-schema AttributeName=path,KeyType=HASH AttributeName=etag,KeyType=RANGE --attribute-definitions AttributeName=path,AttributeType=S AttributeName=etag,AttributeType=S
$ aws dynamodb update-time-to-live --table-name <TABLE_NAME> --time-to-live-specification Enabled=true,AttributeName=ttl
```

To perform a conditional operation on an object with a given `path` and `etag` (`*` if creating), the commit protocol is as follows:

1.  Perform HEAD request on `path` and error on precondition mismatch
2.  Create record in DynamoDB with given `path` and `etag` with the configured timeout
    1.  On Success: Perform operation with the configured timeout
    2.  On Conflict:
        1.  Periodically re-perform HEAD request on `path` and error on precondition mismatch
        2.  If `timeout * max_skew_rate` passed, replace the record incrementing the `"generation"`
            1.  On Success: GOTO 2.1
            2.  On Conflict: GOTO 2.2

Provided no writer modifies an object with a given `path` and `etag` without first adding a corresponding record to DynamoDB, we are guaranteed that only one writer will ever commit.

This is inspired by the [DynamoDB Lock Client](https://aws.amazon.com/blogs/database/building-distributed-locks-with-the-dynamodb-lock-client/) but simplified for the more limited requirements of synchronizing object storage. The major changes are:

- Uses a monotonic generation count instead of a UUID rvn, as this is:
  - Cheaper to generate, serialize and compare
  - Cannot collide
  - More human readable / interpretable
- Relies on [TTL](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/howitworks-ttl.html) to eventually clean up old locks

It also draws inspiration from the DeltaLake [S3 Multi-Cluster](https://docs.google.com/document/d/1Gs4ZsTH19lMxth4BSdwlWjUNR-XhKHicDvBjd2RqNd8/edit#heading=h.mjjuxw9mcz9h) commit protocol, but generalised to not make assumptions about the workload and not rely on first writing to a temporary path.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#impl-DynamoCommit" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.new" class="fn">new</a>(table_name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Create a new [`DynamoCommit`](https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html "struct object_store::aws::DynamoCommit") with a given table name

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.with_timeout" class="fn">with_timeout</a>(self, millis: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

Overrides the lock timeout.

A longer lock timeout reduces the probability of spurious commit failures and multi-writer races, but will increase the time that writers must wait to reclaim a lock lost. The default value of 20 seconds should be appropriate for must use-cases.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.with_max_clock_skew_rate" class="fn">with_max_clock_skew_rate</a>(self, rate: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> Self

The maximum clock skew rate tolerated by the system.

An environment in which the clock on the fastest node ticks twice as fast as the slowest node, would have a clock skew rate of 2. The default value of 3 should be appropriate for most environments.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.with_ttl" class="fn">with_ttl</a>(self, ttl: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

The length of time a record should be retained in DynamoDB before being cleaned up

This should be significantly larger than the configured lock timeout, with the default value of 1 hour appropriate for most use-cases.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#impl-Clone-for-DynamoCommit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#impl-Debug-for-DynamoCommit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#impl-PartialEq-for-DynamoCommit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#impl-Eq-for-DynamoCommit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#impl-StructuralPartialEq-for-DynamoCommit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html#blanket-implementations" class="anchor">§</a>
