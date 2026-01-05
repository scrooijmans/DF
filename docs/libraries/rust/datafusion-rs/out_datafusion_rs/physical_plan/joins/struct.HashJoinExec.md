# Struct HashJoinExec Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/joins/hash_join/exec.rs.html#321" class="src">Source</a>

``` rust
pub struct HashJoinExec {
    pub left: Arc<dyn ExecutionPlan>,
    pub right: Arc<dyn ExecutionPlan>,
    pub on: Vec<(Arc<dyn PhysicalExpr>, Arc<dyn PhysicalExpr>)>,
    pub filter: Option<JoinFilter>,
    pub join_type: JoinType,
    pub mode: PartitionMode,
    pub projection: Option<Vec<usize>>,
    pub null_equality: NullEquality,
    /* private fields */
}
```

Expand description

Join execution plan: Evaluates equijoin predicates in parallel on multiple partitions using a hash table and an optional filter list to apply post join.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#join-expressions" class="doc-anchor">§</a>Join Expressions

This implementation is optimized for evaluating equijoin predicates ( `<col1> = <col2>`) expressions, which are represented as a list of `Columns` in [`Self::on`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.on "method datafusion::physical_plan::joins::HashJoinExec::on").

Non-equality predicates, which can not pushed down to a join inputs (e.g. `<col1> != <col2>`) are known as “filter expressions” and are evaluated after the equijoin predicates.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#build-side-vs-probe-side" class="doc-anchor">§</a>“Build Side” vs “Probe Side”

HashJoin takes two inputs, which are referred to as the “build” and the “probe”. The build side is the first child, and the probe side is the second child.

The two inputs are treated differently and it is VERY important that the *smaller* input is placed on the build side to minimize the work of creating the hash table.

``` text
         ┌───────────┐
         │ HashJoin  │
         │           │
         └───────────┘
             │   │
       ┌─────┘   └─────┐
       ▼               ▼
┌────────────┐  ┌─────────────┐
│   Input    │  │    Input    │
│    [0]     │  │     [1]     │
└────────────┘  └─────────────┘

 "build side"    "probe side"
```

Execution proceeds in 2 stages:

1.  the **build phase** creates a hash table from the tuples of the build side, and single concatenated batch containing data from all fetched record batches. Resulting hash table stores hashed join-key fields for each row as a key, and indices of corresponding rows in concatenated batch.

Hash join uses LIFO data structure as a hash table, and in order to retain original build-side input order while obtaining data during probe phase, hash table is updated by iterating batch sequence in reverse order – it allows to keep rows with smaller indices “on the top” of hash table, and still maintain correct indexing for concatenated build-side data batch.

Example of build phase for 3 record batches:

``` text

 Original build-side data   Inserting build-side values into hashmap    Concatenated build-side batch
                                                                        ┌───────────────────────────┐
                            hashmap.insert(row-hash, row-idx + offset)  │                      idx  │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 1 │        1) update_hash for batch 3 with offset 0    │          │ Row 6 │    0   │
  Batch 1  │       │           - hashmap.insert(Row 7, idx 1)           │ Batch 3  │       │        │
           │ Row 2 │           - hashmap.insert(Row 6, idx 0)           │          │ Row 7 │    1   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 3 │        2) update_hash for batch 2 with offset 2    │          │ Row 3 │    2   │
           │       │           - hashmap.insert(Row 5, idx 4)           │          │       │        │
  Batch 2  │ Row 4 │           - hashmap.insert(Row 4, idx 3)           │ Batch 2  │ Row 4 │    3   │
           │       │           - hashmap.insert(Row 3, idx 2)           │          │       │        │
           │ Row 5 │                                                    │          │ Row 5 │    4   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 6 │        3) update_hash for batch 1 with offset 5    │          │ Row 1 │    5   │
  Batch 3  │       │           - hashmap.insert(Row 2, idx 6)           │ Batch 1  │       │        │
           │ Row 7 │           - hashmap.insert(Row 1, idx 5)           │          │ Row 2 │    6   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
                                                                        └───────────────────────────┘
```

2.  the **probe phase** where the tuples of the probe side are streamed through, checking for matches of the join keys in the hash table.

