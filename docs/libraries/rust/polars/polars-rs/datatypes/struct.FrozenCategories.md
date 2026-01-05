# Struct FrozenCategories Copy item path

<a href="https://docs.rs/polars-dtype/0.51.0/x86_64-unknown-linux-gnu/src/polars_dtype/categorical/mod.rs.html#231" class="src">Source</a>

``` rust
pub struct FrozenCategories { /* private fields */ }
```

Expand description

An ordered collection of unique strings with an associated pre-computed mapping to go from string \<-\> index.

FrozenCategories are globally unique to facilitate constant-time comparison.

## Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#impl-FrozenCategories" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.new" class="fn">new</a>\<'a, I\>(strings: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Creates a new FrozenCategories object (or returns a reference to an existing one in case these are already known). Returns an error if the categories are not unique. It is guaranteed that the nth string ends up with category n (0-indexed).

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.categories" class="fn">categories</a>(&self) -\> &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

The categories contained in this FrozenCategories object.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.physical" class="fn">physical</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::CategoricalPhysical">CategoricalPhysical</a>

The physical dtype of the category ids.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.mapping" class="fn">mapping</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>

The mapping for this FrozenCategories object.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.hash" class="fn">hash</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

A stable hash of the categories.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#impl-Debug-for-FrozenCategories" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#impl-Drop-for-FrozenCategories" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.FrozenCategories.html#blanket-implementations" class="anchor">§</a>
