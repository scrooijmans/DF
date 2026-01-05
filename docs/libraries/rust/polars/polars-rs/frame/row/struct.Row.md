# Struct Row Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/mod.rs.html#91" class="src">Source</a>

``` rust
pub struct Row<'a>(pub Vec<AnyValue<'a>>);
```

## Tuple Fields<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue"><code>AnyValue</code></a>`<'a>>`

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.new" class="fn">new</a>(values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\>) -\> <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-Clone-for-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-Debug-for-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-Default-for-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-From%3C%26Row%3C&#39;_%3E%3E-for-Schema%3CDataType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'\_\>\> for <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(row: &<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'\_\>) -\> <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-PartialEq-for-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-Eq-for-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#impl-StructuralPartialEq-for-Row%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html#blanket-implementations" class="anchor">§</a>
