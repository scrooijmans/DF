- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/cpp/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Cpp ðŸš§

On this page

# Cpp ðŸš§

## Apache OpenDALâ„¢ CPP Binding (WIP)

[<img src="out_opendal/bindings/cpp/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/cpp/)

> **Note**: This C++ binding follows the [Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html) for consistent and maintainable code.

<img src="out_opendal/bindings/cpp/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

Documents: [<img src="out_opendal/bindings/cpp/index_media/48ef1409089f80fe8a94dfc5a43b5e5fc6764f21.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Documents" />](https://opendal.apache.org/docs/cpp/)

## Example<a href="https://opendal.apache.org/bindings/cpp/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

``` prism-code
#include "opendal.hpp"
#include <vector>

int main() {
    auto op = opendal::Operator("memory");
    std::vector<uint8_t> data = {1, 2, 3, 4, 5};
    op.Write("test", data);
    auto result = op.Read("test");  // result == data
}
```

More examples can be found [here](https://opendal.apache.org/examples/cpp).

## Using<a href="https://opendal.apache.org/bindings/cpp/#using" class="hash-link" aria-label="Direct link to Using" translate="no" title="Direct link to Using">â€‹</a>

### CMake<a href="https://opendal.apache.org/bindings/cpp/#cmake" class="hash-link" aria-label="Direct link to CMake" translate="no" title="Direct link to CMake">â€‹</a>

You can use `FetchContent` to add OpenDAL to your project.

``` prism-code
FetchContent_Declare(
  opendal-cpp
  GIT_REPOSITORY https://github.com/apache/opendal.git
  GIT_TAG        v0.40.0
  SOURCE_SUBDIR  bindings/cpp
)
FetchContent_MakeAvailable(opendal-cpp)
```

Or you can download the source code and add it to your project.

``` prism-code
mkdir third_party
cd third_party
git clone https://github.com/apache/opendal.git
git checkout v0.40.0
```

``` prism-code
add_subdirectory(third_party/opendal/bindings/cpp)
```

Now you can use OpenDAL in your project.

``` prism-code
target_link_libraries(your_target opendal_cpp)
```

### Others<a href="https://opendal.apache.org/bindings/cpp/#others" class="hash-link" aria-label="Direct link to Others" translate="no" title="Direct link to Others">â€‹</a>

Support for more package managers is coming soon!

## Compiling<a href="https://opendal.apache.org/bindings/cpp/#compiling" class="hash-link" aria-label="Direct link to Compiling" translate="no" title="Direct link to Compiling">â€‹</a>

### Prerequisites<a href="https://opendal.apache.org/bindings/cpp/#prerequisites" class="hash-link" aria-label="Direct link to Prerequisites" translate="no" title="Direct link to Prerequisites">â€‹</a>

- CMake \>= 3.22
- C++ compiler with C++17 support
- **Currently only Clang or AppleClang are supported**

### Build<a href="https://opendal.apache.org/bindings/cpp/#build" class="hash-link" aria-label="Direct link to Build" translate="no" title="Direct link to Build">â€‹</a>

``` prism-code
mkdir build
cd build
# Add -DOPENDAL_DEV=ON to make development environment for OpenDAL
cmake -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ ..
make
```

### Dev Setup<a href="https://opendal.apache.org/bindings/cpp/#dev-setup" class="hash-link" aria-label="Direct link to Dev Setup" translate="no" title="Direct link to Dev Setup">â€‹</a>

We provide a default VSCode configuration in `.vscode/settings.json` at the project root. After installing the clangd extension in VSCode, restart the editor to enable proper code completion and IntelliSense support.

### Test<a href="https://opendal.apache.org/bindings/cpp/#test" class="hash-link" aria-label="Direct link to Test" translate="no" title="Direct link to Test">â€‹</a>

You should build the project with `OPENDAL_ENABLE_TESTING` option. Then run:

``` prism-code
make test
```

### Docs<a href="https://opendal.apache.org/bindings/cpp/#docs" class="hash-link" aria-label="Direct link to Docs" translate="no" title="Direct link to Docs">â€‹</a>

You should build the project with `OPENDAL_ENABLE_DOCUMENTATION` option. Then run:

``` prism-code
make docs
```

### CMake Options<a href="https://opendal.apache.org/bindings/cpp/#cmake-options" class="hash-link" aria-label="Direct link to CMake Options" translate="no" title="Direct link to CMake Options">â€‹</a>

- `OPENDAL_DEV`: Enable development environment for OpenDAL. It will enable most development options. With this option, you don't need to set other options. Default: `OFF`
- `OPENDAL_ENABLE_ADDRESS_SANITIZER`: Enable address sanitizer. Default: `OFF`
- `OPENDAL_ENABLE_DOCUMENTATION`: Enable documentation. Default: `OFF`
- `OPENDAL_DOCS_ONLY`: Only build documentation. Default: `OFF`
- `OPENDAL_ENABLE_TESTING`: Enable testing. Default: `OFF`
- `OPENDAL_ENABLE_ASYNC`: Enable async support. Requires Clang or AppleClang with C++20. Default: `OFF`
- `OPENDAL_FEATURES`: Specify OpenDAL services to include, like `"opendal/services-s3,opendal/services-memory"`. Default: `""`

## License and Trademarks<a href="https://opendal.apache.org/bindings/cpp/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/cpp.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/cpp/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/bindings/c/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

C ðŸš§

<a href="https://opendal.apache.org/bindings/d/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

D ðŸš§

- <a href="https://opendal.apache.org/bindings/cpp/#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/cpp/#using" class="table-of-contents__link toc-highlight">Using</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#cmake" class="table-of-contents__link toc-highlight">CMake</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#others" class="table-of-contents__link toc-highlight">Others</a>
- <a href="https://opendal.apache.org/bindings/cpp/#compiling" class="table-of-contents__link toc-highlight">Compiling</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#prerequisites" class="table-of-contents__link toc-highlight">Prerequisites</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#build" class="table-of-contents__link toc-highlight">Build</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#dev-setup" class="table-of-contents__link toc-highlight">Dev Setup</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#test" class="table-of-contents__link toc-highlight">Test</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#docs" class="table-of-contents__link toc-highlight">Docs</a>
  - <a href="https://opendal.apache.org/bindings/cpp/#cmake-options" class="table-of-contents__link toc-highlight">CMake Options</a>
- <a href="https://opendal.apache.org/bindings/cpp/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
