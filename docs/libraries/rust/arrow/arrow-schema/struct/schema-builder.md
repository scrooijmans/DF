# SchemaBuilder in arrow_schema - Rust

## Struct SchemaBuilder 

[Source](about:blank/src/arrow_schema/schema.rs.html#29-32)

```
pub struct SchemaBuilder { /* private fields */ }
```

Expand description

A builder to facilitate building a [`Schema`](struct.Schema.html "struct arrow_schema::Schema") from iteratively from [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef")

[Source](about:blank/src/arrow_schema/schema.rs.html#34-123)
[§](#impl-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#36-38)

[Source](about:blank/src/arrow_schema/schema.rs.html#41-46)

Creates a new empty [`SchemaBuilder`](struct.SchemaBuilder.html "struct arrow_schema::SchemaBuilder") with space for `capacity` fields

[Source](about:blank/src/arrow_schema/schema.rs.html#49-51)

[Source](about:blank/src/arrow_schema/schema.rs.html#58-60)

Removes and returns the [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef") as index `idx`

##### [§](#panics)Panics

Panics if index out of bounds

[Source](about:blank/src/arrow_schema/schema.rs.html#67-69)

Returns an immutable reference to the [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef") at index `idx`

##### [§](#panics-1)Panics

Panics if index out of bounds

[Source](about:blank/src/arrow_schema/schema.rs.html#76-78)

Returns a mutable reference to the [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef") at index `idx`

##### [§](#panics-2)Panics

Panics if index out of bounds

[Source](about:blank/src/arrow_schema/schema.rs.html#81-83)

Returns an immutable reference to the Map of custom metadata key-value pairs.

[Source](about:blank/src/arrow_schema/schema.rs.html#86-88)

Returns a mutable reference to the Map of custom metadata key-value pairs.

[Source](about:blank/src/arrow_schema/schema.rs.html#91-93)

Reverse the fileds

[Source](about:blank/src/arrow_schema/schema.rs.html#98-114)

[Source](about:blank/src/arrow_schema/schema.rs.html#117-122)

[Source](about:blank/src/arrow_schema/schema.rs.html#28)
[§](#impl-Debug-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#28)
[§](#impl-Default-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#158-166)
[§](#impl-Extend%3CArc%3CField%3E%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#159-165)
[§](#method.extend)

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#417)
[§](#method.extend_one)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#425)
[§](#method.extend_reserve)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](about:blank/src/arrow_schema/schema.rs.html#168-176)
[§](#impl-Extend%3CField%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#169-175)
[§](#method.extend-1)

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#417)
[§](#method.extend_one-1)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#425)
[§](#method.extend_reserve-1)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](about:blank/src/arrow_schema/schema.rs.html#125-132)
[§](#impl-From%3C%26Fields%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#126-131)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/schema.rs.html#143-147)
[§](#impl-From%3C%26Schema%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#144-146)
[§](#method.from-2)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/schema.rs.html#134-141)
[§](#impl-From%3CFields%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#135-140)
[§](#method.from-1)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/schema.rs.html#149-156)
[§](#impl-From%3CSchema%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#150-155)
[§](#method.from-3)

Converts to this type from the input type.

[§](#impl-Freeze-for-SchemaBuilder)

[§](#impl-RefUnwindSafe-for-SchemaBuilder)

[§](#impl-Send-for-SchemaBuilder)

[§](#impl-Sync-for-SchemaBuilder)

[§](#impl-Unpin-for-SchemaBuilder)

[§](#impl-UnwindSafe-for-SchemaBuilder)
