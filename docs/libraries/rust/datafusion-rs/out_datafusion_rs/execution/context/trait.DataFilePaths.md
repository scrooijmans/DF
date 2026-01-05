# Trait DataFilePaths Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/context/mod.rs.html#105-108" class="src">Source</a>

``` rust
pub trait DataFilePaths {
    // Required method
    fn to_urls(self) -> Result<Vec<ListingTableUrl>>;
}
```

Expand description

DataFilePaths adds a method to convert strings and vector of strings to vector of [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") URLs. This allows methods such [`SessionContext::read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv") and [`SessionContext::read_avro`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_avro "method datafusion::execution::context::SessionContext::read_avro") to take either a single file or multiple files.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#tymethod.to_urls" class="fn">to_urls</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>\>

Parse to a vector of [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") URLs.

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#impl-DataFilePaths-for-%26str" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#method.to_urls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#tymethod.to_urls" class="fn">to_urls</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#impl-DataFilePaths-for-%26String" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a> for &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#method.to_urls-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#tymethod.to_urls" class="fn">to_urls</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#impl-DataFilePaths-for-String" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#method.to_urls-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#tymethod.to_urls" class="fn">to_urls</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#impl-DataFilePaths-for-Vec%3CP%3E" class="anchor">§</a>

### impl\<P\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#method.to_urls-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#tymethod.to_urls" class="fn">to_urls</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html#implementors" class="anchor">§</a>
