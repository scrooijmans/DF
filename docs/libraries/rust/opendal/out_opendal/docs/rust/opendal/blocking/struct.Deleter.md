# Struct Deleter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/delete.rs.html#24-27" class="src">Source</a>

``` rust
pub struct Deleter { /* private fields */ }
```

Available on **crate feature `blocking`** only.

Expand description

BlockingDeleter is designed to continuously remove content from storage.

It leverages batch deletion capabilities provided by storage services for efficient removal.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#impl-Deleter" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html" class="struct" title="struct opendal::blocking::Deleter">Deleter</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#method.delete" class="fn">delete</a>(&mut self, input: impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Delete a path.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#method.delete_iter" class="fn">delete_iter</a>\<I, D\>(&mut self, iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = D\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an infallible iterator of paths.

Also see:

- \[`BlockingDeleter::delete_try_iter`\]: delete an fallible iterator of paths.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#method.delete_try_iter" class="fn">delete_try_iter</a>\<I, D\>(&mut self, try_iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<D\>\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an fallible iterator of paths.

Also see:

- \[`BlockingDeleter::delete_iter`\]: delete an infallible iterator of paths.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Flush the deleter, returns the number of deleted paths.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#method.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Close the deleter, this will flush the deleter and wait until all paths are deleted.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html#blanket-implementations" class="anchor">Â§</a>
