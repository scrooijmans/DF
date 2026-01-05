Description: Parquet error enumeration

Title: ParquetError in parquet::errors - Rust

Docs.rs

- parquet-56.2.0

- parquet 56.2.0
- Permalink
- Docs.rs crate page
- Apache-2.0

- Links
- Homepage
- Repository
- crates.io
- Source

- Owners
- andygrove
- sunchao
- kou
- kszucs
- nevi-me
- sadikovi
- alamb
- tustvold

- Dependencies
- - arrow-array ^56.2.0 _normal_ _optional_
- arrow-buffer ^56.2.0 _normal_ _optional_
- arrow-cast ^56.2.0 _normal_ _optional_
- arrow-csv ^56.2.0 _normal_ _optional_
- arrow-data ^56.2.0 _normal_ _optional_
- arrow-ipc ^56.2.0 _normal_ _optional_
- arrow-schema ^56.2.0 _normal_ _optional_
- arrow-select ^56.2.0 _normal_ _optional_
- base64 ^0.22 _normal_ _optional_
- brotli ^8.0 _normal_ _optional_
- bytes ^1.1 _normal_
- chrono ^0.4.40 _normal_
- clap ^4.1 _normal_ _optional_
- crc32fast ^1.4.2 _normal_ _optional_
- flate2 ^1.1 _normal_ _optional_
- futures ^0.3 _normal_ _optional_
- half ^2.1 _normal_
- hashbrown ^0.16 _normal_
- lz4_flex ^0.11 _normal_ _optional_
- num ^0.4 _normal_
- num-bigint ^0.4 _normal_
- object_store ^0.12.0 _normal_ _optional_
- parquet-variant ^0.1.0 _normal_ _optional_
- parquet-variant-compute ^0.1.0 _normal_ _optional_
- parquet-variant-json ^0.1.0 _normal_ _optional_
- paste ^1.0 _normal_
- ring ^0.17 _normal_ _optional_
- seq-macro ^0.3 _normal_
- serde ^1.0 _normal_ _optional_
- serde_json ^1.0 _normal_ _optional_
- simdutf8 ^0.1.5 _normal_ _optional_
- snap ^1.0 _normal_ _optional_
- thrift ^0.17 _normal_
- tokio ^1.0 _normal_ _optional_
- twox-hash ^2.0 _normal_
- zstd ^0.13 _normal_ _optional_
- arrow ^56.2.0 _dev_
- base64 ^0.22 _dev_
- brotli ^8.0 _dev_
- criterion ^0.5 _dev_
- flate2 ^1.0 _dev_
- insta ^1.43.1 _dev_
- lz4_flex ^0.11 _dev_
- object_store ^0.12.0 _dev_
- rand ^0.9 _dev_
- serde_json ^1.0 _dev_
- snap ^1.0 _dev_
- sysinfo ^0.36.0 _dev_
- tempfile ^3.0 _dev_
- tokio ^1.0 _dev_
- zstd ^0.13 _dev_
- ahash ^0.8 _normal_
- ahash ^0.8 _normal_
- ring ^0.17 _normal_ _optional_

- Versions

- **100%** of the crate is documented

- Platform
- x86_64-unknown-linux-gnu
- Feature flags

- docs.rs
- About docs.rs
- Badges
- Builds
- Metadata
- Shorthand URLs
- Download
- Rustdoc JSON
- Build queue
- Privacy policy

- Rust
- Rust website
- The Book
- Standard Library API Reference
- Rust by Example
- The Cargo Guide
- Clippy Documentation

## ParquetError

parquet::errors

# Enum ParquetError Copy item path

Source

```
#[non_exhaustive]pub enum ParquetError {
General(String),
NYI(String),
EOF(String),
ArrowError(String),
IndexOutOfBound(usize, usize),
External(Box<dyn Error + Send + Sync>),
NeedMoreData(usize),
NeedMoreDataRange(Range<u64>),
}
```

Expand description

Parquet error enumeration

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

§

### General(String)

General Parquet error. Returned when code violates normal workflow of working with Parquet files.

§

### NYI(String)

“Not yet implemented” Parquet error. Returned when functionality is not yet available.

§

### EOF(String)

“End of file” Parquet error. Returned when IO related failures occur, e.g. when there are not enough bytes to decode.

§

### ArrowError(String)

Available on **crate feature `arrow`** only.

Arrow error. Returned when reading into arrow or writing from arrow.

§

### IndexOutOfBound(usize, usize)

