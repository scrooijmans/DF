# Module iterator Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/mod.rs.html#22" class="src">Source</a>

## Modules<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/par/index.html" class="mod" title="mod polars::chunked_array::iterator::par">par</a>

## Structs<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/struct.BoolIterNoNull.html" class="struct" title="struct polars::chunked_array::iterator::BoolIterNoNull">BoolIterNoNull</a>  
The no null iterator for a [`BooleanArray`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html "struct polars_arrow::array::boolean::BooleanArray")

<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/struct.FixedSizeListIterNoNull.html" class="struct" title="struct polars::chunked_array::iterator::FixedSizeListIterNoNull">FixedSizeListIterNoNull</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/struct.SomeIterator.html" class="struct" title="struct polars::chunked_array::iterator::SomeIterator">SomeIterator</a>  
Wrapper struct to convert an iterator of type `T` into one of type [`Option<T>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"). It is useful to make the [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") trait, in which every iterator shall return an [`Option<T>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

## Traits<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/iterator/trait.PolarsIterator.html" class="trait" title="trait polars::chunked_array::iterator::PolarsIterator">PolarsIterator</a>  
A [`PolarsIterator`](https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html "trait polars::prelude::PolarsIterator") is an iterator over a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") which contains polars types. A [`PolarsIterator`](https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html "trait polars::prelude::PolarsIterator") must implement [`ExactSizeIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") and [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator").
