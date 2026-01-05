- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/python/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Python

On this page

# Python

## Apache OpenDALâ„¢ Python Binding

[<img src="out_opendal/bindings/python/index_media/745e4981547972eb827f306a82346e6d4bd99dd0.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Status" />](https://pypi.org/project/opendal/) [<img src="out_opendal/bindings/python/index_media/8f4ff149b3e9e10b48e511feae7fcf1ec32bef63.svg" class="img_KBPg" decoding="async" loading="lazy" alt="PyPI" />](https://pypi.org/project/opendal/) [<img src="out_opendal/bindings/python/index_media/b8908a5bd08b3de6e77ea4c1c7aa4f9e733185d5.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Website" />](https://opendal.apache.org/docs/python/)

This package provides a native Python binding for **Apache OpenDALâ„¢**, a data access layer that allows you to access various storage services in a unified way.

<img src="out_opendal/bindings/python/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" alt="OpenDAL Python Usage Demo" />

## Useful Links<a href="https://opendal.apache.org/bindings/python/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- [Documentation](https://opendal.apache.org/docs/python/)
- [Examples](https://github.com/apache/opendal/blob/main/bindings/python/docs/examples)
- [Upgrade Guide](https://github.com/apache/opendal/blob/main/bindings/python/upgrade.md)

------------------------------------------------------------------------

## Features<a href="https://opendal.apache.org/bindings/python/#features" class="hash-link" aria-label="Direct link to Features" translate="no" title="Direct link to Features">â€‹</a>

- **Unified API**: Access S3, GCS, Azure Blob, HDFS, FTP, and more with the same set of commands.
- **Native Performance**: Built in Rust for high performance and safety.
- **Async Support**: First-class `async` API for modern Python applications.
- **Easy to Use**: Simple and intuitive API design.

------------------------------------------------------------------------

## Installation<a href="https://opendal.apache.org/bindings/python/#installation" class="hash-link" aria-label="Direct link to Installation" translate="no" title="Direct link to Installation">â€‹</a>

Install the package directly from PyPI:

``` prism-code
pip install opendal
```

------------------------------------------------------------------------

## Usage<a href="https://opendal.apache.org/bindings/python/#usage" class="hash-link" aria-label="Direct link to Usage" translate="no" title="Direct link to Usage">â€‹</a>

Here are a few examples of how to use OpenDAL with different storage backends.

### Local Filesystem (`fs`)<a href="https://opendal.apache.org/bindings/python/#local-filesystem-fs" class="hash-link" aria-label="Direct link to local-filesystem-fs" translate="no" title="Direct link to local-filesystem-fs">â€‹</a>

``` prism-code
import opendal

# Initialize the operator for the local filesystem
op = opendal.Operator("fs", root="/tmp")

# Write data to a file
op.write("test.txt", b"Hello World")

# Read data from the file
content = op.read("test.txt")
print(op.read("test.txt"))

# Get metadata
metadata = op.stat("test.txt")
print(f"Content length: {metadata.content_length}") # Output: 11
```

### Amazon S3<a href="https://opendal.apache.org/bindings/python/#amazon-s3" class="hash-link" aria-label="Direct link to Amazon S3" translate="no" title="Direct link to Amazon S3">â€‹</a>

The API remains the sameâ€”just change the scheme and credentials.

``` prism-code
import opendal

# Initialize the operator for S3
op = opendal.Operator(
    "s3",
    bucket="your_bucket_name",
    region="your_region",
    root="/path/to/root"
)

op.write("test.txt", b"Hello World")
print(op.read("test.txt"))
print(op.stat("test.txt").content_length)
```

### Async Usage (`s3`)<a href="https://opendal.apache.org/bindings/python/#async-usage-s3" class="hash-link" aria-label="Direct link to async-usage-s3" translate="no" title="Direct link to async-usage-s3">â€‹</a>

OpenDAL also provides a fully asynchronous API.

``` prism-code
import asyncio
import opendal

async def main():
    # Use AsyncOperator for async operations
    op = opendal.AsyncOperator("s3", root="/tmp", bucket="your_bucket_name", region="your_region")

    await op.write("test.txt", b"Hello World")
    print(await op.read("test.txt"))

asyncio.run(main())
```

------------------------------------------------------------------------

## Development<a href="https://opendal.apache.org/bindings/python/#development" class="hash-link" aria-label="Direct link to Development" translate="no" title="Direct link to Development">â€‹</a>

This project uses [`just`](https://github.com/casey/just) as a command runner to simplify the development workflow.

1.  **Clone the repository and set up the environment:**

    ``` prism-code
    # This will create a virtual environment and install all dependencies
    just setup
    ```

2.  **Run tests:**

    ``` prism-code
    # Example: Run tests for the 'fs' operator
    OPENDAL_TEST=fs OPENDAL_FS_ROOT=/tmp just test
    ```

For a complete guide on building, testing, and contributing, please see our **[CONTRIBUTING.md](https://github.com/apache/opendal/blob/main/bindings/python/CONTRIBUTING.md)** file.

------------------------------------------------------------------------

## Used By<a href="https://opendal.apache.org/bindings/python/#used-by" class="hash-link" aria-label="Direct link to Used By" translate="no" title="Direct link to Used By">â€‹</a>

Check out the [users list](https://github.com/apache/opendal/blob/main/bindings/python/users.md) for more details on who is using OpenDAL.

## License and Trademarks<a href="https://opendal.apache.org/bindings/python/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/python.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/python/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/bindings/php/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

PHP ðŸš§

<a href="https://opendal.apache.org/bindings/ruby/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Ruby

- <a href="https://opendal.apache.org/bindings/python/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/bindings/python/#features" class="table-of-contents__link toc-highlight">Features</a>
- <a href="https://opendal.apache.org/bindings/python/#installation" class="table-of-contents__link toc-highlight">Installation</a>
- <a href="https://opendal.apache.org/bindings/python/#usage" class="table-of-contents__link toc-highlight">Usage</a>
  - <a href="https://opendal.apache.org/bindings/python/#local-filesystem-fs" class="table-of-contents__link toc-highlight">Local Filesystem (<code>fs</code>)</a>
  - <a href="https://opendal.apache.org/bindings/python/#amazon-s3" class="table-of-contents__link toc-highlight">Amazon S3</a>
  - <a href="https://opendal.apache.org/bindings/python/#async-usage-s3" class="table-of-contents__link toc-highlight">Async Usage (<code>s3</code>)</a>
- <a href="https://opendal.apache.org/bindings/python/#development" class="table-of-contents__link toc-highlight">Development</a>
- <a href="https://opendal.apache.org/bindings/python/#used-by" class="table-of-contents__link toc-highlight">Used By</a>
- <a href="https://opendal.apache.org/bindings/python/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
