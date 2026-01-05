# Struct InProcessQuery Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/exitable.rs.html#46" class="src">Source</a>

``` rust
pub struct InProcessQuery { /* private fields */ }
```

Available on **crate feature `lazy`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#impl-InProcessQuery" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html" class="struct" title="struct polars::prelude::InProcessQuery">InProcessQuery</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#method.cancel" class="fn">cancel</a>(&self)

Cancel the query at earliest convenience.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#method.fetch" class="fn">fetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>

Fetch the result.

If it is ready, a materialized DataFrame is returned. If it is not ready it will return `None`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#method.fetch_blocking" class="fn">fetch_blocking</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Await the result synchronously.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#impl-Clone-for-InProcessQuery" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html" class="struct" title="struct polars::prelude::InProcessQuery">InProcessQuery</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html" class="struct" title="struct polars::prelude::InProcessQuery">InProcessQuery</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#impl-Drop-for-InProcessQuery" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html" class="struct" title="struct polars::prelude::InProcessQuery">InProcessQuery</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html#blanket-implementations" class="anchor">§</a>
