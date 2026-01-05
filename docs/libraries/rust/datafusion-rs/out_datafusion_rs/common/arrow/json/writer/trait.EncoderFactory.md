# Trait EncoderFactory Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/encoder.rs.html#173" class="src">Source</a>

``` rust
pub trait EncoderFactory:
    Debug
    + Send
    + Sync {
    // Provided method
    fn make_default_encoder<'a>(
        &self,
        _field: &'a Arc<Field>,
        _array: &'a dyn Array,
        _options: &'a EncoderOptions,
    ) -> Result<Option<NullableEncoder<'a>>, ArrowError> { ... }
}
```

Expand description

A trait to create custom encoders for specific data types.

This allows overriding the default encoders for specific data types, or adding new encoders for custom data types.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.EncoderFactory.html#examples" class="doc-anchor">§</a>Examples

``` rust
use std::io::Write;
use arrow_array::{ArrayAccessor, Array, BinaryArray, Float64Array, RecordBatch};
use arrow_array::cast::AsArray;
use arrow_schema::{DataType, Field, Schema, FieldRef};
use arrow_json::{writer::{WriterBuilder, JsonArray, NullableEncoder}, StructMode};
use arrow_json::{Encoder, EncoderFactory, EncoderOptions};
use arrow_schema::ArrowError;
use std::sync::Arc;
use serde_json::json;
use serde_json::Value;

struct IntArrayBinaryEncoder<B> {
    array: B,
}

impl<'a, B> Encoder for IntArrayBinaryEncoder<B>
where
    B: ArrayAccessor<Item = &'a [u8]>,
{
    fn encode(&mut self, idx: usize, out: &mut Vec<u8>) {
        out.push(b'[');
        let child = self.array.value(idx);
        for (idx, byte) in child.iter().enumerate() {
            write!(out, "{byte}").unwrap();
            if idx < child.len() - 1 {
                out.push(b',');
            }
        }
        out.push(b']');
    }
}

#[derive(Debug)]
struct IntArayBinaryEncoderFactory;

impl EncoderFactory for IntArayBinaryEncoderFactory {
    fn make_default_encoder<'a>(
        &self,
        _field: &'a FieldRef,
        array: &'a dyn Array,
        _options: &'a EncoderOptions,
    ) -> Result<Option<NullableEncoder<'a>>, ArrowError> {
        match array.data_type() {
            DataType::Binary => {
                let array = array.as_binary::<i32>();
                let encoder = IntArrayBinaryEncoder { array };
                let array_encoder = Box::new(encoder) as Box<dyn Encoder + 'a>;
                let nulls = array.nulls().cloned();
                Ok(Some(NullableEncoder::new(array_encoder, nulls)))
            }
            _ => Ok(None),
        }
    }
}

let binary_array = BinaryArray::from_iter([Some(b"a".as_slice()), None, Some(b"b".as_slice())]);
let float_array = Float64Array::from(vec![Some(1.0), Some(2.3), None]);
let fields = vec![
    Field::new("bytes", DataType::Binary, true),
    Field::new("float", DataType::Float64, true),
];
let batch = RecordBatch::try_new(
    Arc::new(Schema::new(fields)),
    vec![
        Arc::new(binary_array) as Arc<dyn Array>,
        Arc::new(float_array) as Arc<dyn Array>,
    ],
)
.unwrap();

let json_value: Value = {
    let mut buf = Vec::new();
    let mut writer = WriterBuilder::new()
        .with_encoder_factory(Arc::new(IntArayBinaryEncoderFactory))
        .build::<_, JsonArray>(&mut buf);
    writer.write_batches(&[&batch]).unwrap();
    writer.finish().unwrap();
    serde_json::from_slice(&buf).unwrap()
};

let expected = json!([
    {"bytes": [97], "float": 1.0},
    {"float": 2.3},
    {"bytes": [98]},
]);

assert_eq!(json_value, expected);
```

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.EncoderFactory.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.EncoderFactory.html#method.make_default_encoder" class="fn">make_default_encoder</a>\<'a\>( &self, \_field: &'a <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, \_array: &'a dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>, \_options: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.NullableEncoder.html" class="struct" title="struct datafusion::common::arrow::json::writer::NullableEncoder">NullableEncoder</a>\<'a\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Make an encoder that overrides the default encoder for a specific field and array or provides an encoder for a custom data type. This can be used to override how e.g. binary data is encoded so that it is an encoded string or an array of integers.

Note that the type of the field may not match the type of the array: for dictionary arrays unless the top-level dictionary is handled this will be called again for the keys and values of the dictionary, at which point the field type will still be the outer dictionary type but the array will have a different type. For example, ``` field`` might have the type  ```Dictionary(i32, Utf8)`but`array`will be`Utf8\`.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.EncoderFactory.html#implementors" class="anchor">§</a>
