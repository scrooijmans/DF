# Enum Literal Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/values.rs.html#1406-1421" class="src">Source</a>

``` rust
pub enum Literal {
    Primitive(PrimitiveLiteral),
    Struct(Struct),
    List(Vec<Option<Literal>>),
    Map(Map),
}
```

Expand description

Values present in iceberg type

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#variant.Primitive" class="anchor">§</a>

### Primitive(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>)

A primitive value

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>)

A struct is a tuple of typed values. Each field in the tuple is named and has an integer id that is unique in the table schema. Each field can be either optional or required, meaning that values can (or cannot) be null. Fields may be any type. Fields may have an optional comment or doc string. Fields can have default values.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>\>)

A list is a collection of values with some element type. The element field has an integer id that is unique in the table schema. Elements can be either optional or required. Element types may be any type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#variant.Map" class="anchor">§</a>

### Map(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>)

A map is a collection of key-value pairs with a key type and a value type. Both the key field and value field each have an integer id that is unique in the table schema. Map keys are required and map values can be either optional or required. Both map keys and map values may be any type, including nested types.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-Literal" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.bool" class="fn">bool</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\>(t: T) -\> Self

Creates a boolean value.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
let t = Literal::bool(true);

assert_eq!(Literal::Primitive(PrimitiveLiteral::Boolean(true)), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.bool_from_str" class="fn">bool_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a boolean value from string. See [Parse bool from str](https://doc.rust-lang.org/stable/std/primitive.bool.html#impl-FromStr-for-bool) for reference.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
let t = Literal::bool_from_str("false").unwrap();

assert_eq!(Literal::Primitive(PrimitiveLiteral::Boolean(false)), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.int" class="fn">int</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>(t: T) -\> Self

Creates an 32bit integer.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
let t = Literal::int(23i8);

assert_eq!(Literal::Primitive(PrimitiveLiteral::Int(23)), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.long" class="fn">long</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>(t: T) -\> Self

Creates an 64bit integer.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
let t = Literal::long(24i8);

assert_eq!(Literal::Primitive(PrimitiveLiteral::Long(24)), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.float" class="fn">float</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\>(t: T) -\> Self

Creates an 32bit floating point number.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
use ordered_float::OrderedFloat;
let t = Literal::float(32.1f32);

assert_eq!(
    Literal::Primitive(PrimitiveLiteral::Float(OrderedFloat(32.1))),
    t
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.double" class="fn">double</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\>(t: T) -\> Self

Creates an 32bit floating point number.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
use ordered_float::OrderedFloat;
let t = Literal::double(32.1f64);

assert_eq!(
    Literal::Primitive(PrimitiveLiteral::Double(OrderedFloat(32.1))),
    t
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.date" class="fn">date</a>(days: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Creates date literal from number of days from unix epoch directly.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.date_from_str" class="fn">date_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a date in `%Y-%m-%d` format, assume in utc timezone.

See [`NaiveDate::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html#method.from_str "associated function chrono::naive::date::NaiveDate::from_str").

Example

``` rust
use iceberg::spec::Literal;
let t = Literal::date_from_str("1970-01-03").unwrap();

assert_eq!(Literal::date(2), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.date_from_ymd" class="fn">date_from_ymd</a>(year: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, month: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, day: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Create a date from calendar date (year, month and day).

See [`NaiveDate::from_ymd_opt`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html#method.from_ymd_opt "associated function chrono::naive::date::NaiveDate::from_ymd_opt").

Example:

``` rust
 use iceberg::spec::Literal;
 let t = Literal::date_from_ymd(1970, 1, 5).unwrap();

 assert_eq!(Literal::date(4), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.time" class="fn">time</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates time in microseconds directly

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.time_from_str" class="fn">time_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates time in microseconds in `%H:%M:%S:.f` format.

See [`NaiveTime::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html#method.from_str "associated function chrono::naive::time::NaiveTime::from_str") for details.

Example:

``` rust
use iceberg::spec::Literal;
let t = Literal::time_from_str("01:02:01.888999777").unwrap();

let micro_secs = {
    1 * 3600 * 1_000_000 + // 1 hour
    2 * 60 * 1_000_000 +   // 2 minutes
    1 * 1_000_000 + // 1 second
    888999 // microseconds
};
assert_eq!(Literal::time(micro_secs), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.time_from_hms_micro" class="fn">time_from_hms_micro</a>( hour: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, min: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, sec: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, micro: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates time literal from hour, minute, second, and microseconds.

See [`NaiveTime::from_hms_micro_opt`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html#method.from_hms_micro_opt "associated function chrono::naive::time::NaiveTime::from_hms_micro_opt").

Example:

``` rust
use iceberg::spec::Literal;
let t = Literal::time_from_hms_micro(22, 15, 33, 111).unwrap();

assert_eq!(Literal::time_from_str("22:15:33.000111").unwrap(), t);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamp" class="fn">timestamp</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates a timestamp from unix epoch in microseconds.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamptz" class="fn">timestamptz</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Creates a timestamp with timezone from unix epoch in microseconds.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamp_from_datetime" class="fn">timestamp_from_datetime</a>\<T: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>(dt: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<T\>) -\> Self

Creates a timestamp from [`DateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html "struct chrono::datetime::DateTime").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamptz_from_datetime" class="fn">timestamptz_from_datetime</a>\<T: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.TimeZone.html" class="trait" title="trait chrono::offset::TimeZone">TimeZone</a>\>(dt: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<T\>) -\> Self

Creates a timestamp with timezone from [`DateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html "struct chrono::datetime::DateTime").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamp_from_str" class="fn">timestamp_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parse a timestamp in RFC3339 format.

See [`DateTime<Utc>::from_str`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html#method.from_str "associated function chrono::datetime::DateTime::from_str").

Example:

``` rust
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use iceberg::spec::Literal;
let t = Literal::timestamp_from_str("2012-12-12 12:12:12.8899-04:00").unwrap();

let t2 = {
    let date = NaiveDate::from_ymd_opt(2012, 12, 12).unwrap();
    let time = NaiveTime::from_hms_micro_opt(12, 12, 12, 889900).unwrap();
    let dt = NaiveDateTime::new(date, time);
    Literal::timestamp_from_datetime(DateTime::<FixedOffset>::from_local(
        dt,
        FixedOffset::west_opt(4 * 3600).unwrap(),
    ))
};

assert_eq!(t, t2);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamptz_from_str" class="fn">timestamptz_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Similar to [`Literal::timestamp_from_str`](https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.timestamp_from_str "associated function iceberg::spec::Literal::timestamp_from_str"), but return timestamp with timezone literal.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.string" class="fn">string</a>\<S: <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>\>(s: S) -\> Self

Creates a string literal.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.uuid" class="fn">uuid</a>(uuid: <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid">Uuid</a>) -\> Self

Creates uuid literal.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.uuid_from_str" class="fn">uuid_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates uuid from str. See [`Uuid::parse_str`](https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html#method.parse_str "associated function uuid::Uuid::parse_str").

Example:

``` rust
use iceberg::spec::Literal;
use uuid::Uuid;
let t1 = Literal::uuid_from_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap();
let t2 = Literal::uuid(Uuid::from_u128_le(0xd8d7d6d5d4d3d2d1c2c1b2b1a4a3a2a1));

assert_eq!(t1, t2);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.fixed" class="fn">fixed</a>\<I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>(input: I) -\> Self

Creates a fixed literal from bytes.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
let t1 = Literal::fixed(vec![1u8, 2u8]);
let t2 = Literal::Primitive(PrimitiveLiteral::Binary(vec![1u8, 2u8]));

assert_eq!(t1, t2);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.binary" class="fn">binary</a>\<I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>(input: I) -\> Self

Creates a binary literal from bytes.

Example:

``` rust
use iceberg::spec::{Literal, PrimitiveLiteral};
let t1 = Literal::binary(vec![1u8, 2u8]);
let t2 = Literal::Primitive(PrimitiveLiteral::Binary(vec![1u8, 2u8]));

assert_eq!(t1, t2);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.decimal" class="fn">decimal</a>(decimal: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> Self

Creates a decimal literal.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.decimal_from_str" class="fn">decimal_from_str</a>\<S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>(s: S) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates decimal literal from string. See [`Decimal::from_str_exact`](https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/decimal/struct.Decimal.html#method.from_str_exact "associated function rust_decimal::decimal::Decimal::from_str_exact").

Example:

``` rust
use iceberg::spec::Literal;
use rust_decimal::Decimal;
let t1 = Literal::decimal(12345);
let t2 = Literal::decimal_from_str("123.45").unwrap();

assert_eq!(t1, t2);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.as_primitive_literal" class="fn">as_primitive_literal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>\>

Attempts to convert the Literal to a PrimitiveLiteral

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-Literal-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.try_from_json" class="fn">try_from_json</a>(value: <a href="https://docs.rs/serde_json/1.0.143/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">JsonValue</a>, data_type: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self\>\>

Create iceberg value from a json value

See [this spec](https://iceberg.apache.org/spec/#json-single-value-serialization) for reference.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.try_into_json" class="fn">try_into_json</a>(self, type: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/serde_json/1.0.143/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">JsonValue</a>\>

Converting iceberg value to json value.

See [this spec](https://iceberg.apache.org/spec/#json-single-value-serialization) for reference.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.into_any" class="fn">into_any</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Convert Value to the any type

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-Clone-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-Debug-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-From%3CDatum%3E-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-Hash-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-PartialEq-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-Eq-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#impl-StructuralPartialEq-for-Literal" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html#blanket-implementations" class="anchor">§</a>
