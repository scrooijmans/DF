# Struct Timestamp Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/time.rs.html#29" class="src">Source</a>

``` rust
pub struct Timestamp(/* private fields */);
```

Expand description

An instant in time represented as the number of nanoseconds since the Unix epoch.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Timestamp" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

#### pub const <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#associatedconstant.MIN" class="constant">MIN</a>: Self

The minimum timestamp value.

#### pub const <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#associatedconstant.MAX" class="constant">MAX</a>: Self

The maximum timestamp value.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.now" class="fn">now</a>() -\> Self

Create the timestamp of now.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.format_http_date" class="fn">format_http_date</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Format the timestamp into http date: `Sun, 06 Nov 1994 08:49:37 GMT`

###### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#note" class="doc-anchor">Â§</a>Note

HTTP date is slightly different from RFC2822.

- Timezone is fixed to GMT.
- Day must be 2 digit.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.new" class="fn">new</a>(second: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, nanosecond: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self, <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>\>

Creates a new instant in time from the number of seconds elapsed since the Unix epoch.

When second is negative, it corresponds to an instant in time before the Unix epoch. A smaller number corresponds to an instant in time further into the past.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.from_millisecond" class="fn">from_millisecond</a>(millis: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Creates a new instant in time from the number of milliseconds elapsed since the Unix epoch.

When `millisecond` is negative, it corresponds to an instant in time before the Unix epoch. A smaller number corresponds to an instant in time further into the past.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.from_second" class="fn">from_second</a>(second: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Creates a new instant in time from the number of seconds elapsed since the Unix epoch.

When `second` is negative, it corresponds to an instant in time before the Unix epoch. A smaller number corresponds to an instant in time further into the past.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.parse_rfc2822" class="fn">parse_rfc2822</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>\>

Parse a timestamp from RFC2822.

All of them are valid time:

- `Sat, 13 Jul 2024 15:09:59 -0400`
- `Mon, 15 Aug 2022 16:50:12 GMT`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.into_inner" class="fn">into_inner</a>(self) -\> Timestamp

Convert to inner `jiff::Timestamp` for compatibility.

This method is provided for accessing the underlying `jiff::Timestamp` when needed for interoperability with jiff-specific APIs.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Add%3CDuration%3E-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#associatedtype.Output" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

The resulting type after applying the `+` operator.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.add" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-AddAssign%3CDuration%3E-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html" class="trait" title="trait core::ops::arith::AddAssign">AddAssign</a>\<<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.add_assign" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign" class="fn">add_assign</a>(&mut self, rhs: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Clone-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Debug-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Default-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Display-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-From%3CTimestamp%3E-for-SystemTime" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>\> for <a href="https://doc.rust-lang.org/nightly/std/time/struct.SystemTime.html" class="struct" title="struct std::time::SystemTime">SystemTime</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.from-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(t: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-From%3CTimestamp%3E-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>\> for Timestamp

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(t: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-From%3CTimestamp%3E-for-Timestamp-1" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Timestamp\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.from-2" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(t: Timestamp) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-FromStr-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.from_str" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parse a timestamp by the default [`DateTimeParser`](jiff::fmt::temporal::DateTimeParser).

All of them are valid time:

- `2022-03-13T07:20:04Z`
- `2022-03-01T08:12:34+00:00`
- `2022-03-01T08:12:34.00+00:00`
- `2022-07-08T02:14:07+02:00[Europe/Paris]`

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#associatedtype.Err" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Hash-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.hash" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.hash_slice" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Ord-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.cmp" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.max" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.min" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.clamp" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-PartialEq-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-PartialOrd-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.partial_cmp" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.lt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.le" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.gt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.ge" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Sub%3CDuration%3E-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#associatedtype.Output-1" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

The resulting type after applying the `-` operator.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.sub" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-SubAssign%3CDuration%3E-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html" class="trait" title="trait core::ops::arith::SubAssign">SubAssign</a>\<<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.sub_assign" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign" class="fn">sub_assign</a>(&mut self, rhs: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-TryFrom%3CSystemTime%3E-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/time/struct.SystemTime.html" class="struct" title="struct std::time::SystemTime">SystemTime</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#associatedtype.Error" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

The type returned in the event of a conversion error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#method.try_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(t: <a href="https://doc.rust-lang.org/nightly/std/time/struct.SystemTime.html" class="struct" title="struct std::time::SystemTime">SystemTime</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Performs the conversion.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Copy-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-Eq-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#impl-StructuralPartialEq-for-Timestamp" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html#blanket-implementations" class="anchor">Â§</a>
