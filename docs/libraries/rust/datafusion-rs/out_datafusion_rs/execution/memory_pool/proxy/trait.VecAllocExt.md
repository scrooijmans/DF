# Trait VecAllocExt Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/utils/proxy.rs.html#27" class="src">Source</a>

``` rust
pub trait VecAllocExt {
    type T;

    // Required methods
    fn push_accounted(&mut self, x: Self::T, accounting: &mut usize);
    fn allocated_size(&self) -> usize;
}
```

Expand description

Extension trait for [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") to account for allocations.

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#associatedtype.T" class="associatedtype">T</a>

Item type.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#tymethod.push_accounted" class="fn">push_accounted</a>(&mut self, x: Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#associatedtype.T" class="associatedtype" title="type datafusion::execution::memory_pool::proxy::VecAllocExt::T">T</a>, accounting: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

[Push](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.push "method alloc::vec::Vec::push") new element to vector and increase `accounting` by any newly allocated bytes.

Note that allocation counts capacity, not size

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#example" class="doc-anchor">§</a>Example:

``` rust
// use allocated to incrementally track how much memory is allocated in the vec
let mut allocated = 0;
let mut vec = Vec::new();
// Push data into the vec and the accounting will be updated to reflect
// memory allocation
vec.push_accounted(1, &mut allocated);
assert_eq!(allocated, 16); // space for 4 u32s
vec.push_accounted(1, &mut allocated);
assert_eq!(allocated, 16); // no new allocation needed

// push more data into the vec
for _ in 0..10 { vec.push_accounted(1, &mut allocated); }
assert_eq!(allocated, 64); // underlying vec has space for 10 u32s
assert_eq!(vec.allocated_size(), 64);
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#example-with-other-allocations" class="doc-anchor">§</a>Example with other allocations:

``` rust
// You can use the same allocated size to track memory allocated by
// another source. For example
let mut allocated = 27;
let mut vec = Vec::new();
vec.push_accounted(1, &mut allocated); // allocates 16 bytes for vec
assert_eq!(allocated, 43); // 16 bytes for vec, 27 bytes for other
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#tymethod.allocated_size" class="fn">allocated_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the amount of memory allocated by this Vec to store elements (`size_of<T> * capacity`).

Note this calculation is not recursive, and does not include any heap allocations contained within the Vec’s elements. Does not include the size of `self`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#example-1" class="doc-anchor">§</a>Example:

``` rust
let mut vec = Vec::new();
// Push data into the vec and the accounting will be updated to reflect
// memory allocation
vec.push(1);
assert_eq!(vec.allocated_size(), 16); // space for 4 u32s
vec.push(1);
assert_eq!(vec.allocated_size(), 16); // no new allocation needed

// push more data into the vec
for _ in 0..10 { vec.push(1); }
assert_eq!(vec.allocated_size(), 64); // space for 64 now
```

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#impl-VecAllocExt-for-Vec%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html" class="trait" title="trait datafusion::execution::memory_pool::proxy::VecAllocExt">VecAllocExt</a> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#associatedtype.T-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#associatedtype.T" class="associatedtype">T</a> = T

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#method.push_accounted" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#tymethod.push_accounted" class="fn">push_accounted</a>( &mut self, x: \<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html" class="trait" title="trait datafusion::execution::memory_pool::proxy::VecAllocExt">VecAllocExt</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#associatedtype.T" class="associatedtype" title="type datafusion::execution::memory_pool::proxy::VecAllocExt::T">T</a>, accounting: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, )

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#method.allocated_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#tymethod.allocated_size" class="fn">allocated_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html#implementors" class="anchor">§</a>
