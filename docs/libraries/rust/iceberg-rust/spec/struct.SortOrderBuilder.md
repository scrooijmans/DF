# Struct SortOrderBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/sort.rs.html#99" class="src">Source</a>

``` rust
pub struct SortOrderBuilder { /* private fields */ }
```

Expand description

Builder for [`SortOrder`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html).

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#impl-SortOrderBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.with_order_id" class="fn">with_order_id</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> &mut Self

Identifier for SortOrder, order_id `0` is no sort order.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.with_fields" class="fn">with_fields</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortField.html" class="struct" title="struct iceberg::spec::SortField">SortField</a>\>) -\> &mut Self

Details of the sort

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.with_sort_field" class="fn">with_sort_field</a>\<VALUE\>(&mut self, item: VALUE) -\> &mut Self

where <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortField.html" class="struct" title="struct iceberg::spec::SortField">SortField</a>\>: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<VALUE\>,

Details of the sort

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#impl-SortOrderBuilder-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.build_unbound" class="fn">build_unbound</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>\>

Creates a new unbound sort order.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.build" class="fn">build</a>(&self, schema: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>\>

Creates a new bound sort order.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#impl-Clone-for-SortOrderBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#impl-Default-for-SortOrderBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html#blanket-implementations" class="anchor">§</a>