``` text
                ┌────────────────┐          ┌────────────────┐
                │ ┌─────────┐    │          │ ┌─────────┐    │
                │ │  Hash   │    │          │ │  Hash   │    │
                │ │  Table  │    │          │ │  Table  │    │
                │ │(keys are│    │          │ │(keys are│    │
                │ │equi join│    │          │ │equi join│    │  Stage 2: batches from
 Stage 1: the   │ │columns) │    │          │ │columns) │    │    the probe side are
*entire* build  │ │         │    │          │ │         │    │  streamed through, and
 side is read   │ └─────────┘    │          │ └─────────┘    │   checked against the
into the hash   │      ▲         │          │          ▲     │   contents of the hash
    table       │       HashJoin │          │  HashJoin      │          table
                └──────┼─────────┘          └──────────┼─────┘
            ─ ─ ─ ─ ─ ─                                 ─ ─ ─ ─ ─ ─ ─
           │                                                         │

           │                                                         │
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘
          ...                                                       ...
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘

       build side                                                probe side
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#example-optimal-plans" class="doc-anchor">§</a>Example “Optimal” Plans

The differences in the inputs means that for classic “Star Schema Query”, the optimal plan will be a **“Right Deep Tree”** . A Star Schema Query is one where there is one large table and several smaller “dimension” tables, joined on `Foreign Key = Primary Key` predicates.

A “Right Deep Tree” looks like this large table as the probe side on the lowest join:

``` text
            ┌───────────┐
            │ HashJoin  │
            │           │
            └───────────┘
                │   │
        ┌───────┘   └──────────┐
        ▼                      ▼
┌───────────────┐        ┌───────────┐
│ small table 1 │        │ HashJoin  │
│  "dimension"  │        │           │
└───────────────┘        └───┬───┬───┘
                  ┌──────────┘   └───────┐
                  │                      │
                  ▼                      ▼
          ┌───────────────┐        ┌───────────┐
          │ small table 2 │        │ HashJoin  │
          │  "dimension"  │        │           │
          └───────────────┘        └───┬───┬───┘
                              ┌────────┘   └────────┐
                              │                     │
                              ▼                     ▼
                      ┌───────────────┐     ┌───────────────┐
                      │ small table 3 │     │  large table  │
                      │  "dimension"  │     │    "fact"     │
                      └───────────────┘     └───────────────┘
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#clone--shared-state" class="doc-anchor">§</a>Clone / Shared State

