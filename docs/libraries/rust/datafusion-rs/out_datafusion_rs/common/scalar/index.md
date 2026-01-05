# Module scalar Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#55" class="src">Source</a>

Expand description

[`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue"): stores single values

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>  
Builder for [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct").

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::common::scalar::ScalarValue">ScalarValue</a>  
A dynamically typed, nullable single value.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::common::scalar::ScalarType">ScalarType</a>  
Trait used to map a NativeType to a ScalarValue

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/fn.copy_array_data.html" class="fn" title="fn datafusion::common::scalar::copy_array_data">copy_array_data</a>

Compacts the data of an `ArrayData` into a new `ArrayData`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/fn.dict_from_values.html" class="fn" title="fn datafusion::common::scalar::dict_from_values">dict_from_values</a>

Create a `DictionaryArray` from the provided values array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/fn.get_dict_value.html" class="fn" title="fn datafusion::common::scalar::get_dict_value">get_dict_value</a>

Return a reference to the values array and the index into it for a dictionary array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/fn.partial_cmp_struct.html" class="fn" title="fn datafusion::common::scalar::partial_cmp_struct">partial_cmp_struct</a>
