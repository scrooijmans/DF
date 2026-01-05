# Struct UnionFind Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/unionfind.rs.html#17-28" class="src">Source</a>

``` rust
pub struct UnionFind<K> { /* private fields */ }
```

Expand description

`UnionFind<K>` is a disjoint-set data structure. It tracks set membership of *n* elements indexed from *0* to *n - 1*. The scalar type is `K` which must be an unsigned integer type.

<http://en.wikipedia.org/wiki/Disjoint-set_data_structure>

Too awesome not to quote:

“The amortized time per operation is **O(α(n))** where **α(n)** is the inverse of **f(x) = A(x, x)** with **A** being the extremely fast-growing Ackermann function.”

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#impl-UnionFind%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html" class="struct" title="struct petgraph::unionfind::UnionFind">UnionFind</a>\<K\>

where K: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.new" class="fn">new</a>(n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new `UnionFind` of `n` disjoint sets.

#### pub const fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.new_empty" class="fn">new_empty</a>() -\> Self

Create a new `UnionFind` with no elements.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of elements in the union-find data-structure.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if there are no elements in the union-find data-structure.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.new_set" class="fn">new_set</a>(&mut self) -\> K

Adds a new disjoint set and returns the index of the new set.

The new disjoint set is always added to the end, so the returned index is the same as the number of elements before calling this function.

**Time Complexity** Takes amortized O(1) time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.find" class="fn">find</a>(&self, x: K) -\> K

Return the representative for `x`.

**Panics** if `x` is out of bounds.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_find" class="fn">try_find</a>(&self, x: K) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<K\>

Return the representative for `x` or `None` if `x` is out of bounds.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.find_mut" class="fn">find_mut</a>(&mut self, x: K) -\> K

Return the representative for `x`.

Write back the found representative, flattening the internal datastructure in the process and quicken future lookups.

**Panics** if `x` is out of bounds.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_find_mut" class="fn">try_find_mut</a>(&mut self, x: K) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<K\>

Return the representative for `x` or `None` if `x` is out of bounds.

Write back the found representative, flattening the internal datastructure in the process and quicken future lookups.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.equiv" class="fn">equiv</a>(&self, x: K, y: K) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the given elements belong to the same set, and returns `false` otherwise.

**Panics** if `x` or `y` is out of bounds.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_equiv" class="fn">try_equiv</a>(&self, x: K, y: K) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, K\>

Returns `Ok(true)` if the given elements belong to the same set, and returns `Ok(false)` otherwise.

If `x` or `y` are out of bounds, it returns `Err` with the first bad index found.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.union" class="fn">union</a>(&mut self, x: K, y: K) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Unify the two sets containing `x` and `y`.

Return `false` if the sets were already the same, `true` if they were unified.

**Panics** if `x` or `y` is out of bounds.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_union" class="fn">try_union</a>(&mut self, x: K, y: K) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, K\>

Unify the two sets containing `x` and `y`.

Return `Ok(false)` if the sets were already the same, `Ok(true)` if they were unified.

If `x` or `y` are out of bounds, it returns `Err` with first found bad index. But if `x == y`, the result will be `Ok(false)` even if the indexes go out of bounds.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.into_labeling" class="fn">into_labeling</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<K\>

Return a vector mapping each element to its representative.

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#impl-UnionFind%3CK%3E-1" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html" class="struct" title="struct petgraph::unionfind::UnionFind">UnionFind</a>\<K\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Constructs a new, empty `UnionFind<K>` with at least the specified capacity.

This acts similarly to [`Vec::with_capacity`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.with_capacity "associated function alloc::vec::Vec::with_capacity").

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of elements the `UnionFind` can hold without reallocating.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::unionfind::UnionFind;

let mut uf: UnionFind<u32> = UnionFind::with_capacity(10);
uf.new_set();
assert!(uf.capacity() >= 10);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves capacity for at least `additional` more elements to be inserted in the given `UnionFind<K>`. The collection may reserve more space to speculatively avoid frequent reallocations. After calling `reserve`, capacity will be greater than or equal to `self.len() + additional`. Does nothing if capacity is already sufficient.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#panics" class="doc-anchor">§</a>Panics

