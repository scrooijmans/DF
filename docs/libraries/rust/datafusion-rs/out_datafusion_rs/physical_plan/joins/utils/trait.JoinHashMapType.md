# Trait JoinHashMapType Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/joins/join_hash_map.rs.html#96" class="src">Source</a>

``` rust
pub trait JoinHashMapType: Send + Sync {
    // Required methods
    fn extend_zero(&mut self, len: usize);
    fn update_from_iter<'a>(
        &mut self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>,
        deleted_offset: usize,
    );
    fn get_matched_indices<'a>(
        &self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>,
        deleted_offset: Option<usize>,
    ) -> (Vec<u32>, Vec<u64>);
    fn get_matched_indices_with_limit_offset(
        &self,
        hash_values: &[u64],
        limit: usize,
        offset: (usize, Option<u64>),
    ) -> (Vec<u32>, Vec<u64>, Option<(usize, Option<u64>)>);
    fn is_empty(&self) -> bool;
}
```

Expand description

Maps a `u64` hash value based on the build side \[“on” values\] to a list of indices with this key’s value.

By allocating a `HashMap` with capacity for *at least* the number of rows for entries at the build side, we make sure that we don’t have to re-hash the hashmap, which needs access to the key (the hash in this case) value.

E.g. 1 -\> \[3, 6, 8\] indicates that the column values map to rows 3, 6 and 8 for hash value 1 As the key is a hash value, we need to check possible hash collisions in the probe stage During this stage it might be the case that a row is contained the same hashmap value, but the values don’t match. Those are checked in the `equal_rows_arr` method.

The indices (values) are stored in a separate chained list stored as `Vec<u32>` or `Vec<u64>`.

The first value (+1) is stored in the hashmap, whereas the next value is stored in array at the position value.

The chain can be followed until the value “0” has been reached, meaning the end of the list. Also see chapter 5.3 of [Balancing vectorized query execution with bandwidth-optimized storage](https://dare.uva.nl/search?identifier=5ccbb60a-38b8-4eeb-858a-e7735dd37487)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#example" class="doc-anchor">§</a>Example

``` text
See the example below:

Insert (10,1)            <-- insert hash value 10 with row index 1
map:
----------
| 10 | 2 |
----------
next:
---------------------
| 0 | 0 | 0 | 0 | 0 |
---------------------
Insert (20,2)
map:
----------
| 10 | 2 |
| 20 | 3 |
----------
next:
---------------------
| 0 | 0 | 0 | 0 | 0 |
---------------------
Insert (10,3)           <-- collision! row index 3 has a hash value of 10 as well
map:
----------
| 10 | 4 |
| 20 | 3 |
----------
next:
---------------------
| 0 | 0 | 0 | 2 | 0 |  <--- hash value 10 maps to 4,2 (which means indices values 3,1)
---------------------
Insert (10,4)          <-- another collision! row index 4 ALSO has a hash value of 10
map:
---------
| 10 | 5 |
| 20 | 3 |
---------
next:
---------------------
| 0 | 0 | 0 | 2 | 4 | <--- hash value 10 maps to 5,4,2 (which means indices values 4,3,1)
---------------------
```

Here we have an option between creating a `JoinHashMapType` using `u32` or `u64` indices based on how many rows were being used for indices.

At runtime we choose between using `JoinHashMapU32` and `JoinHashMapU64` which oth implement `JoinHashMapType`.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#tymethod.extend_zero" class="fn">extend_zero</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#tymethod.update_from_iter" class="fn">update_from_iter</a>\<'a\>( &mut self, iter: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>, deleted_offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, )

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#tymethod.get_matched_indices" class="fn">get_matched_indices</a>\<'a\>( &self, iter: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)\> + 'a\>, deleted_offset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#tymethod.get_matched_indices_with_limit_offset" class="fn">get_matched_indices_with_limit_offset</a>( &self, hash_values: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\], limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, offset: (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>), ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>)\>)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the join hash map contains no entries.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/trait.JoinHashMapType.html#implementors" class="anchor">§</a>
