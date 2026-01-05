# Module partition Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/lib.rs.html#56" class="src">Source</a>

Expand description

Defines partition kernel for `ArrayRef`

## Structs<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/partition/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/partition/struct.Partitions.html" class="struct" title="struct arrow::compute::kernels::partition::Partitions">Partitions</a>  
A computed set of partitions, see [`partition`](https://docs.rs/arrow/latest/arrow/compute/fn.partition.html "fn arrow::compute::partition")

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/partition/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/partition/fn.partition.html" class="fn" title="fn arrow::compute::kernels::partition::partition">partition</a>  
Given a list of lexicographically sorted columns, computes the [`Partitions`](https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html "struct arrow::compute::Partitions"), where a partition consists of the set of consecutive rows with equal values
