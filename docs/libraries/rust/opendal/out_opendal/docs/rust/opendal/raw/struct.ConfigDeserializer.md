# Struct ConfigDeserializer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/serde_util.rs.html#54" class="src">Source</a>

``` rust
pub struct ConfigDeserializer(/* private fields */);
```

Expand description

ConfigDeserializer is used to deserialize given configs from `HashMap<String, String>`.

This is only used by our servicesâ€™ config.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#impl-ConfigDeserializer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html" class="struct" title="struct opendal::raw::ConfigDeserializer">ConfigDeserializer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.new" class="fn">new</a>(map: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Create a new config deserializer.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#impl-Deserializer%3C&#39;de%3E-for-ConfigDeserializer" class="anchor">Â§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html" class="struct" title="struct opendal::raw::ConfigDeserializer">ConfigDeserializer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#associatedtype.Error" class="anchor">Â§</a>

#### type <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/value/struct.Error.html" class="struct" title="struct serde_core::de::value::Error">Error</a>

The error type that can be returned if some error occurs during deserialization.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_any" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any" class="fn">deserialize_any</a>\<V\>(self, visitor: V) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, Self::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_map" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map" class="fn">deserialize_map</a>\<V\>(self, visitor: V) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, Self::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_bool" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool" class="fn">deserialize_bool</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `bool` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_u8" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8" class="fn">deserialize_u8</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `u8` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_u16" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16" class="fn">deserialize_u16</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `u16` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_u32" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32" class="fn">deserialize_u32</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `u32` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_u64" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64" class="fn">deserialize_u64</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `u64` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_i8" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8" class="fn">deserialize_i8</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an `i8` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_i16" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16" class="fn">deserialize_i16</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an `i16` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_i32" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32" class="fn">deserialize_i32</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an `i32` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_i64" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64" class="fn">deserialize_i64</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an `i64` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_f32" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32" class="fn">deserialize_f32</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `f32` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_f64" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64" class="fn">deserialize_f64</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `f64` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_char" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char" class="fn">deserialize_char</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a `char` value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_str" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str" class="fn">deserialize_str</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_string" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string" class="fn">deserialize_string</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_unit" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit" class="fn">deserialize_unit</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a unit value.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_seq" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq" class="fn">deserialize_seq</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a sequence of values.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_bytes" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes" class="fn">deserialize_bytes</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_byte_buf" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf" class="fn">deserialize_byte_buf</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_unit_struct" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct" class="fn">deserialize_unit_struct</a>\<V\>( self, name: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_tuple_struct" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct" class="fn">deserialize_tuple_struct</a>\<V\>( self, name: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_identifier" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier" class="fn">deserialize_identifier</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_tuple" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple" class="fn">deserialize_tuple</a>\<V\>( self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_ignored_any" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any" class="fn">deserialize_ignored_any</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesnâ€™t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_option" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option" class="fn">deserialize_option</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_newtype_struct" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct" class="fn">deserialize_newtype_struct</a>\<V\>( self, name: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_enum" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum" class="fn">deserialize_enum</a>\<V\>( self, name: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, variants: &'static \[&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_struct" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct" class="fn">deserialize_struct</a>\<V\>( self, name: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, fields: &'static \[&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<V::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, \<Self as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_i128" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#method.deserialize_i128" class="fn">deserialize_i128</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<V as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, Self::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.deserialize_u128" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#method.deserialize_u128" class="fn">deserialize_u128</a>\<V\>( self, visitor: V, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<V as <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>\>::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html#associatedtype.Value" class="associatedtype" title="type serde_core::de::Visitor::Value">Value</a>, Self::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where V: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Visitor.html" class="trait" title="trait serde_core::de::Visitor">Visitor</a>\<'de\>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#method.is_human_readable" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#method.is_human_readable" class="fn">is_human_readable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#method.is_human_readable)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html#blanket-implementations" class="anchor">Â§</a>
