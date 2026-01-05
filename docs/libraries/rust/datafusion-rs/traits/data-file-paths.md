# DataFilePaths in datafusion::execution::context - Rust

[![logo](https://raw.githubusercontent.com/apache/datafusion/19fe44cf2f30cbdd63d4a4f52c74055163c6cc38/docs/logos/standalone_logo/logo_original.svg)](../../../datafusion/index.html)

## [datafusion](../../../datafusion/index.html)50.2.0

## [DataFilePaths](#)

### [Required Methods](#required-methods)

- [to_urls](#tymethod.to_urls "to_urls")

### [Implementations on Foreign Types](#foreign-impls)

- [&String](#impl-DataFilePaths-for-%26String "&String")
- [&str](#impl-DataFilePaths-for-%26str "&str")
- [String](#impl-DataFilePaths-for-String "String")
- [Vec<P>](#impl-DataFilePaths-for-Vec%3CP%3E "Vec<P>")

### [Implementors](#implementors)

## [In datafusion::execution::context](index.html)

[datafusion](../../index.html)::[execution](../index.html)::[context](index.html)

## Trait DataFilePaths 

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#105-108)

```
pub trait DataFilePaths {
    // Required method
    fn to_urls(self) -> Result<Vec<ListingTableUrl>>;
}
```

Expand description

DataFilePaths adds a method to convert strings and vector of strings to vector of [`ListingTableUrl`](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") URLs. This allows methods such [`SessionContext::read_csv`](about:blank/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv") and [`SessionContext::read_avro`](about:blank/struct.SessionContext.html#method.read_avro "method datafusion::execution::context::SessionContext::read_avro") to take either a single file or multiple files.

## Required Methods[§](#required-methods)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#107)

#### fn [to_urls](#tymethod.to_urls)(self) -> [Result](../../error/type.Result.html "type datafusion::error::Result")<[Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[ListingTableUrl](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")\>>

Parse to a vector of [`ListingTableUrl`](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") URLs.

## Implementations on Foreign Types[§](#foreign-impls)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#110-114)
[§](#impl-DataFilePaths-for-%26str)

### impl [DataFilePaths](trait.DataFilePaths.html "trait datafusion::execution::context::DataFilePaths") for &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#111-113)
[§](#method.to_urls)

#### fn [to_urls](#tymethod.to_urls)(self) -> [Result](../../error/type.Result.html "type datafusion::error::Result")<[Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[ListingTableUrl](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")\>>

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#122-126)
[§](#impl-DataFilePaths-for-%26String)

### impl [DataFilePaths](trait.DataFilePaths.html "trait datafusion::execution::context::DataFilePaths") for &[String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#123-125)
[§](#method.to_urls-1)

#### fn [to_urls](#tymethod.to_urls)(self) -> [Result](../../error/type.Result.html "type datafusion::error::Result")<[Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[ListingTableUrl](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")\>>

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#116-120)
[§](#impl-DataFilePaths-for-String)

### impl [DataFilePaths](trait.DataFilePaths.html "trait datafusion::execution::context::DataFilePaths") for [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#117-119)
[§](#method.to_urls-2)

#### fn [to_urls](#tymethod.to_urls)(self) -> [Result](../../error/type.Result.html "type datafusion::error::Result")<[Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[ListingTableUrl](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")\>>

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#128-137)
[§](#impl-DataFilePaths-for-Vec%3CP%3E)

### impl<P> [DataFilePaths](trait.DataFilePaths.html "trait datafusion::execution::context::DataFilePaths") for [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<P>

where P: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#132-136)
[§](#method.to_urls-3)

#### fn [to_urls](#tymethod.to_urls)(self) -> [Result](../../error/type.Result.html "type datafusion::error::Result")<[Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[ListingTableUrl](../../datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")\>>

## Implementors[§](#implementors)
