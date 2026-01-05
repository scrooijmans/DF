# Struct Then Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/arity.rs.html#17" class="src">Source</a>

``` rust
pub struct Then { /* private fields */ }
```

Available on **crate feature `lazy`** only.

Expand description

Utility struct for the `when-then-otherwise` expression.

Represents the state of the expression after `when(...).then(...)` is called.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#impl-Then" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html" class="struct" title="struct polars::prelude::Then">Then</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.when" class="fn">when</a>\<E\>(self, condition: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChainedWhen.html" class="struct" title="struct polars::prelude::ChainedWhen">ChainedWhen</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Attach a statement to the corresponding condition.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.otherwise" class="fn">otherwise</a>\<E\>(self, statement: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Define a default for the `when-then-otherwise` expression.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#impl-Clone-for-Then" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html" class="struct" title="struct polars::prelude::Then">Then</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html" class="struct" title="struct polars::prelude::Then">Then</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html#blanket-implementations" class="anchor">§</a>
