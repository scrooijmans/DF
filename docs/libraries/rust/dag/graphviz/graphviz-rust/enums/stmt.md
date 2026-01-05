# Stmt in dot_structures - Rust

```
pub enum Stmt {
    Node(Node),
    Subgraph(Subgraph),
    Attribute(Attribute),
    GAttribute(GraphAttributes),
    Edge(Edge),
}
```

Expand description

the component represents a wrapper to keep sustainability in subgraph and graph bodies.

[§](#variant.Node)

[§](#variant.Subgraph)

[§](#variant.Attribute)

[§](#variant.GAttribute)

[§](#variant.Edge)

[Source](about:blank/src/dot_structures/lib.rs.html#112)
[§](#impl-Clone-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#112)
[§](#impl-Debug-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#139-143)
[§](#impl-From%3CAttribute%3E-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#140-142)
[§](#method.from-3)

Converts to this type from the input type.

[Source](about:blank/src/dot_structures/lib.rs.html#127-131)
[§](#impl-From%3CEdge%3E-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#128-130)
[§](#method.from-1)

Converts to this type from the input type.

[Source](about:blank/src/dot_structures/lib.rs.html#133-137)
[§](#impl-From%3CGraphAttributes%3E-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#134-136)
[§](#method.from-2)

Converts to this type from the input type.

[Source](about:blank/src/dot_structures/lib.rs.html#121-125)
[§](#impl-From%3CNode%3E-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#122-124)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/dot_structures/lib.rs.html#145-149)
[§](#impl-From%3CSubgraph%3E-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#146-148)
[§](#method.from-4)

Converts to this type from the input type.

[Source](about:blank/src/dot_structures/lib.rs.html#112)
[§](#impl-PartialEq-for-Stmt)

[Source](about:blank/src/dot_structures/lib.rs.html#112)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/dot_structures/lib.rs.html#112)
[§](#impl-StructuralPartialEq-for-Stmt)

[§](#impl-Freeze-for-Stmt)

[§](#impl-RefUnwindSafe-for-Stmt)

[§](#impl-Send-for-Stmt)

[§](#impl-Sync-for-Stmt)

[§](#impl-Unpin-for-Stmt)

[§](#impl-UnwindSafe-for-Stmt)
