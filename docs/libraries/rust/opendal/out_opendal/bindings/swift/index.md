- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/swift/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Swift ðŸš§

On this page

# Swift ðŸš§

## Apache OpenDALâ„¢ Swift Binding (WIP)

[<img src="out_opendal/bindings/swift/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/swift/)

<img src="out_opendal/bindings/swift/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

## Using the Package<a href="https://opendal.apache.org/bindings/swift/#using-the-package" class="hash-link" aria-label="Direct link to Using the Package" translate="no" title="Direct link to Using the Package">â€‹</a>

### Build C Dependencies<a href="https://opendal.apache.org/bindings/swift/#build-c-dependencies" class="hash-link" aria-label="Direct link to Build C Dependencies" translate="no" title="Direct link to Build C Dependencies">â€‹</a>

The Swift binding depends on the C binding to OpenDAL. Before using this package, you need to build the C library first:

``` prism-code
cd bindings/swift
make build-c
```

To check whether the package is ready to use, simply run the test:

``` prism-code
make test
```

### Add Dependency to Your Project<a href="https://opendal.apache.org/bindings/swift/#add-dependency-to-your-project" class="hash-link" aria-label="Direct link to Add Dependency to Your Project" translate="no" title="Direct link to Add Dependency to Your Project">â€‹</a>

The package manifest is not located at the root directory of its repository. To use it, add the path of this package to the `Package.swift` manifest of your project:

``` prism-code
// swift-tools-version:5.7
import PackageDescription

let package = Package(
  name: "MyTool",
  dependencies: [
    .package(path: "/path/to/opendal/bindings/swift/OpenDAL"),
  ],
  targets: [
    .target(name: "MyTool", dependencies: [
      .product(name: "OpenDAL", package: "OpenDAL"),
    ]),
  ]
)
```

## Example<a href="https://opendal.apache.org/bindings/swift/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

The demo below shows how to write a key to the memory storage, and read it back:

``` prism-code
import OpenDAL

// Create an operator with `memory` backend.
let op = try Operator(scheme: "memory")

// Write some data into path `/demo`.
let someData = Data([1, 2, 3, 4])
try op.blockingWrite(someData, to: "/demo")

// Read the data back.
let readData = try op.blockingRead("/demo")

// You can use the read data here.
print(readData!)
```

## License and Trademarks<a href="https://opendal.apache.org/bindings/swift/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/swift.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/swift/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/bindings/ruby/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Ruby

<a href="https://opendal.apache.org/bindings/zig/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Zig ðŸš§

- <a href="https://opendal.apache.org/bindings/swift/#using-the-package" class="table-of-contents__link toc-highlight">Using the Package</a>
  - <a href="https://opendal.apache.org/bindings/swift/#build-c-dependencies" class="table-of-contents__link toc-highlight">Build C Dependencies</a>
  - <a href="https://opendal.apache.org/bindings/swift/#add-dependency-to-your-project" class="table-of-contents__link toc-highlight">Add Dependency to Your Project</a>
- <a href="https://opendal.apache.org/bindings/swift/#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/swift/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
