# Enum SchemaError Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#165" class="src">Source</a>

``` rust
pub enum SchemaError {
    AmbiguousReference {
        field: Box<Column>,
    },
    DuplicateQualifiedField {
        qualifier: Box<TableReference>,
        name: String,
    },
    DuplicateUnqualifiedField {
        name: String,
    },
    FieldNotFound {
        field: Box<Column>,
        valid_fields: Vec<Column>,
    },
}
```

Expand description

Schema-related errors

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.AmbiguousReference" class="anchor">§</a>

### AmbiguousReference

Schema contains a (possibly) qualified and unqualified field with same unqualified name

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.AmbiguousReference.field.field" class="anchor field">§</a>`field: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column"><code>Column</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.DuplicateQualifiedField" class="anchor">§</a>

### DuplicateQualifiedField

Schema contains duplicate qualified field name

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.DuplicateQualifiedField.field.qualifier" class="anchor field">§</a>`qualifier: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference"><code>TableReference</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.DuplicateQualifiedField.field.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.DuplicateUnqualifiedField" class="anchor">§</a>

### DuplicateUnqualifiedField

Schema contains duplicate unqualified field name

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.DuplicateUnqualifiedField.field.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.FieldNotFound" class="anchor">§</a>

### FieldNotFound

No field with this name

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.FieldNotFound.field.field" class="anchor field">§</a>`field: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column"><code>Column</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#variant.FieldNotFound.field.valid_fields" class="anchor field">§</a>`valid_fields: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column"><code>Column</code></a>`>`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#impl-Debug-for-SchemaError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html" class="enum" title="enum datafusion::common::SchemaError">SchemaError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#impl-Display-for-SchemaError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html" class="enum" title="enum datafusion::common::SchemaError">SchemaError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#impl-Error-for-SchemaError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html" class="enum" title="enum datafusion::common::SchemaError">SchemaError</a>

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#111" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html#blanket-implementations" class="anchor">§</a>
