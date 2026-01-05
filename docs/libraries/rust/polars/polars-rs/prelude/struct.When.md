# Struct When Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/arity.rs.html#9" class="src">Source</a>

``` rust
pub struct When { /* private fields */ }
```

Available on **crate feature `lazy`** only.

Expand description

Utility struct for the `when-then-otherwise` expression.

Represents the state of the expression after [when](https://docs.rs/polars/latest/polars/prelude/fn.when.html "fn polars::prelude::when") is called.

In this state, `then` must be called to continue to finish the expression.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#impl-When" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html" class="struct" title="struct polars::prelude::When">When</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#method.then" class="fn">then</a>\<E\>(self, expr: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Then.html" class="struct" title="struct polars::prelude::Then">Then</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Add a condition to the `when-then-otherwise` expression.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#impl-Clone-for-When" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html" class="struct" title="struct polars::prelude::When">When</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html" class="struct" title="struct polars::prelude::When">When</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.When.html#blanket-implementations" class="anchor">§</a>
