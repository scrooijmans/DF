# Type Alias StringDictionaryBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_bytes_dictionary_builder.rs.html#557" class="src">Source</a>

``` rust
pub type StringDictionaryBuilder<K> = GenericByteDictionaryBuilder<K, GenericStringType<i32>>;
```

Expand description

Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray")

``` rust
// Create a dictionary array indexed by bytes whose values are Strings.
// It can thus hold up to 256 distinct string values.


let mut builder = StringDictionaryBuilder::<Int8Type>::new();

// The builder builds the dictionary value by value
builder.append("abc").unwrap();
builder.append_null();
builder.append_n("def", 2).unwrap();  // appends "def" twice with a single lookup
builder.append("abc").unwrap();
let array = builder.finish();

assert_eq!(
  array.keys(),
  &Int8Array::from(vec![Some(0), None, Some(1), Some(1), Some(0)])
);

// Values are polymorphic and so require a downcast.
let av = array.values();
let ava: &StringArray = av.as_any().downcast_ref::<StringArray>().unwrap();

assert_eq!(ava.value(0), "abc");
assert_eq!(ava.value(1), "def");
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.StringDictionaryBuilder.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StringDictionaryBuilder<K> { /* private fields */ }
```
