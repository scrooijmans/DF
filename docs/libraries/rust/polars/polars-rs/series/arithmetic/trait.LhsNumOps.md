# Trait LhsNumOps Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/arithmetic/borrowed.rs.html#831" class="src">Source</a>

``` rust
pub trait LhsNumOps {
    type Output;

    // Required methods
    fn add(self, rhs: &Series) -> Self::Output;
    fn sub(self, rhs: &Series) -> Self::Output;
    fn div(self, rhs: &Series) -> Self::Output;
    fn mul(self, rhs: &Series) -> Self::Output;
    fn rem(self, rem: &Series) -> Self::Output;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype">Output</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#tymethod.add" class="fn">add</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype" title="type polars::prelude::LhsNumOps::Output">Output</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#tymethod.sub" class="fn">sub</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype" title="type polars::prelude::LhsNumOps::Output">Output</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#tymethod.div" class="fn">div</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype" title="type polars::prelude::LhsNumOps::Output">Output</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#tymethod.mul" class="fn">mul</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype" title="type polars::prelude::LhsNumOps::Output">Output</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#tymethod.rem" class="fn">rem</a>(self, rem: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype" title="type polars::prelude::LhsNumOps::Output">Output</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#impl-LhsNumOps-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.LhsNumOps.html" class="trait" title="trait polars::prelude::LhsNumOps">LhsNumOps</a> for T

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/series/arithmetic/trait.LhsNumOps.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
