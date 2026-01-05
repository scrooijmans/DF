# Enum PartitionTargetCallbackResult Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/sink.rs.html#321" class="src">Source</a>

``` rust
pub enum PartitionTargetCallbackResult {
    Str(String),
    Dyn(SpecialEq<Arc<Mutex<Option<Box<dyn DynWriteable>>>>>),
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#variant.Str" class="anchor">§</a>

### Str(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#variant.Dyn" class="anchor">§</a>

### Dyn(<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq">SpecialEq</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/sync/poison/mutex/struct.Mutex.html" class="struct" title="struct std::sync::poison::mutex::Mutex">Mutex</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html" class="trait" title="trait polars::prelude::file::DynWriteable">DynWriteable</a>\>\>\>\>\>)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#impl-Clone-for-PartitionTargetCallbackResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html" class="enum" title="enum polars::prelude::PartitionTargetCallbackResult">PartitionTargetCallbackResult</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html" class="enum" title="enum polars::prelude::PartitionTargetCallbackResult">PartitionTargetCallbackResult</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallbackResult.html#blanket-implementations" class="anchor">§</a>