Error when the requested column index is more than the number of columns in the row group

§

### External(Box<dyn Error + Send + Sync\>)

An external error variant

§

### NeedMoreData(usize)

Returned when a function needs more data to complete properly. The `usize` field indicates the total number of bytes required, not the number of additional bytes.

§

### NeedMoreDataRange(Range<u64\>)

Returned when a function needs more data to complete properly. The `Range<u64>` indicates the range of bytes that are needed.

## Trait Implementations§

Source§

### impl Debug for ParquetError

Source§

#### fn fmt(&self, f: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

Source§

### impl Display for ParquetError

Source§

#### fn fmt(&self, fmt: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

Source§

### impl Error for ParquetError

Source§

#### fn source(&self) -> Option<&(dyn Error + 'static)>

Returns the lower-level source of this error, if any. Read more

1.0.0 · Source§

#### fn description(&self) -> &str

👎Deprecated since 1.42.0: use the Display impl or to_string()

Read more

1.0.0 · Source§

#### fn cause(&self) -> Option<&dyn Error\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

Source§

#### fn provide<'a>(&'a self, request: &mut Request<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. Read more

Source§

### impl From<ArrowError\> for ParquetError

Available on **crate feature `arrow`** only.

Source§

#### fn from(e: ArrowError) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<BorrowMutError\> for ParquetError

Source§

#### fn from(e: BorrowMutError) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<Error\> for ParquetError

Source§

#### fn from(e: Error) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<Error\> for ParquetError

Available on **crate feature `snap`** only.

Source§

#### fn from(e: Error) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<Error\> for ParquetError

Source§

#### fn from(e: Error) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<Error\> for ParquetError

Available on **crate feature `object_store`** only.

Source§

#### fn from(e: Error) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<ParquetError\> for ArrowError

Available on **crate feature `arrow`** only.

Source§

#### fn from(p: ParquetError) -> Self

Converts to this type from the input type.

Source§

### impl From<ParquetError\> for Error

Source§

#### fn from(e: ParquetError) -> Self

Converts to this type from the input type.

Source§

### impl From<TryFromIntError\> for ParquetError

Source§

#### fn from(e: TryFromIntError) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<Unspecified\> for ParquetError

Available on **crate feature `encryption`** only.

Source§

#### fn from(e: Unspecified) -> ParquetError

Converts to this type from the input type.

Source§

### impl From<Utf8Error\> for ParquetError

Source§

#### fn from(e: Utf8Error) -> ParquetError

Converts to this type from the input type.

## Auto Trait Implementations§

§

### impl Freeze for ParquetError

§

### impl !RefUnwindSafe for ParquetError

§

### impl Send for ParquetError

§

### impl Sync for ParquetError

§

### impl Unpin for ParquetError

§

### impl !UnwindSafe for ParquetError

## Blanket Implementations§

Source§

### impl<T> Any for T

where T: 'static + ?Sized,

Source§

#### fn type_id(&self) -> TypeId

Gets the `TypeId` of `self`. Read more

Source§

### impl<T> Borrow<T> for T

where T: ?Sized,

Source§

#### fn borrow(&self) -> &T

Immutably borrows from an owned value. Read more

Source§

### impl<T> BorrowMut<T> for T

where T: ?Sized,

Source§

#### fn borrow_mut(&mut self) -> &mut T

Mutably borrows from an owned value. Read more

Source§

### impl<T> From<T> for T

Source§

#### fn from(t: T) -> T

Returns the argument unchanged.

Source§

### impl<T, U> Into<U> for T

where U: From<T>,

Source§

#### fn into(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

Source§

### impl<T> IntoEither for T

Source§

#### fn into_either(self, into_left: bool) -> Either<Self, Self>

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left` is `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

#### fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>

where F: FnOnce(&Self) -> bool,

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left(&self)` returns `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

### impl<T> ToString for T

where T: Display + ?Sized,

Source§

#### fn to_string(&self) -> String

Converts the given value to a `String`. Read more

Source§

### impl<T, U> TryFrom<U> for T

where U: Into<T>,

Source§

#### type Error = Infallible

The type returned in the event of a conversion error.

Source§

#### fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error\>

Performs the conversion.

Source§

### impl<T, U> TryInto<U> for T

where U: TryFrom<T>,

Source§

#### type Error = <U as TryFrom<T>>::Error

The type returned in the event of a conversion error.

Source§

#### fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error\>

Performs the conversion.

Source§

### impl<T> ErasedDestructor for T

where T: 'static,
