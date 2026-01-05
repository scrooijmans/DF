# Struct Arc Copy item path

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#261-264" class="src">Source</a>

``` rust
pub struct Arc<T, A = Global>where
    A: Allocator,
    T: ?Sized,{ /* private fields */ }
```

Expand description

A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.

The type `Arc<T>` provides shared ownership of a value of type `T`, allocated in the heap. Invoking [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone") on `Arc` produces a new `Arc` instance, which points to the same allocation on the heap as the source `Arc`, while increasing a reference count. When the last `Arc` pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as “inner value”) is also dropped.

Shared references in Rust disallow mutation by default, and `Arc` is no exception: you cannot generally obtain a mutable reference to something inside an `Arc`. If you do need to mutate through an `Arc`, you have several options:

1.  Use interior mutability with synchronization primitives like [`Mutex`](https://docs.rs/polars/latest/std/sync/struct.Mutex.html), [`RwLock`](https://docs.rs/polars/latest/std/sync/struct.RwLock.html), or one of the [`Atomic`](https://doc.rust-lang.org/nightly/core/sync/atomic/index.html "mod core::sync::atomic") types.

2.  Use clone-on-write semantics with [`Arc::make_mut`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.make_mut "associated function polars::prelude::Arc::make_mut") which provides efficient mutation without requiring interior mutability. This approach clones the data only when needed (when there are multiple references) and can be more efficient when mutations are infrequent.

3.  Use [`Arc::get_mut`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_mut "associated function polars::prelude::Arc::get_mut") when you know your `Arc` is not shared (has a reference count of 1), which provides direct mutable access to the inner value without any cloning.

``` rust
use std::sync::Arc;

let mut data = Arc::new(vec![1, 2, 3]);

// This will clone the vector only if there are other references to it
Arc::make_mut(&mut data).push(4);

assert_eq!(*data, vec![1, 2, 3, 4]);
```

**Note**: This type is only available on platforms that support atomic loads and stores of pointers, which includes all platforms that support the `std` crate but not all those which only support [`alloc`](https://doc.rust-lang.org/nightly/alloc/index.html "mod alloc"). This may be detected at compile time using `#[cfg(target_has_atomic = "ptr")]`.

### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#thread-safety" class="doc-anchor">§</a>Thread Safety

Unlike [`Rc<T>`](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc"), `Arc<T>` uses atomic operations for its reference counting. This means that it is thread-safe. The disadvantage is that atomic operations are more expensive than ordinary memory accesses. If you are not sharing reference-counted allocations between threads, consider using [`Rc<T>`](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc") for lower overhead. [`Rc<T>`](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc") is a safe default, because the compiler will catch any attempt to send an [`Rc<T>`](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc") between threads. However, a library might choose `Arc<T>` in order to give library consumers more flexibility.

`Arc<T>` will implement [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") and [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") as long as the `T` implements [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") and [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"). Why can’t you put a non-thread-safe type `T` in an `Arc<T>` to make it thread-safe? This may be a bit counter-intuitive at first: after all, isn’t the point of `Arc<T>` thread safety? The key is this: `Arc<T>` makes it thread safe to have multiple ownership of the same data, but it doesn’t add thread safety to its data. Consider `Arc<`[`RefCell<T>`](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html "struct core::cell::RefCell")`>`. [`RefCell<T>`](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html "struct core::cell::RefCell") isn’t [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), and if `Arc<T>` was always [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), `Arc<`[`RefCell<T>`](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html "struct core::cell::RefCell")`>` would be as well. But then we’d have a problem: [`RefCell<T>`](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html "struct core::cell::RefCell") is not thread safe; it keeps track of the borrowing count using non-atomic operations.

In the end, this means that you may need to pair `Arc<T>` with some sort of [`std::sync`](https://docs.rs/polars/latest/std/sync/index.html) type, usually [`Mutex<T>`](https://docs.rs/polars/latest/std/sync/struct.Mutex.html).

### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#breaking-cycles-with-weak" class="doc-anchor">§</a>Breaking cycles with `Weak`

The [`downgrade`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.downgrade "associated function polars::prelude::Arc::downgrade") method can be used to create a non-owning [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointer. A [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointer can be [`upgrade`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html#method.upgrade "method alloc::sync::Weak::upgrade")d to an `Arc`, but this will return [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the value stored in the allocation has already been dropped. In other words, `Weak` pointers do not keep the value inside the allocation alive; however, they *do* keep the allocation (the backing store for the value) alive.

A cycle between `Arc` pointers will never be deallocated. For this reason, [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") is used to break cycles. For example, a tree could have strong `Arc` pointers from parent nodes to children, and [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers from children back to their parents.

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#cloning-references" class="doc-anchor">§</a>Cloning references

Creating a new reference from an existing reference-counted pointer is done using the `Clone` trait implemented for [`Arc<T>`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") and [`Weak<T>`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak").

``` rust
use std::sync::Arc;
let foo = Arc::new(vec![1.0, 2.0, 3.0]);
// The two syntaxes below are equivalent.
let a = foo.clone();
let b = Arc::clone(&foo);
// a, b, and foo are all Arcs that point to the same memory location
```

### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#deref-behavior" class="doc-anchor">§</a>`Deref` behavior

`Arc<T>` automatically dereferences to `T` (via the [`Deref`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") trait), so you can call `T`’s methods on a value of type `Arc<T>`. To avoid name clashes with `T`’s methods, the methods of `Arc<T>` itself are associated functions, called using [fully qualified syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name):

``` rust
use std::sync::Arc;

let my_arc = Arc::new(());
let my_weak = Arc::downgrade(&my_arc);
```

`Arc<T>`’s implementations of traits like `Clone` may also be called using fully qualified syntax. Some people prefer to use fully qualified syntax, while others prefer using method-call syntax.

``` rust
use std::sync::Arc;

let arc = Arc::new(());
// Method-call syntax
let arc2 = arc.clone();
// Fully qualified syntax
let arc3 = Arc::clone(&arc);
```

[`Weak<T>`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") does not auto-dereference to `T`, because the inner value may have already been dropped.

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples" class="doc-anchor">§</a>Examples

Sharing some immutable data between threads:

``` rust
use std::sync::Arc;
use std::thread;

let five = Arc::new(5);

for _ in 0..10 {
    let five = Arc::clone(&five);

    thread::spawn(move || {
        println!("{five:?}");
    });
}
```

Sharing a mutable [`AtomicUsize`](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicUsize.html "sync::atomic::AtomicUsize"):

``` rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let val = Arc::new(AtomicUsize::new(5));

for _ in 0..10 {
    let val = Arc::clone(&val);

    thread::spawn(move || {
        let v = val.fetch_add(1, Ordering::Relaxed);
        println!("{v:?}");
    });
}
```

See the [`rc` documentation](https://doc.rust-lang.org/nightly/alloc/rc/index.html#examples "mod alloc::rc") for more examples of reference counting in general.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#406" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new" class="fn">new</a>(data: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

Constructs a new `Arc<T>`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);
```

1.60.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#471-473" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_cyclic" class="fn">new_cyclic</a>\<F\>(data_fn: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html" class="struct" title="struct alloc::sync::Weak">Weak</a>\<T\>) -\> T,

Constructs a new `Arc<T>` while giving you a `Weak<T>` to the allocation, to allow you to construct a `T` which holds a weak pointer to itself.

Generally, a structure circularly referencing itself, either directly or indirectly, should not hold a strong reference to itself to prevent a memory leak. Using this function, you get access to the weak pointer during the initialization of `T`, before the `Arc<T>` is created, such that you can clone and store it inside the `T`.

`new_cyclic` first allocates the managed allocation for the `Arc<T>`, then calls your closure, giving it a `Weak<T>` to this allocation, and only afterwards completes the construction of the `Arc<T>` by placing the `T` returned from your closure into the allocation.

Since the new `Arc<T>` is not fully-constructed until `Arc<T>::new_cyclic` returns, calling [`upgrade`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html#method.upgrade "method alloc::sync::Weak::upgrade") on the weak reference inside your closure will fail and result in a `None` value.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#panics" class="doc-anchor">§</a>Panics

If `data_fn` panics, the panic is propagated to the caller, and the temporary [`Weak<T>`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") is dropped normally.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example" class="doc-anchor">§</a>Example

``` rust
use std::sync::{Arc, Weak};

struct Gadget {
    me: Weak<Gadget>,
}

impl Gadget {
    /// Constructs a reference counted Gadget.
    fn new() -> Arc<Self> {
        // `me` is a `Weak<Gadget>` pointing at the new allocation of the
        // `Arc` we're constructing.
        Arc::new_cyclic(|me| {
            // Create the actual struct here.
            Gadget { me: me.clone() }
        })
    }

    /// Returns a reference counted pointer to Self.
    fn me(&self) -> Arc<Self> {
        self.me.upgrade().unwrap()
    }
}
```

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#500" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_uninit" class="fn">new_uninit</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\>

Constructs a new `Arc` with uninitialized contents.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut five = Arc::<u32>::new_uninit();

// Deferred initialization:
Arc::get_mut(&mut five).unwrap().write(5);

let five = unsafe { five.assume_init() };

assert_eq!(*five, 5)
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_zeroed" class="fn">new_zeroed</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\>

🔬This is a nightly-only experimental API. (`new_zeroed_alloc`)

Constructs a new `Arc` with uninitialized contents, with the memory being filled with `0` bytes.

See [`MaybeUninit::zeroed`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.zeroed "associated function core::mem::maybe_uninit::MaybeUninit::zeroed") for examples of correct and incorrect usage of this method.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
#![feature(new_zeroed_alloc)]

use std::sync::Arc;

let zero = Arc::<u32>::new_zeroed();
let zero = unsafe { zero.assume_init() };

assert_eq!(*zero, 0)
```

1.33.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#549" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.pin" class="fn">pin</a>(data: T) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>\>

Constructs a new `Pin<Arc<T>>`. If `T` does not implement `Unpin`, then `data` will be pinned in memory and unable to be moved.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_pin" class="fn">try_pin</a>(data: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Pin<Arc<T>>`, return an error if allocation fails.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_new" class="fn">try_new</a>(data: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc<T>`, returning an error if allocation fails.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]
use std::sync::Arc;

let five = Arc::try_new(5)?;
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_new_uninit" class="fn">try_new_uninit</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc` with uninitialized contents, returning an error if allocation fails.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut five = Arc::<u32>::try_new_uninit()?;

// Deferred initialization:
Arc::get_mut(&mut five).unwrap().write(5);

let five = unsafe { five.assume_init() };

assert_eq!(*five, 5);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_new_zeroed" class="fn">try_new_zeroed</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc` with uninitialized contents, with the memory being filled with `0` bytes, returning an error if allocation fails.

See [`MaybeUninit::zeroed`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.zeroed "associated function core::mem::maybe_uninit::MaybeUninit::zeroed") for examples of correct and incorrect usage of this method.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-6" class="doc-anchor">§</a>Examples

``` rust
#![feature( allocator_api)]

use std::sync::Arc;

let zero = Arc::<u32>::try_new_zeroed()?;
let zero = unsafe { zero.assume_init() };

assert_eq!(*zero, 0);
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_in" class="fn">new_in</a>(data: T, alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc<T>` in the provided allocator.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-7" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let five = Arc::new_in(5, System);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_uninit_in" class="fn">new_uninit_in</a>(alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>, A\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc` with uninitialized contents in the provided allocator.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-8" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let mut five = Arc::<u32, _>::new_uninit_in(System);

let five = unsafe {
    // Deferred initialization:
    Arc::get_mut_unchecked(&mut five).as_mut_ptr().write(5);

    five.assume_init()
};

assert_eq!(*five, 5)
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_zeroed_in" class="fn">new_zeroed_in</a>(alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>, A\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc` with uninitialized contents, with the memory being filled with `0` bytes, in the provided allocator.

See [`MaybeUninit::zeroed`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.zeroed "associated function core::mem::maybe_uninit::MaybeUninit::zeroed") for examples of correct and incorrect usage of this method.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-9" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let zero = Arc::<u32, _>::new_zeroed_in(System);
let zero = unsafe { zero.assume_init() };

assert_eq!(*zero, 0)
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_cyclic_in" class="fn">new_cyclic_in</a>\<F\>(data_fn: F, alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html" class="struct" title="struct alloc::sync::Weak">Weak</a>\<T, A\>) -\> T,

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc<T, A>` in the given allocator while giving you a `Weak<T, A>` to the allocation, to allow you to construct a `T` which holds a weak pointer to itself.

Generally, a structure circularly referencing itself, either directly or indirectly, should not hold a strong reference to itself to prevent a memory leak. Using this function, you get access to the weak pointer during the initialization of `T`, before the `Arc<T, A>` is created, such that you can clone and store it inside the `T`.

`new_cyclic_in` first allocates the managed allocation for the `Arc<T, A>`, then calls your closure, giving it a `Weak<T, A>` to this allocation, and only afterwards completes the construction of the `Arc<T, A>` by placing the `T` returned from your closure into the allocation.

Since the new `Arc<T, A>` is not fully-constructed until `Arc<T, A>::new_cyclic_in` returns, calling [`upgrade`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html#method.upgrade "method alloc::sync::Weak::upgrade") on the weak reference inside your closure will fail and result in a `None` value.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#panics-1" class="doc-anchor">§</a>Panics

If `data_fn` panics, the panic is propagated to the caller, and the temporary [`Weak<T>`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") is dropped normally.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-1" class="doc-anchor">§</a>Example

See [`new_cyclic`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_cyclic "associated function polars::prelude::Arc::new_cyclic")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.pin_in" class="fn">pin_in</a>(data: T, alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>\>

where A: 'static,

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Pin<Arc<T, A>>` in the provided allocator. If `T` does not implement `Unpin`, then `data` will be pinned in memory and unable to be moved.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_pin_in" class="fn">try_pin_in</a>(data: T, alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

where A: 'static,

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Pin<Arc<T, A>>` in the provided allocator, return an error if allocation fails.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_new_in" class="fn">try_new_in</a>(data: T, alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc<T, A>` in the provided allocator, returning an error if allocation fails.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-10" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let five = Arc::try_new_in(5, System)?;
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_new_uninit_in" class="fn">try_new_uninit_in</a>(alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>, A\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc` with uninitialized contents, in the provided allocator, returning an error if allocation fails.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-11" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]

use std::sync::Arc;
use std::alloc::System;

let mut five = Arc::<u32, _>::try_new_uninit_in(System)?;

let five = unsafe {
    // Deferred initialization:
    Arc::get_mut_unchecked(&mut five).as_mut_ptr().write(5);

    five.assume_init()
};

assert_eq!(*five, 5);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_new_zeroed_in" class="fn">try_new_zeroed_in</a>(alloc: A) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>, A\>, <a href="https://doc.rust-lang.org/nightly/core/alloc/struct.AllocError.html" class="struct" title="struct core::alloc::AllocError">AllocError</a>\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new `Arc` with uninitialized contents, with the memory being filled with `0` bytes, in the provided allocator, returning an error if allocation fails.

See [`MaybeUninit::zeroed`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.zeroed "associated function core::mem::maybe_uninit::MaybeUninit::zeroed") for examples of correct and incorrect usage of this method.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-12" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let zero = Arc::<u32, _>::try_new_zeroed_in(System)?;
let zero = unsafe { zero.assume_init() };

assert_eq!(*zero, 0);
```

1.4.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1018" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_unwrap" class="fn">try_unwrap</a>(this: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>\>

Returns the inner value, if the `Arc` has exactly one strong reference.

Otherwise, an [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned with the same `Arc` that was passed in.

This will succeed even if there are outstanding weak references.

It is strongly recommended to use [`Arc::into_inner`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_inner "associated function polars::prelude::Arc::into_inner") instead if you don’t keep the `Arc` in the [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") case. Immediately dropping the [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err")-value, as the expression `Arc::try_unwrap(this).ok()` does, can cause the strong count to drop to zero and the inner value of the `Arc` to be dropped. For instance, if two threads execute such an expression in parallel, there is a race condition without the possibility of unsafety: The threads could first both check whether they own the last instance in `Arc::try_unwrap`, determine that they both do not, and then both discard and drop their instance in the call to [`ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.ok "method core::result::Result::ok"). In this scenario, the value inside the `Arc` is safely destroyed by exactly one of the threads, but neither thread will ever be able to use the value.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-13" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let x = Arc::new(3);
assert_eq!(Arc::try_unwrap(x), Ok(3));

let x = Arc::new(4);
let _y = Arc::clone(&x);
assert_eq!(*Arc::try_unwrap(x).unwrap_err(), 4);
```

1.70.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1133" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_inner" class="fn">into_inner</a>(this: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

Returns the inner value, if the `Arc` has exactly one strong reference.

Otherwise, [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") is returned and the `Arc` is dropped.

This will succeed even if there are outstanding weak references.

If `Arc::into_inner` is called on every clone of this `Arc`, it is guaranteed that exactly one of the calls returns the inner value. This means in particular that the inner value is not dropped.

[`Arc::try_unwrap`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_unwrap "associated function polars::prelude::Arc::try_unwrap") is conceptually similar to `Arc::into_inner`, but it is meant for different use-cases. If used as a direct replacement for `Arc::into_inner` anyway, such as with the expression [`Arc::try_unwrap`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_unwrap "associated function polars::prelude::Arc::try_unwrap")`(this).`[`ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.ok "method core::result::Result::ok")`()`, then it does **not** give the same guarantee as described in the previous paragraph. For more information, see the examples below and read the documentation of [`Arc::try_unwrap`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_unwrap "associated function polars::prelude::Arc::try_unwrap").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-14" class="doc-anchor">§</a>Examples

Minimal example demonstrating the guarantee that `Arc::into_inner` gives.

``` rust
use std::sync::Arc;

let x = Arc::new(3);
let y = Arc::clone(&x);

// Two threads calling `Arc::into_inner` on both clones of an `Arc`:
let x_thread = std::thread::spawn(|| Arc::into_inner(x));
let y_thread = std::thread::spawn(|| Arc::into_inner(y));

let x_inner_value = x_thread.join().unwrap();
let y_inner_value = y_thread.join().unwrap();

// One of the threads is guaranteed to receive the inner value:
assert!(matches!(
    (x_inner_value, y_inner_value),
    (None, Some(3)) | (Some(3), None)
));
// The result could also be `(None, None)` if the threads called
// `Arc::try_unwrap(x).ok()` and `Arc::try_unwrap(y).ok()` instead.
```

A more practical example demonstrating the need for `Arc::into_inner`:

``` rust
use std::sync::Arc;

// Definition of a simple singly linked list using `Arc`:
#[derive(Clone)]
struct LinkedList<T>(Option<Arc<Node<T>>>);
struct Node<T>(T, Option<Arc<Node<T>>>);

// Dropping a long `LinkedList<T>` relying on the destructor of `Arc`
// can cause a stack overflow. To prevent this, we can provide a
// manual `Drop` implementation that does the destruction in a loop:
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.0.take();
        while let Some(arc_node) = link.take() {
            if let Some(Node(_value, next)) = Arc::into_inner(arc_node) {
                link = next;
            }
        }
    }
}

// Implementation of `new` and `push` omitted
impl<T> LinkedList<T> {
    /* ... */
}

// The following code could have still caused a stack overflow
// despite the manual `Drop` impl if that `Drop` impl had used
// `Arc::try_unwrap(arc).ok()` instead of `Arc::into_inner(arc)`.

// Create a long list and clone it
let mut x = LinkedList::new();
let size = 100000;
for i in 0..size {
    x.push(i); // Adds i to the front of x
}
let y = x.clone();

// Drop the clones in parallel
let x_thread = std::thread::spawn(|| drop(x));
let y_thread = std::thread::spawn(|| drop(y));
x_thread.join().unwrap();
y_thread.join().unwrap();
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1187" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_uninit_slice" class="fn">new_uninit_slice</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\]\>

Constructs a new atomically reference-counted slice with uninitialized contents.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-15" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut values = Arc::<[u32]>::new_uninit_slice(3);

// Deferred initialization:
let data = Arc::get_mut(&mut values).unwrap();
data[0].write(1);
data[1].write(2);
data[2].write(3);

let values = unsafe { values.assume_init() };

assert_eq!(*values, [1, 2, 3])
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_zeroed_slice" class="fn">new_zeroed_slice</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\]\>

🔬This is a nightly-only experimental API. (`new_zeroed_alloc`)

Constructs a new atomically reference-counted slice with uninitialized contents, with the memory being filled with `0` bytes.

See [`MaybeUninit::zeroed`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.zeroed "associated function core::mem::maybe_uninit::MaybeUninit::zeroed") for examples of correct and incorrect usage of this method.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-16" class="doc-anchor">§</a>Examples

``` rust
#![feature(new_zeroed_alloc)]

use std::sync::Arc;

let values = Arc::<[u32]>::new_zeroed_slice(3);
let values = unsafe { values.assume_init() };

assert_eq!(*values, [0, 0, 0])
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_array" class="fn">into_array</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\>\>

🔬This is a nightly-only experimental API. (`slice_as_array`)

Converts the reference-counted slice into a reference-counted array.

This operation does not reallocate; the underlying array of the slice is simply reinterpreted as an array type.

If `N` is not exactly equal to the length of `self`, then this method returns `None`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3C%5BT%5D,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_uninit_slice_in" class="fn">new_uninit_slice_in</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\], A\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new atomically reference-counted slice with uninitialized contents in the provided allocator.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-17" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let mut values = Arc::<[u32], _>::new_uninit_slice_in(3, System);

let values = unsafe {
    // Deferred initialization:
    Arc::get_mut_unchecked(&mut values)[0].as_mut_ptr().write(1);
    Arc::get_mut_unchecked(&mut values)[1].as_mut_ptr().write(2);
    Arc::get_mut_unchecked(&mut values)[2].as_mut_ptr().write(3);

    values.assume_init()
};

assert_eq!(*values, [1, 2, 3])
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_zeroed_slice_in" class="fn">new_zeroed_slice_in</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\], A\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs a new atomically reference-counted slice with uninitialized contents, with the memory being filled with `0` bytes, in the provided allocator.

See [`MaybeUninit::zeroed`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.zeroed "associated function core::mem::maybe_uninit::MaybeUninit::zeroed") for examples of correct and incorrect usage of this method.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-18" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let values = Arc::<[u32], _>::new_zeroed_slice_in(3, System);
let values = unsafe { values.assume_init() };

assert_eq!(*values, [0, 0, 0])
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CMaybeUninit%3CT%3E,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1355" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.assume_init" class="fn">assume_init</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

Converts to `Arc<T>`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety" class="doc-anchor">§</a>Safety

As with [`MaybeUninit::assume_init`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.assume_init "method core::mem::maybe_uninit::MaybeUninit::assume_init"), it is up to the caller to guarantee that the inner value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-19" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut five = Arc::<u32>::new_uninit();

// Deferred initialization:
Arc::get_mut(&mut five).unwrap().write(5);

let five = unsafe { five.assume_init() };

assert_eq!(*five, 5)
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3C%5BMaybeUninit%3CT%3E%5D,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html" class="union" title="union core::mem::maybe_uninit::MaybeUninit">MaybeUninit</a>\<T\>\], A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1396" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.assume_init-1" class="fn">assume_init</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>

Converts to `Arc<[T]>`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-1" class="doc-anchor">§</a>Safety

As with [`MaybeUninit::assume_init`](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html#method.assume_init "method core::mem::maybe_uninit::MaybeUninit::assume_init"), it is up to the caller to guarantee that the inner value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-20" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut values = Arc::<[u32]>::new_uninit_slice(3);

// Deferred initialization:
let data = Arc::get_mut(&mut values).unwrap();
data[0].write(1);
data[1].write(2);
data[2].write(3);

let values = unsafe { values.assume_init() };

assert_eq!(*values, [1, 2, 3])
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT%3E-1" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

1.17.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1466" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw" class="fn">from_raw</a>(ptr: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

Constructs an `Arc<T>` from a raw pointer.

The raw pointer must have been previously returned by a call to [`Arc<U>::into_raw`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_raw "associated function polars::prelude::Arc::into_raw") with the following requirements:

- If `U` is sized, it must have the same size and alignment as `T`. This is trivially true if `U` is `T`.
- If `U` is unsized, its data pointer must have the same size and alignment as `T`. This is trivially true if `Arc<U>` was constructed through `Arc<T>` and then converted to `Arc<U>` through an [unsized coercion](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions).

Note that if `U` or `U`’s data pointer is not `T` but has the same size and alignment, this is basically like transmuting references of different types. See [`mem::transmute`](https://doc.rust-lang.org/nightly/core/intrinsics/fn.transmute.html "fn core::intrinsics::transmute") for more information on what restrictions apply in this case.

The raw pointer must point to a block of memory allocated by the global allocator.

The user of `from_raw` has to make sure a specific value of `T` is only dropped once.

This function is unsafe because improper use may lead to memory unsafety, even if the returned `Arc<T>` is never accessed.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-21" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let x = Arc::new("hello".to_owned());
let x_ptr = Arc::into_raw(x);

unsafe {
    // Convert back to an `Arc` to prevent leak.
    let x = Arc::from_raw(x_ptr);
    assert_eq!(&*x, "hello");

    // Further calls to `Arc::from_raw(x_ptr)` would be memory-unsafe.
}

// The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
```

Convert a slice back into its original array:

``` rust
use std::sync::Arc;

let x: Arc<[u32]> = Arc::new([1, 2, 3]);
let x_ptr: *const [u32] = Arc::into_raw(x);

unsafe {
    let x: Arc<[u32; 3]> = Arc::from_raw(x_ptr.cast::<[u32; 3]>());
    assert_eq!(&*x, &[1, 2, 3]);
}
```

1.17.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1489" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_raw" class="fn">into_raw</a>(this: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>

Consumes the `Arc`, returning the wrapped pointer.

To avoid a memory leak the pointer must be converted back to an `Arc` using [`Arc::from_raw`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw "associated function polars::prelude::Arc::from_raw").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-22" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let x = Arc::new("hello".to_owned());
let x_ptr = Arc::into_raw(x);
assert_eq!(unsafe { &*x_ptr }, "hello");
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1528" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.increment_strong_count" class="fn">increment_strong_count</a>(ptr: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>)

Increments the strong reference count on the `Arc<T>` associated with the provided pointer by one.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-2" class="doc-anchor">§</a>Safety

The pointer must have been obtained through `Arc::into_raw` and must satisfy the same layout requirements specified in [`Arc::from_raw_in`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw_in "associated function polars::prelude::Arc::from_raw_in"). The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) for the duration of this method, and `ptr` must point to a block of memory allocated by the global allocator.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-23" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

unsafe {
    let ptr = Arc::into_raw(five);
    Arc::increment_strong_count(ptr);

    // This assertion is deterministic because we haven't shared
    // the `Arc` between threads.
    let five = Arc::from_raw(ptr);
    assert_eq!(2, Arc::strong_count(&five));
}
```

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1568" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.decrement_strong_count" class="fn">decrement_strong_count</a>(ptr: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>)

Decrements the strong reference count on the `Arc<T>` associated with the provided pointer by one.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-3" class="doc-anchor">§</a>Safety

The pointer must have been obtained through `Arc::into_raw` and must satisfy the same layout requirements specified in [`Arc::from_raw_in`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw_in "associated function polars::prelude::Arc::from_raw_in"). The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) when invoking this method, and `ptr` must point to a block of memory allocated by the global allocator. This method can be used to release the final `Arc` and backing storage, but **should not** be called after the final `Arc` has been released.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-24" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

unsafe {
    let ptr = Arc::into_raw(five);
    Arc::increment_strong_count(ptr);

    // Those assertions are deterministic because we haven't shared
    // the `Arc` between threads.
    let five = Arc::from_raw(ptr);
    assert_eq!(2, Arc::strong_count(&five));
    Arc::decrement_strong_count(ptr);
    assert_eq!(1, Arc::strong_count(&five));
}
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT,+A%3E-1" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.allocator" class="fn">allocator</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;A</a>

🔬This is a nightly-only experimental API. (`allocator_api`)

Returns a reference to the underlying allocator.

Note: this is an associated function, which means that you have to call it as `Arc::allocator(&a)` instead of `a.allocator()`. This is so that there is no conflict with a method on the inner type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_raw_with_allocator" class="fn">into_raw_with_allocator</a>(this: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>, A)

🔬This is a nightly-only experimental API. (`allocator_api`)

Consumes the `Arc`, returning the wrapped pointer and allocator.

To avoid a memory leak the pointer must be converted back to an `Arc` using [`Arc::from_raw_in`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw_in "associated function polars::prelude::Arc::from_raw_in").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-25" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]
use std::sync::Arc;
use std::alloc::System;

let x = Arc::new_in("hello".to_owned(), System);
let (ptr, alloc) = Arc::into_raw_with_allocator(x);
assert_eq!(unsafe { &*ptr }, "hello");
let x = unsafe { Arc::from_raw_in(ptr, alloc) };
assert_eq!(&*x, "hello");
```

1.45.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1632" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.as_ptr" class="fn">as_ptr</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>

Provides a raw pointer to the data.

The counts are not affected in any way and the `Arc` is not consumed. The pointer is valid for as long as there are strong counts in the `Arc`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-26" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let x = Arc::new("hello".to_owned());
let y = Arc::clone(&x);
let x_ptr = Arc::as_ptr(&x);
assert_eq!(x_ptr, Arc::as_ptr(&y));
assert_eq!(unsafe { &*x_ptr }, "hello");
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw_in" class="fn">from_raw_in</a>(ptr: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>, alloc: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

🔬This is a nightly-only experimental API. (`allocator_api`)

Constructs an `Arc<T, A>` from a raw pointer.

The raw pointer must have been previously returned by a call to [`Arc<U, A>::into_raw`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_raw "associated function polars::prelude::Arc::into_raw") with the following requirements:

- If `U` is sized, it must have the same size and alignment as `T`. This is trivially true if `U` is `T`.
- If `U` is unsized, its data pointer must have the same size and alignment as `T`. This is trivially true if `Arc<U>` was constructed through `Arc<T>` and then converted to `Arc<U>` through an [unsized coercion](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions).

Note that if `U` or `U`’s data pointer is not `T` but has the same size and alignment, this is basically like transmuting references of different types. See [`mem::transmute`](https://doc.rust-lang.org/nightly/core/intrinsics/fn.transmute.html "fn core::intrinsics::transmute") for more information on what restrictions apply in this case.

The raw pointer must point to a block of memory allocated by `alloc`

The user of `from_raw` has to make sure a specific value of `T` is only dropped once.

This function is unsafe because improper use may lead to memory unsafety, even if the returned `Arc<T>` is never accessed.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-27" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let x = Arc::new_in("hello".to_owned(), System);
let (x_ptr, alloc) = Arc::into_raw_with_allocator(x);

unsafe {
    // Convert back to an `Arc` to prevent leak.
    let x = Arc::from_raw_in(x_ptr, System);
    assert_eq!(&*x, "hello");

    // Further calls to `Arc::from_raw(x_ptr)` would be memory-unsafe.
}

// The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
```

Convert a slice back into its original array:

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let x: Arc<[u32], _> = Arc::new_in([1, 2, 3], System);
let x_ptr: *const [u32] = Arc::into_raw_with_allocator(x).0;

unsafe {
    let x: Arc<[u32; 3], _> = Arc::from_raw_in(x_ptr.cast::<[u32; 3]>(), System);
    assert_eq!(&*x, &[1, 2, 3]);
}
```

1.4.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1735-1737" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.downgrade" class="fn">downgrade</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html" class="struct" title="struct alloc::sync::Weak">Weak</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Creates a new [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointer to this allocation.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-28" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

let weak_five = Arc::downgrade(&five);
```

1.15.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1795" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.weak_count" class="fn">weak_count</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Gets the number of [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers to this allocation.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-4" class="doc-anchor">§</a>Safety

This method by itself is safe, but using it correctly requires extra care. Another thread can change the weak count at any time, including potentially between calling this method and acting on the result.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-29" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);
let _weak_five = Arc::downgrade(&five);

// This assertion is deterministic because we haven't shared
// the `Arc` or `Weak` between threads.
assert_eq!(1, Arc::weak_count(&five));
```

1.15.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1825" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.strong_count" class="fn">strong_count</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Gets the number of strong (`Arc`) pointers to this allocation.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-5" class="doc-anchor">§</a>Safety

This method by itself is safe, but using it correctly requires extra care. Another thread can change the strong count at any time, including potentially between calling this method and acting on the result.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-30" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);
let _also_five = Arc::clone(&five);

// This assertion is deterministic because we haven't shared
// the `Arc` between threads.
assert_eq!(2, Arc::strong_count(&five));
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.increment_strong_count_in" class="fn">increment_strong_count_in</a>(ptr: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>, alloc: A)

where A: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

🔬This is a nightly-only experimental API. (`allocator_api`)

Increments the strong reference count on the `Arc<T>` associated with the provided pointer by one.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-6" class="doc-anchor">§</a>Safety

The pointer must have been obtained through `Arc::into_raw` and must satisfy the same layout requirements specified in [`Arc::from_raw_in`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw_in "associated function polars::prelude::Arc::from_raw_in"). The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) for the duration of this method, and `ptr` must point to a block of memory allocated by `alloc`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-31" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let five = Arc::new_in(5, System);

unsafe {
    let (ptr, _alloc) = Arc::into_raw_with_allocator(five);
    Arc::increment_strong_count_in(ptr, System);

    // This assertion is deterministic because we haven't shared
    // the `Arc` between threads.
    let five = Arc::from_raw_in(ptr, System);
    assert_eq!(2, Arc::strong_count(&five));
}
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.decrement_strong_count_in" class="fn">decrement_strong_count_in</a>(ptr: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const T</a>, alloc: A)

🔬This is a nightly-only experimental API. (`allocator_api`)

Decrements the strong reference count on the `Arc<T>` associated with the provided pointer by one.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-7" class="doc-anchor">§</a>Safety

The pointer must have been obtained through `Arc::into_raw` and must satisfy the same layout requirements specified in [`Arc::from_raw_in`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_raw_in "associated function polars::prelude::Arc::from_raw_in"). The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) when invoking this method, and `ptr` must point to a block of memory allocated by `alloc`. This method can be used to release the final `Arc` and backing storage, but **should not** be called after the final `Arc` has been released.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-32" class="doc-anchor">§</a>Examples

``` rust
#![feature(allocator_api)]

use std::sync::Arc;
use std::alloc::System;

let five = Arc::new_in(5, System);

unsafe {
    let (ptr, _alloc) = Arc::into_raw_with_allocator(five);
    Arc::increment_strong_count_in(ptr, System);

    // Those assertions are deterministic because we haven't shared
    // the `Arc` between threads.
    let five = Arc::from_raw_in(ptr, System);
    assert_eq!(2, Arc::strong_count(&five));
    Arc::decrement_strong_count_in(ptr, System);
    assert_eq!(1, Arc::strong_count(&five));
}
```

1.17.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#1965" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.ptr_eq" class="fn">ptr_eq</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the two `Arc`s point to the same allocation in a vein similar to [`ptr::eq`](https://doc.rust-lang.org/nightly/core/ptr/fn.eq.html "ptr::eq"). This function ignores the metadata of `dyn Trait` pointers.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-33" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);
let same_five = Arc::clone(&five);
let other_five = Arc::new(5);

assert!(Arc::ptr_eq(&five, &same_five));
assert!(!Arc::ptr_eq(&five, &other_five));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT,+A%3E-2" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html" class="trait" title="trait core::clone::CloneToUninit">CloneToUninit</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

1.4.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2311" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.make_mut" class="fn">make_mut</a>(this: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

Makes a mutable reference into the given `Arc`.

If there are other `Arc` pointers to the same allocation, then `make_mut` will [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone") the inner value to a new allocation to ensure unique ownership. This is also referred to as clone-on-write.

However, if there are no other `Arc` pointers to this allocation, but some [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers, then the [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers will be dissociated and the inner value will not be cloned.

See also [`get_mut`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_mut "associated function polars::prelude::Arc::get_mut"), which will fail rather than cloning the inner value or dissociating [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-34" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let mut data = Arc::new(5);

*Arc::make_mut(&mut data) += 1;         // Won't clone anything
let mut other_data = Arc::clone(&data); // Won't clone inner data
*Arc::make_mut(&mut data) += 1;         // Clones inner data
*Arc::make_mut(&mut data) += 1;         // Won't clone anything
*Arc::make_mut(&mut other_data) *= 2;   // Won't clone anything

// Now `data` and `other_data` point to different allocations.
assert_eq!(*data, 8);
assert_eq!(*other_data, 12);
```

[`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers will be dissociated:

``` rust
use std::sync::Arc;

let mut data = Arc::new(75);
let weak = Arc::downgrade(&data);

assert!(75 == *data);
assert!(75 == *weak.upgrade().unwrap());

*Arc::make_mut(&mut data) += 1;

assert!(76 == *data);
assert!(weak.upgrade().is_none());
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT,+A%3E-3" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

1.76.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2415" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.unwrap_or_clone" class="fn">unwrap_or_clone</a>(this: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> T

If we have the only reference to `T` then unwrap it. Otherwise, clone `T` and return the clone.

Assuming `arc_t` is of type `Arc<T>`, this function is functionally equivalent to `(*arc_t).clone()`, but will avoid cloning the inner value where possible.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-35" class="doc-anchor">§</a>Examples

``` rust
let inner = String::from("test");
let ptr = inner.as_ptr();

let arc = Arc::new(inner);
let inner = Arc::unwrap_or_clone(arc);
// The inner value was not cloned
assert!(ptr::eq(ptr, inner.as_ptr()));

let arc = Arc::new(inner);
let arc2 = arc.clone();
let inner = Arc::unwrap_or_clone(arc);
// Because there were 2 references, we had to clone the inner value.
assert!(!ptr::eq(ptr, inner.as_ptr()));
// `arc2` is the last reference, so when we unwrap it we get back
// the original `String`.
let inner = Arc::unwrap_or_clone(arc2);
assert!(ptr::eq(ptr, inner.as_ptr()));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3CT,+A%3E-4" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

1.4.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2447" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_mut" class="fn">get_mut</a>(this: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

Returns a mutable reference into the given `Arc`, if there are no other `Arc` or [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers to the same allocation.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") otherwise, because it is not safe to mutate a shared value.

See also [`make_mut`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.make_mut "associated function polars::prelude::Arc::make_mut"), which will [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone") the inner value when there are other `Arc` pointers.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-36" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let mut x = Arc::new(3);
*Arc::get_mut(&mut x).unwrap() = 4;
assert_eq!(*x, 4);

let _y = Arc::clone(&x);
assert!(Arc::get_mut(&mut x).is_none());
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_mut_unchecked" class="fn">get_mut_unchecked</a>(this: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

🔬This is a nightly-only experimental API. (`get_mut_unchecked`)

Returns a mutable reference into the given `Arc`, without any check.

See also [`get_mut`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_mut "associated function polars::prelude::Arc::get_mut"), which is safe and does appropriate checks.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-8" class="doc-anchor">§</a>Safety

If any other `Arc` or [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers to the same allocation exist, then they must not be dereferenced or have active borrows for the duration of the returned borrow, and their inner type must be exactly the same as the inner type of this Rc (including lifetimes). This is trivially the case if no such pointers exist, for example immediately after `Arc::new`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-37" class="doc-anchor">§</a>Examples

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let mut x = Arc::new(String::new());
unsafe {
    Arc::get_mut_unchecked(&mut x).push_str("foo")
}
assert_eq!(*x, "foo");
```

Other `Arc` pointers to the same allocation must be to the same type.

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let x: Arc<str> = Arc::from("Hello, world!");
let mut y: Arc<[u8]> = x.clone().into();
unsafe {
    // this is Undefined Behavior, because x's inner type is str, not [u8]
    Arc::get_mut_unchecked(&mut y).fill(0xff); // 0xff is invalid in UTF-8
}
println!("{}", &*x); // Invalid UTF-8 in a str
```

Other `Arc` pointers to the same allocation must be to the exact same type, including lifetimes.

``` rust
#![feature(get_mut_unchecked)]

use std::sync::Arc;

let x: Arc<&str> = Arc::new("Hello, world!");
{
    let s = String::from("Oh, no!");
    let mut y: Arc<&str> = x.clone();
    unsafe {
        // this is Undefined Behavior, because x's inner type
        // is &'long str, not &'short str
        *Arc::get_mut_unchecked(&mut y) = &s;
    }
}
println!("{}", &*x); // Use-after-free
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.is_unique" class="fn">is_unique</a>(this: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

🔬This is a nightly-only experimental API. (`arc_is_unique`)

Determine whether this is the unique reference to the underlying data.

Returns `true` if there are no other `Arc` or [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak") pointers to the same allocation; returns `false` otherwise.

If this function returns `true`, then is guaranteed to be safe to call [`get_mut_unchecked`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_mut_unchecked "associated function polars::prelude::Arc::get_mut_unchecked") on this `Arc`, so long as no clones occur in between.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-38" class="doc-anchor">§</a>Examples

``` rust
#![feature(arc_is_unique)]

use std::sync::Arc;

let x = Arc::new(3);
assert!(Arc::is_unique(&x));

let y = Arc::clone(&x);
assert!(!Arc::is_unique(&x));
drop(y);

// Weak references also count, because they could be upgraded at any time.
let z = Arc::downgrade(&x);
assert!(!Arc::is_unique(&x));
```

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#pointer-invalidation" class="doc-anchor">§</a>Pointer invalidation

This function will always return the same value as `Arc::get_mut(arc).is_some()`. However, unlike that operation it does not produce any mutable references to the underlying data, meaning no pointers to the data inside the `Arc` are invalidated by the call. Thus, the following code is valid, even though it would be UB if it used `Arc::get_mut`:

``` rust
#![feature(arc_is_unique)]

use std::sync::Arc;

let arc = Arc::new(5);
let pointer: *const i32 = &*arc;
assert!(Arc::is_unique(&arc));
assert_eq!(unsafe { *pointer }, 5);
```

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#atomic-orderings" class="doc-anchor">§</a>Atomic orderings

Concurrent drops to other `Arc` pointers to the same allocation will synchronize with this call - that is, this call performs an `Acquire` operation on the underlying strong and weak ref counts. This ensures that calling `get_mut_unchecked` is safe.

Note that this operation requires locking the weak ref count, so concurrent calls to `downgrade` may spin-loop for a short period of time.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Arc%3Cdyn+Any+%2B+Send+%2B+Sync,+A%3E" class="anchor">§</a>

### impl\<A\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

1.29.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2711-2713" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.downcast" class="fn">downcast</a>\<T\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, A\>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Attempts to downcast the `Arc<dyn Any + Send + Sync>` to a concrete type.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-39" class="doc-anchor">§</a>Examples

``` rust
use std::any::Any;
use std::sync::Arc;

fn print_if_string(value: Arc<dyn Any + Send + Sync>) {
    if let Ok(string) = value.downcast::<String>() {
        println!("String ({}): {}", string.len(), string);
    }
}

let my_string = "Hello World".to_string();
print_if_string(Arc::new(my_string));
print_if_string(Arc::new(0i8));
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.downcast_unchecked" class="fn">downcast_unchecked</a>\<T\>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

🔬This is a nightly-only experimental API. (`downcast_unchecked`)

Downcasts the `Arc<dyn Any + Send + Sync>` to a concrete type.

For a safe alternative see [`downcast`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.downcast "method polars::prelude::Arc::downcast").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-40" class="doc-anchor">§</a>Examples

``` rust
#![feature(downcast_unchecked)]

use std::any::Any;
use std::sync::Arc;

let x: Arc<dyn Any + Send + Sync> = Arc::new(1_usize);

unsafe {
    assert_eq!(*x.downcast_unchecked::<usize>(), 1);
}
```

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#safety-9" class="doc-anchor">§</a>Safety

The contained value must be of type `T`. Calling this method with the incorrect type is *undefined behavior*.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#trait-implementations" class="anchor">§</a>

1.64.0 · <a href="https://doc.rust-lang.org/nightly/src/std/os/fd/owned.rs.html#426" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-AsFd-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/std/os/fd/owned/trait.AsFd.html" class="trait" title="trait std::os::fd::owned::AsFd">AsFd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/std/os/fd/owned/trait.AsFd.html" class="trait" title="trait std::os::fd::owned::AsFd">AsFd</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

This impl allows implementing traits that require `AsFd` on Arc.

``` rust
use std::net::UdpSocket;
use std::sync::Arc;

trait MyTrait: AsFd {}
impl MyTrait for Arc<UdpSocket> {}
impl MyTrait for Box<UdpSocket> {}
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.as_fd" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/os/fd/owned/trait.AsFd.html#tymethod.as_fd" class="fn">as_fd</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/os/fd/owned/struct.BorrowedFd.html" class="struct" title="struct std::os::fd::owned::BorrowedFd">BorrowedFd</a>\<'\_\>

Borrows the file descriptor. [Read more](https://doc.rust-lang.org/nightly/std/os/fd/owned/trait.AsFd.html#tymethod.as_fd)

1.63.0 · <a href="https://doc.rust-lang.org/nightly/src/std/os/fd/raw.rs.html#257" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-AsRawFd-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.AsRawFd.html" class="trait" title="trait std::os::fd::raw::AsRawFd">AsRawFd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.AsRawFd.html" class="trait" title="trait std::os::fd::raw::AsRawFd">AsRawFd</a>,

This impl allows implementing traits that require `AsRawFd` on Arc.

``` rust
use std::net::UdpSocket;
use std::sync::Arc;
trait MyTrait: AsRawFd {
}
impl MyTrait for Arc<UdpSocket> {}
impl MyTrait for Box<UdpSocket> {}
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.as_raw_fd" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.AsRawFd.html#tymethod.as_raw_fd" class="fn">as_raw_fd</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Extracts the raw file descriptor. [Read more](https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.AsRawFd.html#tymethod.as_raw_fd)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#4020" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-AsRef%3CT%3E-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Converts this type into a shared reference of the (usually inferred) input type.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#4013" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Borrow%3CT%3E-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.borrow" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow" class="fn">borrow</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-BorrowDecode%3C&#39;de,+Context%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<'de, T, Context\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html" class="trait" title="trait bincode::de::BorrowDecode">BorrowDecode</a>\<'de, Context\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where T: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html" class="trait" title="trait bincode::de::BorrowDecode">BorrowDecode</a>\<'de, Context\> + 'de,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.borrow_decode-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html#tymethod.borrow_decode" class="fn">borrow_decode</a>\<D\>(decoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut D</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.DecodeError.html" class="enum" title="enum bincode::error::DecodeError">DecodeError</a>\>

where D: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecoder.html" class="trait" title="trait bincode::de::BorrowDecoder">BorrowDecoder</a>\<'de, Context = Context\>,

Attempt to decode this type with the given [BorrowDecode](https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html "trait bincode::de::BorrowDecode").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-BorrowDecode%3C&#39;de,+Context%3E-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<'de, T, Context\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html" class="trait" title="trait bincode::de::BorrowDecode">BorrowDecode</a>\<'de, Context\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html" class="trait" title="trait bincode::de::BorrowDecode">BorrowDecode</a>\<'de, Context\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.borrow_decode" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html#tymethod.borrow_decode" class="fn">borrow_decode</a>\<D\>(decoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut D</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.DecodeError.html" class="enum" title="enum bincode::error::DecodeError">DecodeError</a>\>

where D: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecoder.html" class="trait" title="trait bincode::de::BorrowDecoder">BorrowDecoder</a>\<'de, Context = Context\>,

Attempt to decode this type with the given [BorrowDecode](https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html "trait bincode::de::BorrowDecode").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-BorrowDecode%3C&#39;de,+Context%3E-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl\<'de, Context\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html" class="trait" title="trait bincode::de::BorrowDecode">BorrowDecode</a>\<'de, Context\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.borrow_decode-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html#tymethod.borrow_decode" class="fn">borrow_decode</a>\<D\>(decoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut D</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.DecodeError.html" class="enum" title="enum bincode::error::DecodeError">DecodeError</a>\>

where D: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecoder.html" class="trait" title="trait bincode::de::BorrowDecoder">BorrowDecoder</a>\<'de, Context = Context\>,

Attempt to decode this type with the given [BorrowDecode](https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.BorrowDecode.html "trait bincode::de::BorrowDecode").

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2179" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Clone-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

Makes a clone of the `Arc` pointer.

This creates another pointer to the same allocation, increasing the strong reference count.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-42" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

let _ = Arc::clone(&five);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3531" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Debug-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Decode%3CContext%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<Context, T\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html" class="trait" title="trait bincode::de::Decode">Decode</a>\<Context\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where T: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html" class="trait" title="trait bincode::de::Decode">Decode</a>\<Context\> + 'static,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.decode-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html#tymethod.decode" class="fn">decode</a>\<D\>(decoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut D</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.DecodeError.html" class="enum" title="enum bincode::error::DecodeError">DecodeError</a>\>

where D: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decoder.html" class="trait" title="trait bincode::de::Decoder">Decoder</a>\<Context = Context\>,

Attempt to decode this type with the given [Decode](https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html "trait bincode::de::Decode").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Decode%3CContext%3E-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<Context, T\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html" class="trait" title="trait bincode::de::Decode">Decode</a>\<Context\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html" class="trait" title="trait bincode::de::Decode">Decode</a>\<Context\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.decode" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html#tymethod.decode" class="fn">decode</a>\<D\>(decoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut D</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.DecodeError.html" class="enum" title="enum bincode::error::DecodeError">DecodeError</a>\>

where D: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decoder.html" class="trait" title="trait bincode::de::Decoder">Decoder</a>\<Context = Context\>,

Attempt to decode this type with the given [Decode](https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html "trait bincode::de::Decode").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Decode%3CContext%3E-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl\<Context\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html" class="trait" title="trait bincode::de::Decode">Decode</a>\<Context\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.decode-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html#tymethod.decode" class="fn">decode</a>\<D\>(decoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut D</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.DecodeError.html" class="enum" title="enum bincode::error::DecodeError">DecodeError</a>\>

where D: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decoder.html" class="trait" title="trait bincode::de::Decoder">Decoder</a>\<Context = Context\>,

Attempt to decode this type with the given [Decode](https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/de/trait.Decode.html "trait bincode::de::Decode").

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3632" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Default-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.default-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

Creates an empty `[T]` inside an Arc

This may or may not share an allocation with other Arcs.

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3613" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Default-for-Arc%3CCStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.default-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

Creates an empty CStr inside an Arc

This may or may not share an allocation with other Arcs.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3546" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Default-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

Creates a new `Arc<T>`, with the `Default` value for `T`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-43" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let x: Arc<i32> = Default::default();
assert_eq!(*x, 0);
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3598" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Default-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.default-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Creates an empty str inside an Arc

This may or may not share an allocation with other Arcs.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2236" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Deref-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = T

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Deserialize%3C&#39;de%3E-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<'de, T\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T\>: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

This impl requires the [`"rc"`](https://serde.rs/feature-flags.html#-features-rc) Cargo feature of Serde.

Deserializing a data structure containing `Arc` will not attempt to deduplicate `Arc` references to the same data. Every deserialized `Arc` will end up with a strong count of 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>( deserializer: D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>, \<D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3524" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Display-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#2611" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Drop-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Drops the `Arc`.

This will decrement the strong reference count. If the strong reference count reaches zero then the only other references (if any) are [`Weak`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Weak.html "struct alloc::sync::Weak"), so we `drop` the inner value.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-41" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropped!");
    }
}

let foo  = Arc::new(Foo);
let foo2 = Arc::clone(&foo);

drop(foo);    // Doesn't print anything
drop(foo2);   // Prints "dropped!"
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Encode-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/enc/trait.Encode.html" class="trait" title="trait bincode::enc::Encode">Encode</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/enc/trait.Encode.html" class="trait" title="trait bincode::enc::Encode">Encode</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.encode" class="anchor">§</a>

#### fn <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/enc/trait.Encode.html#tymethod.encode" class="fn">encode</a>\<E\>(&self, encoder: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/error/enum.EncodeError.html" class="enum" title="enum bincode::error::EncodeError">EncodeError</a>\>

where E: <a href="https://docs.rs/bincode/2.0.1/x86_64-unknown-linux-gnu/bincode/enc/trait.Encoder.html" class="trait" title="trait bincode::enc::Encoder">Encoder</a>,

Encode a given type.

1.52.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#4115" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Error-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, req: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#131" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3722" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26%5BT%5D%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

Allocates a reference-counted slice and fills it by cloning `v`’s items.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-4" class="doc-anchor">§</a>Example

``` rust
let original: &[i32] = &[1, 2, 3];
let shared: Arc<[i32]> = Arc::from(original);
assert_eq!(&[1, 2, 3], &shared[..]);
```

1.24.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#912" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26CStr%3E-for-Arc%3CCStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

Converts a `&CStr` into a `Arc<CStr>`, by copying the contents into a newly allocated [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc").

1.24.0 · <a href="https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1362" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26OsStr%3E-for-Arc%3COsStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\>

Copies the string into a newly allocated [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc")`<`[`OsStr`](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")`>`.

1.24.0 · <a href="https://doc.rust-lang.org/nightly/src/std/path.rs.html#2037" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26Path%3E-for-Arc%3CPath%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

Converts a [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") into an [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") by copying the [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") data into a new [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") buffer.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26RiAbsoluteStr%3CS%3E%3E-for-Arc%3CRiAbsoluteStr%3CS%3E%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/absolute/struct.RiAbsoluteStr.html" class="struct" title="struct iri_string::types::generic::absolute::RiAbsoluteStr">RiAbsoluteStr</a>\<S\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/absolute/struct.RiAbsoluteStr.html" class="struct" title="struct iri_string::types::generic::absolute::RiAbsoluteStr">RiAbsoluteStr</a>\<S\>\>

where S: <a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/spec/trait.Spec.html" class="trait" title="trait iri_string::spec::Spec">Spec</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-24" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/absolute/struct.RiAbsoluteStr.html" class="struct" title="struct iri_string::types::generic::absolute::RiAbsoluteStr">RiAbsoluteStr</a>\<S\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/absolute/struct.RiAbsoluteStr.html" class="struct" title="struct iri_string::types::generic::absolute::RiAbsoluteStr">RiAbsoluteStr</a>\<S\>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26RiFragmentStr%3CS%3E%3E-for-Arc%3CRiFragmentStr%3CS%3E%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/fragment/struct.RiFragmentStr.html" class="struct" title="struct iri_string::types::generic::fragment::RiFragmentStr">RiFragmentStr</a>\<S\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/fragment/struct.RiFragmentStr.html" class="struct" title="struct iri_string::types::generic::fragment::RiFragmentStr">RiFragmentStr</a>\<S\>\>

where S: <a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/spec/trait.Spec.html" class="trait" title="trait iri_string::spec::Spec">Spec</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-25" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/fragment/struct.RiFragmentStr.html" class="struct" title="struct iri_string::types::generic::fragment::RiFragmentStr">RiFragmentStr</a>\<S\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/fragment/struct.RiFragmentStr.html" class="struct" title="struct iri_string::types::generic::fragment::RiFragmentStr">RiFragmentStr</a>\<S\>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26RiQueryStr%3CS%3E%3E-for-Arc%3CRiQueryStr%3CS%3E%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/query/struct.RiQueryStr.html" class="struct" title="struct iri_string::types::generic::query::RiQueryStr">RiQueryStr</a>\<S\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/query/struct.RiQueryStr.html" class="struct" title="struct iri_string::types::generic::query::RiQueryStr">RiQueryStr</a>\<S\>\>

where S: <a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/spec/trait.Spec.html" class="trait" title="trait iri_string::spec::Spec">Spec</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-27" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/query/struct.RiQueryStr.html" class="struct" title="struct iri_string::types::generic::query::RiQueryStr">RiQueryStr</a>\<S\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/query/struct.RiQueryStr.html" class="struct" title="struct iri_string::types::generic::query::RiQueryStr">RiQueryStr</a>\<S\>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26RiReferenceStr%3CS%3E%3E-for-Arc%3CRiReferenceStr%3CS%3E%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/reference/struct.RiReferenceStr.html" class="struct" title="struct iri_string::types::generic::reference::RiReferenceStr">RiReferenceStr</a>\<S\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/reference/struct.RiReferenceStr.html" class="struct" title="struct iri_string::types::generic::reference::RiReferenceStr">RiReferenceStr</a>\<S\>\>

where S: <a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/spec/trait.Spec.html" class="trait" title="trait iri_string::spec::Spec">Spec</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-28" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/reference/struct.RiReferenceStr.html" class="struct" title="struct iri_string::types::generic::reference::RiReferenceStr">RiReferenceStr</a>\<S\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/reference/struct.RiReferenceStr.html" class="struct" title="struct iri_string::types::generic::reference::RiReferenceStr">RiReferenceStr</a>\<S\>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26RiRelativeStr%3CS%3E%3E-for-Arc%3CRiRelativeStr%3CS%3E%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/relative/struct.RiRelativeStr.html" class="struct" title="struct iri_string::types::generic::relative::RiRelativeStr">RiRelativeStr</a>\<S\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/relative/struct.RiRelativeStr.html" class="struct" title="struct iri_string::types::generic::relative::RiRelativeStr">RiRelativeStr</a>\<S\>\>

where S: <a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/spec/trait.Spec.html" class="trait" title="trait iri_string::spec::Spec">Spec</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-29" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/relative/struct.RiRelativeStr.html" class="struct" title="struct iri_string::types::generic::relative::RiRelativeStr">RiRelativeStr</a>\<S\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/relative/struct.RiRelativeStr.html" class="struct" title="struct iri_string::types::generic::relative::RiRelativeStr">RiRelativeStr</a>\<S\>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26RiStr%3CS%3E%3E-for-Arc%3CRiStr%3CS%3E%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/normal/struct.RiStr.html" class="struct" title="struct iri_string::types::generic::normal::RiStr">RiStr</a>\<S\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/normal/struct.RiStr.html" class="struct" title="struct iri_string::types::generic::normal::RiStr">RiStr</a>\<S\>\>

where S: <a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/spec/trait.Spec.html" class="trait" title="trait iri_string::spec::Spec">Spec</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-26" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/normal/struct.RiStr.html" class="struct" title="struct iri_string::types::generic::normal::RiStr">RiStr</a>\<S\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/types/generic/normal/struct.RiStr.html" class="struct" title="struct iri_string::types::generic::normal::RiStr">RiStr</a>\<S\>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26UriTemplateStr%3E-for-Arc%3CUriTemplateStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/template/string/struct.UriTemplateStr.html" class="struct" title="struct iri_string::template::string::UriTemplateStr">UriTemplateStr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/template/string/struct.UriTemplateStr.html" class="struct" title="struct iri_string::template::string::UriTemplateStr">UriTemplateStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-23" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/template/string/struct.UriTemplateStr.html" class="struct" title="struct iri_string::template::string::UriTemplateStr">UriTemplateStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/iri-string/0.7.8/x86_64-unknown-linux-gnu/iri_string/template/string/struct.UriTemplateStr.html" class="struct" title="struct iri_string::template::string::UriTemplateStr">UriTemplateStr</a>\>

Converts to this type from the input type.

1.84.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3741" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26mut+%5BT%5D%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

Allocates a reference-counted slice and fills it by cloning `v`’s items.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-5" class="doc-anchor">§</a>Example

``` rust
let mut original = [1, 2, 3];
let original: &mut [i32] = &mut original;
let shared: Arc<[i32]> = Arc::from(original);
assert_eq!(&[1, 2, 3], &shared[..]);
```

1.84.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#924" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26mut+CStr%3E-for-Arc%3CCStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&mut <a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &mut <a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

Converts a `&mut CStr` into a `Arc<CStr>`, by copying the contents into a newly allocated [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc").

1.84.0 · <a href="https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1372" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26mut+OsStr%3E-for-Arc%3COsStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &mut <a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\>

Copies the string into a newly allocated [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc")`<`[`OsStr`](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")`>`.

1.84.0 · <a href="https://doc.rust-lang.org/nightly/src/std/path.rs.html#2047" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26mut+Path%3E-for-Arc%3CPath%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: &mut <a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

Converts a [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") into an [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") by copying the [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") data into a new [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") buffer.

1.84.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3780" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26mut+str%3E-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-16" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Allocates a reference-counted `str` and copies `v` into it.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-7" class="doc-anchor">§</a>Example

``` rust
let mut original = String::from("eggplant");
let original: &mut str = &mut original;
let shared: Arc<str> = Arc::from(original);
assert_eq!("eggplant", &shared[..]);
```

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3761" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%26str%3E-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Allocates a reference-counted `str` and copies `v` into it.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-6" class="doc-anchor">§</a>Example

``` rust
let shared: Arc<str> = Arc::from("eggplant");
assert_eq!("eggplant", &shared[..]);
```

1.74.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3701" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3C%5BT;+N%5D%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

Converts a [`[T; N]`](https://doc.rust-lang.org/nightly/std/primitive.array.html "primitive array") into an `Arc<[T]>`.

The conversion moves the array into a newly allocated `Arc`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-3" class="doc-anchor">§</a>Example

``` rust
let original: [i32; 3] = [1, 2, 3];
let shared: Arc<[i32]> = Arc::from(original);
assert_eq!(&[1, 2, 3], &shared[..]);
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CArc%3C%5Bu8%5D%3E%3E-for-Arc%3CByteStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html" class="struct" title="struct core::bstr::ByteStr">ByteStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html" class="struct" title="struct core::bstr::ByteStr">ByteStr</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CArc%3CByteStr%3E%3E-for-Arc%3C%5Bu8%5D%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html" class="struct" title="struct core::bstr::ByteStr">ByteStr</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html" class="struct" title="struct core::bstr::ByteStr">ByteStr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Converts to this type from the input type.

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/task.rs.html#109" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CArc%3CW%3E%3E-for-Waker" class="anchor">§</a>

### impl\<W\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<W\>\> for <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html" class="struct" title="struct core::task::wake::Waker">Waker</a>

where W: <a href="https://doc.rust-lang.org/nightly/alloc/task/trait.Wake.html" class="trait" title="trait alloc::task::Wake">Wake</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-22" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(waker: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<W\>) -\> <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html" class="struct" title="struct core::task::wake::Waker">Waker</a>

Use a [`Wake`](https://doc.rust-lang.org/nightly/alloc/task/trait.Wake.html "trait alloc::task::Wake")-able type as a `Waker`.

No heap allocations or atomic operations are used for this conversion.

1.62.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3894" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CArc%3Cstr%3E%3E-for-Arc%3C%5Bu8%5D%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-21" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(rc: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Converts an atomically reference-counted string slice into a byte slice.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-12" class="doc-anchor">§</a>Example

``` rust
let string: Arc<str> = Arc::from("eggplant");
let bytes: Arc<[u8]> = Arc::from(string);
assert_eq!("eggplant".as_bytes(), bytes.as_ref());
```

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3819" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CBox%3CT,+A%3E%3E-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T, A\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-18" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T, A\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

Move a boxed object to a new, reference-counted allocation.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-9" class="doc-anchor">§</a>Example

``` rust
let unique: Box<str> = Box::from("eggplant");
let shared: Arc<str> = Arc::from(unique);
assert_eq!("eggplant", &shared[..]);
```

1.24.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#900" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CCString%3E-for-Arc%3CCStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html" class="struct" title="struct alloc::ffi::c_str::CString">CString</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html" class="struct" title="struct alloc::ffi::c_str::CString">CString</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html" class="struct" title="struct core::ffi::c_str::CStr">CStr</a>\>

Converts a [`CString`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString") into an [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc")`<`[`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")`>` by moving the [`CString`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString") data into a new [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") buffer.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CCompactString%3E-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/compact_str/0.9.0/x86_64-unknown-linux-gnu/compact_str/struct.CompactString.html" class="struct" title="struct compact_str::CompactString">CompactString</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-30" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/compact_str/0.9.0/x86_64-unknown-linux-gnu/compact_str/struct.CompactString.html" class="struct" title="struct compact_str::CompactString">CompactString</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Converts to this type from the input type.

1.45.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3867-3870" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CCow%3C&#39;a,+B%3E%3E-for-Arc%3CB%3E" class="anchor">§</a>

### impl\<'a, B\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, B\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<B\>

where B: <a href="https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html" class="trait" title="trait alloc::borrow::ToOwned">ToOwned</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<B\>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a B</a>\> + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<\<B as <a href="https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html" class="trait" title="trait alloc::borrow::ToOwned">ToOwned</a>\>::<a href="https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned" class="associatedtype" title="type alloc::borrow::ToOwned::Owned">Owned</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-20" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(cow: <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, B\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<B\>

Creates an atomically reference-counted pointer from a clone-on-write pointer by copying its content.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-11" class="doc-anchor">§</a>Example

``` rust
let cow: Cow<'_, str> = Cow::Borrowed("eggplant");
let shared: Arc<str> = Arc::from(cow);
assert_eq!("eggplant", &shared[..]);
```

1.24.0 · <a href="https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1351" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3COsString%3E-for-Arc%3COsStr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html" class="struct" title="struct std::ffi::os_str::OsString">OsString</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html" class="struct" title="struct std::ffi::os_str::OsString">OsString</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html" class="struct" title="struct std::ffi::os_str::OsStr">OsStr</a>\>

Converts an [`OsString`](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString") into an [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc")`<`[`OsStr`](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")`>` by moving the [`OsString`](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString") data into a new [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") buffer.

1.24.0 · <a href="https://doc.rust-lang.org/nightly/src/std/path.rs.html#2026" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CPathBuf%3E-for-Arc%3CPath%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>

Converts a [`PathBuf`](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf") into an [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc")`<`[`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")`>` by moving the [`PathBuf`](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf") data into a new [`Arc`](https://docs.rs/polars/latest/polars/prelude/struct.Arc.html "struct polars::prelude::Arc") buffer.

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3800" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CString%3E-for-Arc%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-17" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Allocates a reference-counted `str` and copies `v` into it.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-8" class="doc-anchor">§</a>Example

``` rust
let unique: String = "eggplant".to_owned();
let shared: Arc<str> = Arc::from(unique);
assert_eq!("eggplant", &shared[..]);
```

1.6.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3679" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CT%3E-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(t: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

Converts a `T` into an `Arc<T>`

The conversion moves the value into a newly allocated `Arc`. It is equivalent to calling `Arc::new(t)`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-2" class="doc-anchor">§</a>Example

``` rust
let x = 5;
let arc = Arc::new(5);

assert_eq!(Arc::from(x), arc);
```

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3838" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-From%3CVec%3CT,+A%3E%3E-for-Arc%3C%5BT%5D,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T, A\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from-19" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T, A\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>

Allocates a reference-counted slice and moves `v`’s items into it.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#example-10" class="doc-anchor">§</a>Example

``` rust
let unique: Vec<i32> = vec![1, 2, 3];
let shared: Arc<[i32]> = Arc::from(unique);
assert_eq!(&[1, 2, 3], &shared[..]);
```

1.37.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3928" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-FromIterator%3CT%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

Takes each element in the `Iterator` and collects it into an `Arc<[T]>`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#performance-characteristics" class="doc-anchor">§</a>Performance characteristics

###### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#the-general-case" class="doc-anchor">§</a>The general case

In the general case, collecting into `Arc<[T]>` is done by first collecting into a `Vec<T>`. That is, when writing the following:

``` rust
let evens: Arc<[u8]> = (0..10).filter(|&x| x % 2 == 0).collect();
```

this behaves as if we wrote:

``` rust
let evens: Arc<[u8]> = (0..10).filter(|&x| x % 2 == 0)
    .collect::<Vec<_>>() // The first set of allocations happens here.
    .into(); // A second allocation for `Arc<[T]>` happens here.
```

This will allocate as many times as needed for constructing the `Vec<T>` and then it will allocate once for turning the `Vec<T>` into the `Arc<[T]>`.

###### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#iterators-of-known-length" class="doc-anchor">§</a>Iterators of known length

When your `Iterator` implements `TrustedLen` and is of an exact size, a single allocation will be made for the `Arc<[T]>`. For example:

``` rust
let evens: Arc<[u8]> = (0..10).collect(); // Just a single allocation happens here.
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-FromParallelIterator%3CT%3E-for-Arc%3C%5BT%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html" class="trait" title="trait rayon::iter::FromParallelIterator">FromParallelIterator</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Collects items from a parallel iterator into an atomically-reference-counted slice.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.from_par_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html#tymethod.from_par_iter" class="fn">from_par_iter</a>\<I\>(par_iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\>

where I: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = T\>,

Creates an instance of the collection from the parallel iterator `par_iter`. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html#tymethod.from_par_iter)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3671" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Hash-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-IntoSeries-for-Arc%3Cdyn+SeriesTrait%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.is_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-LocalSpawn-for-Arc%3CSp%3E" class="anchor">§</a>

### impl\<Sp\> <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.LocalSpawn.html" class="trait" title="trait futures_task::spawn::LocalSpawn">LocalSpawn</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<Sp\>

where Sp: <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.LocalSpawn.html" class="trait" title="trait futures_task::spawn::LocalSpawn">LocalSpawn</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.spawn_local_obj" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.LocalSpawn.html#tymethod.spawn_local_obj" class="fn">spawn_local_obj</a>( &self, future: <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/future_obj/struct.LocalFutureObj.html" class="struct" title="struct futures_task::future_obj::LocalFutureObj">LocalFutureObj</a>\<'static, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/struct.SpawnError.html" class="struct" title="struct futures_task::spawn::SpawnError">SpawnError</a>\>

Spawns a future that will be run to completion. [Read more](https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.LocalSpawn.html#tymethod.spawn_local_obj)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.status_local" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.LocalSpawn.html#method.status_local" class="fn">status_local</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/struct.SpawnError.html" class="struct" title="struct futures_task::spawn::SpawnError">SpawnError</a>\>

Determines whether the executor is able to spawn new tasks. [Read more](https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.LocalSpawn.html#method.status_local)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-ObjectStore-for-Arc%3Cdyn+ObjectStore%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.put" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.put" class="fn">put</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, payload: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/payload/struct.PutPayload.html" class="struct" title="struct object_store::payload::PutPayload">PutPayload</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Save the provided bytes to the specified location [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.put)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.put_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.put_opts" class="fn">put_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, payload: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/payload/struct.PutPayload.html" class="struct" title="struct object_store::payload::PutPayload">PutPayload</a>, opts: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Save the provided `payload` to `location` with the given options

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.put_multipart" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.put_multipart" class="fn">put_multipart</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/upload/trait.MultipartUpload.html" class="trait" title="trait object_store::upload::MultipartUpload">MultipartUpload</a>\>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Perform a multipart upload [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.put_multipart)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.put_multipart_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts" class="fn">put_multipart_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, opts: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.PutMultipartOpts.html" class="struct" title="struct object_store::PutMultipartOpts">PutMultipartOpts</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/upload/trait.MultipartUpload.html" class="trait" title="trait object_store::upload::MultipartUpload">MultipartUpload</a>\>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Perform a multipart upload with options [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.get" class="fn">get</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Return the bytes that are stored at the specified location.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.get_opts" class="fn">get_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, options: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Perform a get request with options

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.get_range" class="fn">get_range</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Return the bytes that are stored at the specified location in the given byte range. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.get_range)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.get_ranges" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.get_ranges" class="fn">get_ranges</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ranges: &'life2 \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Return the bytes that are stored at the specified location in the given byte ranges

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.head" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.head" class="fn">head</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Return the metadata for the specified location

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.delete" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.delete" class="fn">delete</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Delete the object at the specified location.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.delete_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.delete_stream" class="fn">delete_stream</a>\<'a\>( &'a self, locations: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

Delete all the objects at the specified locations [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.delete_stream)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.list" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.list" class="fn">list</a>( &self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>

List all the objects with the given prefix. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.list)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.list_with_offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.list_with_offset" class="fn">list_with_offset</a>( &self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, offset: &<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>

List all the objects with the given prefix and a location greater than `offset` [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.list_with_offset)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.list_with_delimiter" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.list_with_delimiter" class="fn">list_with_delimiter</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.ListResult.html" class="struct" title="struct object_store::ListResult">ListResult</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

List objects with the given prefix and an implementation specific delimiter. Returns common prefixes (directories) in addition to object metadata. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.list_with_delimiter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.copy" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.copy" class="fn">copy</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Copy an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.copy)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.rename" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.rename" class="fn">rename</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Move an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.rename)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.copy_if_not_exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists" class="fn">copy_if_not_exists</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Copy an object from one path to another, only if destination is empty. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.rename_if_not_exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.rename_if_not_exists" class="fn">rename_if_not_exists</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>: 'async_trait,

Move an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html#method.rename_if_not_exists)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3501" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Ord-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

Comparison for two `Arc`s.

The two are compared by calling `cmp()` on their inner values.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-44" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;
use std::cmp::Ordering;

let five = Arc::new(5);

assert_eq!(Ordering::Less, five.cmp(&Arc::new(6)));
```

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1023-1025" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1062-1064" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1088-1090" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-OwnedRetriever%3CU%3E-for-Arc%3CRwLock%3CU%3E%3E" class="anchor">§</a>

### impl\<U\> <a href="https://docs.rs/brotli/8.0.1/x86_64-unknown-linux-gnu/brotli/enc/threading/trait.OwnedRetriever.html" class="trait" title="trait brotli::enc::threading::OwnedRetriever">OwnedRetriever</a>\<U\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/sync/poison/rwlock/struct.RwLock.html" class="struct" title="struct std::sync::poison::rwlock::RwLock">RwLock</a>\<U\>\>

where U: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.view" class="anchor">§</a>

#### fn <a href="https://docs.rs/brotli/8.0.1/x86_64-unknown-linux-gnu/brotli/enc/threading/trait.OwnedRetriever.html#tymethod.view" class="fn">view</a>\<T, F\>(&self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;U</a>) -\> T,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.unwrap" class="anchor">§</a>

#### fn <a href="https://docs.rs/brotli/8.0.1/x86_64-unknown-linux-gnu/brotli/enc/threading/trait.OwnedRetriever.html#tymethod.unwrap" class="fn">unwrap</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-PartialEq%3Cdyn+Array%3E-for-Arc%3Cdyn+Array%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + '\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, that: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.ne-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-PartialEq%3Cdyn+Scalar%3E-for-Arc%3Cdyn+Scalar%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/scalar/trait.Scalar.html" class="trait" title="trait polars_arrow::scalar::Scalar">Scalar</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/scalar/trait.Scalar.html" class="trait" title="trait polars_arrow::scalar::Scalar">Scalar</a> + '\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.eq-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, that: &(dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/scalar/trait.Scalar.html" class="trait" title="trait polars_arrow::scalar::Scalar">Scalar</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.ne-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3367" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-PartialEq-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Equality for two `Arc`s.

Two `Arc`s are equal if their inner values are equal, even if they are stored in different allocation.

If `T` also implements `Eq` (implying reflexivity of equality), two `Arc`s that point to the same allocation are always equal.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-45" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

assert!(five == Arc::new(5));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Inequality for two `Arc`s.

Two `Arc`s are not equal if their inner values are not equal.

If `T` also implements `Eq` (implying reflexivity of equality), two `Arc`s that point to the same value are always equal.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-46" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

assert!(five != Arc::new(6));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3413" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-PartialOrd-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

Partial comparison for two `Arc`s.

The two are compared by calling `partial_cmp()` on their inner values.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-47" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;
use std::cmp::Ordering;

let five = Arc::new(5);

assert_eq!(Some(Ordering::Less), five.partial_cmp(&Arc::new(6)));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less-than comparison for two `Arc`s.

The two are compared by calling `<` on their inner values.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-48" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

assert!(five < Arc::new(6));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

‘Less than or equal to’ comparison for two `Arc`s.

The two are compared by calling `<=` on their inner values.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-49" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

assert!(five <= Arc::new(5));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater-than comparison for two `Arc`s.

The two are compared by calling `>` on their inner values.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-50" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

assert!(five > Arc::new(4));
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

‘Greater than or equal to’ comparison for two `Arc`s.

The two are compared by calling `>=` on their inner values.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#examples-51" class="doc-anchor">§</a>Examples

``` rust
use std::sync::Arc;

let five = Arc::new(5);

assert!(five >= Arc::new(5));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3538" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Pointer-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html" class="trait" title="trait core::fmt::Pointer">Pointer</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.fmt-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html#tymethod.fmt)

1.73.0 · <a href="https://doc.rust-lang.org/nightly/src/std/fs.rs.html#1426" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Read-for-Arc%3CFile%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self, buf: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Pull some bytes from this source into the specified buffer, returning how many bytes were read. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored" class="fn">read_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSliceMut.html" class="struct" title="struct std::io::IoSliceMut">IoSliceMut</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like `read`, except that it reads into a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read_buf" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf" class="fn">read_buf</a>(&mut self, cursor: <a href="https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html" class="struct" title="struct core::io::borrowed_buf::BorrowedCursor">BorrowedCursor</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`read_buf`)

Pull some bytes from this source into the specified buffer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.is_read_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored" class="fn">is_read_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Read`er has an efficient `read_vectored` implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read_to_end" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end" class="fn">read_to_end</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until EOF in this source, placing them into `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read_to_string" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string" class="fn">read_to_string</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until EOF in this source, appending them to `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

1.6.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1044" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read_exact" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact" class="fn">read_exact</a>(&mut self, buf: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads the exact number of bytes required to fill `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.read_buf_exact" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact" class="fn">read_buf_exact</a>(&mut self, cursor: <a href="https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html" class="struct" title="struct core::io::borrowed_buf::BorrowedCursor">BorrowedCursor</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`read_buf`)

Reads the exact number of bytes required to fill `cursor`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1119-1121" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.by_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a “by reference” adaptor for this instance of `Read`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1162-1164" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.bytes" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes" class="fn">bytes</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Bytes.html" class="struct" title="struct std::io::Bytes">Bytes</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Transforms this `Read` instance to an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over its bytes. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1200-1202" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.chain" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain" class="fn">chain</a>\<R\>(self, next: R) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Chain.html" class="struct" title="struct std::io::Chain">Chain</a>\<Self, R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adapter which will chain this stream with another. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1239-1241" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.take" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take" class="fn">take</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Take.html" class="struct" title="struct std::io::Take">Take</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adapter which will read at most `limit` bytes from it. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take)

1.73.0 · <a href="https://doc.rust-lang.org/nightly/src/std/fs.rs.html#1465" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Seek-for-Arc%3CFile%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html" class="trait" title="trait std::io::Seek">Seek</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.seek" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek" class="fn">seek</a>(&mut self, pos: <a href="https://doc.rust-lang.org/nightly/std/io/enum.SeekFrom.html" class="enum" title="enum std::io::SeekFrom">SeekFrom</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Seek to an offset, in bytes, in a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.stream_len" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len" class="fn">stream_len</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`seek_stream_len`)

Returns the length of this stream (in bytes). [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.stream_position" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position" class="fn">stream_position</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Returns the current seek position from the start of the stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position)

1.55.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2064" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.rewind" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind" class="fn">rewind</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Rewind to the beginning of a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind)

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2160" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.seek_relative" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative" class="fn">seek_relative</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Seeks relative to the current position. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Serialize-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

This impl requires the [`"rc"`](https://serde.rs/feature-flags.html#-features-rc) Cargo feature of Serde.

Serializing a data structure containing `Arc` will serialize a copy of the contents of the `Arc` each time the `Arc` is referenced within the data structure. Serialization will not attempt to deduplicate these repeated data.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>( &self, serializer: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Service%3CRequest%3E-for-Arc%3CS%3E" class="anchor">§</a>

### impl\<Request, S\> <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html" class="trait" title="trait hyper::service::service::Service">Service</a>\<Request\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<S\>

where S: <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html" class="trait" title="trait hyper::service::service::Service">Service</a>\<Request\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#associatedtype.Response" class="anchor">§</a>

#### type <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Response" class="associatedtype">Response</a> = \<S as <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html" class="trait" title="trait hyper::service::service::Service">Service</a>\<Request\>\>::<a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Response" class="associatedtype" title="type hyper::service::service::Service::Response">Response</a>

Responses given by the service.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Error" class="associatedtype">Error</a> = \<S as <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html" class="trait" title="trait hyper::service::service::Service">Service</a>\<Request\>\>::<a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Error" class="associatedtype" title="type hyper::service::service::Service::Error">Error</a>

Errors produced by the service. [Read more](https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Error)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#associatedtype.Future" class="anchor">§</a>

#### type <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Future" class="associatedtype">Future</a> = \<S as <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html" class="trait" title="trait hyper::service::service::Service">Service</a>\<Request\>\>::<a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Future" class="associatedtype" title="type hyper::service::service::Service::Future">Future</a>

The future response value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.call" class="anchor">§</a>

#### fn <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#tymethod.call" class="fn">call</a>(&self, req: Request) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<S\> as <a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html" class="trait" title="trait hyper::service::service::Service">Service</a>\<Request\>\>::<a href="https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#associatedtype.Future" class="associatedtype" title="type hyper::service::service::Service::Future">Future</a> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#" class="tooltip" data-notable-ty="&lt;Arc&lt;S&gt; as Service&lt;Request&gt;&gt;::Future">ⓘ</a>

Process the request and return the response asynchronously. `call` takes `&self` instead of `mut &self` because: [Read more](https://docs.rs/hyper/1.6.0/x86_64-unknown-linux-gnu/hyper/service/service/trait.Service.html#tymethod.call)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Spawn-for-Arc%3CSp%3E" class="anchor">§</a>

### impl\<Sp\> <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.Spawn.html" class="trait" title="trait futures_task::spawn::Spawn">Spawn</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<Sp\>

where Sp: <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.Spawn.html" class="trait" title="trait futures_task::spawn::Spawn">Spawn</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.spawn_obj" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.Spawn.html#tymethod.spawn_obj" class="fn">spawn_obj</a>(&self, future: <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/future_obj/struct.FutureObj.html" class="struct" title="struct futures_task::future_obj::FutureObj">FutureObj</a>\<'static, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/struct.SpawnError.html" class="struct" title="struct futures_task::spawn::SpawnError">SpawnError</a>\>

Spawns a future that will be run to completion. [Read more](https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.Spawn.html#tymethod.spawn_obj)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.status" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.Spawn.html#method.status" class="fn">status</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/struct.SpawnError.html" class="struct" title="struct futures_task::spawn::SpawnError">SpawnError</a>\>

Determines whether the executor is able to spawn new tasks. [Read more](https://docs.rs/futures-task/0.3.31/x86_64-unknown-linux-gnu/futures_task/spawn/trait.Spawn.html#method.status)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Subscriber-for-Arc%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html" class="trait" title="trait tracing_core::subscriber::Subscriber">Subscriber</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<S\>

where S: <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html" class="trait" title="trait tracing_core::subscriber::Subscriber">Subscriber</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.register_callsite" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.register_callsite" class="fn">register_callsite</a>(&self, metadata: &'static <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.Metadata.html" class="struct" title="struct tracing_core::metadata::Metadata">Metadata</a>\<'static\>) -\> <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/struct.Interest.html" class="struct" title="struct tracing_core::subscriber::Interest">Interest</a>

Registers a new [callsite](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/callsite/index.html "mod tracing_core::callsite") with this subscriber, returning whether or not the subscriber is interested in being notified about the callsite. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.register_callsite)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.enabled" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.enabled" class="fn">enabled</a>(&self, metadata: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.Metadata.html" class="struct" title="struct tracing_core::metadata::Metadata">Metadata</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if a span or event with the specified [metadata](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.Metadata.html "struct tracing_core::metadata::Metadata") would be recorded. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.enabled)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.max_level_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.max_level_hint" class="fn">max_level_hint</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.LevelFilter.html" class="struct" title="struct tracing_core::metadata::LevelFilter">LevelFilter</a>\>

Returns the highest [verbosity level](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/metadata/struct.Level.html "struct tracing_core::metadata::Level") that this `Subscriber` will enable, or `None`, if the subscriber does not implement level-based filtering or chooses not to implement this method. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.max_level_hint)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.new_span" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.new_span" class="fn">new_span</a>(&self, span: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Attributes.html" class="struct" title="struct tracing_core::span::Attributes">Attributes</a>\<'\_\>) -\> <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>

Visit the construction of a new span, returning a new [span ID](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html "struct tracing_core::span::Id") for the span being constructed. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.new_span)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.record" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.record" class="fn">record</a>(&self, span: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>, values: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Record.html" class="struct" title="struct tracing_core::span::Record">Record</a>\<'\_\>)

Record a set of values on a span. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.record)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.record_follows_from" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.record_follows_from" class="fn">record_follows_from</a>(&self, span: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>, follows: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>)

Adds an indication that `span` follows from the span with the id `follows`. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.record_follows_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.event_enabled" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.event_enabled" class="fn">event_enabled</a>(&self, event: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/event/struct.Event.html" class="struct" title="struct tracing_core::event::Event">Event</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine if an [`Event`](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/event/struct.Event.html "struct tracing_core::event::Event") should be recorded. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.event_enabled)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.event" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.event" class="fn">event</a>(&self, event: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/event/struct.Event.html" class="struct" title="struct tracing_core::event::Event">Event</a>\<'\_\>)

Records that an [`Event`](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/event/struct.Event.html "struct tracing_core::event::Event") has occurred. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.event)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.enter" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.enter" class="fn">enter</a>(&self, span: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>)

Records that a span has been entered. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.enter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.exit" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.exit" class="fn">exit</a>(&self, span: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>)

Records that a span has been exited. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#tymethod.exit)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.clone_span" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.clone_span" class="fn">clone_span</a>(&self, id: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>) -\> <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>

Notifies the subscriber that a [span ID](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html "struct tracing_core::span::Id") has been cloned. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.clone_span)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_close" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.try_close" class="fn">try_close</a>(&self, id: <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Notifies the subscriber that a [span ID](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html "struct tracing_core::span::Id") has been dropped, and returns `true` if there are now 0 IDs that refer to that span. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.try_close)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.drop_span" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.drop_span" class="fn">drop_span</a>(&self, id: <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Id.html" class="struct" title="struct tracing_core::span::Id">Id</a>)

👎Deprecated since 0.1.2: use `Subscriber::try_close` instead

**This method is deprecated.** [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.drop_span)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.current_span" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.current_span" class="fn">current_span</a>(&self) -\> <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/span/struct.Current.html" class="struct" title="struct tracing_core::span::Current">Current</a>

Returns a type representing this subscriber’s view of the current span. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.current_span)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.downcast_raw" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.downcast_raw" class="fn">downcast_raw</a>(&self, id: <a href="https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html" class="struct" title="struct core::any::TypeId">TypeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

If `self` is the same type as the provided `TypeId`, returns an untyped `*const` pointer to that type. Otherwise, returns `None`. [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.downcast_raw)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.on_register_dispatch" class="anchor">§</a>

#### fn <a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.on_register_dispatch" class="fn">on_register_dispatch</a>(&self, subscriber: &<a href="https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/dispatcher/struct.Dispatch.html" class="struct" title="struct tracing_core::dispatcher::Dispatch">Dispatch</a>)

Invoked when this subscriber becomes a [`Dispatch`](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/dispatcher/struct.Dispatch.html "struct tracing_core::dispatcher::Dispatch"). [Read more](https://docs.rs/tracing-core/0.1.34/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html#method.on_register_dispatch)

1.43.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3913" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-TryFrom%3CArc%3C%5BT%5D,+A%3E%3E-for-Arc%3C%5BT;+N%5D,+A%3E" class="anchor">§</a>

### impl\<T, A, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( boxed_slice: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>, A\>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">[T; N]</a>, A\> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, A\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

1.73.0 · <a href="https://doc.rust-lang.org/nightly/src/std/fs.rs.html#1448" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Write-for-Arc%3CFile%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored" class="fn">write_vectored</a>(&mut self, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write "method std::io::Write::write"), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.is_write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored" class="fn">is_write_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored "method std::io::Write::write_vectored") implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.flush" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1835" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_all" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all" class="fn">write_all</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_all_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored" class="fn">write_all_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`write_all_vectored`)

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1950" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt" class="fn">write_fmt</a>(&mut self, args: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html" class="struct" title="struct core::fmt::Arguments">Arguments</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1980-1982" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.by_ref-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a “by reference” adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Writeable-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html" class="trait" title="trait writeable::Writeable">Writeable</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html" class="trait" title="trait writeable::Writeable">Writeable</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_to" class="anchor">§</a>

#### fn <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to" class="fn">write_to</a>\<W\>(&self, sink: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html" class="trait" title="trait core::fmt::Write">Write</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Writes a string to the given sink. Errors from the sink are bubbled up. The default implementation delegates to `write_to_parts`, and discards any `Part` annotations.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_to_parts" class="anchor">§</a>

#### fn <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to_parts" class="fn">write_to_parts</a>\<W\>(&self, sink: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

where W: <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.PartsWrite.html" class="trait" title="trait writeable::PartsWrite">PartsWrite</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Write bytes and `Part` annotations to the given sink. Errors from the sink are bubbled up. The default implementation delegates to `write_to`, and doesn’t produce any `Part` annotations.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.writeable_length_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.writeable_length_hint" class="fn">writeable_length_hint</a>(&self) -\> <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/struct.LengthHint.html" class="struct" title="struct writeable::LengthHint">LengthHint</a>

Returns a hint for the number of UTF-8 bytes that will be written to the sink. [Read more](https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.writeable_length_hint)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#method.write_to_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to_string" class="fn">write_to_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Creates a new `String` with the data from this `Writeable`. Like `ToString`, but smaller and faster. [Read more](https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to_string)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-CartablePointerLike-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/yoke/0.8.0/x86_64-unknown-linux-gnu/yoke/cartable_ptr/trait.CartablePointerLike.html" class="trait" title="trait yoke::cartable_ptr::CartablePointerLike">CartablePointerLike</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-CloneStableDeref-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/stable_deref_trait/1.2.0/x86_64-unknown-linux-gnu/stable_deref_trait/trait.CloneStableDeref.html" class="trait" title="trait stable_deref_trait::CloneStableDeref">CloneStableDeref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-CloneableCart-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/yoke/0.8.0/x86_64-unknown-linux-gnu/yoke/yoke/trait.CloneableCart.html" class="trait" title="trait yoke::yoke::CloneableCart">CloneableCart</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-CloneableCartablePointerLike-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/yoke/0.8.0/x86_64-unknown-linux-gnu/yoke/cartable_ptr/trait.CloneableCartablePointerLike.html" class="trait" title="trait yoke::cartable_ptr::CloneableCartablePointerLike">CloneableCartablePointerLike</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-CoerceUnsized%3CArc%3CU,+A%3E%3E-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, U, A\> <a href="https://doc.rust-lang.org/nightly/core/ops/unsize/trait.CoerceUnsized.html" class="trait" title="trait core::ops::unsize::CoerceUnsized">CoerceUnsized</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<U, A\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unsize.html" class="trait" title="trait core::marker::Unsize">Unsize</a>\<U\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, U: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-DerefPure-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefPure.html" class="trait" title="trait core::ops::deref::DerefPure">DerefPure</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-DispatchFromDyn%3CArc%3CU%3E%3E-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T, U\> <a href="https://doc.rust-lang.org/nightly/core/ops/unsize/trait.DispatchFromDyn.html" class="trait" title="trait core::ops::unsize::DispatchFromDyn">DispatchFromDyn</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<U\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unsize.html" class="trait" title="trait core::marker::Unsize">Unsize</a>\<U\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3521" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Eq-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-LifetimeFree-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/castaway/0.2.3/x86_64-unknown-linux-gnu/castaway/lifetime_free/trait.LifetimeFree.html" class="trait" title="trait castaway::lifetime_free::LifetimeFree">LifetimeFree</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/castaway/0.2.3/x86_64-unknown-linux-gnu/castaway/lifetime_free/trait.LifetimeFree.html" class="trait" title="trait castaway::lifetime_free::LifetimeFree">LifetimeFree</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-PinCoerceUnsized-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/pin/trait.PinCoerceUnsized.html" class="trait" title="trait core::pin::PinCoerceUnsized">PinCoerceUnsized</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#271" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Send-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-StableDeref-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/stable_deref_trait/1.2.0/x86_64-unknown-linux-gnu/stable_deref_trait/trait.StableDeref.html" class="trait" title="trait stable_deref_trait::StableDeref">StableDeref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T\>

where T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#273" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Sync-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

1.33.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#4027" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-Unpin-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

1.9.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#276" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-UnwindSafe-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html" class="trait" title="trait core::panic::unwind_safe::UnwindSafe">UnwindSafe</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html" class="trait" title="trait core::panic::unwind_safe::RefUnwindSafe">RefUnwindSafe</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html" class="trait" title="trait core::panic::unwind_safe::UnwindSafe">UnwindSafe</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#impl-UseCloned-for-Arc%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.UseCloned.html" class="trait" title="trait core::clone::UseCloned">UseCloned</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<T, A\>

where A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html#blanket-implementations" class="anchor">§</a>
