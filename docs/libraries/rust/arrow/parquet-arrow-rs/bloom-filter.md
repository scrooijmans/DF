Description: Bloom filter implementation specific to Parquet, as described in the spec.

Title: parquet::bloom_filter - Rust

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

## Module bloom_filter

parquet

# Module bloom_filter Copy item path

Source

Expand description

Bloom filter implementation specific to Parquet, as described in the spec.

## §Bloom Filter Size

Parquet uses the Split Block Bloom Filter (SBBF) as its bloom filter implementation. For each column upon which bloom filters are enabled, the offset and length of an SBBF is stored in the metadata for each row group in the parquet file. The size of each filter is initialized using a calculation based on the desired number of distinct values (NDV) and false positive probability (FPP). The FPP for a SBBF can be approximated as1:

```
f = (1 - e^(-k * n / m))^k
```

Where, `f` is the FPP, `k` the number of hash functions, `n` the NDV, and `m` the total number of bits in the bloom filter. This can be re-arranged to determine the total number of bits required to achieve a given FPP and NDV:

```
m = -k * n / ln(1 - f^(1/k))
```

SBBFs use eight hash functions to cleanly fit in SIMD lanes2, therefore `k` is set to 8. The SBBF will spread those `m` bits accross a set of `b` blocks that are each 256 bits, i.e., 32 bytes, in size. The number of blocks is chosen as:

```
b = NP2(m/8) / 32
```

Where, `NP2` denotes _the next power of two_, and `m` is divided by 8 to be represented as bytes.

Here is a table of calculated sizes for various FPP and NDV:

| NDV       | FPP      | b       | Size (KB) |
| --------- | -------- | ------- | --------- |
| 10,000    | 0.1      | 256     | 8         |
| 10,000    | 0.01     | 512     | 16        |
| 10,000    | 0.001    | 1,024   | 32        |
| 10,000    | 0.0001   | 1,024   | 32        |
| 100,000   | 0.1      | 4,096   | 128       |
| 100,000   | 0.01     | 4,096   | 128       |
| 100,000   | 0.001    | 8,192   | 256       |
| 100,000   | 0.0001   | 16,384  | 512       |
| 100,000   | 0.00001  | 16,384  | 512       |
| 1,000,000 | 0.1      | 32,768  | 1,024     |
| 1,000,000 | 0.01     | 65,536  | 2,048     |
| 1,000,000 | 0.001    | 65,536  | 2,048     |
| 1,000,000 | 0.0001   | 131,072 | 4,096     |
| 1,000,000 | 0.00001  | 131,072 | 4,096     |
| 1,000,000 | 0.000001 | 262,144 | 8,192     |

## Structs§

Sbbf

A split block Bloom filter.
