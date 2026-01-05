# Module series Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/lib.rs.html#30" class="src">Source</a>

Expand description

Type agnostic columnar data structure.

## Modules<a href="https://docs.rs/polars/latest/polars/series/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/index.html" class="mod" title="mod polars::series::amortized_iter">amortized_iter</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/index.html" class="mod" title="mod polars::series::arithmetic">arithmetic</a>

<a href="https://docs.rs/polars/latest/polars/series/builder/index.html" class="mod" title="mod polars::series::builder">builder</a>

<a href="https://docs.rs/polars/latest/polars/series/implementations/index.html" class="mod" title="mod polars::series::implementations">implementations</a>

<a href="https://docs.rs/polars/latest/polars/series/ops/index.html" class="mod" title="mod polars::series::ops">ops</a>

## Structs<a href="https://docs.rs/polars/latest/polars/series/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html" class="struct" title="struct polars::series::Series">Series</a>

Series

<a href="https://docs.rs/polars/latest/polars/series/struct.SeriesIter.html" class="struct" title="struct polars::series::SeriesIter">SeriesIter</a>

## Enums<a href="https://docs.rs/polars/latest/polars/series/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/enum.BitRepr.html" class="enum" title="enum polars::series::BitRepr">BitRepr</a>

<a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>

## Traits<a href="https://docs.rs/polars/latest/polars/series/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/trait.ChunkCompareEq.html" class="trait" title="trait polars::series::ChunkCompareEq">ChunkCompareEq</a>

Compare [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")’s and get a `boolean` mask that can be used to filter rows.

<a href="https://docs.rs/polars/latest/polars/series/trait.IntoSeries.html" class="trait" title="trait polars::series::IntoSeries">IntoSeries</a>

Used to convert a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), `&dyn SeriesTrait` and [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") into a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

<a href="https://docs.rs/polars/latest/polars/series/trait.SeriesTrait.html" class="trait" title="trait polars::series::SeriesTrait">SeriesTrait</a>

## Type Aliases<a href="https://docs.rs/polars/latest/polars/series/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/type.SeriesPhysIter.html" class="type" title="type polars::series::SeriesPhysIter">SeriesPhysIter</a>
