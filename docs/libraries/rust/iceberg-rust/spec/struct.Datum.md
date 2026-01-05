# Struct Datum Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/values.rs.html#108-111" class="src">Source</a>

``` rust
pub struct Datum { /* private fields */ }
```

Expand description

Literal associated with its type. The value and type pair is checked when construction, so the type and value is guaranteed to be correct when used.

By default, we decouple the type and value of a literal, so we can use avoid the cost of storing extra type info for each literal. But associate type with literal can be useful in some cases, for example, in unbound expression.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Datum" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.try_from_bytes" class="fn">try_from_bytes</a>(bytes: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], data_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create iceberg value from bytes.

See [this spec](https://iceberg.apache.org/spec/#binary-single-value-serialization) for reference.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.to_bytes" class="fn">to_bytes</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ByteBuf.html" class="struct" title="struct iceberg::spec::ByteBuf">ByteBuf</a>\>

Convert the value to bytes

See [this spec](https://iceberg.apache.org/spec/#binary-single-value-serialization) for reference.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.bool" class="fn">bool</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\>(t: T) -\> Self

Creates a boolean value.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
let t = Datum::bool(true);

assert_eq!(format!("{}", t), "true".to_string());
assert_eq!(
    Literal::from(t),
    Literal::Primitive(PrimitiveLiteral::Boolean(true))
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.bool_from_str" class="fn">bool_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a boolean value from string. See [Parse bool from str](https://doc.rust-lang.org/stable/std/primitive.bool.html#impl-FromStr-for-bool) for reference.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
let t = Datum::bool_from_str("false").unwrap();

assert_eq!(&format!("{}", t), "false");
assert_eq!(
    Literal::Primitive(PrimitiveLiteral::Boolean(false)),
    t.into()
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.int" class="fn">int</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>(t: T) -\> Self

Creates an 32bit integer.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
let t = Datum::int(23i8);

assert_eq!(&format!("{}", t), "23");
assert_eq!(Literal::Primitive(PrimitiveLiteral::Int(23)), t.into());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.long" class="fn">long</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>(t: T) -\> Self

Creates an 64bit integer.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
let t = Datum::long(24i8);

assert_eq!(&format!("{t}"), "24");
assert_eq!(Literal::Primitive(PrimitiveLiteral::Long(24)), t.into());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.float" class="fn">float</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\>(t: T) -\> Self

Creates an 32bit floating point number.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
use ordered_float::OrderedFloat;
let t = Datum::float(32.1f32);

assert_eq!(&format!("{t}"), "32.1");
assert_eq!(
    Literal::Primitive(PrimitiveLiteral::Float(OrderedFloat(32.1))),
    t.into()
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.double" class="fn">double</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\>(t: T) -\> Self

Creates an 64bit floating point number.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
use ordered_float::OrderedFloat;
let t = Datum::double(32.1f64);

assert_eq!(&format!("{t}"), "32.1");
assert_eq!(
    Literal::Primitive(PrimitiveLiteral::Double(OrderedFloat(32.1))),
    t.into()
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.date" class="fn">date</a>(days: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Creates date literal from number of days from unix epoch directly.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
// 2 days after 1970-01-01
let t = Datum::date(2);

assert_eq!(&format!("{t}"), "1970-01-03");
assert_eq!(Literal::Primitive(PrimitiveLiteral::Int(2)), t.into());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.date_from_str" class="fn">date_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates date literal in `%Y-%m-%d` format, assume in utc timezone.

See [`NaiveDate::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html#method.from_str "associated function chrono::naive::date::NaiveDate::from_str").

Example

``` rust
use iceberg::spec::{Datum, Literal};
let t = Datum::date_from_str("1970-01-05").unwrap();

assert_eq!(&format!("{t}"), "1970-01-05");
assert_eq!(Literal::date(4), t.into());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.date_from_ymd" class="fn">date_from_ymd</a>(year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, month: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, day: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create date literal from calendar date (year, month and day).

See [`NaiveDate::from_ymd_opt`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html#method.from_ymd_opt "associated function chrono::naive::date::NaiveDate::from_ymd_opt").

Example:

``` rust
 use iceberg::spec::{Datum, Literal};
 let t = Datum::date_from_ymd(1970, 1, 5).unwrap();

 assert_eq!(&format!("{t}"), "1970-01-05");
 assert_eq!(Literal::date(4), t.into());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.time_micros" class="fn">time_micros</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates time literal in microseconds directly.

It will return error when it’s negative or too large to fit in 24 hours.

Example:

``` rust
use iceberg::spec::{Datum, Literal};
let micro_secs = {
    1 * 3600 * 1_000_000 + // 1 hour
    2 * 60 * 1_000_000 +   // 2 minutes
    1 * 1_000_000 + // 1 second
    888999 // microseconds
};

let t = Datum::time_micros(micro_secs).unwrap();

assert_eq!(&format!("{t}"), "01:02:01.888999");
assert_eq!(Literal::time(micro_secs), t.into());

let negative_value = -100;
assert!(Datum::time_micros(negative_value).is_err());

let too_large_value = 36 * 60 * 60 * 1_000_000; // Too large to fit in 24 hours.
assert!(Datum::time_micros(too_large_value).is_err());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.time_from_str" class="fn">time_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates time literal in microseconds in `%H:%M:%S:.f` format.

See [`NaiveTime::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html#method.from_str "associated function chrono::naive::time::NaiveTime::from_str") for details.

Example:

``` rust
use iceberg::spec::{Datum, Literal};
let t = Datum::time_from_str("01:02:01.888999777").unwrap();

assert_eq!(&format!("{t}"), "01:02:01.888999");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.time_from_hms_micro" class="fn">time_from_hms_micro</a>( hour: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, min: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, sec: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, micro: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates time literal from hour, minute, second, and microseconds.

See [`NaiveTime::from_hms_micro_opt`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html#method.from_hms_micro_opt "associated function chrono::naive::time::NaiveTime::from_hms_micro_opt").

Example:

``` rust
use iceberg::spec::{Datum, Literal};
let t = Datum::time_from_hms_micro(22, 15, 33, 111).unwrap();

assert_eq!(&format!("{t}"), "22:15:33.000111");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamp_micros" class="fn">timestamp_micros</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates a timestamp from unix epoch in microseconds.

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::timestamp_micros(1000);

assert_eq!(&format!("{t}"), "1970-01-01 00:00:00.001");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamp_nanos" class="fn">timestamp_nanos</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates a timestamp from unix epoch in nanoseconds.

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::timestamp_nanos(1000);

assert_eq!(&format!("{t}"), "1970-01-01 00:00:00.000001");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamp_from_datetime" class="fn">timestamp_from_datetime</a>(dt: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>) -\> Self

Creates a timestamp from [`DateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html "struct chrono::datetime::DateTime").

Example:

``` rust
use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use iceberg::spec::Datum;
let t = Datum::timestamp_from_datetime(
    NaiveDate::from_ymd_opt(1992, 3, 1)
        .unwrap()
        .and_hms_micro_opt(1, 2, 3, 88)
        .unwrap(),
);

assert_eq!(&format!("{t}"), "1992-03-01 01:02:03.000088");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamp_from_str" class="fn">timestamp_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parse a timestamp in \[`%Y-%m-%dT%H:%M:%S%.f`\] format.

See [`NaiveDateTime::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html#method.from_str "associated function chrono::naive::datetime::NaiveDateTime::from_str").

Example:

``` rust
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use iceberg::spec::{Datum, Literal};
let t = Datum::timestamp_from_str("1992-03-01T01:02:03.000088").unwrap();

assert_eq!(&format!("{t}"), "1992-03-01 01:02:03.000088");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamptz_micros" class="fn">timestamptz_micros</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates a timestamp with timezone from unix epoch in microseconds.

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::timestamptz_micros(1000);

assert_eq!(&format!("{t}"), "1970-01-01 00:00:00.001 UTC");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamptz_nanos" class="fn">timestamptz_nanos</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates a timestamp with timezone from unix epoch in nanoseconds.

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::timestamptz_nanos(1000);

assert_eq!(&format!("{t}"), "1970-01-01 00:00:00.000001 UTC");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamptz_from_datetime" class="fn">timestamptz_from_datetime</a>\<T: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>(dt: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<T\>) -\> Self

Creates a timestamp with timezone from [`DateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html "struct chrono::datetime::DateTime"). Example:

``` rust
use chrono::{TimeZone, Utc};
use iceberg::spec::Datum;
let t = Datum::timestamptz_from_datetime(Utc.timestamp_opt(1000, 0).unwrap());

assert_eq!(&format!("{t}"), "1970-01-01 00:16:40 UTC");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.timestamptz_from_str" class="fn">timestamptz_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parse timestamp with timezone in RFC3339 format.

See [`DateTime::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html#method.from_str "associated function chrono::datetime::DateTime::from_str").

Example:

``` rust
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use iceberg::spec::{Datum, Literal};
let t = Datum::timestamptz_from_str("1992-03-01T01:02:03.000088+08:00").unwrap();

assert_eq!(&format!("{t}"), "1992-02-29 17:02:03.000088 UTC");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.string" class="fn">string</a>\<S: <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>\>(s: S) -\> Self

Creates a string literal.

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::string("ss");

assert_eq!(&format!("{t}"), r#""ss""#);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.uuid" class="fn">uuid</a>(uuid: <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid">Uuid</a>) -\> Self

Creates uuid literal.

Example:

``` rust
use iceberg::spec::Datum;
use uuid::uuid;
let t = Datum::uuid(uuid!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"));

assert_eq!(&format!("{t}"), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.uuid_from_str" class="fn">uuid_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates uuid from str. See [`Uuid::parse_str`](https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html#method.parse_str "associated function uuid::Uuid::parse_str").

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::uuid_from_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();

assert_eq!(&format!("{t}"), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.fixed" class="fn">fixed</a>\<I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>(input: I) -\> Self

Creates a fixed literal from bytes.

Example:

``` rust
use iceberg::spec::{Datum, Literal, PrimitiveLiteral};
let t = Datum::fixed(vec![1u8, 2u8]);

assert_eq!(&format!("{t}"), "0102");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.binary" class="fn">binary</a>\<I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>(input: I) -\> Self

Creates a binary literal from bytes.

Example:

``` rust
use iceberg::spec::Datum;
let t = Datum::binary(vec![1u8, 100u8]);

assert_eq!(&format!("{t}"), "0164");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.decimal_from_str" class="fn">decimal_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates decimal literal from string. See [`Decimal::from_str_exact`](https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/decimal/struct.Decimal.html#method.from_str_exact "associated function rust_decimal::decimal::Decimal::from_str_exact").

Example:

``` rust
use iceberg::spec::Datum;
use itertools::assert_equal;
use rust_decimal::Decimal;
let t = Datum::decimal_from_str("123.45").unwrap();

assert_eq!(&format!("{t}"), "123.45");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.decimal" class="fn">decimal</a>(value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/decimal/struct.Decimal.html" class="struct" title="struct rust_decimal::decimal::Decimal">Decimal</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Try to create a decimal literal from [`Decimal`](https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/decimal/struct.Decimal.html "struct rust_decimal::decimal::Decimal").

Example:

``` rust
use iceberg::spec::Datum;
use rust_decimal::Decimal;

let t = Datum::decimal(Decimal::new(123, 2)).unwrap();

assert_eq!(&format!("{t}"), "1.23");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.decimal_with_precision" class="fn">decimal_with_precision</a>( value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/decimal/struct.Decimal.html" class="struct" title="struct rust_decimal::decimal::Decimal">Decimal</a>\>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Try to create a decimal literal from [`Decimal`](https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/decimal/struct.Decimal.html "struct rust_decimal::decimal::Decimal") with precision.

Example:

``` rust
use iceberg::spec::Datum;
use rust_decimal::Decimal;

let t = Datum::decimal_with_precision(Decimal::new(123, 2), 30).unwrap();

assert_eq!(&format!("{t}"), "1.23");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.to" class="fn">to</a>(self, target_type: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>

Convert the datum to `target_type`.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.literal" class="fn">literal</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

Get the primitive literal from datum.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

Get the primitive type from datum.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.is_nan" class="fn">is_nan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the Literal represents a primitive type that can be a NaN, and that it’s value is NaN

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Clone-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Debug-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Deserialize%3C&#39;de%3E-for-Datum" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Display-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-From%3CDatum%3E-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-From%3CDatum%3E-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Hash-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-PartialEq-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-PartialOrd-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Serialize-for-Datum" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-Eq-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#impl-StructuralPartialEq-for-Datum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html#blanket-implementations" class="anchor">§</a>
