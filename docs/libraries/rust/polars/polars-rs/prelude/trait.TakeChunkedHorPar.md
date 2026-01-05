# Trait TakeChunkedHorPar Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/gather/chunked.rs.html#78" class="src">Source</a>

``` rust
pub trait TakeChunkedHorPar: IntoDf {
    // Provided methods
    unsafe fn _take_chunked_unchecked_hor_par<const B: u64>(
        &self,
        idx: &[ChunkId<B>],
        sorted: IsSorted,
    ) -> DataFrame { ... }
    unsafe fn _take_opt_chunked_unchecked_hor_par<const B: u64>(
        &self,
        idx: &[ChunkId<B>],
    ) -> DataFrame { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#provided-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#method._take_chunked_unchecked_hor_par" class="fn">_take_chunked_unchecked_hor_par</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, idx: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#safety" class="doc-anchor">§</a>Safety

Doesn’t perform any bound checks

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#method._take_opt_chunked_unchecked_hor_par" class="fn">_take_opt_chunked_unchecked_hor_par</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, idx: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#safety-1" class="doc-anchor">§</a>Safety

Doesn’t perform any bound checks

Check for null state in `ChunkId`.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#impl-TakeChunkedHorPar-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html" class="trait" title="trait polars::prelude::TakeChunkedHorPar">TakeChunkedHorPar</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
