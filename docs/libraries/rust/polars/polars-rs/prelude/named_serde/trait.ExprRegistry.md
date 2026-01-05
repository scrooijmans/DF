# Trait ExprRegistry Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/named_serde.rs.html#7" class="src">Source</a>

``` rust
pub trait ExprRegistry: Sync + Send {
    // Required method
    fn get_function(
        &self,
        name: &str,
        payload: &[u8],
    ) -> Option<Arc<dyn AnonymousColumnsUdf>>;
}
```

Available on **crate feature `lazy`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/named_serde/trait.ExprRegistry.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/named_serde/trait.ExprRegistry.html#tymethod.get_function" class="fn">get_function</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, payload: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf">AnonymousColumnsUdf</a>\>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/named_serde/trait.ExprRegistry.html#implementors" class="anchor">§</a>
