# Struct Timestamp Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#3576" class="src">Source</a>

``` rust
pub struct Timestamp<'a> {
    pub _tab: Table<'a>,
}
```

Expand description

Timestamp is a 64-bit signed integer representing an elapsed time since a fixed epoch, stored in either of four units: seconds, milliseconds, microseconds or nanoseconds, and is optionally annotated with a timezone.

Timestamp values do not include any leap seconds (in other words, all days are considered 86400 seconds long).

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#timestamps-with-a-non-empty-timezone" class="doc-anchor">§</a>Timestamps with a non-empty timezone

If a Timestamp column has a non-empty timezone value, its epoch is 1970-01-01 00:00:00 (January 1st 1970, midnight) in the *UTC* timezone (the Unix epoch), regardless of the Timestamp’s own timezone.

Therefore, timestamp values with a non-empty timezone correspond to physical points in time together with some additional information about how the data was obtained and/or how to display it (the timezone).

For example, the timestamp value 0 with the timezone string “Europe/Paris” corresponds to “January 1st 1970, 00h00” in the UTC timezone, but the application may prefer to display it as “January 1st 1970, 01h00” in the Europe/Paris timezone (which is the same physical point in time).

One consequence is that timestamp values with a non-empty timezone can be compared and ordered directly, since they all share the same well-known point of reference (the Unix epoch).

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#timestamps-with-an-unset--empty-timezone" class="doc-anchor">§</a>Timestamps with an unset / empty timezone

If a Timestamp column has no timezone value, its epoch is 1970-01-01 00:00:00 (January 1st 1970, midnight) in an *unknown* timezone.

Therefore, timestamp values without a timezone cannot be meaningfully interpreted as physical points in time, but only as calendar / clock indications (“wall clock time”) in an unspecified timezone.

For example, the timestamp value 0 with an empty timezone string corresponds to “January 1st 1970, 00h00” in an unknown timezone: there is not enough information to interpret it as a well-defined physical point in time.

One consequence is that timestamp values without a timezone cannot be reliably compared or ordered, since they may have different points of reference. In particular, it is *not* possible to interpret an unset or empty timezone as the same as “UTC”.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#conversion-between-timezones" class="doc-anchor">§</a>Conversion between timezones

If a Timestamp column has a non-empty timezone, changing the timezone to a different non-empty value is a metadata-only operation: the timestamp values need not change as their point of reference remains the same (the Unix epoch).

However, if a Timestamp column has no timezone value, changing it to a non-empty value requires to think about the desired semantics. One possibility is to assume that the original timestamp values are relative to the epoch of the timezone being set; timestamp values should then adjusted to the Unix epoch (for example, changing the timezone from empty to “Europe/Paris” would require converting the timestamp values from “Europe/Paris” to “UTC”, which seems counter-intuitive but is nevertheless correct).

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#guidelines-for-encoding-data-from-external-libraries" class="doc-anchor">§</a>Guidelines for encoding data from external libraries

Date & time libraries often have multiple different data types for temporal data. In order to ease interoperability between different implementations the Arrow project has some recommendations for encoding these types into a Timestamp column.

An “instant” represents a physical point in time that has no relevant timezone (for example, astronomical data). To encode an instant, use a Timestamp with the timezone string set to “UTC”, and make sure the Timestamp values are relative to the UTC epoch (January 1st 1970, midnight).

A “zoned date-time” represents a physical point in time annotated with an informative timezone (for example, the timezone in which the data was recorded). To encode a zoned date-time, use a Timestamp with the timezone string set to the name of the timezone, and make sure the Timestamp values are relative to the UTC epoch (January 1st 1970, midnight).

(There is some ambiguity between an instant and a zoned date-time with the UTC timezone. Both of these are stored the same in Arrow. Typically, this distinction does not matter. If it does, then an application should use custom metadata or an extension type to distinguish between the two cases.)

An “offset date-time” represents a physical point in time combined with an explicit offset from UTC. To encode an offset date-time, use a Timestamp with the timezone string set to the numeric timezone offset string (e.g. “+03:00”), and make sure the Timestamp values are relative to the UTC epoch (January 1st 1970, midnight).

A “naive date-time” (also called “local date-time” in some libraries) represents a wall clock time combined with a calendar date, but with no indication of how to map this information to a physical point in time. Naive date-times must be handled with care because of this missing information, and also because daylight saving time (DST) may make some values ambiguous or nonexistent. A naive date-time may be stored as a struct with Date and Time fields. However, it may also be encoded into a Timestamp column with an empty timezone. The timestamp values should be computed “as if” the timezone of the date-time values was UTC; for example, the naive date-time “January 1st 1970, 00h00” would be encoded as timestamp value 0.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#structfield._tab" class="anchor field">§</a>`_tab: `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table"><code>Table</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-Timestamp%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#associatedconstant.VT_UNIT" class="constant">VT_UNIT</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 4u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#associatedconstant.VT_TIMEZONE" class="constant">VT_TIMEZONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 6u16

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.init_from_table" class="fn">init_from_table</a>(table: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.create" class="fn">create</a>\<'bldr, 'args, 'mut_bldr, A\>( \_fbb: &'mut_bldr mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'bldr, A\>, args: &'args <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::TimestampArgs">TimestampArgs</a>\<'args\>, ) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'bldr\>\>

where 'bldr: 'args, 'args: 'mut_bldr, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'bldr,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.unit" class="fn">unit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimeUnit.html" class="struct" title="struct datafusion::common::arrow::ipc::TimeUnit">TimeUnit</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.timezone" class="fn">timezone</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

The timezone is an optional string indicating the name of a timezone, one of:

- As used in the Olson timezone database (the “tz database” or “tzdata”), such as “America/New_York”.
- An absolute timezone offset of the form “+XX:XX” or “-XX:XX”, such as “+07:30”.

Whether a timezone string is present indicates different semantics about the data (see above).

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-Clone-for-Timestamp%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-Debug-for-Timestamp%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-Follow%3C&#39;a%3E-for-Timestamp%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>( buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-PartialEq-for-Timestamp%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-Verifiable-for-Timestamp%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-Copy-for-Timestamp%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#impl-StructuralPartialEq-for-Timestamp%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html#blanket-implementations" class="anchor">§</a>
