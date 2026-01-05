# Function try_for_each_valid_idx Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_iterator.rs.html#307-313" class="src">Source</a>

``` rust
pub fn try_for_each_valid_idx<E, F>(
    len: usize,
    offset: usize,
    null_count: usize,
    nulls: Option<&[u8]>,
    f: F,
) -> Result<(), E>where
    F: FnMut(usize) -> Result<(), E>,
```

Expand description

Calls the provided closure for each index in the provided null mask that is set, using an adaptive strategy based on the null count

Ideally this would be encapsulated in an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") that would determine the optimal strategy up front, and then yield indexes based on this.

Unfortunately, external iteration based on the resulting [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") would match the strategy variant on each call to [`Iterator::next`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next "method core::iter::traits::iterator::Iterator::next"), and LLVM generally cannot eliminate this.

One solution to this might be internal iteration, e.g. [`Iterator::try_fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold "method core::iter::traits::iterator::Iterator::try_fold"), however, it is currently [not possible](https://github.com/rust-lang/rust/issues/69595) to override this for custom iterators in stable Rust.

As such this is the next best option
