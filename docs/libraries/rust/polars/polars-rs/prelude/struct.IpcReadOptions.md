# Struct IpcReadOptions Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/ipc_reader_async.rs.html#30" class="src">Source</a>

``` rust
pub struct IpcReadOptions { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#impl-IpcReadOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.with_projection" class="fn">with_projection</a>( self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\]\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.with_row_limit" class="fn">with_row_limit</a>( self, row_limit: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.with_row_index" class="fn">with_row_index</a>( self, row_index: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.with_predicate" class="fn">with_predicate</a>( self, predicate: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/predicates/trait.PhysicalIoExpr.html" class="trait" title="trait polars_io::predicates::PhysicalIoExpr">PhysicalIoExpr</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#impl-Clone-for-IpcReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#impl-Default-for-IpcReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html#blanket-implementations" class="anchor">§</a>
