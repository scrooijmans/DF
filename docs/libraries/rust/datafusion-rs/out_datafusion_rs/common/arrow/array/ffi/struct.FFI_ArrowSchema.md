# Struct FFI_ArrowSchema Copy item path

<a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/src/arrow_schema/ffi.rs.html#77" class="src">Source</a>

``` rust
pub struct FFI_ArrowSchema { /* private fields */ }
```

Expand description

ABI-compatible struct for `ArrowSchema` from C Data Interface See <https://arrow.apache.org/docs/format/CDataInterface.html#structure-definitions>

``` rust
fn array_schema(data_type: &DataType) -> FFI_ArrowSchema {
    FFI_ArrowSchema::try_from(data_type).unwrap()
}
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_new" class="fn">try_new</a>( format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\>, dictionary: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

create a new [`FFI_ArrowSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html "struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema"). This fails if the fields’ [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") is not supported.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.with_name" class="fn">with_name</a>(self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Set the name of the schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.with_flags" class="fn">with_flags</a>(self, flags: <a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/ffi/struct.Flags.html" class="struct" title="struct arrow_schema::ffi::Flags">Flags</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Set the flags of the schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.with_metadata" class="fn">with_metadata</a>\<I, S\>( self, metadata: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(S, S)</a>\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Add metadata to the schema

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.from_raw" class="fn">from_raw</a>(schema: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut</a> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

Takes ownership of the pointed to [`FFI_ArrowSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html "struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema")

This acts to [move](https://arrow.apache.org/docs/format/CDataInterface.html#moving-an-array) the data out of `schema`, setting the release callback to NULL

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#safety" class="doc-anchor">§</a>Safety

- `schema` must be [valid](https://doc.rust-lang.org/std/ptr/index.html#safety) for reads and writes
- `schema` must be properly aligned
- `schema` must point to a properly initialized value of [`FFI_ArrowSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html "struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

Create an empty [`FFI_ArrowSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html "struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.format" class="fn">format</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the format of this schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.name" class="fn">name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the name of this schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.flags" class="fn">flags</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/ffi/struct.Flags.html" class="struct" title="struct arrow_schema::ffi::Flags">Flags</a>\>

Returns the flags of this schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.child" class="fn">child</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

Returns the child of this schema at `index`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#panics" class="doc-anchor">§</a>Panics

Panics if `index` is greater than or equal to the number of children.

This is to make sure that the unsafe acces to raw pointer is sound.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.children" class="fn">children</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\>

Returns an iterator to the schema’s children.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.nullable" class="fn">nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns if the field is semantically nullable, regardless of whether it actually has null values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.dictionary" class="fn">dictionary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\>

Returns the reference to the underlying dictionary of the schema. Check [ArrowSchema.dictionary](https://arrow.apache.org/docs/format/CDataInterface.html#c.ArrowSchema.dictionary).

This must be `Some` if the schema represents a dictionary-encoded type, `None` otherwise.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.map_keys_sorted" class="fn">map_keys_sorted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

For map types, returns whether the keys within each map value are sorted.

Refer to [Arrow Flags](https://arrow.apache.org/docs/format/CDataInterface.html#c.ArrowSchema.flags)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.dictionary_ordered" class="fn">dictionary_ordered</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

For dictionary-encoded types, returns whether the ordering of dictionary indices is semantically meaningful.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.metadata" class="fn">metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Returns the metadata in the schema as `Key-Value` pairs

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-Debug-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-Drop-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26Arc%3CField%3E%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( value: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26DataType%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(dtype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

See [CDataInterface docs](https://arrow.apache.org/docs/format/CDataInterface.html#data-type-description-format-strings)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(c_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

See [CDataInterface docs](https://arrow.apache.org/docs/format/CDataInterface.html#data-type-description-format-strings)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(c_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(c_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26Field%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-5" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(field: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3C%26Schema%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-6" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3CDataType%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-7" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(dtype: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3CField%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-8" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(field: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-TryFrom%3CSchema%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#associatedtype.Error-9" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#method.try_from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(schema: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#impl-Send-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html#blanket-implementations" class="anchor">§</a>
