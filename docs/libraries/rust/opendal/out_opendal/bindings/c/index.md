- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/c/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- C ðŸš§

On this page

# C ðŸš§

## Apache OpenDALâ„¢ C Binding (WIP)

[<img src="out_opendal/bindings/c/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/c/)

<img src="out_opendal/bindings/c/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

## Example<a href="https://opendal.apache.org/bindings/c/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

A simple read and write example

``` prism-code
#include "assert.h"
#include "opendal.h"
#include "stdio.h"

int main()
{
    /* Initialize a operator for "memory" backend, with no options */
    opendal_result_operator_new result = opendal_operator_new("memory", 0);
    assert(result.operator_ptr != NULL);
    assert(result.error == NULL);

    /* Prepare some data to be written */
    opendal_bytes data = {
        .data = (uint8_t*)"this_string_length_is_24",
        .len = 24,
    };

    /* Write this into path "/testpath" */
    opendal_error *error = opendal_operator_write(op, "/testpath", &data);
    assert(error == NULL);

    /* We can read it out, make sure the data is the same */
    opendal_result_read r = opendal_operator_read(op, "/testpath");
    opendal_bytes read_bytes = r.data;
    assert(r.error == NULL);
    assert(read_bytes.len == 24);

    /* Lets print it out */
    for (int i = 0; i < 24; ++i) {
        printf("%c", read_bytes.data[i]);
    }
    printf("\n");

    /* the opendal_bytes read is heap allocated, please free it */
    opendal_bytes_free(&read_bytes);

    /* the operator_ptr is also heap allocated */
    opendal_operator_free(&op);
}
```

For more examples, please refer to `./examples`

## Prerequisites<a href="https://opendal.apache.org/bindings/c/#prerequisites" class="hash-link" aria-label="Direct link to Prerequisites" translate="no" title="Direct link to Prerequisites">â€‹</a>

To build OpenDAL C binding, the following is all you need:

- A compiler that supports **C11** and **C++14**, *e.g.* clang and gcc

- To format the code, you need to install **clang-format**

  - The `opendal.h` is not formatted by hands when you contribute, please do not format the file. **Use `make format` only.**
  - If your contribution is related to the files under `./tests`, you may format it before submitting your pull request. But notice that different versions of `clang-format` may format the files differently.

- (optional) **Doxygen** need to be installed to generate documentations.

For Ubuntu and Debian:

``` prism-code
# install C/C++ toolchain
sudo apt install -y build-essential

# install clang-format
sudo apt install clang-format

# install and build GTest library under /usr/lib and softlink to /usr/local/lib
sudo apt-get install libgtest-dev

# install CMake
sudo apt-get install cmake

# install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Makefile<a href="https://opendal.apache.org/bindings/c/#makefile" class="hash-link" aria-label="Direct link to Makefile" translate="no" title="Direct link to Makefile">â€‹</a>

- To **build the library and header file**.

  ``` prism-code
  mkdir -p build && cd build
  cmake ..
  make
  ```

  - The header file `opendal.h` is under `./include`

  - The library is under `../../target/debug` after building.

  - use `FEATURES` to enable services, like `cmake .. -DFEATURES="opendal/services-memory,opendal/services-fs"`

- To **clean** the build results.

  ``` prism-code
  cargo clean
  cd build && make clean
  ```

- To build and run the **tests**. (Note that you need to install Valgrind and GTest)

  ``` prism-code
  cd build
  make tests && ./tests
  ```

- To build the **examples**

  ``` prism-code
  cd build
  make basic error_handle
  ```

## Documentation<a href="https://opendal.apache.org/bindings/c/#documentation" class="hash-link" aria-label="Direct link to Documentation" translate="no" title="Direct link to Documentation">â€‹</a>

The documentation index page source is under `./docs/doxygen/html/index.html`. If you want to build the documentations yourself, you could use

``` prism-code
# this requires you to install doxygen
make doc
```

## Used by<a href="https://opendal.apache.org/bindings/c/#used-by" class="hash-link" aria-label="Direct link to Used by" translate="no" title="Direct link to Used by">â€‹</a>

Check out the [users](https://github.com/apache/opendal/blob/main/bindings/c/users.md) list for more details on who is using OpenDAL.

## License and Trademarks<a href="https://opendal.apache.org/bindings/c/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/c.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/c/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/category/bindings/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Bindings

<a href="https://opendal.apache.org/bindings/cpp/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Cpp ðŸš§

- <a href="https://opendal.apache.org/bindings/c/#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/c/#prerequisites" class="table-of-contents__link toc-highlight">Prerequisites</a>
- <a href="https://opendal.apache.org/bindings/c/#makefile" class="table-of-contents__link toc-highlight">Makefile</a>
- <a href="https://opendal.apache.org/bindings/c/#documentation" class="table-of-contents__link toc-highlight">Documentation</a>
- <a href="https://opendal.apache.org/bindings/c/#used-by" class="table-of-contents__link toc-highlight">Used by</a>
- <a href="https://opendal.apache.org/bindings/c/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
