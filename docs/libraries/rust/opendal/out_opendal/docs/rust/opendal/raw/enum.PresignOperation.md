# Enum PresignOperation Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#270-279" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum PresignOperation {
    Stat(OpStat),
    Read(OpRead),
    Write(OpWrite),
    Delete(OpDelete),
}
```

Expand description

Presign operation used for presign.

## Variants (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#variants" class="anchor">Â§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#variant.Stat" class="anchor">Â§</a>

### Stat(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>)

Presign a stat(head) operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#variant.Read" class="anchor">Â§</a>

### Read(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>)

Presign a read operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#variant.Write" class="anchor">Â§</a>

### Write(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>)

Presign a write operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#variant.Delete" class="anchor">Â§</a>

### Delete(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>)

Presign a delete operation.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#impl-Clone-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#impl-Debug-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#impl-From%3COpDelete%3E-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.from-3" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#impl-From%3COpRead%3E-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.from-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#impl-From%3COpStat%3E-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(op: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#impl-From%3COpWrite%3E-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#method.from-2" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html#blanket-implementations" class="anchor">Â§</a>
