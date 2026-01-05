# Struct IcebergSchema Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/schema/iceberg.rs.html#21" class="src">Source</a>

``` rust
pub struct IcebergSchema(/* private fields */);
```

Expand description

Maps Iceberg physical IDs to columns.

Note: This doesn’t use `Schema<D>` as the keys are u32’s.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-IcebergSchema" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.from_arrow_schema" class="fn">from_arrow_schema</a>( schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Constructs a schema keyed by the physical ID stored in the arrow field metadata.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.try_from_arrow_fields_iter" class="fn">try_from_arrow_fields_iter</a>\<'a, I\>( iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>,

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergColumn.html" class="struct" title="struct polars::prelude::iceberg::IcebergColumn">IcebergColumn</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>\><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#deref-methods-IndexMap%3Cu32,+IcebergColumn,+RandomState%3E" class="anchor">§</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of elements the map can hold without reallocating.

This number is a lower bound; the map might be able to hold more, but is guaranteed to be able to hold at least this many.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.hasher" class="fn">hasher</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;S</a>

Return a reference to the map’s `BuildHasher`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of key-value pairs in the map.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the map contains no elements.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html" class="struct" title="struct indexmap::map::iter::Iter">Iter</a>\<'\_, K, V\>

Return an iterator over the key-value pairs of the map, in their order

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.iter_mut" class="fn">iter_mut</a>(&mut self) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.IterMut.html" class="struct" title="struct indexmap::map::iter::IterMut">IterMut</a>\<'\_, K, V\>

Return an iterator over the key-value pairs of the map, in their order

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.keys" class="fn">keys</a>(&self) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html" class="struct" title="struct indexmap::map::iter::Keys">Keys</a>\<'\_, K, V\>

Return an iterator over the keys of the map, in their order

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.values" class="fn">values</a>(&self) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html" class="struct" title="struct indexmap::map::iter::Values">Values</a>\<'\_, K, V\>

Return an iterator over the values of the map, in their order

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.values_mut" class="fn">values_mut</a>(&mut self) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html" class="struct" title="struct indexmap::map::iter::ValuesMut">ValuesMut</a>\<'\_, K, V\>

Return an iterator over mutable references to the values of the map, in their order

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.clear" class="fn">clear</a>(&mut self)

Remove all key-value pairs in the map, while preserving its capacity.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.truncate" class="fn">truncate</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Shortens the map, keeping the first `len` elements and dropping the rest.

If `len` is greater than the map’s current length, this has no effect.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.drain" class="fn">drain</a>\<R\>(&mut self, range: R) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Drain.html" class="struct" title="struct indexmap::map::iter::Drain">Drain</a>\<'\_, K, V\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Clears the `IndexMap` in the given index range, returning those key-value pairs as a drain iterator.

The range may be any type that implements [`RangeBounds<usize>`](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds"), including all of the `std::ops::Range*` types, or even a tuple pair of `Bound` start and end values. To drain the map entirely, use `RangeFull` like `map.drain(..)`.

This shifts down all entries following the drained range to fill the gap, and keeps the allocated memory for reuse.

***Panics*** if the starting point is greater than the end point or if the end point is greater than the length of the map.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.extract_if" class="fn">extract_if</a>\<F, R\>(&mut self, range: R, pred: F) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.ExtractIf.html" class="struct" title="struct indexmap::map::iter::ExtractIf">ExtractIf</a>\<'\_, K, V, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Creates an iterator which uses a closure to determine if an element should be removed, for all elements in the given range.

If the closure returns true, the element is removed from the map and yielded. If the closure returns false, or panics, the element remains in the map and will not be yielded.

Note that `extract_if` lets you mutate every value in the filter closure, regardless of whether you choose to keep or remove it.

The range may be any type that implements [`RangeBounds<usize>`](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds"), including all of the `std::ops::Range*` types, or even a tuple pair of `Bound` start and end values. To check the entire map, use `RangeFull` like `map.extract_if(.., predicate)`.

If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating or the iteration short-circuits, then the remaining elements will be retained. Use [`retain`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.retain "method indexmap::map::IndexMap::retain") with a negated predicate if you do not need the returned iterator.

***Panics*** if the starting point is greater than the end point or if the end point is greater than the length of the map.

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples" class="doc-anchor">§</a>Examples

Splitting a map into even and odd keys, reusing the original map:

``` rust
use indexmap::IndexMap;

let mut map: IndexMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
let extracted: IndexMap<i32, i32> = map.extract_if(.., |k, _v| k % 2 == 0).collect();

let evens = extracted.keys().copied().collect::<Vec<_>>();
let odds = map.keys().copied().collect::<Vec<_>>();

assert_eq!(evens, vec![0, 2, 4, 6]);
assert_eq!(odds, vec![1, 3, 5, 7]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.split_off" class="fn">split_off</a>(&mut self, at: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<K, V, S\>

where S: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Splits the collection into two at the given index.

Returns a newly allocated map containing the elements in the range `[at, len)`. After the call, the original map will be left containing the elements `[0, at)` with its previous capacity unchanged.

***Panics*** if `at > len`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserve capacity for `additional` more key-value pairs.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.reserve_exact" class="fn">reserve_exact</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserve capacity for `additional` more key-value pairs, without over-allocating.

Unlike `reserve`, this does not deliberately over-allocate the entry capacity to avoid frequent re-allocations. However, the underlying data structures may still have internal capacity requirements, and the allocator itself may give more space than requested, so this cannot be relied upon to be precisely minimal.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.try_reserve" class="fn">try_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/struct.TryReserveError.html" class="struct" title="struct indexmap::TryReserveError">TryReserveError</a>\>

Try to reserve capacity for `additional` more key-value pairs.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.try_reserve_exact" class="fn">try_reserve_exact</a>( &mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/struct.TryReserveError.html" class="struct" title="struct indexmap::TryReserveError">TryReserveError</a>\>

Try to reserve capacity for `additional` more key-value pairs, without over-allocating.

Unlike `try_reserve`, this does not deliberately over-allocate the entry capacity to avoid frequent re-allocations. However, the underlying data structures may still have internal capacity requirements, and the allocator itself may give more space than requested, so this cannot be relied upon to be precisely minimal.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrink the capacity of the map as much as possible.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shrink_to" class="fn">shrink_to</a>(&mut self, min_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Shrink the capacity of the map with a lower limit.

Computes in **O(n)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.insert" class="fn">insert</a>(&mut self, key: K, value: V) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>

Insert a key-value pair in the map.

If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with `value`, and the older value is returned inside `Some(_)`.

If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and `None` is returned.

Computes in **O(1)** time (amortized average).

See also [`entry`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert *or* modify, or [`insert_full`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_full "method indexmap::map::IndexMap::insert_full") if you need to get the index of the corresponding key-value pair.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.insert_full" class="fn">insert_full</a>(&mut self, key: K, value: V) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>)

Insert a key-value pair in the map, and get their index.

If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with `value`, and the older value is returned inside `(index, Some(_))`.

If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and `(index, None)` is returned.

Computes in **O(1)** time (amortized average).

See also [`entry`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert *or* modify.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.insert_sorted" class="fn">insert_sorted</a>(&mut self, key: K, value: V) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>)

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Insert a key-value pair in the map at its ordered position among sorted keys.

This is equivalent to finding the position with [`binary_search_keys`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.binary_search_keys "method indexmap::map::IndexMap::binary_search_keys"), then either updating it or calling [`insert_before`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_before "method indexmap::map::IndexMap::insert_before") for a new key.

If the sorted key is found in the map, its corresponding value is updated with `value`, and the older value is returned inside `(index, Some(_))`. Otherwise, the new key-value pair is inserted at the sorted position, and `(index, None)` is returned.

If the existing keys are **not** already sorted, then the insertion index is unspecified (like [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search")), but the key-value pair is moved to or inserted at that position regardless.

Computes in **O(n)** time (average). Instead of repeating calls to `insert_sorted`, it may be faster to call batched [`insert`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert "method indexmap::map::IndexMap::insert") or [`extend`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.extend "method indexmap::map::IndexMap::extend") and only call [`sort_keys`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_keys "method indexmap::map::IndexMap::sort_keys") or [`sort_unstable_keys`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_unstable_keys "method indexmap::map::IndexMap::sort_unstable_keys") once.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.insert_before" class="fn">insert_before</a>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, key: K, value: V, ) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>)

Insert a key-value pair in the map before the entry at the given index, or at the end.

If an equivalent key already exists in the map: the key remains and is moved to the new position in the map, its corresponding value is updated with `value`, and the older value is returned inside `Some(_)`. The returned index will either be the given index or one less, depending on how the entry moved. (See [`shift_insert`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.shift_insert "method indexmap::map::IndexMap::shift_insert") for different behavior here.)

If no equivalent key existed in the map: the new key-value pair is inserted exactly at the given index, and `None` is returned.

***Panics*** if `index` is out of bounds. Valid indices are `0..=map.len()` (inclusive).

Computes in **O(n)** time (average).

See also [`entry`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert *or* modify, perhaps only using the index for new entries with [`VacantEntry::shift_insert`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/core/entry/struct.VacantEntry.html#method.shift_insert "method indexmap::map::core::entry::VacantEntry::shift_insert").

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
use indexmap::IndexMap;
let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

// The new key '*' goes exactly at the given index.
assert_eq!(map.get_index_of(&'*'), None);
assert_eq!(map.insert_before(10, '*', ()), (10, None));
assert_eq!(map.get_index_of(&'*'), Some(10));

// Moving the key 'a' up will shift others down, so this moves *before* 10 to index 9.
assert_eq!(map.insert_before(10, 'a', ()), (9, Some(())));
assert_eq!(map.get_index_of(&'a'), Some(9));
assert_eq!(map.get_index_of(&'*'), Some(10));

// Moving the key 'z' down will shift others up, so this moves to exactly 10.
assert_eq!(map.insert_before(10, 'z', ()), (10, Some(())));
assert_eq!(map.get_index_of(&'z'), Some(10));
assert_eq!(map.get_index_of(&'*'), Some(11));

// Moving or inserting before the endpoint is also valid.
assert_eq!(map.len(), 27);
assert_eq!(map.insert_before(map.len(), '*', ()), (26, Some(())));
assert_eq!(map.get_index_of(&'*'), Some(26));
assert_eq!(map.insert_before(map.len(), '+', ()), (27, None));
assert_eq!(map.get_index_of(&'+'), Some(27));
assert_eq!(map.len(), 28);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shift_insert" class="fn">shift_insert</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, key: K, value: V) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>

Insert a key-value pair in the map at the given index.

If an equivalent key already exists in the map: the key remains and is moved to the given index in the map, its corresponding value is updated with `value`, and the older value is returned inside `Some(_)`. Note that existing entries **cannot** be moved to `index == map.len()`! (See [`insert_before`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_before "method indexmap::map::IndexMap::insert_before") for different behavior here.)

If no equivalent key existed in the map: the new key-value pair is inserted at the given index, and `None` is returned.

***Panics*** if `index` is out of bounds. Valid indices are `0..map.len()` (exclusive) when moving an existing entry, or `0..=map.len()` (inclusive) when inserting a new key.

Computes in **O(n)** time (average).

See also [`entry`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert *or* modify, perhaps only using the index for new entries with [`VacantEntry::shift_insert`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/core/entry/struct.VacantEntry.html#method.shift_insert "method indexmap::map::core::entry::VacantEntry::shift_insert").

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
use indexmap::IndexMap;
let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

// The new key '*' goes exactly at the given index.
assert_eq!(map.get_index_of(&'*'), None);
assert_eq!(map.shift_insert(10, '*', ()), None);
assert_eq!(map.get_index_of(&'*'), Some(10));

// Moving the key 'a' up to 10 will shift others down, including the '*' that was at 10.
assert_eq!(map.shift_insert(10, 'a', ()), Some(()));
assert_eq!(map.get_index_of(&'a'), Some(10));
assert_eq!(map.get_index_of(&'*'), Some(9));

// Moving the key 'z' down to 9 will shift others up, including the '*' that was at 9.
assert_eq!(map.shift_insert(9, 'z', ()), Some(()));
assert_eq!(map.get_index_of(&'z'), Some(9));
assert_eq!(map.get_index_of(&'*'), Some(10));

// Existing keys can move to len-1 at most, but new keys can insert at the endpoint.
assert_eq!(map.len(), 27);
assert_eq!(map.shift_insert(map.len() - 1, '*', ()), Some(()));
assert_eq!(map.get_index_of(&'*'), Some(26));
assert_eq!(map.shift_insert(map.len(), '+', ()), None);
assert_eq!(map.get_index_of(&'+'), Some(27));
assert_eq!(map.len(), 28);
```

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#" class="tooltip" title="This example panics">ⓘ</a>

``` rust
use indexmap::IndexMap;
let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

// This is an invalid index for moving an existing key!
map.shift_insert(map.len(), 'a', ());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.entry" class="fn">entry</a>(&mut self, key: K) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/core/entry/enum.Entry.html" class="enum" title="enum indexmap::map::core::entry::Entry">Entry</a>\<'\_, K, V\>

Get the given key’s corresponding entry in the map for insertion and/or in-place manipulation.

Computes in **O(1)** time (amortized average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.splice" class="fn">splice</a>\<R, I\>( &mut self, range: R, replace_with: I, ) -\> <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Splice.html" class="struct" title="struct indexmap::map::iter::Splice">Splice</a>\<'\_, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>, K, V, S\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>,

Creates a splicing iterator that replaces the specified range in the map with the given `replace_with` key-value iterator and yields the removed items. `replace_with` does not need to be the same length as `range`.

The `range` is removed even if the iterator is not consumed until the end. It is unspecified how many elements are removed from the map if the `Splice` value is leaked.

The input iterator `replace_with` is only consumed when the `Splice` value is dropped. If a key from the iterator matches an existing entry in the map (outside of `range`), then the value will be updated in that position. Otherwise, the new key-value pair will be inserted in the replaced `range`.

***Panics*** if the starting point is greater than the end point or if the end point is greater than the length of the map.

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
use indexmap::IndexMap;

let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
let removed: Vec<_> = map.splice(2..4, new).collect();

// 1 and 4 got new values, while 5, 3, and 2 were newly inserted.
assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')]));
assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.append" class="fn">append</a>\<S2\>(&mut self, other: &mut <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<K, V, S2\>)

Moves all key-value pairs from `other` into `self`, leaving `other` empty.

This is equivalent to calling [`insert`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert "method indexmap::map::IndexMap::insert") for each key-value pair from `other` in order, which means that for keys that already exist in `self`, their value is updated in the current position.

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
use indexmap::IndexMap;

// Note: Key (3) is present in both maps.
let mut a = IndexMap::from([(3, "c"), (2, "b"), (1, "a")]);
let mut b = IndexMap::from([(3, "d"), (4, "e"), (5, "f")]);
let old_capacity = b.capacity();

a.append(&mut b);

assert_eq!(a.len(), 5);
assert_eq!(b.len(), 0);
assert_eq!(b.capacity(), old_capacity);

assert!(a.keys().eq(&[3, 2, 1, 4, 5]));
assert_eq!(a[&3], "d"); // "c" was overwritten.
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.contains_key" class="fn">contains_key</a>\<Q\>(&self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Return `true` if an equivalent to `key` exists in the map.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get" class="fn">get</a>\<Q\>(&self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Return a reference to the value stored for `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_key_value" class="fn">get_key_value</a>\<Q\>(&self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>)\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Return references to the key-value pair stored for `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_full" class="fn">get_full</a>\<Q\>(&self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>)\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Return item index, key and value

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_index_of" class="fn">get_index_of</a>\<Q\>(&self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Return item index, if it exists in the map

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_mut" class="fn">get_mut</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_full_mut" class="fn">get_full_mut</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>)\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_disjoint_mut" class="fn">get_disjoint_mut</a>\<Q, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, keys: \[<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\], ) -\> \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>\>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Return the values for `N` keys. If any key is duplicated, this function will panic.

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
assert_eq!(map.get_disjoint_mut([&2, &1]), [Some(&mut 'c'), Some(&mut 'a')]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.remove" class="fn">remove</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

👎Deprecated: `remove` disrupts the map order – use `swap_remove` or `shift_remove` for explicit behavior.

Remove the key-value pair equivalent to `key` and return its value.

**NOTE:** This is equivalent to [`.swap_remove(key)`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.swap_remove "method indexmap::map::IndexMap::swap_remove"), replacing this entry’s position with the last element, and it is deprecated in favor of calling that explicitly. If you need to preserve the relative order of the keys in the map, use [`.shift_remove(key)`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.shift_remove "method indexmap::map::IndexMap::shift_remove") instead.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.remove_entry" class="fn">remove_entry</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

👎Deprecated: `remove_entry` disrupts the map order – use `swap_remove_entry` or `shift_remove_entry` for explicit behavior.

Remove and return the key-value pair equivalent to `key`.

**NOTE:** This is equivalent to [`.swap_remove_entry(key)`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.swap_remove_entry "method indexmap::map::IndexMap::swap_remove_entry"), replacing this entry’s position with the last element, and it is deprecated in favor of calling that explicitly. If you need to preserve the relative order of the keys in the map, use [`.shift_remove_entry(key)`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.shift_remove_entry "method indexmap::map::IndexMap::shift_remove_entry") instead.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.swap_remove" class="fn">swap_remove</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove the key-value pair equivalent to `key` and return its value.

Like [`Vec::swap_remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.swap_remove "method alloc::vec::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Return `None` if `key` is not in map.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.swap_remove_entry" class="fn">swap_remove_entry</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove and return the key-value pair equivalent to `key`.

Like [`Vec::swap_remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.swap_remove "method alloc::vec::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Return `None` if `key` is not in map.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.swap_remove_full" class="fn">swap_remove_full</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, K, V)\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove the key-value pair equivalent to `key` and return it and the index it had.

Like [`Vec::swap_remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.swap_remove "method alloc::vec::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Return `None` if `key` is not in map.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shift_remove" class="fn">shift_remove</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove the key-value pair equivalent to `key` and return its value.

Like [`Vec::remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.remove "method alloc::vec::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Return `None` if `key` is not in map.

Computes in **O(n)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shift_remove_entry" class="fn">shift_remove_entry</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove and return the key-value pair equivalent to `key`.

Like [`Vec::remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.remove "method alloc::vec::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Return `None` if `key` is not in map.

Computes in **O(n)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shift_remove_full" class="fn">shift_remove_full</a>\<Q\>(&mut self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, K, V)\>

where Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Equivalent.html" class="trait" title="trait equivalent::Equivalent">Equivalent</a>\<K\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove the key-value pair equivalent to `key` and return it and the index it had.

Like [`Vec::remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.remove "method alloc::vec::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Return `None` if `key` is not in map.

Computes in **O(n)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.pop" class="fn">pop</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>

Remove the last key-value pair

This preserves the order of the remaining elements.

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.retain" class="fn">retain</a>\<F\>(&mut self, keep: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Scan through each key-value pair in the map and keep those where the closure `keep` returns `true`.

The elements are visited in order, and remaining elements keep their order.

Computes in **O(n)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.sort_keys" class="fn">sort_keys</a>(&mut self)

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sort the map’s key-value pairs by the default ordering of the keys.

This is a stable sort – but equivalent keys should not normally coexist in a map at all, so [`sort_unstable_keys`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_unstable_keys "method indexmap::map::IndexMap::sort_unstable_keys") is preferred because it is generally faster and doesn’t allocate auxiliary memory.

See [`sort_by`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_by "method indexmap::map::IndexMap::sort_by") for details.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.sort_by" class="fn">sort_by</a>\<F\>(&mut self, cmp: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Sort the map’s key-value pairs in place using the comparison function `cmp`.

The comparison function receives two key and value pairs to compare (you can sort by keys or values or their combination as needed).

Computes in **O(n log n + c)** time and **O(n)** space where *n* is the length of the map and *c* the capacity. The sort is stable.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.sort_unstable_keys" class="fn">sort_unstable_keys</a>(&mut self)

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Sort the map’s key-value pairs by the default ordering of the keys, but may not preserve the order of equal elements.

See [`sort_unstable_by`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_unstable_by "method indexmap::map::IndexMap::sort_unstable_by") for details.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.sort_unstable_by" class="fn">sort_unstable_by</a>\<F\>(&mut self, cmp: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Sort the map’s key-value pairs in place using the comparison function `cmp`, but may not preserve the order of equal elements.

The comparison function receives two key and value pairs to compare (you can sort by keys or values or their combination as needed).

Computes in **O(n log n + c)** time where *n* is the length of the map and *c* is the capacity. The sort is unstable.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.sort_by_cached_key" class="fn">sort_by_cached_key</a>\<T, F\>(&mut self, sort_key: F)

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>) -\> T,

Sort the map’s key-value pairs in place using a sort-key extraction function.

During sorting, the function is called at most once per entry, by using temporary storage to remember the results of its evaluation. The order of calls to the function is unspecified and may change between versions of `indexmap` or the standard library.

Computes in **O(m n + n log n + c)** time () and **O(n)** space, where the function is **O(m)**, *n* is the length of the map, and *c* the capacity. The sort is stable.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.binary_search_keys" class="fn">binary_search_keys</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Search over a sorted map for a key.

Returns the position where that key is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search") for more details.

Computes in **O(log(n))** time, which is notably less scalable than looking the key up using [`get_index_of`](https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.get_index_of "method indexmap::map::IndexMap::get_index_of"), but this can also position missing keys.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.binary_search_by" class="fn">binary_search_by</a>\<'a, F\>(&'a self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Search over a sorted map with a comparator function.

Returns the position where that value is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by") for more details.

Computes in **O(log(n))** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.binary_search_by_key" class="fn">binary_search_by_key</a>\<'a, B, F\>( &'a self, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;B</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a>) -\> B, B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Search over a sorted map with an extraction function.

Returns the position where that value is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key") for more details.

Computes in **O(log(n))** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.partition_point" class="fn">partition_point</a>\<P\>(&self, pred: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Returns the index of the partition point of a sorted map according to the given predicate (the index of the first element of the second partition).

See [`slice::partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point") for more details.

Computes in **O(log(n))** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.reverse" class="fn">reverse</a>(&mut self)

Reverses the order of the map’s key-value pairs in place.

Computes in **O(n)** time and **O(1)** space.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.as_slice" class="fn">as_slice</a>(&self) -\> &<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html" class="struct" title="struct indexmap::map::slice::Slice">Slice</a>\<K, V\>

Returns a slice of all the key-value pairs in the map.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.as_mut_slice" class="fn">as_mut_slice</a>(&mut self) -\> &mut <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html" class="struct" title="struct indexmap::map::slice::Slice">Slice</a>\<K, V\>

Returns a mutable slice of all the key-value pairs in the map.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_index" class="fn">get_index</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>)\>

Get a key-value pair by index

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_index_mut" class="fn">get_index_mut</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>)\>

Get a key-value pair by index

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_index_entry" class="fn">get_index_entry</a>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/core/entry/struct.IndexedEntry.html" class="struct" title="struct indexmap::map::core::entry::IndexedEntry">IndexedEntry</a>\<'\_, K, V\>\>

Get an entry in the map by index for in-place manipulation.

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_disjoint_indices_mut" class="fn">get_disjoint_indices_mut</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, indices: \[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\[(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>); <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\], <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/enum.GetDisjointMutError.html" class="enum" title="enum indexmap::GetDisjointMutError">GetDisjointMutError</a>\>

Get an array of `N` key-value pairs by `N` indices

Valid indices are *0 \<= index \< self.len()* and each index needs to be unique.

##### <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#examples-6" class="doc-anchor">§</a>Examples

``` rust
let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
assert_eq!(map.get_disjoint_indices_mut([2, 0]), Ok([(&2, &mut 'c'), (&1, &mut 'a')]));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_range" class="fn">get_range</a>\<R\>(&self, range: R) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html" class="struct" title="struct indexmap::map::slice::Slice">Slice</a>\<K, V\>\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Returns a slice of key-value pairs in the given range of indices.

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.get_range_mut" class="fn">get_range_mut</a>\<R\>(&mut self, range: R) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html" class="struct" title="struct indexmap::map::slice::Slice">Slice</a>\<K, V\>\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Returns a mutable slice of key-value pairs in the given range of indices.

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.first" class="fn">first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>)\>

Get the first key-value pair

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.first_mut" class="fn">first_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>)\>

Get the first key-value pair, with mutable access to the value

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.first_entry" class="fn">first_entry</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/core/entry/struct.IndexedEntry.html" class="struct" title="struct indexmap::map::core::entry::IndexedEntry">IndexedEntry</a>\<'\_, K, V\>\>

Get the first entry in the map for in-place manipulation.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;V</a>)\>

Get the last key-value pair

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.last_mut" class="fn">last_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>)\>

Get the last key-value pair, with mutable access to the value

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.last_entry" class="fn">last_entry</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/core/entry/struct.IndexedEntry.html" class="struct" title="struct indexmap::map::core::entry::IndexedEntry">IndexedEntry</a>\<'\_, K, V\>\>

Get the last entry in the map for in-place manipulation.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.swap_remove_index" class="fn">swap_remove_index</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>

Remove the key-value pair by index

Valid indices are `0 <= index < self.len()`.

Like [`Vec::swap_remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.swap_remove "method alloc::vec::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Computes in **O(1)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.shift_remove_index" class="fn">shift_remove_index</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>

Remove the key-value pair by index

Valid indices are `0 <= index < self.len()`.

Like [`Vec::remove`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.remove "method alloc::vec::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Computes in **O(n)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.move_index" class="fn">move_index</a>(&mut self, from: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, to: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Moves the position of a key-value pair from one index to another by shifting all other pairs in-between.

- If `from < to`, the other pairs will shift down while the targeted pair moves up.
- If `from > to`, the other pairs will shift up while the targeted pair moves down.

***Panics*** if `from` or `to` are out of bounds.

Computes in **O(n)** time (average).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.swap_indices" class="fn">swap_indices</a>(&mut self, a: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Swaps the position of two key-value pairs in the map.

***Panics*** if `a` or `b` are out of bounds.

Computes in **O(1)** time (average).

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Clone-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Debug-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Deref-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergColumn.html" class="struct" title="struct polars::prelude::iceberg::IcebergColumn">IcebergColumn</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-DerefMut-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html" class="trait" title="trait core::ops::deref::DerefMut">DerefMut</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.deref_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut" class="fn">deref_mut</a>(&mut self) -\> &mut \<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Mutably dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Deserialize%3C&#39;de%3E-for-IcebergSchema" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-FromIterator%3CT%3E-for-IcebergSchema" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

where <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergColumn.html" class="struct" title="struct polars::prelude::iceberg::IcebergColumn">IcebergColumn</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<T\>,

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Hash-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-PartialEq-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Serialize-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-Eq-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#impl-StructuralPartialEq-for-IcebergSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html" class="struct" title="struct polars::prelude::iceberg::IcebergSchema">IcebergSchema</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/iceberg/struct.IcebergSchema.html#blanket-implementations" class="anchor">§</a>
