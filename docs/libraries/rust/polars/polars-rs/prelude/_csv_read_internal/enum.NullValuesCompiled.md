# Enum NullValuesCompiled Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/options.rs.html#413" class="src">Source</a>

``` rust
pub enum NullValuesCompiled {
    AllColumnsSingle(PlSmallStr),
    AllColumns(Vec<PlSmallStr>),
    Columns(Vec<PlSmallStr>),
}
```

Available on **crate feature `polars-io`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#variant.AllColumnsSingle" class="anchor">§</a>

### AllColumnsSingle(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

A single value that’s used for all columns

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#variant.AllColumns" class="anchor">§</a>

### AllColumns(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#variant.Columns" class="anchor">§</a>

### Columns(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>)

A different null value per column, computed from `NullValues::Named`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#impl-Clone-for-NullValuesCompiled" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html" class="enum" title="enum polars::prelude::_csv_read_internal::NullValuesCompiled">NullValuesCompiled</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html" class="enum" title="enum polars::prelude::_csv_read_internal::NullValuesCompiled">NullValuesCompiled</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#impl-Debug-for-NullValuesCompiled" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html" class="enum" title="enum polars::prelude::_csv_read_internal::NullValuesCompiled">NullValuesCompiled</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/enum.NullValuesCompiled.html#blanket-implementations" class="anchor">§</a>
