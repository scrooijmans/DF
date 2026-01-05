# Struct QueryPairsWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/uri.rs.html#69-72" class="src">Source</a>

``` rust
pub struct QueryPairsWriter { /* private fields */ }
```

Expand description

QueryPairsWriter is used to write query pairs to a url.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#impl-QueryPairsWriter" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html" class="struct" title="struct opendal::raw::QueryPairsWriter">QueryPairsWriter</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#method.new" class="fn">new</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Create a new QueryPairsWriter with the given base.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#method.push" class="fn">push</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Push a new pair of key and value to the url.

The input key and value must already been percent encoded correctly.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Finish the url and return it.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html#blanket-implementations" class="anchor">Â§</a>
