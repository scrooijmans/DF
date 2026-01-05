# Macro create_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/record_batch.rs.html#79" class="src">Source</a>

``` rust
macro_rules! create_array {
    (@from Boolean) => { ... };
    (@from Int8) => { ... };
    (@from Int16) => { ... };
    (@from Int32) => { ... };
    (@from Int64) => { ... };
    (@from UInt8) => { ... };
    (@from UInt16) => { ... };
    (@from UInt32) => { ... };
    (@from UInt64) => { ... };
    (@from Float16) => { ... };
    (@from Float32) => { ... };
    (@from Float64) => { ... };
    (@from Utf8) => { ... };
    (@from Utf8View) => { ... };
    (@from LargeUtf8) => { ... };
    (@from IntervalDayTime) => { ... };
    (@from IntervalYearMonth) => { ... };
    (@from Second) => { ... };
    (@from Millisecond) => { ... };
    (@from Microsecond) => { ... };
    (@from Nanosecond) => { ... };
    (@from Second32) => { ... };
    (@from Millisecond32) => { ... };
    (@from Microsecond64) => { ... };
    (@from Nanosecond64) => { ... };
    (@from DurationSecond) => { ... };
    (@from DurationMillisecond) => { ... };
    (@from DurationMicrosecond) => { ... };
    (@from DurationNanosecond) => { ... };
    (@from Decimal32) => { ... };
    (@from Decimal64) => { ... };
    (@from Decimal128) => { ... };
    (@from Decimal256) => { ... };
    (@from TimestampSecond) => { ... };
    (@from TimestampMillisecond) => { ... };
    (@from TimestampMicrosecond) => { ... };
    (@from TimestampNanosecond) => { ... };
    (@from $ty: ident) => { ... };
    (Null, $size: expr) => { ... };
    (Binary, [$($values: expr),*]) => { ... };
    (LargeBinary, [$($values: expr),*]) => { ... };
    ($ty: tt, [$($values: expr),*]) => { ... };
}
```

Expand description

Creates an array from a literal slice of values, suitable for rapid testing and development.

Example:

``` rust

use arrow_array::create_array;

let array = create_array!(Int32, [1, 2, 3, 4, 5]);
let array = create_array!(Utf8, [Some("a"), Some("b"), None, Some("e")]);
```

Support for limited data types is available. The macro will return a compile error if an unsupported data type is used. Presently supported data types are:

- `Boolean`, `Null`
- `Decimal32`, `Decimal64`, `Decimal128`, `Decimal256`
- `Float16`, `Float32`, `Float64`
- `Int8`, `Int16`, `Int32`, `Int64`
- `UInt8`, `UInt16`, `UInt32`, `UInt64`
- `IntervalDayTime`, `IntervalYearMonth`
- `Second`, `Millisecond`, `Microsecond`, `Nanosecond`
- `Second32`, `Millisecond32`, `Microsecond64`, `Nanosecond64`
- `DurationSecond`, `DurationMillisecond`, `DurationMicrosecond`, `DurationNanosecond`
- `TimestampSecond`, `TimestampMillisecond`, `TimestampMicrosecond`, `TimestampNanosecond`
- `Utf8`, `Utf8View`, `LargeUtf8`, `Binary`, `LargeBinary`
