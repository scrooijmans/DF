# Trait ControlFlow Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/dfsvisit.rs.html#76-80" class="src">Source</a>

``` rust
pub trait ControlFlow {
    // Required methods
    fn continuing() -> Self;
    fn should_break(&self) -> bool;
    fn should_prune(&self) -> bool;
}
```

Expand description

Control flow for callbacks.

The empty return value `()` is equivalent to continue.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.continuing" class="fn">continuing</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_break" class="fn">should_break</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_prune" class="fn">should_prune</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#impl-ControlFlow-for-()" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html" class="trait" title="trait petgraph::visit::ControlFlow">ControlFlow</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#method.continuing" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.continuing" class="fn">continuing</a>()

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#method.should_break" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_break" class="fn">should_break</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#method.should_prune" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_prune" class="fn">should_prune</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#impl-ControlFlow-for-Result%3CC,+E%3E" class="anchor">§</a>

### impl\<C: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html" class="trait" title="trait petgraph::visit::ControlFlow">ControlFlow</a>, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html" class="trait" title="trait petgraph::visit::ControlFlow">ControlFlow</a> for <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<C, E\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#method.continuing-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.continuing" class="fn">continuing</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#method.should_break-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_break" class="fn">should_break</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#method.should_prune-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_prune" class="fn">should_prune</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#impl-ControlFlow-for-Control%3CB%3E" class="anchor">§</a>

### impl\<B\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html" class="trait" title="trait petgraph::visit::ControlFlow">ControlFlow</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>