Note this structure includes a \[`OnceAsync`\] that is used to coordinate the loading of the left side with the processing in each output stream. Therefore it can not be [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.left" class="anchor field">§</a>`left: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan"><code>ExecutionPlan</code></a>`>`

left (build) side which gets hashed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.right" class="anchor field">§</a>`right: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan"><code>ExecutionPlan</code></a>`>`

right (probe) side which are filtered by the hash table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.on" class="anchor field">§</a>`on: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>, `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>)>`

Set of equijoin columns from the relations: `(left_col, right_col)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.filter" class="anchor field">§</a>`filter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html" class="struct" title="struct datafusion::physical_plan::joins::utils::JoinFilter"><code>JoinFilter</code></a>`>`

Filters which are applied while finding matching rows

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.join_type" class="anchor field">§</a>`join_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType"><code>JoinType</code></a>

How the join is performed (`OUTER`, `INNER`, etc)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.mode" class="anchor field">§</a>`mode: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/enum.PartitionMode.html" class="enum" title="enum datafusion::physical_plan::joins::PartitionMode"><code>PartitionMode</code></a>

Partitioning mode to use

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.projection" class="anchor field">§</a>`projection: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>>`

The projection indices of the columns in the output schema of join

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#structfield.null_equality" class="anchor field">§</a>`null_equality: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality"><code>NullEquality</code></a>

The equality null-handling behavior of the join algorithm.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#impl-HashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.try_new" class="fn">try_new</a>( left: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, right: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, on: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>)\>, filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html" class="struct" title="struct datafusion::physical_plan::joins::utils::JoinFilter">JoinFilter</a>\>, join_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, partition_mode: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/enum.PartitionMode.html" class="enum" title="enum datafusion::physical_plan::joins::PartitionMode">PartitionMode</a>, null_equality: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality">NullEquality</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tries to create a new [HashJoinExec](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html "struct datafusion::physical_plan::joins::HashJoinExec").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#error" class="doc-anchor">§</a>Error

This function errors when it is not possible to join the left and right sides on keys `on`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.left" class="fn">left</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>

left (build) side which gets hashed

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.right" class="fn">right</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>

right (probe) side which are filtered by the hash table

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.on" class="fn">on</a>(&self) -\> &\[(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>)\]

Set of common columns used to join on

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.filter" class="fn">filter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html" class="struct" title="struct datafusion::physical_plan::joins::utils::JoinFilter">JoinFilter</a>\>

Filters applied before join output

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.join_type" class="fn">join_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>

How the join is performed

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.join_schema" class="fn">join_schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

The schema after join. Please be careful when using this schema, if there is a projection, the schema isn’t the same as the output schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.partition_mode" class="fn">partition_mode</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/enum.PartitionMode.html" class="enum" title="enum datafusion::physical_plan::joins::PartitionMode">PartitionMode</a>

The partitioning mode of this hash join

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.null_equality" class="fn">null_equality</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality">NullEquality</a>

Get null_equality

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.probe_side" class="fn">probe_side</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

Get probe side information for the hash join.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.contains_projection" class="fn">contains_projection</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return whether the join contains a projection

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.with_projection" class="fn">with_projection</a>( &self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return new instance of [HashJoinExec](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html "struct datafusion::physical_plan::joins::HashJoinExec") with the given projection.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.swap_inputs" class="fn">swap_inputs</a>( &self, partition_mode: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/enum.PartitionMode.html" class="enum" title="enum datafusion::physical_plan::joins::PartitionMode">PartitionMode</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a new `ExecutionPlan` that computes the same join as this one, with the left and right inputs swapped using the specified `partition_mode`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#notes" class="doc-anchor">§</a>Notes:

This function is public so other downstream projects can use it to construct `HashJoinExec` with right side as the build side.

For using this interface directly, please refer to below:

Hash join execution may require specific input partitioning (for example, the left child may have a single partition while the right child has multiple).

Calling this function on join nodes whose children have already been repartitioned (e.g., after a `RepartitionExec` has been inserted) may break the partitioning requirements of the hash join. Therefore, ensure you call this function before inserting any repartitioning operators on the join’s children.

In DataFusion’s default SQL interface, this function is used by the `JoinSelection` physical optimizer rule to determine a good join order, which is executed before the `EnforceDistribution` rule (the rule that may insert `RepartitionExec` operators).

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#impl-Debug-for-HashJoinExec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#impl-DisplayAs-for-HashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.fmt_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format according to `DisplayFormatType`, used when verbose representation looks different from the default one [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#impl-EmbeddedProjection-for-HashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/trait.EmbeddedProjection.html" class="trait" title="trait datafusion::physical_plan::projection::EmbeddedProjection">EmbeddedProjection</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.with_projection-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/trait.EmbeddedProjection.html#tymethod.with_projection" class="fn">with_projection</a>( &self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#impl-ExecutionPlan-for-HashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.with_new_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children" class="fn">with_new_children</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Creates a new HashJoinExec with different children while preserving configuration.

This method is called during query optimization when the optimizer creates new plan nodes. Importantly, it creates a fresh bounds_accumulator via `try_new` rather than cloning the existing one because partitioning may have changed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.try_swapping_with_projection" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.try_swapping_with_projection" class="fn">try_swapping_with_projection</a>( &self, projection: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tries to push `projection` down through `hash_join`. If possible, performs the pushdown and returns a new [`HashJoinExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html "struct datafusion::physical_plan::joins::HashJoinExec") as the top plan which has projections as its children. Otherwise, returns `None`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name" class="fn">name</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Short name for the ExecutionPlan, such as ‘DataSourceExec’. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the execution plan as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.properties" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties" class="fn">properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Return properties of the output of the `ExecutionPlan`, such as output ordering(s), partitioning information etc. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.required_input_distribution" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_distribution" class="fn">required_input_distribution</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>\>

Specifies the data distribution requirements for all the children for this `ExecutionPlan`, By default it’s \[[Distribution::UnspecifiedDistribution](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.UnspecifiedDistribution "variant datafusion::physical_expr::Distribution::UnspecifiedDistribution")\] for each child,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.maintains_input_order" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order" class="fn">maintains_input_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns `false` if this `ExecutionPlan`’s implementation may reorder rows within or between partitions. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.children" class="fn">children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Get a list of children `ExecutionPlan`s that act as inputs to this plan. The returned list will be empty for leaf nodes such as scans, will contain a single value for unary nodes, or two values for binary nodes (such as joins).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.reset_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state" class="fn">reset_state</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reset any internal state within this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.execute" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute" class="fn">execute</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, context: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Begin execution of `partition`, returning a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.metrics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Return a snapshot of the set of [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s for this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). If no `Metric`s are available, return None. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 48.0.0: Use `partition_statistics` method instead

Returns statistics for this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.partition_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.partition_statistics" class="fn">partition_statistics</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns statistics for a specific partition of this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. If `partition` is `None`, it returns statistics for the entire plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.gather_filters_for_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown" class="fn">gather_filters_for_pushdown</a>( &self, phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, parent_filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterDescription">FilterDescription</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Collect filters that this node can push down to its children. Filters that are being pushed down from parents are passed in, and the node may generate additional filters to push down. For example, given the plan FilterExec -\> HashJoinExec -\> DataSourceExec, what will happen is that we recurse down the plan calling `ExecutionPlan::gather_filters_for_pushdown`: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.handle_child_pushdown_result" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result" class="fn">handle_child_pushdown_result</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Handle the result of a child pushdown. This method is called as we recurse back up the plan tree after pushing filters down to child nodes via [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown"). It allows the current node to process the results of filter pushdown from its children, deciding whether to absorb filters, modify the plan, or pass filters back up to its parent. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.static_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.static_name" class="fn">static_name</a>() -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Short name for the ExecutionPlan, such as ‘DataSourceExec’. Like [`name`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name "method datafusion::physical_plan::ExecutionPlan::name") but can be called without an instance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get the schema for this execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.check_invariants" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants" class="fn">check_invariants</a>(&self, check: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.InvariantLevel.html" class="enum" title="enum datafusion::physical_plan::execution_plan::InvariantLevel">InvariantLevel</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns an error if this individual node does not conform to its invariants. These invariants are typically only checked in debug mode. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.required_input_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering" class="fn">required_input_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.OrderingRequirements.html" class="enum" title="enum datafusion::physical_expr::OrderingRequirements">OrderingRequirements</a>\>\>

Specifies the ordering required for all of the children of this `ExecutionPlan`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.benefits_from_input_partitioning" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning" class="fn">benefits_from_input_partitioning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Specifies whether the `ExecutionPlan` benefits from increased parallelization at its input for each child. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.repartitioned" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned" class="fn">repartitioned</a>( &self, \_target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported, attempt to increase the partitioning of this `ExecutionPlan` to produce `target_partitions` partitions. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.supports_limit_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown" class="fn">supports_limit_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if a limit can be safely pushed down through this `ExecutionPlan` node. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.with_fetch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_fetch" class="fn">with_fetch</a>(&self, \_limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Returns a fetching variant of this `ExecutionPlan` node, if it supports fetch limits. Returns `None` otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.fetch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.fetch" class="fn">fetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Gets the fetch count for the operator, `None` means there is no fetch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.cardinality_effect" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.cardinality_effect" class="fn">cardinality_effect</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html" class="enum" title="enum datafusion::physical_plan::execution_plan::CardinalityEffect">CardinalityEffect</a>

Gets the effect on cardinality, if known

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.with_new_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state" class="fn">with_new_state</a>( &self, \_state: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Injects arbitrary run-time state into this execution plan, returning a new plan instance that incorporates that state *if* it is relevant to the concrete node implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#blanket-implementations" class="anchor">§</a>
