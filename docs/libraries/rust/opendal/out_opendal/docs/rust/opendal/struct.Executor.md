# Struct Executor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/execute/executor.rs.html#37-39" class="src">Source</a>

``` rust
pub struct Executor { /* private fields */ }
```

Expand description

Executor that runs futures in background.

Executor is created by users and used by opendal. So itâ€™s by design that Executor only expose constructor methods.

Executor will run futures in background and return a `Task` as handle to the future. Users can call `task.await` to wait for the future to complete or drop the `Task` to cancel it.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#impl-Executor" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#method.new" class="fn">new</a>() -\> Self

Create a default executor.

The default executor is enabled by feature flags. If no feature flags enabled, the default executor will always return error if users try to perform concurrent tasks.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#method.with" class="fn">with</a>(exec: impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html" class="trait" title="trait opendal::Execute">Execute</a>) -\> Self

Create a new executor with given execute impl.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#impl-Clone-for-Executor" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#impl-Debug-for-Executor" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#impl-Default-for-Executor" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html#blanket-implementations" class="anchor">Â§</a>
