- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/ocaml/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- OCaml ðŸš§

On this page

# OCaml ðŸš§

## Apache OpenDALâ„¢ OCaml Binding (WIP)

[<img src="out_opendal/bindings/ocaml/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/ocaml)

## Requirements<a href="https://opendal.apache.org/bindings/ocaml#requirements" class="hash-link" aria-label="Direct link to Requirements" translate="no" title="Direct link to Requirements">â€‹</a>

- OCaml version \> 4.03 and \< 5.0.0

## Contributing<a href="https://opendal.apache.org/bindings/ocaml#contributing" class="hash-link" aria-label="Direct link to Contributing" translate="no" title="Direct link to Contributing">â€‹</a>

### Setup<a href="https://opendal.apache.org/bindings/ocaml#setup" class="hash-link" aria-label="Direct link to Setup" translate="no" title="Direct link to Setup">â€‹</a>

We recommend using `OPAM the OCaml Package Manager` to install and manage the OCaml environment.

#### Install OPAM<a href="https://opendal.apache.org/bindings/ocaml#install-opam" class="hash-link" aria-label="Direct link to Install OPAM" translate="no" title="Direct link to Install OPAM">â€‹</a>

The quickest way to get the latest opam up and working is to run this script:

``` prism-code
bash -c "sh <(curl -fsSL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)"
```

Similarly, you can also use your distribution's package manager to install

##### Arch<a href="https://opendal.apache.org/bindings/ocaml#arch" class="hash-link" aria-label="Direct link to Arch" translate="no" title="Direct link to Arch">â€‹</a>

``` prism-code
pacman -S opam
```

##### Debian \| Ubuntu<a href="https://opendal.apache.org/bindings/ocaml#debian--ubuntu" class="hash-link" aria-label="Direct link to Debian | Ubuntu" translate="no" title="Direct link to Debian | Ubuntu">â€‹</a>

``` prism-code
apt-get install opam
```

#### macOS<a href="https://opendal.apache.org/bindings/ocaml#macos" class="hash-link" aria-label="Direct link to macOS" translate="no" title="Direct link to macOS">â€‹</a>

``` prism-code
brew install opam
```

#### Init OPAM<a href="https://opendal.apache.org/bindings/ocaml#init-opam" class="hash-link" aria-label="Direct link to Init OPAM" translate="no" title="Direct link to Init OPAM">â€‹</a>

*Do not put sudo in front of any opam commands. That would break your OCaml installation.*

After Installing OPAM, we need to initialize it

For the general case, we can execute

``` prism-code
opam init --bare -a -y
```

If you are using WSL1 on windows, run:

``` prism-code
opam init --bare -a -y --disable-sandboxing
```

#### Create OPAM Switch<a href="https://opendal.apache.org/bindings/ocaml#create-opam-switch" class="hash-link" aria-label="Direct link to Create OPAM Switch" translate="no" title="Direct link to Create OPAM Switch">â€‹</a>

Using opam, we can have multiple versions of ocaml at the same time; this is called switch.

Due to the upstream `ocaml-rs`, we currently do not support OCaml5, and recommend using the latest version of OCaml4 We can create use this command:

``` prism-code
opam switch create opendal-ocaml4.14 ocaml-base-compiler.4.14.0

eval $(opam env)
```

#### Install OPAM Package<a href="https://opendal.apache.org/bindings/ocaml#install-opam-package" class="hash-link" aria-label="Direct link to Install OPAM Package" translate="no" title="Direct link to Install OPAM Package">â€‹</a>

OpenDAL does not depend on opam package except `ounit2` for testing. However, to facilitate development in an IDE such as vscode, it is usually necessary to install the following content

``` prism-code
opam install -y utop odoc ounit2 ocaml-lsp-server ocamlformat ocamlformat-rpc
```

### Build<a href="https://opendal.apache.org/bindings/ocaml#build" class="hash-link" aria-label="Direct link to Build" translate="no" title="Direct link to Build">â€‹</a>

``` prism-code
cd bindings/ocaml
dune build
```

### Test<a href="https://opendal.apache.org/bindings/ocaml#test" class="hash-link" aria-label="Direct link to Test" translate="no" title="Direct link to Test">â€‹</a>

To execute unit tests, we can simply use the following command:

``` prism-code
cd bindings/ocaml
dune test
```

## License and Trademarks<a href="https://opendal.apache.org/bindings/ocaml#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/ocaml.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/ocaml/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **May 21, 2025** by **yihong**

<a href="https://opendal.apache.org/bindings/nodejs/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Node.js

<a href="https://opendal.apache.org/bindings/php/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

PHP ðŸš§

- <a href="https://opendal.apache.org/bindings/ocaml#requirements" class="table-of-contents__link toc-highlight">Requirements</a>
- <a href="https://opendal.apache.org/bindings/ocaml#contributing" class="table-of-contents__link toc-highlight">Contributing</a>
  - <a href="https://opendal.apache.org/bindings/ocaml#setup" class="table-of-contents__link toc-highlight">Setup</a>
  - <a href="https://opendal.apache.org/bindings/ocaml#build" class="table-of-contents__link toc-highlight">Build</a>
  - <a href="https://opendal.apache.org/bindings/ocaml#test" class="table-of-contents__link toc-highlight">Test</a>
- <a href="https://opendal.apache.org/bindings/ocaml#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
