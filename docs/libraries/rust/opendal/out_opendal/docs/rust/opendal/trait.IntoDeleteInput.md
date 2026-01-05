# Trait IntoDeleteInput Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/delete/input.rs.html#32-35" class="src">Source</a>

``` rust
pub trait IntoDeleteInput:
    Send
    + Sync
    + Unpin {
    // Required method
    fn into_delete_input(self) -> DeleteInput;
}
```

Expand description

IntoDeleteInput is a helper trait that makes it easier for users to play with `Deleter`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#tymethod.into_delete_input" class="fn">into_delete_input</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

Convert `self` into a `DeleteInput`.

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#impl-IntoDeleteInput-for-%26str" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Implement `IntoDeleteInput` for `&str` so we can use `&str` as a DeleteInput.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#method.into_delete_input" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#tymethod.into_delete_input" class="fn">into_delete_input</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#impl-IntoDeleteInput-for-(String,+OpDelete)" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>)

Implement `IntoDeleteInput` for `(String, OpDelete)` so we can use `(String, OpDelete)` as a DeleteInput stream.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#method.into_delete_input-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#tymethod.into_delete_input" class="fn">into_delete_input</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#impl-IntoDeleteInput-for-String" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Implement `IntoDeleteInput` for `String` so we can use `Vec<String>` as a DeleteInput stream.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#method.into_delete_input-2" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#tymethod.into_delete_input" class="fn">into_delete_input</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#impl-IntoDeleteInput-for-DeleteInput" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

Implement `IntoDeleteInput` for `DeleteInput` self.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#impl-IntoDeleteInput-for-Entry" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>

Implement `IntoDeleteInput` for `Entry` so we can use `Lister` as a DeleteInput stream.
