# Trait PolarsTemporalGroupby Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/group_by/dynamic.rs.html#85" class="src">Source</a>

``` rust
pub trait PolarsTemporalGroupby {
    // Required methods
    fn rolling(
        &self,
        group_by: Option<Vec<[u32; 2]>>,
        options: &RollingGroupOptions,
    ) -> Result<(Column, GroupPositions), PolarsError>;
    fn group_by_dynamic(
        &self,
        group_by: Option<Vec<[u32; 2]>>,
        options: &DynamicGroupOptions,
    ) -> Result<(Column, Vec<Column>, GroupPositions), PolarsError>;
}
```

Available on **crate feature `lazy`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#tymethod.rolling" class="fn">rolling</a>( &self, group_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>\>, options: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingGroupOptions.html" class="struct" title="struct polars::prelude::RollingGroupOptions">RollingGroupOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#tymethod.group_by_dynamic" class="fn">group_by_dynamic</a>( &self, group_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>\>, options: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#impl-PolarsTemporalGroupby-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html" class="trait" title="trait polars::prelude::PolarsTemporalGroupby">PolarsTemporalGroupby</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
