# Trait IntoColumn Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/column/mod.rs.html#50" class="src">Source</a>

``` rust
pub trait IntoColumn: Sized {
    // Required method
    fn into_column(self) -> Column;
}
```

Expand description

Convert `Self` into a [`Column`](https://docs.rs/polars/latest/polars/prelude/enum.Column.html "enum polars::prelude::Column")

## Required Methods<a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#tymethod.into_column" class="fn">into_column</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#impl-IntoColumn-for-Column" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#impl-IntoColumn-for-ScalarColumn" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/trait.IntoColumn.html#impl-IntoColumn-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a> for T

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a>,
