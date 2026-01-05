# Struct SortOptions Copy item path

<a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/src/arrow_schema/lib.rs.html#84" class="src">Source</a>

``` rust
pub struct SortOptions {
    pub descending: bool,
    pub nulls_first: bool,
}
```

Expand description

Options that define the sort order of a given column

The default sorts equivalently to of `ASC NULLS FIRST` in SQL (i.e. ascending order with nulls sorting before any other values).

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#example-creation" class="doc-anchor">§</a>Example creation

``` rust
// configure using explicit initialization
let options = SortOptions {
  descending: false,
  nulls_first: true,
};
// Default is ASC NULLs First
assert_eq!(options, SortOptions::default());
assert_eq!(options.to_string(), "ASC NULLS FIRST");

// Configure using builder APIs
let options = SortOptions::default()
 .desc()
 .nulls_first();
assert_eq!(options.to_string(), "DESC NULLS FIRST");

// configure using explicit field values
let options = SortOptions::default()
 .with_descending(false)
 .with_nulls_first(false);
assert_eq!(options.to_string(), "ASC NULLS LAST");
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#example-operations" class="doc-anchor">§</a>Example operations

It is also possible to negate the sort options using the `!` operator.

``` rust
use arrow_schema::SortOptions;
let options = !SortOptions::default();
assert_eq!(options.to_string(), "DESC NULLS LAST");
```

## Fields<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#structfield.descending" class="anchor field">§</a>`descending: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to sort in descending order

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#structfield.nulls_first" class="anchor field">§</a>`nulls_first: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to sort nulls first

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-SortOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.new" class="fn">new</a>(descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Create a new `SortOptions` struct

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.desc" class="fn">desc</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Set this sort options to sort in descending order

See [Self::with_descending](https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html#method.with_descending "method arrow::compute::SortOptions::with_descending") to explicitly set the underlying field

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.asc" class="fn">asc</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Set this sort options to sort in ascending order

See [Self::with_descending](https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html#method.with_descending "method arrow::compute::SortOptions::with_descending") to explicitly set the underlying field

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.nulls_first" class="fn">nulls_first</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Set this sort options to sort nulls first

See [Self::with_nulls_first](https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html#method.with_nulls_first "method arrow::compute::SortOptions::with_nulls_first") to explicitly set the underlying field

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.nulls_last" class="fn">nulls_last</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Set this sort options to sort nulls last

See [Self::with_nulls_first](https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html#method.with_nulls_first "method arrow::compute::SortOptions::with_nulls_first") to explicitly set the underlying field

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.with_descending" class="fn">with_descending</a>(self, descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Set this sort options to sort descending if argument is true

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.with_nulls_first" class="fn">with_nulls_first</a>(self, nulls_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Set this sort options to sort nulls first if argument is true

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Clone-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Debug-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Default-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Display-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Hash-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Not-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

`!` operator is overloaded for `SortOptions` to invert boolean fields of the struct.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.not" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Ord-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-PartialEq-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-PartialOrd-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Copy-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-Eq-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#impl-StructuralPartialEq-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortOptions.html" class="struct" title="struct arrow::compute::SortOptions">SortOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html#blanket-implementations" class="anchor">§</a>
