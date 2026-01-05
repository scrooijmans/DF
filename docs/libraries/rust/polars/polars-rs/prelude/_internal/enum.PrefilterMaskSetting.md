# Enum PrefilterMaskSetting Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/read_impl.rs.html#502" class="src">Source</a>

``` rust
pub enum PrefilterMaskSetting {
    Auto,
    Pre,
    Post,
}
```

Available on **crate feature `polars-io`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#variant.Auto" class="anchor">§</a>

### Auto

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#variant.Pre" class="anchor">§</a>

### Pre

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#variant.Post" class="anchor">§</a>

### Post

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#impl-PrefilterMaskSetting" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html" class="enum" title="enum polars::prelude::_internal::PrefilterMaskSetting">PrefilterMaskSetting</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#method.init_from_env" class="fn">init_from_env</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html" class="enum" title="enum polars::prelude::_internal::PrefilterMaskSetting">PrefilterMaskSetting</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#method.should_prefilter" class="fn">should_prefilter</a>( &self, prefilter_cost: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#impl-Clone-for-PrefilterMaskSetting" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html" class="enum" title="enum polars::prelude::_internal::PrefilterMaskSetting">PrefilterMaskSetting</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html" class="enum" title="enum polars::prelude::_internal::PrefilterMaskSetting">PrefilterMaskSetting</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#impl-Copy-for-PrefilterMaskSetting" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html" class="enum" title="enum polars::prelude::_internal::PrefilterMaskSetting">PrefilterMaskSetting</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_internal/enum.PrefilterMaskSetting.html#blanket-implementations" class="anchor">§</a>
