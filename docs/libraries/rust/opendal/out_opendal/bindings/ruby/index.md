- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/ruby/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Ruby

On this page

# Ruby

## Apache OpenDALâ„¢ Ruby Binding

[<img src="out_opendal/bindings/ruby/index_media/f596ab3846ae9a4ef3244fecee657b85b8452f25.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Gem Version" />](https://rubygems.org/gems/opendal) [<img src="out_opendal/bindings/ruby/index_media/956e264c34954efcdc3371ddc9688493727e84fe.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Gem Downloads (for latest version)" />](https://rubygems.org/gems/opendal)

OpenDAL's Ruby [gem](https://rubygems.org/gems/opendal).

<img src="out_opendal/bindings/ruby/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

Read OpenDAL's [ruby](https://opendal.apache.org/docs/ruby/) documentation.

You can find Rust documentation [here](https://docs.rs/opendal/latest/opendal/index.html).

## Get started<a href="https://opendal.apache.org/bindings/ruby/#get-started" class="hash-link" aria-label="Direct link to Get started" translate="no" title="Direct link to Get started">â€‹</a>

### Installation<a href="https://opendal.apache.org/bindings/ruby/#installation" class="hash-link" aria-label="Direct link to Installation" translate="no" title="Direct link to Installation">â€‹</a>

Install gem:

``` prism-code
bundle add opendal
```

or add it in Gemfile:

``` prism-code
# Gemfile

source "https://rubygems.org"

gem 'opendal'
```

### Examples<a href="https://opendal.apache.org/bindings/ruby/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

#### File operations with an in-memory storage<a href="https://opendal.apache.org/bindings/ruby/#file-operations-with-an-in-memory-storage" class="hash-link" aria-label="Direct link to File operations with an in-memory storage" translate="no" title="Direct link to File operations with an in-memory storage">â€‹</a>

``` prism-code
require 'opendal'

op = OpenDal::Operator.new("memory", {})
op.write("file", "hello world")
puts op.read("file") # => "hello world"
puts ""

puts "List:", op.list("").map { |e| e.path }
puts ""

puts "Stat"
puts op.stat("file").inspect # => #<OpenDal::Metadata mode: File,         content_type: ,         content_length: 11>
puts ""

puts "Deleting 'file'"
op.delete("/file")
puts ""

puts "Exist?", op.exist?("/file") # => false
puts ""

puts "Info:", op.info.inspect # => #<OpenDal::OperatorInfo scheme: "memory", root: "/">
```

#### A S3 operator<a href="https://opendal.apache.org/bindings/ruby/#a-s3-operator" class="hash-link" aria-label="Direct link to A S3 operator" translate="no" title="Direct link to A S3 operator">â€‹</a>

``` prism-code
require 'opendal'

op = OpenDal::Operator.new("s3", {
  "endpoint" => "http://localhost:9000",
  "access_key_id" => "minioadmin" ,
  "secret_access_key" => "minioadmin",
  "bucket" => "test",
  "region" => "us-east-1",
})
op.write("file", "hello world")
puts op.read("file") # => "hello world"
puts ""

puts "List:", op.list("").map { |e| e.path }
puts ""

puts "Stat"
puts op.stat("file").inspect # => #<OpenDal::Metadata mode: File,         content_type: binary/octet-stream,         content_length: 11>
puts ""

puts "Deleting 'file'"
op.delete("file")
puts ""

puts "Exist?", op.exist?("file") # => false
puts ""

puts "Info:", op.info.inspect # => #<OpenDal::OperatorInfo scheme: "s3", root: "/">
```

#### Use middleware<a href="https://opendal.apache.org/bindings/ruby/#use-middleware" class="hash-link" aria-label="Direct link to Use middleware" translate="no" title="Direct link to Use middleware">â€‹</a>

``` prism-code
require 'opendal'

op = OpenDal::Operator.new("s3", {
  "endpoint" => "http://localhost:9000",
  "access_key_id" => "minioadmin" ,
  "secret_access_key" => "minioadmin",
  "bucket" => "test",
  "region" => "us-east-1",
})

op.middleware(OpenDal::Middleware::ConcurrentLimit.new(5))
op.middleware(OpenDal::Middleware::Retry.new)
op.middleware(OpenDal::Middleware::Timeout.new(1, 2))

op.list("/").map do |e|
  puts e.inspect
end
```

## Documentation<a href="https://opendal.apache.org/bindings/ruby/#documentation" class="hash-link" aria-label="Direct link to Documentation" translate="no" title="Direct link to Documentation">â€‹</a>

More detailed documentation is a work in progress.

- OpenDAL's [ruby](https://opendal.apache.org/docs/ruby/) documentation
- Rust [documentation](https://docs.rs/opendal/latest/opendal/index.html)
- Rust documentation for [services](https://docs.rs/opendal/latest/opendal/services/index.html)
- Rust documentation for [layers](https://docs.rs/opendal/latest/opendal/layers/index.html) (middlewares in Ruby)

## Development<a href="https://opendal.apache.org/bindings/ruby/#development" class="hash-link" aria-label="Direct link to Development" translate="no" title="Direct link to Development">â€‹</a>

Install gem and its dependencies:

``` prism-code
bundle
```

Build bindings:

``` prism-code
bundle exec rake compile
```

Run tests:

``` prism-code
bundle exec rake test
```

Run linters:

``` prism-code
bundle exec rake standard:fix
rustfmt --config-path ../../rustfmt.toml src/*.rs # Run rustfmt for Rust files
cargo clippy --fix --all-targets # Run rust linter clippy
```

## License and Trademarks<a href="https://opendal.apache.org/bindings/ruby/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/ruby.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/ruby/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Oct 1, 2025** by **Erick Guan**

<a href="https://opendal.apache.org/bindings/python/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Python

<a href="https://opendal.apache.org/bindings/swift/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Swift ðŸš§

- <a href="https://opendal.apache.org/bindings/ruby/#get-started" class="table-of-contents__link toc-highlight">Get started</a>
  - <a href="https://opendal.apache.org/bindings/ruby/#installation" class="table-of-contents__link toc-highlight">Installation</a>
  - <a href="https://opendal.apache.org/bindings/ruby/#examples" class="table-of-contents__link toc-highlight">Examples</a>
- <a href="https://opendal.apache.org/bindings/ruby/#documentation" class="table-of-contents__link toc-highlight">Documentation</a>
- <a href="https://opendal.apache.org/bindings/ruby/#development" class="table-of-contents__link toc-highlight">Development</a>
- <a href="https://opendal.apache.org/bindings/ruby/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
