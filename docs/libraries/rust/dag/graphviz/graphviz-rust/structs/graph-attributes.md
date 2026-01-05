# GraphAttributes in dot_structures - Rust

## Enum GraphAttributes 

[Source](about:blank/src/dot_structures/lib.rs.html#60-64)

```
pub enum GraphAttributes {
    Graph(Vec<Attribute>),
    Node(Vec<Attribute>),
    Edge(Vec<Attribute>),
}
```

Expand description

the component represents a set of attributes with prefix denoting a type in the language.

[§](#variant.Graph)

[§](#variant.Node)

[§](#variant.Edge)

[Source](about:blank/src/dot_structures/lib.rs.html#59)
[§](#impl-Clone-for-GraphAttributes)

[Source](about:blank/src/dot_structures/lib.rs.html#59)
[§](#impl-Debug-for-GraphAttributes)

[Source](about:blank/src/dot_structures/lib.rs.html#133-137)
[§](#impl-From%3CGraphAttributes%3E-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#134-136)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/dot_structures/lib.rs.html#59)
[§](#impl-PartialEq-for-GraphAttributes)

[Source](about:blank/src/dot_structures/lib.rs.html#59)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/dot_structures/lib.rs.html#59)
[§](#impl-StructuralPartialEq-for-GraphAttributes)

[§](#impl-Freeze-for-GraphAttributes)

[§](#impl-RefUnwindSafe-for-GraphAttributes)

[§](#impl-Send-for-GraphAttributes)

[§](#impl-Sync-for-GraphAttributes)

[§](#impl-Unpin-for-GraphAttributes)

[§](#impl-UnwindSafe-for-GraphAttributes)
