# Trait IntoLazy Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/mod.rs.html#44" class="src">Source</a>

``` rust
pub trait IntoLazy {
    // Required method
    fn lazy(self) -> LazyFrame;
}
```

Available on **crate feature `lazy`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#tymethod.lazy" class="fn">lazy</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#impl-IntoLazy-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html" class="trait" title="trait polars::prelude::IntoLazy">IntoLazy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#impl-IntoLazy-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html" class="trait" title="trait polars::prelude::IntoLazy">IntoLazy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>
