# Enum Control Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/dfsvisit.rs.html#48-58" class="src">Source</a>

``` rust
pub enum Control<B> {
    Continue,
    Prune,
    Break(B),
}
```

Expand description

Control flow for `depth_first_search` callbacks.

## Variants<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#variants" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#variant.Continue" class="anchor">§</a>

### Continue

Continue the DFS traversal as normal.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#variant.Prune" class="anchor">§</a>

### Prune

Prune the current node from the DFS traversal. No more edges from this node will be reported to the callback. A `DfsEvent::Finish` for this node will still be reported. This can be returned in response to any `DfsEvent`, except `Finish`, which will panic.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#variant.Break" class="anchor">§</a>

### Break(B)

Stop the DFS traversal and return the provided value.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#impl-Control%3CB%3E" class="anchor">§</a>

### impl\<B\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.breaking" class="fn">breaking</a>() -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.break_value" class="fn">break_value</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>

Get the value in `Control::Break(_)`, if present.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#impl-Clone-for-Control%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#impl-ControlFlow-for-Control%3CB%3E" class="anchor">§</a>

### impl\<B\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html" class="trait" title="trait petgraph::visit::ControlFlow">ControlFlow</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.continuing" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.continuing" class="fn">continuing</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.should_break" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_break" class="fn">should_break</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.should_prune" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.ControlFlow.html#tymethod.should_prune" class="fn">should_prune</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#impl-Debug-for-Control%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#impl-Default-for-Control%3CB%3E" class="anchor">§</a>

### impl\<B\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

The default is `Continue`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#impl-Copy-for-Control%3CB%3E" class="anchor">§</a>

### impl\<B: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html" class="enum" title="enum petgraph::visit::Control">Control</a>\<B\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.Control.html#blanket-implementations" class="anchor">§</a>
