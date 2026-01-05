# Trait JoinDispatch Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/hash_join/mod.rs.html#55" class="src">Source</a>

``` rust
pub trait JoinDispatch: IntoDf {
    // Provided methods
    unsafe fn create_left_df_chunked(
        &self,
        chunk_ids: &[ChunkId],
        left_join: bool,
        was_sliced: bool,
    ) -> DataFrame { ... }
    unsafe fn _create_left_df_from_slice(
        &self,
        join_tuples: &[u32],
        left_join: bool,
        was_sliced: bool,
        sorted_tuple_idx: bool,
    ) -> DataFrame { ... }
    unsafe fn _finish_anti_semi_join(
        &self,
        idx: &[u32],
        slice: Option<(i64, usize)>,
    ) -> DataFrame { ... }
    fn _semi_anti_join_from_series(
        &self,
        s_left: &Series,
        s_right: &Series,
        slice: Option<(i64, usize)>,
        anti: bool,
        nulls_equal: bool,
    ) -> Result<DataFrame, PolarsError> { ... }
    fn _full_join_from_series(
        &self,
        other: &DataFrame,
        s_left: &Series,
        s_right: &Series,
        args: JoinArgs,
    ) -> Result<DataFrame, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#provided-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method.create_left_df_chunked" class="fn">create_left_df_chunked</a>( &self, chunk_ids: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\], left_join: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, was_sliced: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#safety" class="doc-anchor">§</a>Safety

Join tuples must be in bounds

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._create_left_df_from_slice" class="fn">_create_left_df_from_slice</a>( &self, join_tuples: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], left_join: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, was_sliced: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, sorted_tuple_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#safety-1" class="doc-anchor">§</a>Safety

Join tuples must be in bounds

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._finish_anti_semi_join" class="fn">_finish_anti_semi_join</a>( &self, idx: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#safety-2" class="doc-anchor">§</a>Safety

`idx` must be in bounds

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._semi_anti_join_from_series" class="fn">_semi_anti_join_from_series</a>( &self, s_left: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, s_right: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, anti: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._full_join_from_series" class="fn">_full_join_from_series</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, s_left: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, s_right: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinArgs.html" class="struct" title="struct polars::prelude::JoinArgs">JoinArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#impl-JoinDispatch-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html" class="trait" title="trait polars::prelude::JoinDispatch">JoinDispatch</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
