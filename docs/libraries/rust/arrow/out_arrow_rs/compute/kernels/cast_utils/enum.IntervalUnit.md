# Enum IntervalUnit Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#1067" class="src">Source</a>

``` rust
#[repr(u16)]pub enum IntervalUnit {
    Century = 1,
    Decade = 2,
    Year = 4,
    Month = 8,
    Week = 16,
    Day = 32,
    Hour = 64,
    Minute = 128,
    Second = 256,
    Millisecond = 512,
    Microsecond = 1_024,
    Nanosecond = 2_048,
}
```

Expand description

Represents the units of an interval, with each variant corresponding to a bit in the interval’s bitfield representation

## Variants<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variants" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Century" class="anchor">§</a>

### Century = 1

A Century

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Decade" class="anchor">§</a>

### Decade = 2

A Decade

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Year" class="anchor">§</a>

### Year = 4

A Year

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Month" class="anchor">§</a>

### Month = 8

A Month

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Week" class="anchor">§</a>

### Week = 16

A Week

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Day" class="anchor">§</a>

### Day = 32

A Day

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Hour" class="anchor">§</a>

### Hour = 64

An Hour

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Minute" class="anchor">§</a>

### Minute = 128

A Minute

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Second" class="anchor">§</a>

### Second = 256

A Second

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Millisecond" class="anchor">§</a>

### Millisecond = 512

A Millisecond

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Microsecond" class="anchor">§</a>

### Microsecond = 1_024

A Microsecond

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#variant.Nanosecond" class="anchor">§</a>

### Nanosecond = 2_048

A Nanosecond

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#impl-Clone-for-IntervalUnit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#impl-Debug-for-IntervalUnit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#impl-FromStr-for-IntervalUnit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>

Logic for parsing interval unit strings

See <https://github.com/postgres/postgres/blob/2caa85f4aae689e6f6721d7363b4c66a2a6417d6/src/backend/utils/adt/datetime.c#L189> for a list of unit names supported by PostgreSQL which we try to match here.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#impl-Copy-for-IntervalUnit" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html#blanket-implementations" class="anchor">§</a>
