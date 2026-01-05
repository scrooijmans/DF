# Trait PolarsIterator Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/iterator/mod.rs.html#24-25" class="src">Source</a>

``` rust
pub trait PolarsIterator:
    ExactSizeIterator
    + DoubleEndedIterator
    + Send
    + Sync
    + TrustedLen { }
```

Expand description

A [`PolarsIterator`](https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html "trait polars::prelude::PolarsIterator") is an iterator over a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") which contains polars types. A [`PolarsIterator`](https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html "trait polars::prelude::PolarsIterator") must implement [`ExactSizeIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") and [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator").

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html#impl-TrustedLen-for-Box%3Cdyn+PolarsIterator%3CItem+=+I%3E%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html" class="trait" title="trait polars::prelude::PolarsIterator">PolarsIterator</a>\<Item = I\> + '\_\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html#impl-PolarsIterator-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html" class="trait" title="trait polars::prelude::PolarsIterator">PolarsIterator</a> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html" class="trait" title="trait core::iter::traits::double_ended::DoubleEndedIterator">DoubleEndedIterator</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Implement [`PolarsIterator`](https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html "trait polars::prelude::PolarsIterator") for every iterator that implements the needed traits.
