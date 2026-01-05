- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/dart/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Dart ðŸš§

On this page

# Dart ðŸš§

## Apache OpenDALâ„¢ Dart Binding (WIP)

## Useful Links<a href="https://opendal.apache.org/bindings/dart/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- [Examples](https://github.com/apache/opendal/blob/main/bindings/dart/examples)

## Usage<a href="https://opendal.apache.org/bindings/dart/#usage" class="hash-link" aria-label="Direct link to Usage" translate="no" title="Direct link to Usage">â€‹</a>

Api is designed to be like stdlib style.

This is stdlib

``` prism-code
import 'dart:io';

void main() async {
  final file = File('file.txt');
  var is_exists = await file.exists();
  print(is_exists);
}
```

This is opendal

``` prism-code
import 'package:opendal/opendal.dart';

void main() async {
  final storage = await Storage.init(schemeStr: "fs", map: {"root": "/tmp"});
  final File = storage.initFile();
  // drop-in
  final file = File('file.txt');
  var is_exists = await file.exists();
  print(is_exists);
}
```

## Test<a href="https://opendal.apache.org/bindings/dart/#test" class="hash-link" aria-label="Direct link to Test" translate="no" title="Direct link to Test">â€‹</a>

``` prism-code
dart run tests/opendal_test.dart
```

## Development<a href="https://opendal.apache.org/bindings/dart/#development" class="hash-link" aria-label="Direct link to Development" translate="no" title="Direct link to Development">â€‹</a>

``` prism-code
flutter pub get
flutter_rust_bridge_codegen generate
cd rust
cargo build -r --target x86_64-unknown-linux-gnu # change to your arch, refer to https://doc.rust-lang.org/beta/rustc/platform-support.html
```

## Update generated code<a href="https://opendal.apache.org/bindings/dart/#update-generated-code" class="hash-link" aria-label="Direct link to Update generated code" translate="no" title="Direct link to Update generated code">â€‹</a>

This binding uses <https://github.com/fzyzcjy/flutter_rust_bridge>, when updating the codegen. First check `FLUTTER_RUST_BRIDGE_CODEGEN_VERSION`, then pin the version of `flutter_rust_bridge` in `pubspec.yaml` and `rust/Cargo.toml`. Make sure the runtime versions are matched.

## License and Trademarks<a href="https://opendal.apache.org/bindings/dart/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/dart.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/dart/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/bindings/d/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

D ðŸš§

<a href="https://opendal.apache.org/bindings/dotnet/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Dotnet ðŸš§

- <a href="https://opendal.apache.org/bindings/dart/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/bindings/dart/#usage" class="table-of-contents__link toc-highlight">Usage</a>
- <a href="https://opendal.apache.org/bindings/dart/#test" class="table-of-contents__link toc-highlight">Test</a>
- <a href="https://opendal.apache.org/bindings/dart/#development" class="table-of-contents__link toc-highlight">Development</a>
- <a href="https://opendal.apache.org/bindings/dart/#update-generated-code" class="table-of-contents__link toc-highlight">Update generated code</a>
- <a href="https://opendal.apache.org/bindings/dart/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
