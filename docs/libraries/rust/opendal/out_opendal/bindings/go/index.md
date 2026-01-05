- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/go/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Go

On this page

# Go

## Apache OpenDALâ„¢ Go Binding

[<img src="out_opendal/bindings/go/index_media/745e4981547972eb827f306a82346e6d4bd99dd0.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://pkg.go.dev/github.com/apache/opendal/bindings/go) [<img src="out_opendal/bindings/go/index_media/dca7434755cb04db93ad1dacf6dbc900368e9b80.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Go Reference" />](https://pkg.go.dev/github.com/apache/opendal/bindings/go)

opendal-go is a **Native** support Go binding without CGO enabled and is built on top of opendal-c.

``` prism-code
go get github.com/apache/opendal/bindings/go@latest
```

opendal-go requires **libffi** to be installed.

## Basic Usage<a href="https://opendal.apache.org/bindings/go/#basic-usage" class="hash-link" aria-label="Direct link to Basic Usage" translate="no" title="Direct link to Basic Usage">â€‹</a>

``` prism-code
package main

import (
 "fmt"
  "github.com/apache/opendal-go-services/memory"
 opendal "github.com/apache/opendal/bindings/go"
)

func main() {
  // Initialize a new in-memory operator
  op, err := opendal.NewOperator(memory.Scheme, opendal.OperatorOptions{})
    if err != nil {
        panic(err)
    }
 defer op.Close()

   // Write data to a file named "test"
    err = op.Write("test", []byte("Hello opendal go binding!"))
    if err != nil {
        panic(err)
    }

    // Read data from the file "test"
   data, err := op.Read("test")
 if err != nil {
        panic(err)
    }
 fmt.Printf("Read content: %s\n", data)

 // List all entries under the root directory "/"
    lister, err := op.List("/")
  if err != nil {
        panic(err)
    }
 defer lister.Close()

   // Iterate through all entries
  for lister.Next() {
       entry := lister.Entry()

       // Get entry name (not used in this example)
        _ = entry.Name()

        // Get metadata for the current entry
       meta, _ := op.Stat(entry.Path())

        // Print file size
      fmt.Printf("Size: %d bytes\n", meta.ContentLength())

      // Print last modified time
     fmt.Printf("Last modified: %s\n", meta.LastModified())

        // Check if the entry is a directory or a file
      fmt.Printf("Is directory: %v, Is file: %v\n", meta.IsDir(), meta.IsFile())
  }

    // Check for any errors that occurred during iteration
  if err := lister.Error(); err != nil {
       panic(err)
    }

    // Copy a file
  op.Copy("test", "test_copy")

   // Rename a file
    op.Rename("test", "test_rename")

   // Delete a file
    op.Delete("test_rename")
}
```

## Run Tests<a href="https://opendal.apache.org/bindings/go/#run-tests" class="hash-link" aria-label="Direct link to Run Tests" translate="no" title="Direct link to Run Tests">â€‹</a>

### Behavior Tests<a href="https://opendal.apache.org/bindings/go/#behavior-tests" class="hash-link" aria-label="Direct link to Behavior Tests" translate="no" title="Direct link to Behavior Tests">â€‹</a>

``` prism-code
cd tests/behavior_tests
# Test a specific backend
export OPENDAL_TEST=memory
# Run all tests
CGO_ENABLE=0 go test -v -run TestBehavior
# Run specific test
CGO_ENABLE=0 go test -v -run TestBehavior/Write
# Run synchronously
CGO_ENABLE=0 GOMAXPROCS=1 go test -v -run TestBehavior
```

### Benchmark<a href="https://opendal.apache.org/bindings/go/#benchmark" class="hash-link" aria-label="Direct link to Benchmark" translate="no" title="Direct link to Benchmark">â€‹</a>

``` prism-code
cd tests/behavior_tests
# Benchmark a specific backend
export OPENDAL_TEST=memory

go test -bench .
```

A benchmark between OpenDAL Go binding and aws-sdk-go on minio S3 compatible storage

``` prism-code
goos: linux
goarch: amd64
pkg: opendal_test
cpu: AMD EPYC 7763 64-Core Processor                
BenchmarkWrite
BenchmarkWrite/4KiB/OpenDAL
BenchmarkWrite/4KiB/OpenDAL-4                 405       3430652 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4KiB/OpenDAL-4                  313       3781825 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4KiB/OpenDAL-4                  346       3354802 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4KiB/OpenDAL-4                  397       3374467 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4KiB/OpenDAL-4                  364       6171386 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4KiB/OpenDAL-4                  388       3789795 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4KiB/AWS_S3
BenchmarkWrite/4KiB/AWS_S3-4                 384       3716060 ns/op       62929 B/op        312 allocs/op
BenchmarkWrite/4KiB/AWS_S3-4                   402       3145797 ns/op       62077 B/op        312 allocs/op
BenchmarkWrite/4KiB/AWS_S3-4                   372       3050911 ns/op       61902 B/op        312 allocs/op
BenchmarkWrite/4KiB/AWS_S3-4                   400       3081028 ns/op       61556 B/op        312 allocs/op
BenchmarkWrite/4KiB/AWS_S3-4                   342       3111741 ns/op       61970 B/op        312 allocs/op
BenchmarkWrite/4KiB/AWS_S3-4                   464       2933844 ns/op       61617 B/op        312 allocs/op
BenchmarkWrite/256KiB/OpenDAL
BenchmarkWrite/256KiB/OpenDAL-4               228       6253966 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/256KiB/OpenDAL-4                190       5859882 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/256KiB/OpenDAL-4                216       6008253 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/256KiB/OpenDAL-4                200       5958440 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/256KiB/OpenDAL-4                193       5658798 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/256KiB/OpenDAL-4                210       6250594 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/256KiB/AWS_S3
BenchmarkWrite/256KiB/AWS_S3-4                 208       5665223 ns/op       90323 B/op        312 allocs/op
BenchmarkWrite/256KiB/AWS_S3-4                 196       5368602 ns/op       88719 B/op        312 allocs/op
BenchmarkWrite/256KiB/AWS_S3-4                 218       6108315 ns/op       90784 B/op        312 allocs/op
BenchmarkWrite/256KiB/AWS_S3-4                 217       5828966 ns/op       90379 B/op        312 allocs/op
BenchmarkWrite/256KiB/AWS_S3-4                 210       6447429 ns/op       90981 B/op        312 allocs/op
BenchmarkWrite/256KiB/AWS_S3-4                 177       6323915 ns/op       91548 B/op        313 allocs/op
BenchmarkWrite/4MiB/OpenDAL
BenchmarkWrite/4MiB/OpenDAL-4                62      19049500 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4MiB/OpenDAL-4                   54      19271521 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4MiB/OpenDAL-4                   69      17849900 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4MiB/OpenDAL-4                   73      18117693 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4MiB/OpenDAL-4                   74      18249816 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4MiB/OpenDAL-4                   62      19556730 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/4MiB/AWS_S3
BenchmarkWrite/4MiB/AWS_S3-4                  43      29514084 ns/op       92211 B/op        320 allocs/op
BenchmarkWrite/4MiB/AWS_S3-4                    39      29297735 ns/op       92514 B/op        320 allocs/op
BenchmarkWrite/4MiB/AWS_S3-4                    42      28956593 ns/op       92282 B/op        320 allocs/op
BenchmarkWrite/4MiB/AWS_S3-4                    42      29218821 ns/op       92282 B/op        320 allocs/op
BenchmarkWrite/4MiB/AWS_S3-4                    40      28988214 ns/op       91391 B/op        320 allocs/op
BenchmarkWrite/4MiB/AWS_S3-4                    43      28668319 ns/op       91242 B/op        320 allocs/op
BenchmarkWrite/16MiB/OpenDAL
BenchmarkWrite/16MiB/OpenDAL-4                  21      53528117 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/16MiB/OpenDAL-4                  21      55328986 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/16MiB/OpenDAL-4                  21      54221620 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/16MiB/OpenDAL-4                  20      54044030 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/16MiB/OpenDAL-4                  21      53692610 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/16MiB/OpenDAL-4                  21      53288370 ns/op         384 B/op         10 allocs/op
BenchmarkWrite/16MiB/AWS_S3
BenchmarkWrite/16MiB/AWS_S3-4                12      99294840 ns/op       99850 B/op        322 allocs/op
BenchmarkWrite/16MiB/AWS_S3-4                   12      97405067 ns/op       99850 B/op        322 allocs/op
BenchmarkWrite/16MiB/AWS_S3-4                   12      97906212 ns/op       99848 B/op        322 allocs/op
BenchmarkWrite/16MiB/AWS_S3-4                   12      98766864 ns/op       96378 B/op        322 allocs/op
BenchmarkWrite/16MiB/AWS_S3-4                   12      97967605 ns/op       99850 B/op        322 allocs/op
BenchmarkWrite/16MiB/AWS_S3-4                   12      97842268 ns/op       96376 B/op        322 allocs/op
BenchmarkRead
BenchmarkRead/4KiB/OpenDAL
BenchmarkRead/4KiB/OpenDAL-4                972       1217197 ns/op        4760 B/op         15 allocs/op
BenchmarkRead/4KiB/OpenDAL-4                   985       1226752 ns/op        4760 B/op         15 allocs/op
BenchmarkRead/4KiB/OpenDAL-4                   982       1216827 ns/op        4760 B/op         15 allocs/op
BenchmarkRead/4KiB/OpenDAL-4                   987       1227682 ns/op        4760 B/op         15 allocs/op
BenchmarkRead/4KiB/OpenDAL-4                   987       1214420 ns/op        4760 B/op         15 allocs/op
BenchmarkRead/4KiB/OpenDAL-4                   987       1215074 ns/op        4760 B/op         15 allocs/op
BenchmarkRead/4KiB/AWS_S3
BenchmarkRead/4KiB/AWS_S3-4                  1594        759864 ns/op       34988 B/op        344 allocs/op
BenchmarkRead/4KiB/AWS_S3-4                   1605        751628 ns/op       34990 B/op        344 allocs/op
BenchmarkRead/4KiB/AWS_S3-4                   1620        744649 ns/op       34991 B/op        344 allocs/op
BenchmarkRead/4KiB/AWS_S3-4                   1623        752885 ns/op       34994 B/op        344 allocs/op
BenchmarkRead/4KiB/AWS_S3-4                   1629        742307 ns/op       34991 B/op        344 allocs/op
BenchmarkRead/4KiB/AWS_S3-4                   1622        750455 ns/op       34992 B/op        344 allocs/op
BenchmarkRead/256KiB/OpenDAL
BenchmarkRead/256KiB/OpenDAL-4                 741       1612259 ns/op      262812 B/op         15 allocs/op
BenchmarkRead/256KiB/OpenDAL-4                 727       1633753 ns/op      262812 B/op         15 allocs/op
BenchmarkRead/256KiB/OpenDAL-4                 751       1619770 ns/op      262812 B/op         15 allocs/op
BenchmarkRead/256KiB/OpenDAL-4                 756       1619435 ns/op      262812 B/op         15 allocs/op
BenchmarkRead/256KiB/OpenDAL-4                 750       1629211 ns/op      262812 B/op         15 allocs/op
BenchmarkRead/256KiB/OpenDAL-4                 757       1611646 ns/op      262812 B/op         15 allocs/op
BenchmarkRead/256KiB/AWS_S3
BenchmarkRead/256KiB/AWS_S3-4               736       1655133 ns/op     1209847 B/op        365 allocs/op
BenchmarkRead/256KiB/AWS_S3-4                  735       1673714 ns/op     1209874 B/op        365 allocs/op
BenchmarkRead/256KiB/AWS_S3-4                  735       1653903 ns/op     1209852 B/op        365 allocs/op
BenchmarkRead/256KiB/AWS_S3-4                  728       1643222 ns/op     1209855 B/op        365 allocs/op
BenchmarkRead/256KiB/AWS_S3-4                  726       1674202 ns/op     1209842 B/op        365 allocs/op
BenchmarkRead/256KiB/AWS_S3-4                  717       1656092 ns/op     1209876 B/op        365 allocs/op
BenchmarkRead/4MiB/OpenDAL
BenchmarkRead/4MiB/OpenDAL-4                 250       4933426 ns/op     4194991 B/op         15 allocs/op
BenchmarkRead/4MiB/OpenDAL-4                   255       4874283 ns/op     4194991 B/op         15 allocs/op
BenchmarkRead/4MiB/OpenDAL-4                   252       4769956 ns/op     4194992 B/op         16 allocs/op
BenchmarkRead/4MiB/OpenDAL-4                   244       4800674 ns/op     4194992 B/op         16 allocs/op
BenchmarkRead/4MiB/OpenDAL-4                   242       4826974 ns/op     4194992 B/op         15 allocs/op
BenchmarkRead/4MiB/OpenDAL-4                   254       4882666 ns/op     4194996 B/op         16 allocs/op
BenchmarkRead/4MiB/AWS_S3
BenchmarkRead/4MiB/AWS_S3-4                   122      10379507 ns/op    21133764 B/op        388 allocs/op
BenchmarkRead/4MiB/AWS_S3-4                    100      10838399 ns/op    21133778 B/op        389 allocs/op
BenchmarkRead/4MiB/AWS_S3-4                     92      11831700 ns/op    21133738 B/op        388 allocs/op
BenchmarkRead/4MiB/AWS_S3-4                    100      10002235 ns/op    21133764 B/op        388 allocs/op
BenchmarkRead/4MiB/AWS_S3-4                    100      10416467 ns/op    21133731 B/op        388 allocs/op
BenchmarkRead/4MiB/AWS_S3-4                    100      10452328 ns/op    21133719 B/op        388 allocs/op
BenchmarkRead/16MiB/OpenDAL
BenchmarkRead/16MiB/OpenDAL-4                81      14654347 ns/op    16777903 B/op         15 allocs/op
BenchmarkRead/16MiB/OpenDAL-4                   78      14977822 ns/op    16777908 B/op         16 allocs/op
BenchmarkRead/16MiB/OpenDAL-4                   81      14833921 ns/op    16777906 B/op         16 allocs/op
BenchmarkRead/16MiB/OpenDAL-4                   76      14663855 ns/op    16777906 B/op         16 allocs/op
BenchmarkRead/16MiB/OpenDAL-4                   75      14545209 ns/op    16777906 B/op         16 allocs/op
BenchmarkRead/16MiB/OpenDAL-4                   80      14585250 ns/op    16777905 B/op         16 allocs/op
BenchmarkRead/16MiB/AWS_S3
BenchmarkRead/16MiB/AWS_S3-4                  52      24826707 ns/op    102660545 B/op       398 allocs/op
BenchmarkRead/16MiB/AWS_S3-4                    46      24588321 ns/op    102660529 B/op       397 allocs/op
BenchmarkRead/16MiB/AWS_S3-4                    44      26526367 ns/op    102660527 B/op       397 allocs/op
BenchmarkRead/16MiB/AWS_S3-4                    44      26996660 ns/op    102660522 B/op       397 allocs/op
BenchmarkRead/16MiB/AWS_S3-4                    42      26319939 ns/op    102660552 B/op       398 allocs/op
BenchmarkRead/16MiB/AWS_S3-4                    45      25286454 ns/op    102660542 B/op       398 allocs/op
PASS
ok    opendal_test    147.053s
```

**Diff** with [benchstat](https://pkg.go.dev/golang.org/x/perf/cmd/benchstat)

``` prism-code
benchstat aws.txt opendal.txt
goos: linux
goarch: amd64
pkg: opendal_test
cpu: AMD EPYC 7763 64-Core Processor                
             â    aws.txt    â             opendal.txt              â
             â    sec/op     â    sec/op      vs base               â
Write/4KiB      3.096m Â± 20%    3.606m Â± 71%  +16.47% (p=0.015 n=6)
Write/256KiB    5.969m Â± 10%    5.983m Â±  5%        ~ (p=1.000 n=6)
Write/4MiB      29.10m Â±  1%    18.65m Â±  5%  -35.92% (p=0.002 n=6)
Write/16MiB     97.94m Â±  1%    53.87m Â±  3%  -45.00% (p=0.002 n=6)
Read/4KiB       751.0Âµ Â±  1%   1217.0Âµ Â±  1%  +62.04% (p=0.002 n=6)
Read/256KiB     1.656m Â±  1%    1.620m Â±  1%   -2.18% (p=0.002 n=6)
Read/4MiB      10.434m Â± 13%    4.851m Â±  2%  -53.51% (p=0.002 n=6)
Read/16MiB      25.80m Â±  5%    14.66m Â±  2%  -43.19% (p=0.002 n=6)
geomean         8.050m          6.461m        -19.73%

             â    aws.txt    â             opendal.txt             â
             â     B/op      â     B/op      vs base               â
Write/4KiB      61936.0 Â± 2%     384.0 Â± 0%  -99.38% (p=0.002 n=6)
Write/256KiB    90581.5 Â± 2%     384.0 Â± 0%  -99.58% (p=0.002 n=6)
Write/4MiB      92246.5 Â± 1%     384.0 Â± 0%  -99.58% (p=0.002 n=6)
Write/16MiB     99849.0 Â± 3%     384.0 Â± 0%  -99.62% (p=0.002 n=6)
Read/4KiB      34.171Ki Â± 0%   4.648Ki Â± 0%  -86.40% (p=0.002 n=6)
Read/256KiB    1181.5Ki Â± 0%   256.7Ki Â± 0%  -78.28% (p=0.002 n=6)
Read/4MiB      20.155Mi Â± 0%   4.001Mi Â± 0%  -80.15% (p=0.002 n=6)
Read/16MiB      97.90Mi Â± 0%   16.00Mi Â± 0%  -83.66% (p=0.002 n=6)
geomean         500.3Ki        14.12Ki       -97.18%

             â   aws.txt   â            opendal.txt            â
             â  allocs/op  â allocs/op   vs base               â
Write/4KiB     312.00 Â± 0%   10.00 Â± 0%  -96.79% (p=0.002 n=6)
Write/256KiB   312.00 Â± 0%   10.00 Â± 0%  -96.79% (p=0.002 n=6)
Write/4MiB     320.00 Â± 0%   10.00 Â± 0%  -96.88% (p=0.002 n=6)
Write/16MiB    322.00 Â± 0%   10.00 Â± 0%  -96.89% (p=0.002 n=6)
Read/4KiB      344.00 Â± 0%   15.00 Â± 0%  -95.64% (p=0.002 n=6)
Read/256KiB    365.00 Â± 0%   15.00 Â± 0%  -95.89% (p=0.002 n=6)
Read/4MiB      388.00 Â± 0%   15.50 Â± 3%  -96.01% (p=0.002 n=6)
Read/16MiB     397.50 Â± 0%   16.00 Â± 6%  -95.97% (p=0.002 n=6)
geomean         343.6        12.40       -96.39%
```

## Capabilities<a href="https://opendal.apache.org/bindings/go/#capabilities" class="hash-link" aria-label="Direct link to Capabilities" translate="no" title="Direct link to Capabilities">â€‹</a>

- [x] OperatorInfo
- [x] Stat
  - [x] Metadata
- [x] IsExist
- [x] Read
  - [x] Read
  - [x] Reader -- implement as `io.ReadSeekCloser`
- [x] Write
  - [x] Write
  - [x] Writer -- implement as `io.WriteCloser`
- [x] Delete
- [x] CreateDir
- [ ] Lister
  - [x] Entry
  - [ ] Metadata -- Need support from the C binding
- [x] Copy
- [x] Rename

## Development<a href="https://opendal.apache.org/bindings/go/#development" class="hash-link" aria-label="Direct link to Development" translate="no" title="Direct link to Development">â€‹</a>

The guide is based on Linux and Windows. For other platforms, please adjust the commands accordingly.

To develop the Go binding, you need to have the following dependencies installed:

- zstd
- Rust toolchain
- Go
- (Required for Windows) libffi-8.dll in the root of the workspace directory

We use `go workspace` to manage and build the dependencies. To set up the workspace, run the following commands:

For Linux, macOS and Windows (MSVC)

  

``` prism-code
mkdir opendal_workspace
cd opendal_workspace
git clone --depth 1 git@github.com:apache/opendal.git
git clone --depth 1 git@github.com:apache/opendal-go-services.git

go work init
go work use ./opendal/bindings/go
go work use ./opendal/bindings/go/tests/behavior_tests

cp opendal/bindings/go/Makefile .

cd -
```

To build and run tests, run the following commands:

``` prism-code
cd opendal_workspace

# specify the backend to test
export OPENDAL_TEST=fs
export OPENDAL_FS_ROOT=/tmp/opendal

make tests

cd -
```

To run the benchmarks, you can use the following command:

``` prism-code
make bench
```

## License and Trademarks<a href="https://opendal.apache.org/bindings/go/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/go.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/go/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/bindings/dotnet/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Dotnet ðŸš§

<a href="https://opendal.apache.org/bindings/haskell/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Haskell ðŸš§

- <a href="https://opendal.apache.org/bindings/go/#basic-usage" class="table-of-contents__link toc-highlight">Basic Usage</a>
- <a href="https://opendal.apache.org/bindings/go/#run-tests" class="table-of-contents__link toc-highlight">Run Tests</a>
  - <a href="https://opendal.apache.org/bindings/go/#behavior-tests" class="table-of-contents__link toc-highlight">Behavior Tests</a>
  - <a href="https://opendal.apache.org/bindings/go/#benchmark" class="table-of-contents__link toc-highlight">Benchmark</a>
- <a href="https://opendal.apache.org/bindings/go/#capabilities" class="table-of-contents__link toc-highlight">Capabilities</a>
- <a href="https://opendal.apache.org/bindings/go/#development" class="table-of-contents__link toc-highlight">Development</a>
- <a href="https://opendal.apache.org/bindings/go/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
