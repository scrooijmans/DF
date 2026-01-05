# arrow_schema::ffi - Rust

Available on **crate feature `ffi`** only.

Expand description

Contains declarations to bind to the [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html).

```

// Create from data type
let ffi_data_type = FFI_ArrowSchema::try_from(&DataType::LargeUtf8).unwrap();
let back = DataType::try_from(&ffi_data_type).unwrap();
assert_eq!(back, DataType::LargeUtf8);

// Create from schema
let schema = Schema::new(vec![Field::new("foo", DataType::Int64, false)]);
let ffi_schema = FFI_ArrowSchema::try_from(&schema).unwrap();
let back = Schema::try_from(&ffi_schema).unwrap();

assert_eq!(schema, back);
```

[FFI_ArrowSchema](struct.FFI_ArrowSchema.html "struct arrow_schema::ffi::FFI_ArrowSchema")

ABI-compatible struct for `ArrowSchema` from C Data Interface See [https://arrow.apache.org/docs/format/CDataInterface.html#structure-definitions](https://arrow.apache.org/docs/format/CDataInterface.html#structure-definitions)

[Flags](struct.Flags.html "struct arrow_schema::ffi::Flags")

Flags for [`FFI_ArrowSchema`](struct.FFI_ArrowSchema.html "struct arrow_schema::ffi::FFI_ArrowSchema")
