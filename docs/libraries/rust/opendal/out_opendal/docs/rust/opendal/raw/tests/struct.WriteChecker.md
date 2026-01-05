# Struct WriteChecker Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/tests/write.rs.html#37-40" class="src">Source</a>

``` rust
pub struct WriteChecker { /* private fields */ }
```

Available on **crate feature `tests`** only.

Expand description

WriteAction is used to check the correctness of the write process.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#impl-WriteChecker" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html" class="struct" title="struct opendal::raw::tests::WriteChecker">WriteChecker</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#method.new" class="fn">new</a>(size: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> Self

Create a new WriteChecker with given size.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#method.chunks" class="fn">chunks</a>(&self) -\> &\[Bytes\]

Get the checkâ€™s chunks.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#method.check" class="fn">check</a>(&self, actual: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\])

Check the correctness of the write process.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/struct.WriteChecker.html#blanket-implementations" class="anchor">Â§</a>
