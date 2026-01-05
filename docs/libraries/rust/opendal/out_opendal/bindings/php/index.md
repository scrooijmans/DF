- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/php/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- PHP ðŸš§

On this page

# PHP ðŸš§

## Apache OpenDALâ„¢ PHP Binding (WIP)

[<img src="out_opendal/bindings/php/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/php/)

## Example<a href="https://opendal.apache.org/bindings/php/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

``` prism-code
use OpenDAL\Operator;

$op = new Operator("fs", ["root" => "/tmp"]);
$op->write("test.txt", "hello world");

echo $op->read("test.txt"); // hello world
```

## Requirements<a href="https://opendal.apache.org/bindings/php/#requirements" class="hash-link" aria-label="Direct link to Requirements" translate="no" title="Direct link to Requirements">â€‹</a>

- PHP 8.1+
- Composer

## Install Extension<a href="https://opendal.apache.org/bindings/php/#install-extension" class="hash-link" aria-label="Direct link to Install Extension" translate="no" title="Direct link to Install Extension">â€‹</a>

We use [ext-php-rs](https://github.com/davidcole1340/ext-php-rs) to build PHP extensions natively in Rust, it's different from the traditional PHP extension development and cannot be installed using `pecl` or `phpize`. Before installing the extension, it is necessary to install Rust and Cargo. For instructions on how to install them, please refer to [Rust's website](https://www.rust-lang.org/tools/install).

1.  Clone the repository

``` prism-code
git clone git@github.com:apache/opendal.git
```

2.  Build the opendal-php extension

``` prism-code
cd opendal/bindings/php
cargo build
```

> don't forget to add `--release` flag for production use.

3.  Enable extension for PHP Manually

``` prism-code
cd opendal

# Linux
cp target/debug/libopendal_php.so $(php -r "echo ini_get('extension_dir');")/libopendal_php.so
echo "extension=libopendal_php.so" >> $(php -r "echo php_ini_loaded_file();")

# macOS
cp target/debug/libopendal_php.dylib $(php -r "echo ini_get('extension_dir');")/libopendal_php.dylib
echo "extension=libopendal_php.dylib" >> $(php -r "echo php_ini_loaded_file();")

# Windows
cp target/debug/libopendal_php.dll $(php -r "echo ini_get('extension_dir');")/libopendal_php.dll
echo "extension=libopendal_php.dll" >> $(php -r "echo php_ini_loaded_file();")
```

4.  Enable extension for PHP using cargo-php

You can also use cargo-php directly to install the extension, see [cargo-php](https://davidcole1340.github.io/ext-php-rs/getting-started/cargo-php.html) for more details.

``` prism-code
cargo install cargo-php
cd opendal/bindings/php
cargo php install
```

This command will automatically build the extension and copy it to the extension directory of the current PHP version.

5.  Test

use `php -m` to check if the extension is installed successfully.

``` prism-code
php -m | grep opendal-php
```

Composer test:

``` prism-code
cd opendal/bindings/php

composer install
composer test
```

## License and Trademarks<a href="https://opendal.apache.org/bindings/php/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/php.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/php/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/bindings/ocaml/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

OCaml ðŸš§

<a href="https://opendal.apache.org/bindings/python/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Python

- <a href="https://opendal.apache.org/bindings/php/#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/php/#requirements" class="table-of-contents__link toc-highlight">Requirements</a>
- <a href="https://opendal.apache.org/bindings/php/#install-extension" class="table-of-contents__link toc-highlight">Install Extension</a>
- <a href="https://opendal.apache.org/bindings/php/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
