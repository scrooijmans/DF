# Struct AmortSeries Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/amortized_iter.rs.html#9" class="src">Source</a>

``` rust
pub struct AmortSeries { /* private fields */ }
```

Expand description

A [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") that amortizes a few allocations during iteration.

## Implementations<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#impl-AmortSeries" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.new" class="fn">new</a>(series: <a href="https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html" class="struct" title="struct alloc::rc::Rc">Rc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.deep_clone" class="fn">deep_clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.swap" class="fn">swap</a>(&mut self, array: &mut <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)

Swaps inner state with the `array`. Prefer `AmortSeries::with_array` as this restores the state.

##### <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#safety" class="doc-anchor">§</a>Safety

This swaps an underlying pointer that might be hold by other cloned series.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.with_array" class="fn">with_array</a>\<F, T\>(&mut self, array: &mut <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, f: F) -\> T

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>) -\> T,

Temporary swaps out the array, and restores the original state when application of the function `f` is done.

##### <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#safety-1" class="doc-anchor">§</a>Safety

Array must be from `Series` physical dtype.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#impl-AsRef%3CSeries%3E-for-AmortSeries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>

We don’t implement Deref so that the caller is aware of converting to Series

<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#impl-Clone-for-AmortSeries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>

<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html#blanket-implementations" class="anchor">§</a>
