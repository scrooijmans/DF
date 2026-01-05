Description: What data is needed to read the next item from a decoder.

Title: DecodeResult in parquet - Rust

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

## DecodeResult

parquet

# Enum DecodeResult Copy item path

Source

```
pub enum DecodeResult<T: Debug> {
NeedsData(Vec<Range<u64>>),
Data(T),
Finished,
}
```

Expand description

What data is needed to read the next item from a decoder.

This is used to communicate between the decoder and the caller to indicate what data is needed next, or what the result of decoding is.

## Variants§

§

### NeedsData(Vec<Range<u64\>>)

The ranges of data necessary to proceed

§

### Data(T)

The decoder produced an output item

§

### Finished

The decoder finished processing

## Trait Implementations§

Source§

### impl<T: Debug + Debug\> Debug for DecodeResult<T>

Source§

#### fn fmt(&self, f: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

## Auto Trait Implementations§

§

### impl<T> Freeze for DecodeResult<T>

where T: Freeze,

§

### impl<T> RefUnwindSafe for DecodeResult<T>

where T: RefUnwindSafe,

§

### impl<T> Send for DecodeResult<T>

where T: Send,

§

### impl<T> Sync for DecodeResult<T>

where T: Sync,

§

### impl<T> Unpin for DecodeResult<T>

where T: Unpin,

§

### impl<T> UnwindSafe for DecodeResult<T>

where T: UnwindSafe,

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

### impl<T> Allocation for T

where T: RefUnwindSafe + Send + Sync,

Source§

### impl<T> ErasedDestructor for T

where T: 'static,
