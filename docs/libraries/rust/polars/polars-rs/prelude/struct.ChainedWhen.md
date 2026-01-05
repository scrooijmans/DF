# Struct ChainedWhen Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/arity.rs.html#28" class="src">Source</a>

``` rust
pub struct ChainedWhen { /* private fields */ }
```

Available on **crate feature `lazy`** only.

Expand description

Utility struct for the `when-then-otherwise` expression.

Represents the state of the expression after an additional `when` is called.

In this state, `then` must be called to continue to finish the expression.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#impl-ChainedWhen" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html" class="struct" title="struct polars::prelude::ChainedWhen">ChainedWhen</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#method.then" class="fn">then</a>\<E\>(self, statement: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedThen.html" class="struct" title="struct polars::prelude::ChainedThen">ChainedThen</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#impl-Clone-for-ChainedWhen" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html" class="struct" title="struct polars::prelude::ChainedWhen">ChainedWhen</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html" class="struct" title="struct polars::prelude::ChainedWhen">ChainedWhen</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html#blanket-implementations" class="anchor">§</a>
