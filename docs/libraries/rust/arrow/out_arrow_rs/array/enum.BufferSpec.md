# Enum BufferSpec Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/data.rs.html#1775" class="src">Source</a>

``` rust
pub enum BufferSpec {
    FixedWidth {
        byte_width: usize,
        alignment: usize,
    },
    VariableWidth,
    BitMap,
    AlwaysNull,
}
```

Expand description

Layout specification for a single data type buffer

## Variants<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variants" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.FixedWidth" class="anchor">§</a>

### FixedWidth

Each element is a fixed width primitive, with the given `byte_width` and `alignment`

`alignment` is the alignment required by Rust for an array of the corresponding primitive, see [`Layout::array`](https://doc.rust-lang.org/nightly/core/alloc/layout/struct.Layout.html#method.array "associated function core::alloc::layout::Layout::array") and [`std::mem::align_of`](https://doc.rust-lang.org/nightly/core/mem/fn.align_of.html "fn core::mem::align_of").

Arrow-rs requires that all buffers have at least this alignment, to allow for [slice](https://doc.rust-lang.org/nightly/alloc/slice/index.html "mod alloc::slice") based APIs. Alignment in excess of this is not required to allow for array slicing and interoperability with `Vec`, which cannot be over-aligned.

Note that these alignment requirements will vary between architectures

#### Fields

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.FixedWidth.field.byte_width" class="anchor field">§</a>`byte_width: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The width of each element in bytes

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.FixedWidth.field.alignment" class="anchor field">§</a>`alignment: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The alignment required by Rust for an array of the corresponding primitive

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.VariableWidth" class="anchor">§</a>

### VariableWidth

Variable width, such as string data for utf8 data

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.BitMap" class="anchor">§</a>

### BitMap

Buffer holds a bitmap.

Note: Unlike the C++ implementation, the null/validity buffer is handled specially rather than as another of the buffers in the spec, so this variant is only used for the Boolean type.

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#variant.AlwaysNull" class="anchor">§</a>

### AlwaysNull

Buffer is always null. Unused currently in Rust implementation, (used in C++ for Union type)

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#impl-Debug-for-BufferSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec">BufferSpec</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#impl-PartialEq-for-BufferSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec">BufferSpec</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec">BufferSpec</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#impl-Eq-for-BufferSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec">BufferSpec</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#impl-StructuralPartialEq-for-BufferSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec">BufferSpec</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html#blanket-implementations" class="anchor">§</a>
