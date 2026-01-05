# SortOptions in arrow_schema - Rust

## Struct SortOptions 

[Source](about:blank/src/arrow_schema/lib.rs.html#84-89)

```
pub struct SortOptions {
    pub descending: bool,
    pub nulls_first: bool,
}
```

Expand description

Options that define the sort order of a given column

The default sorts equivalently to of `ASC NULLS FIRST` in SQL (i.e. ascending order with nulls sorting before any other values).

## [§](#example-creation)Example creation

```
// configure using explicit initialization
let options = SortOptions {
  descending: false,
  nulls_first: true,
};
// Default is ASC NULLs First
assert_eq!(options, SortOptions::default());
assert_eq!(options.to_string(), "ASC NULLS FIRST");

// Configure using builder APIs
let options = SortOptions::default()
 .desc()
 .nulls_first();
assert_eq!(options.to_string(), "DESC NULLS FIRST");

// configure using explicit field values
let options = SortOptions::default()
 .with_descending(false)
 .with_nulls_first(false);
assert_eq!(options.to_string(), "ASC NULLS LAST");
```

## [§](#example-operations)Example operations

It is also possible to negate the sort options using the `!` operator.

```
use arrow_schema::SortOptions;
let options = !SortOptions::default();
assert_eq!(options.to_string(), "DESC NULLS LAST");
```

Whether to sort in descending order

Whether to sort nulls first

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-Clone-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-Debug-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#161-169)
[§](#impl-Default-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#91-105)
[§](#impl-Display-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-Hash-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#173-182)
[§](#impl-Not-for-SortOptions)

`!` operator is overloaded for `SortOptions` to invert boolean fields of the struct.

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-Ord-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-PartialEq-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-PartialOrd-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#method.partial_cmp)

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398)
[§](#method.lt)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416)
[§](#method.le)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434)
[§](#method.gt)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452)
[§](#method.ge)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-Copy-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-Eq-for-SortOptions)

[Source](about:blank/src/arrow_schema/lib.rs.html#83)
[§](#impl-StructuralPartialEq-for-SortOptions)

[§](#impl-Freeze-for-SortOptions)

[§](#impl-RefUnwindSafe-for-SortOptions)

[§](#impl-Send-for-SortOptions)

[§](#impl-Sync-for-SortOptions)

[§](#impl-Unpin-for-SortOptions)

[§](#impl-UnwindSafe-for-SortOptions)
