# Enum Capacities Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/transform/mod.rs.html#360" class="src">Source</a>

``` rust
pub enum Capacities {
    Binary(usize, Option<usize>),
    List(usize, Option<Box<Capacities>>),
    Struct(usize, Option<Vec<Capacities>>),
    Dictionary(usize, Option<Box<Capacities>>),
    Array(usize),
}
```

Expand description

Define capacities to pre-allocate for child data or data buffers.

## Variants<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#variants" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#variant.Binary" class="anchor">§</a>

### Binary(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Binary, Utf8 and LargeUtf8 data types

Defines

- the capacity of the array offsets
- the capacity of the binary/ str buffer

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>\>\>)

List and LargeList data types

Defines

- the capacity of the array offsets
- the capacity of the child data

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>\>\>)

Struct type

Defines

- the capacity of the array
- the capacities of the fields

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#variant.Dictionary" class="anchor">§</a>

### Dictionary(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>\>\>)

Dictionary type

Defines

- the capacity of the array/keys
- the capacity of the values

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#variant.Array" class="anchor">§</a>

### Array(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Don’t preallocate inner buffers and rely on array growth strategy

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#impl-Clone-for-Capacities" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#impl-Debug-for-Capacities" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html#blanket-implementations" class="anchor">§</a>
