# Trait Measure Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#611" class="src">Source</a>

``` rust
pub trait Measure:
    Debug
    + PartialOrd
    + Add<Self, Output = Self>
    + Default
    + Clone { }
```

Expand description

Associated data that can be used for measures (such as length).

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.Measure.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.Measure.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.Measure.html#impl-Measure-for-M" class="anchor">§</a>

### impl\<M\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.Measure.html" class="trait" title="trait petgraph::algo::Measure">Measure</a> for M

where M: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<M, Output = M\> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,
