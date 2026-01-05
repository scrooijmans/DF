# Enum AnyValue Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/any_value.rs.html#26" class="src">Source</a>

``` rust
pub enum AnyValue<'a> {
Show 33 variants    Null,
    Boolean(bool),
    String(&'a str),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    Date(i32),
    Datetime(i64, TimeUnit, Option<&'a TimeZone>),
    DatetimeOwned(i64, TimeUnit, Option<Arc<TimeZone>>),
    Duration(i64, TimeUnit),
    Time(i64),
    Categorical(u32, &'a Arc<CategoricalMapping>),
    CategoricalOwned(u32, Arc<CategoricalMapping>),
    Enum(u32, &'a Arc<CategoricalMapping>),
    EnumOwned(u32, Arc<CategoricalMapping>),
    List(Series),
    Array(Series, usize),
    Object(&'a (dyn PolarsObjectSafe + 'static)),
    ObjectOwned(OwnedObject),
    Struct(usize, &'a StructArray, &'a [Field]),
    StructOwned(Box<(Vec<AnyValue<'a>>, Vec<Field>)>),
    StringOwned(PlSmallStr),
    Binary(&'a [u8]),
    BinaryOwned(Vec<u8>),
    Decimal(i128, usize),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Null" class="anchor">§</a>

### Null

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Boolean" class="anchor">§</a>

### Boolean(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

A binary true or false.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.String" class="anchor">§</a>

### String(&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

A UTF8 encoded string type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.UInt8" class="anchor">§</a>

### UInt8(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>)

An unsigned 8-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.UInt16" class="anchor">§</a>

### UInt16(<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>)

An unsigned 16-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.UInt32" class="anchor">§</a>

### UInt32(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>)

An unsigned 32-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.UInt64" class="anchor">§</a>

### UInt64(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

An unsigned 64-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Int8" class="anchor">§</a>

### Int8(<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

An 8-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Int16" class="anchor">§</a>

### Int16(<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>)

A 16-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Int32" class="anchor">§</a>

### Int32(<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>)

A 32-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Int64" class="anchor">§</a>

### Int64(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

A 64-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Int128" class="anchor">§</a>

### Int128(<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>)

A 128-bit integer number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Float32" class="anchor">§</a>

### Float32(<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>)

A 32-bit floating point number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Float64" class="anchor">§</a>

### Float64(<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>)

A 64-bit floating point number.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Date" class="anchor">§</a>

### Date(<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>)

A 32-bit date representing the elapsed time since UNIX epoch (1970-01-01) in days (32 bits).

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Datetime" class="anchor">§</a>

### Datetime(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>)

A 64-bit date representing the elapsed time since UNIX epoch (1970-01-01) in nanoseconds (64 bits).

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.DatetimeOwned" class="anchor">§</a>

### DatetimeOwned(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>\>)

A 64-bit date representing the elapsed time since UNIX epoch (1970-01-01) in nanoseconds (64 bits).

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Duration" class="anchor">§</a>

### Duration(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

A 64-bit integer representing difference between date-times in [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit")

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Time" class="anchor">§</a>

### Time(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

A 64-bit time representing the elapsed time since midnight in nanoseconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Categorical" class="anchor">§</a>

### Categorical(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.CategoricalOwned" class="anchor">§</a>

### CategoricalOwned(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Enum" class="anchor">§</a>

### Enum(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.EnumOwned" class="anchor">§</a>

### EnumOwned(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.List" class="anchor">§</a>

### List(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>)

Nested type, contains arrays that are filled with one of the datatypes.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Array" class="anchor">§</a>

### Array(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Object" class="anchor">§</a>

### Object(&'a (dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static))

Can be used to fmt and implements Any, so can be downcasted to the proper value type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.ObjectOwned" class="anchor">§</a>

### ObjectOwned(<a href="https://docs.rs/polars/latest/polars/prelude/struct.OwnedObject.html" class="struct" title="struct polars::prelude::OwnedObject">OwnedObject</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>, &'a \[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\])

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.StructOwned" class="anchor">§</a>

### StructOwned(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>)\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.StringOwned" class="anchor">§</a>

### StringOwned(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

An UTF8 encoded string type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Binary" class="anchor">§</a>

### Binary(&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\])

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.BinaryOwned" class="anchor">§</a>

### BinaryOwned(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Decimal" class="anchor">§</a>

### Decimal(<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

A 128-bit fixed point decimal number with a scale.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method._iter_struct_av" class="fn">_iter_struct_av</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method._materialize_struct_av" class="fn">_materialize_struct_av</a>(&'a self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-AnyValue%3C&#39;static%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.zero_sum" class="fn">zero_sum</a>(dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.can_have_dtype" class="fn">can_have_dtype</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Can the [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") exist as having `dtype` as its `DataType`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.default_value" class="fn">default_value</a>( dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, numeric_to_one: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, num_list_values: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

Generate a default dummy value for a given datatype.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-AnyValue%3C&#39;a%3E-1" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.dtype" class="fn">dtype</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get the matching [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") for this [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue")\`.

Note: For `Categorical` and `Enum` values, the exact mapping information is not preserved in the result for performance reasons.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.try_extract" class="fn">try_extract</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_boolean" class="fn">is_boolean</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_primitive_numeric" class="fn">is_primitive_numeric</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_float" class="fn">is_float</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_integer" class="fn">is_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_signed_integer" class="fn">is_signed_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_unsigned_integer" class="fn">is_unsigned_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_nan" class="fn">is_nan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.is_nested_null" class="fn">is_nested_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.strict_cast" class="fn">strict_cast</a>(&self, dtype: &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\>

Cast `AnyValue` to the provided data type and return a new `AnyValue` with type `dtype`, if possible.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.try_strict_cast" class="fn">try_strict_cast</a>( &self, dtype: &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast `AnyValue` to the provided data type and return a new `AnyValue` with type `dtype`, if possible.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.cast" class="fn">cast</a>(&self, dtype: &'a <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.idx" class="fn">idx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.str_value" class="fn">str_value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.to_physical" class="fn">to_physical</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.extract_bool" class="fn">extract_bool</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.extract_str" class="fn">extract_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.extract_bytes" class="fn">extract_bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.hash_impl" class="fn">hash_impl</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>, cheap: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-AnyValue%3C&#39;a%3E-2" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.add" class="fn">add</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.as_borrowed" class="fn">as_borrowed</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.into_static" class="fn">into_static</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

Try to coerce to an AnyValue with static lifetime. This can be done if it does not borrow any values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.get_str" class="fn">get_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get a reference to the `&str` contained within [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue").

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-AnyValue%3C&#39;_%3E-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.eq_missing" class="fn">eq_missing</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, null_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-Clone-for-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-Debug-for-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-Default-for-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-Display-for-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3C%26%5Bu8%5D%3E-for-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3C%26AnyValue%3C&#39;a%3E%3E-for-DataType" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3C%26AnyValue%3C&#39;a%3E%3E-for-Field" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3C%26str%3E-for-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3CAnyValue%3C&#39;_%3E%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3CAnyValue%3C&#39;_%3E%3E-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3CAnyValue%3C&#39;a%3E%3E-for-Option%3Ci64%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\> for <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3CK%3E-for-AnyValue%3C&#39;static%3E" class="anchor">§</a>

### impl\<K\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<K\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

where K: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: K) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3COption%3CT%3E%3E-for-AnyValue%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-From%3Cbool%3E-for-AnyValue%3C&#39;static%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-Hash-for-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-PartialEq-for-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-PartialOrd-for-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

Only implemented for the same types and physical types!

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1399" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1417" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1435" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1453" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-TotalEq-for-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalEq.html" class="trait" title="trait polars_utils::total_ord::TotalEq">TotalEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.tot_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalEq.html#tymethod.tot_eq" class="fn">tot_eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.tot_ne" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalEq.html#method.tot_ne" class="fn">tot_ne</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#impl-Eq-for-AnyValue%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#blanket-implementations" class="anchor">§</a>
