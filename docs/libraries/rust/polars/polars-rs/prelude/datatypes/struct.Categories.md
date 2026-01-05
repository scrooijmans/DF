# Struct Categories Copy item path

<a href="https://docs.rs/polars-dtype/0.51.0/x86_64-unknown-linux-gnu/src/polars_dtype/categorical/mod.rs.html#124" class="src">Source</a>

``` rust
pub struct Categories { /* private fields */ }
```

Expand description

A (named) object which is used to indicate which categorical data types have the same mapping.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#impl-Categories" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.new" class="fn">new</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, namespace: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, physical: <a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::CategoricalPhysical">CategoricalPhysical</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>\>

Creates a new Categories object with the given name, namespace and physical type if none exists, otherwise get a reference to an existing object with the same name, namespace and physical type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.global" class="fn">global</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>\>

Returns the global Categories.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.is_global" class="fn">is_global</a>(self: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this refers to the global categories.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.random" class="fn">random</a>( namespace: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, physical: <a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::CategoricalPhysical">CategoricalPhysical</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>\>

Generates a Categories with a random (UUID) name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

The name of this Categories object.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.namespace" class="fn">namespace</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

The namespace of this Categories object.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.physical" class="fn">physical</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::CategoricalPhysical">CategoricalPhysical</a>

The physical dtype of the category ids.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.hash" class="fn">hash</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

A stable hash of this Categories object, not the contained categories.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.mapping" class="fn">mapping</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>

The mapping for this Categories object. If no mapping currently exists it creates a new empty mapping.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.freeze" class="fn">freeze</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#impl-Debug-for-Categories" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#impl-Drop-for-Categories" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Categories.html#blanket-implementations" class="anchor">§</a>
