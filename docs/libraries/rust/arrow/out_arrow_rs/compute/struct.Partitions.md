# Struct Partitions Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/partition.rs.html#31" class="src">Source</a>

``` rust
pub struct Partitions(/* private fields */);
```

Expand description

A computed set of partitions, see [`partition`](https://docs.rs/arrow/latest/arrow/compute/fn.partition.html "fn arrow::compute::partition")

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#impl-Partitions" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html" class="struct" title="struct arrow::compute::Partitions">Partitions</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#method.ranges" class="fn">ranges</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

Returns the range of each partition

Consecutive ranges will be contiguous: i.e \[`(a, b)` and `(b, c)`\], and `start = 0` and `end = self.len()` for the first and last range respectively

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of partitions

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this contains no partitions

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#impl-Clone-for-Partitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html" class="struct" title="struct arrow::compute::Partitions">Partitions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html" class="struct" title="struct arrow::compute::Partitions">Partitions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#impl-Debug-for-Partitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html" class="struct" title="struct arrow::compute::Partitions">Partitions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html#blanket-implementations" class="anchor">§</a>
