# UnionFields in arrow_schema - Rust

## Struct UnionFields 

[Source](about:blank/src/arrow_schema/fields.rs.html#324)

```
pub struct UnionFields(/* private fields */);
```

Expand description

A cheaply cloneable, owned collection of [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef") and their corresponding type ids

[Source](about:blank/src/arrow_schema/fields.rs.html#332-430)
[§](#impl-UnionFields)

[Source](about:blank/src/arrow_schema/fields.rs.html#334-336)

[Source](about:blank/src/arrow_schema/fields.rs.html#355-375)

Create a new [`UnionFields`](struct.UnionFields.html "struct arrow_schema::UnionFields") from a [`Fields`](struct.Fields.html "struct arrow_schema::Fields") and array of type_ids

See [https://arrow.apache.org/docs/format/Columnar.html#union-layout](https://arrow.apache.org/docs/format/Columnar.html#union-layout)

```
use arrow_schema::{DataType, Field, UnionFields};
// Create a new UnionFields with type id mapping
// 1 -> DataType::UInt8
// 3 -> DataType::Utf8
UnionFields::new(
    vec![1, 3],
    vec![
        Field::new("field1", DataType::UInt8, false),
        Field::new("field3", DataType::Utf8, false),
    ],
);
```

[Source](about:blank/src/arrow_schema/fields.rs.html#378-382)

Return size of this instance in bytes.

[Source](about:blank/src/arrow_schema/fields.rs.html#385-387)

Returns the number of fields in this [`UnionFields`](struct.UnionFields.html "struct arrow_schema::UnionFields")

[Source](about:blank/src/arrow_schema/fields.rs.html#390-392)

Returns `true` if this is empty

[Source](about:blank/src/arrow_schema/fields.rs.html#395-397)

Returns an iterator over the fields and type ids in this [`UnionFields`](struct.UnionFields.html "struct arrow_schema::UnionFields")

[§](#impl-Freeze-for-UnionFields)

[§](#impl-RefUnwindSafe-for-UnionFields)

[§](#impl-Send-for-UnionFields)

[§](#impl-Sync-for-UnionFields)

[§](#impl-Unpin-for-UnionFields)

[§](#impl-UnwindSafe-for-UnionFields)
