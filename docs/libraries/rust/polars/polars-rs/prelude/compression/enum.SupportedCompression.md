# Enum SupportedCompression Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/compression.rs.html#7" class="src">Source</a>

``` rust
pub enum SupportedCompression {
    GZIP,
    ZLIB,
    ZSTD,
}
```

Available on **crate feature `polars-io`** only.

Expand description

Represents the compression algorithms that we have decoders for

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#variant.GZIP" class="anchor">§</a>

### GZIP

<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#variant.ZLIB" class="anchor">§</a>

### ZLIB

<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#variant.ZSTD" class="anchor">§</a>

### ZSTD

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#impl-SupportedCompression" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html" class="enum" title="enum polars::prelude::compression::SupportedCompression">SupportedCompression</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#method.check" class="fn">check</a>(bytes: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html" class="enum" title="enum polars::prelude::compression::SupportedCompression">SupportedCompression</a>\>

If the given byte slice starts with the “magic” bytes for a supported compression family, return that family, for unsupported/uncompressed slices, return None. Based on <https://en.wikipedia.org/wiki/List_of_file_signatures>.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html#blanket-implementations" class="anchor">§</a>
