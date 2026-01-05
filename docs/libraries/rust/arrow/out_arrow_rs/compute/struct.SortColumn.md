# Struct SortColumn Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#835" class="src">Source</a>

``` rust
pub struct SortColumn {
    pub values: Arc<dyn Array>,
    pub options: Option<SortOptions>,
}
```

Expand description

One column to be used in lexicographical sort

## Fields<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#structfield.values" class="anchor field">§</a>`values: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array"><code>Array</code></a>`>`

The column to sort

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions"><code>SortOptions</code></a>`>`

Sort options for this column

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#impl-Clone-for-SortColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html" class="struct" title="struct arrow::compute::SortColumn">SortColumn</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html" class="struct" title="struct arrow::compute::SortColumn">SortColumn</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#impl-Debug-for-SortColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html" class="struct" title="struct arrow::compute::SortColumn">SortColumn</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html#blanket-implementations" class="anchor">§</a>
