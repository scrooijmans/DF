# Enum IcebergColumnType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/schema/iceberg.rs.html#70" class="src">Source</a>

``` rust
pub enum IcebergColumnType {
    Primitive {
        dtype: DataType,
    },
    List(Box<IcebergColumn>),
    FixedSizeList(Box<IcebergColumn>, usize),
    Struct(IcebergSchema),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#variant.Primitive" class="anchor">§</a>

### Primitive

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#variant.Primitive.field.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>

This must not be a nested data type.

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergColumn.html" class="struct" title="struct polars::prelude::iceberg::IcebergColumn">IcebergColumn</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#variant.FixedSizeList" class="anchor">§</a>

### FixedSizeList(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergColumn.html" class="struct" title="struct polars::prelude::iceberg::IcebergColumn">IcebergColumn</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

(values, width)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.to_polars_dtype" class="fn">to_polars_dtype</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.is_nested" class="fn">is_nested</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-Clone-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-Debug-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-Deserialize%3C&#39;de%3E-for-IcebergColumnType" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-Hash-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-PartialEq-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-Serialize-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-Eq-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#impl-StructuralPartialEq-for-IcebergColumnType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html" class="enum" title="enum polars::prelude::iceberg::IcebergColumnType">IcebergColumnType</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/enum.IcebergColumnType.html#blanket-implementations" class="anchor">§</a>
