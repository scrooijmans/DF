# Struct Field Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/field.rs.html#17" class="src">Source</a>

``` rust
pub struct Field {
    pub name: PlSmallStr,
    pub dtype: DataType,
}
```

Expand description

Characterizes the name and the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") of a column.

## Fields<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Field" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

Creates a new `Field`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#example" class="doc-anchor">§</a>Example

``` rust
let f1 = Field::new("Fruit name".into(), DataType::String);
let f2 = Field::new("Lawful".into(), DataType::Boolean);
let f2 = Field::new("Departure".into(), DataType::Time);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

Returns a reference to the `Field` name.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#example-1" class="doc-anchor">§</a>Example

``` rust
let f = Field::new("Year".into(), DataType::Int32);

assert_eq!(f.name(), "Year");
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Returns a reference to the `Field` datatype.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#example-2" class="doc-anchor">§</a>Example

``` rust
let f = Field::new("Birthday".into(), DataType::Date);

assert_eq!(f.dtype(), &DataType::Date);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.coerce" class="fn">coerce</a>(&mut self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>)

Sets the `Field` datatype.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#example-3" class="doc-anchor">§</a>Example

``` rust
let mut f = Field::new("Temperature".into(), DataType::Int32);
f.coerce(DataType::Float32);

assert_eq!(f, Field::new("Temperature".into(), DataType::Float32));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.set_name" class="fn">set_name</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

Sets the `Field` name.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#example-4" class="doc-anchor">§</a>Example

``` rust
let mut f = Field::new("Atomic number".into(), DataType::UInt32);
f.set_name("Proton".into());

assert_eq!(f, Field::new("Proton".into(), DataType::UInt32));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.with_name" class="fn">with_name</a>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

Returns this `Field`, renamed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.to_arrow" class="fn">to_arrow</a>(&self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>

Converts the `Field` to an `arrow::datatypes::Field`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#example-5" class="doc-anchor">§</a>Example

``` rust
let f = Field::new("Value".into(), DataType::Int64);
let af = arrow::datatypes::Field::new("Value".into(), arrow::datatypes::ArrowDataType::Int64, true);

assert_eq!(f.to_arrow(CompatLevel::newest()), af);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.to_physical" class="fn">to_physical</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-AsRef%3CDataType%3E-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Clone-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Debug-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Deserialize%3C&#39;de%3E-for-Field" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-From%3C%26AnyValue%3C&#39;a%3E%3E-for-Field" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-From%3C%26ArrowField%3E-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(f: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Hash-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-PartialEq-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Serialize-for-Field" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-Eq-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#impl-StructuralPartialEq-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.Field.html#blanket-implementations" class="anchor">§</a>
