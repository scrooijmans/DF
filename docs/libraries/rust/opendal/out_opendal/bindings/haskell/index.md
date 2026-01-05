- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/haskell/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Haskell ðŸš§

On this page

# Haskell ðŸš§

## Apache OpenDALâ„¢ Haskell Binding (WIP)

[<img src="out_opendal/bindings/haskell/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/haskell)

<img src="out_opendal/bindings/haskell/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

## Example<a href="https://opendal.apache.org/bindings/haskell#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

Basic usage

``` prism-code
import OpenDAL

main :: IO ()
main = do
  Right op <- newOperator "memory"
  runOp op operation
  where
    operation = do
      writeOp op "key1" "value1"
      writeOp op "key2" "value2"
      value1 <- readOp op "key1"
      value2 <- readOp op "key2"
```

Use logger

``` prism-code
import OpenDAL
import Colog (simpleMessageAction)

main :: IO ()
main = do
  Right op <- newOperator "memory" {ocLogAction = Just simpleMessageAction}
  return ()
```

## Build<a href="https://opendal.apache.org/bindings/haskell#build" class="hash-link" aria-label="Direct link to Build" translate="no" title="Direct link to Build">â€‹</a>

``` prism-code
cabal build
```

## Test<a href="https://opendal.apache.org/bindings/haskell#test" class="hash-link" aria-label="Direct link to Test" translate="no" title="Direct link to Test">â€‹</a>

``` prism-code
cabal test
```

## Doc<a href="https://opendal.apache.org/bindings/haskell#doc" class="hash-link" aria-label="Direct link to Doc" translate="no" title="Direct link to Doc">â€‹</a>

To generate the documentation:

``` prism-code
cabal haddock
```

If your `cabal` version is greater than `3.8`, you can use `cabal haddock --open` to open the documentation in your browser. Otherwise, you can visit the documentation from `dist-newstyle/build/$ARCH/ghc-$VERSION/opendal-$VERSION/doc/html/opendal/index.html`.

## License and Trademarks<a href="https://opendal.apache.org/bindings/haskell#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/haskell.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/haskell/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/bindings/go/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Go

<a href="https://opendal.apache.org/bindings/java/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Java

- <a href="https://opendal.apache.org/bindings/haskell#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/haskell#build" class="table-of-contents__link toc-highlight">Build</a>
- <a href="https://opendal.apache.org/bindings/haskell#test" class="table-of-contents__link toc-highlight">Test</a>
- <a href="https://opendal.apache.org/bindings/haskell#doc" class="table-of-contents__link toc-highlight">Doc</a>
- <a href="https://opendal.apache.org/bindings/haskell#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
