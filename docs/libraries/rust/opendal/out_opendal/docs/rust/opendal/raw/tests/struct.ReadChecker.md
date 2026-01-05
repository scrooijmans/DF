# Struct ReadChecker Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/tests/read.rs.html#38-41" class="src">Source</a>

``` rust
pub struct ReadChecker { /* private fields */ }
```

Available on **crate feature `tests`** only.

Expand description

ReadChecker is used to check the correctness of the read process.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#impl-ReadChecker" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html" class="struct" title="struct opendal::raw::tests::ReadChecker">ReadChecker</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#method.new" class="fn">new</a>(size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new read checker by given size and range.

Itâ€™s by design that we use a random generator to generate the raw data. The content of data is not important, we only care about the correctness of the read process.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#method.data" class="fn">data</a>(&self) -\> Bytes

Return the raw data of this read checker.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#method.check" class="fn">check</a>(&mut self, r: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Reader.html" class="struct" title="struct opendal::Reader">Reader</a>, actions: &\[<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/enum.ReadAction.html" class="enum" title="enum opendal::raw::tests::ReadAction">ReadAction</a>\])

Check will check the correctness of the read process via given actions.

Check will panic if any check failed.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.ReadChecker.html#blanket-implementations" class="anchor">Â§</a>
