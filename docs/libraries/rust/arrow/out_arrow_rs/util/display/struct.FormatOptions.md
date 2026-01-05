# Struct FormatOptions Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/display.rs.html#57" class="src">Source</a>

``` rust
pub struct FormatOptions<'a> { /* private fields */ }
```

Expand description

Options for formatting arrays

By default nulls are formatted as `""` and temporal types formatted according to RFC3339

## Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Creates a new set of format options

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_display_error" class="fn">with_display_error</a>(self, safe: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

If set to `true` any formatting errors will be written to the output instead of being converted into a [`std::fmt::Error`](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_null" class="fn">with_null</a>(self, null: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the string used to represent a null

Defaults to `""`

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_date_format" class="fn">with_date_format</a>( self, date_format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the format used for [`DataType::Date32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Date32 "variant arrow::datatypes::DataType::Date32") columns

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_datetime_format" class="fn">with_datetime_format</a>( self, datetime_format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the format used for [`DataType::Date64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Date64 "variant arrow::datatypes::DataType::Date64") columns

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_timestamp_format" class="fn">with_timestamp_format</a>( self, timestamp_format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the format used for [`DataType::Timestamp`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Timestamp "variant arrow::datatypes::DataType::Timestamp") columns without a timezone

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_timestamp_tz_format" class="fn">with_timestamp_tz_format</a>( self, timestamp_tz_format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the format used for [`DataType::Timestamp`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Timestamp "variant arrow::datatypes::DataType::Timestamp") columns with a timezone

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_time_format" class="fn">with_time_format</a>( self, time_format: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the format used for [`DataType::Time32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Time32 "variant arrow::datatypes::DataType::Time32") and [`DataType::Time64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Time64 "variant arrow::datatypes::DataType::Time64") columns

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_duration_format" class="fn">with_duration_format</a>( self, duration_format: <a href="https://docs.rs/arrow/latest/arrow/util/display/enum.DurationFormat.html" class="enum" title="enum arrow::util::display::DurationFormat">DurationFormat</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides the format used for duration columns

Defaults to [`DurationFormat::ISO8601`](https://docs.rs/arrow/latest/arrow/util/display/enum.DurationFormat.html#variant.ISO8601 "variant arrow::util::display::DurationFormat::ISO8601")

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_types_info" class="fn">with_types_info</a>(self, types_info: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Overrides if types should be shown

Defaults to [`false`](https://doc.rust-lang.org/nightly/std/primitive.bool.html "primitive bool")

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.types_info" class="fn">types_info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if type info should be included in visual representation of batches

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-Clone-for-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-Debug-for-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-Default-for-FormatOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'\_\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'\_\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-Hash-for-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-PartialEq-for-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-Eq-for-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#impl-StructuralPartialEq-for-FormatOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#blanket-implementations" class="anchor">§</a>
