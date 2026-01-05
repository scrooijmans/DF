# Module compression Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/mod.rs.html#1" class="src">Source</a>

Available on **crate feature `polars-io`** only.

## Enums<a href="https://docs.rs/polars/latest/polars/prelude/compression/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/compression/enum.SupportedCompression.html" class="enum" title="enum polars::prelude::compression::SupportedCompression">SupportedCompression</a>  
Represents the compression algorithms that we have decoders for

## Functions<a href="https://docs.rs/polars/latest/polars/prelude/compression/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/compression/fn.maybe_decompress_bytes.html" class="fn" title="fn polars::prelude::compression::maybe_decompress_bytes">maybe_decompress_bytes</a>  
Decompress `bytes` if compression is detected, otherwise simply return it. An `out` vec must be given for ownership of the decompressed data.