Panics if the new capacity exceeds `isize::MAX` *bytes*.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
use petgraph::unionfind::UnionFind;

let mut uf: UnionFind<u32> = UnionFind::new(3);
uf.reserve(10);
assert!(uf.capacity() >= 13);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.reserve_exact" class="fn">reserve_exact</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves the minimum capacity for at least `additional` more elements to be inserted in the given `UnionFind<K>`. Unlike [`reserve`](https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.reserve "method petgraph::unionfind::UnionFind::reserve"), this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling `reserve_exact`, capacity will be greater than or equal to `self.len() + additional`. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer [`reserve`](https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.reserve "method petgraph::unionfind::UnionFind::reserve") if future insertions are expected.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#panics-1" class="doc-anchor">§</a>Panics

Panics if the new capacity exceeds `isize::MAX` *bytes*.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
use petgraph::unionfind::UnionFind;

let mut uf: UnionFind<u32> =  UnionFind::new_empty();
uf.reserve_exact(10);
assert!(uf.capacity() >= 10);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_reserve" class="fn">try_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html" class="struct" title="struct alloc::collections::TryReserveError">TryReserveError</a>\>

Tries to reserve capacity for at least `additional` more elements to be inserted in the given `UnionFind<K>`. The collection may reserve more space to speculatively avoid frequent reallocations. After calling `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#errors" class="doc-anchor">§</a>Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_reserve_exact" class="fn">try_reserve_exact</a>( &mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html" class="struct" title="struct alloc::collections::TryReserveError">TryReserveError</a>\>

Tries to reserve the minimum capacity for at least `additional` elements to be inserted in the given `UnionFind<K>`. Unlike [`try_reserve`](https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_reserve "method petgraph::unionfind::UnionFind::try_reserve"), this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling `try_reserve_exact`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer [`try_reserve`](https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.try_reserve "method petgraph::unionfind::UnionFind::try_reserve") if future insertions are expected.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#errors-1" class="doc-anchor">§</a>Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of the `UnionFind` as much as possible.

The behavior of this method depends on the allocator, which may either shrink the collection in-place or reallocate. The resulting `UnionFind` might still have some excess capacity, just as is the case for [`with_capacity`](https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.with_capacity "associated function petgraph::unionfind::UnionFind::with_capacity"). See [`Vec::shrink_to_fit`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.shrink_to_fit "method alloc::vec::Vec::shrink_to_fit") for more details, since the implementation is based on this method.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
use petgraph::unionfind::UnionFind;

let mut uf: UnionFind<u32> = UnionFind::with_capacity(10);

for _ in 0..3 {
    uf.new_set();
}

assert!(uf.capacity() >= 10);
uf.shrink_to_fit();
assert!(uf.capacity() >= 3);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.shrink_to" class="fn">shrink_to</a>(&mut self, min_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Shrinks the capacity of the `UnionFind` with a lower bound.

The capacity will remain at least as large as both the length and the supplied value.

If the current capacity is less than the lower limit, this is a no-op.

##### <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
use petgraph::unionfind::UnionFind;

let mut uf: UnionFind<u32> = UnionFind::with_capacity(10);

for _ in 0..3 {
    uf.new_set();
}

assert!(uf.capacity() >= 10);
uf.shrink_to(4);
assert!(uf.capacity() >= 4);
uf.shrink_to(0);
assert!(uf.capacity() >= 3);
```

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#impl-Clone-for-UnionFind%3CK%3E" class="anchor">§</a>

### impl\<K: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html" class="struct" title="struct petgraph::unionfind::UnionFind">UnionFind</a>\<K\>

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html" class="struct" title="struct petgraph::unionfind::UnionFind">UnionFind</a>\<K\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#impl-Debug-for-UnionFind%3CK%3E" class="anchor">§</a>

### impl\<K: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html" class="struct" title="struct petgraph::unionfind::UnionFind">UnionFind</a>\<K\>

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#impl-Default-for-UnionFind%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html" class="struct" title="struct petgraph::unionfind::UnionFind">UnionFind</a>\<K\>

<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html#blanket-implementations" class="anchor">§</a>
