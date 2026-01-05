Description: An asynchronous `Mutex`-like type.

Title: Mutex in tokio::sync - Rust

Docs.rs

*   tokio-1.47.0

*   tokio 1.47.0
*   Permalink
*   Docs.rs crate page
*   MIT

*   Links
*   Homepage
*   Repository
*   crates.io
*   Source

*   Owners
*   carllerche
*   Darksonn
*   github:tokio-rs:core

*   Dependencies
*   *   bytes ^1.2.1 _normal_ _optional_
*   mio ^1.0.1 _normal_ _optional_
*   parking\_lot ^0.12.0 _normal_ _optional_
*   pin-project-lite ^0.2.11 _normal_
*   tokio-macros ~2.5.0 _normal_ _optional_
*   async-stream ^0.3 _dev_
*   futures ^0.3.0 _dev_
*   futures-concurrency ^7.6.3 _dev_
*   mockall ^0.13.0 _dev_
*   tokio-stream ^0.1 _dev_
*   tokio-test ^0.4.0 _dev_
*   wasm-bindgen-test ^0.3.0 _dev_
*   tracing-mock =0.1.0-beta.1 _dev_
*   io-uring ^0.7.6 _normal_
*   libc ^0.2.168 _normal_
*   mio ^1.0.1 _normal_
*   slab ^0.4.9 _normal_
*   loom ^0.7 _dev_
*   rand ^0.9 _dev_
*   socket2 ^0.6.0 _normal_ _optional_
*   proptest ^1 _dev_
*   socket2 ^0.6.0 _dev_
*   tempfile ^3.1.0 _dev_
*   mio-aio ^1 _dev_
*   backtrace ^0.3.58 _normal_
*   tracing ^0.1.29 _normal_ _optional_
*   libc ^0.2.168 _normal_ _optional_
*   signal-hook-registry ^1.1.1 _normal_ _optional_
*   libc ^0.2.168 _dev_
*   nix ^0.29.0 _dev_
*   windows-sys ^0.59 _normal_ _optional_
*   windows-sys ^0.59 _dev_

*   Versions

*   **100%** of the crate is documented

*   Platform
*   i686-pc-windows-msvc
*   i686-unknown-linux-gnu
*   x86\_64-apple-darwin
*   x86\_64-pc-windows-msvc
*   x86\_64-unknown-linux-gnu
*   Feature flags

*   docs.rs
*   About docs.rs
*   Badges
*   Builds
*   Metadata
*   Shorthand URLs
*   Download
*   Rustdoc JSON
*   Build queue
*   Privacy policy

*   Rust
*   Rust website
*   The Book
*   Standard Library API Reference
*   Rust by Example
*   The Cargo Guide
*   Clippy Documentation

tokio::sync

Struct MutexCopy item path
==========================

Source

```
pub struct Mutex<T: ?Sized> { /* private fields */ }
```

Available on **crate feature `sync`** only.

Expand description

An asynchronous `Mutex`\-like type.

This type acts similarly to `std::sync::Mutex`, with two major differences: `lock` is an async method so does not block, and the lock guard is designed to be held across `.await` points.

Tokio’s Mutex operates on a guaranteed FIFO basis. This means that the order in which tasks call the `lock` method is the exact order in which they will acquire the lock.

§Which kind of mutex should you use?
------------------------------------

Contrary to popular belief, it is ok and often preferred to use the ordinary `Mutex` from the standard library in asynchronous code.

The feature that the async mutex offers over the blocking mutex is the ability to keep it locked across an `.await` point. This makes the async mutex more expensive than the blocking mutex, so the blocking mutex should be preferred in the cases where it can be used. The primary use case for the async mutex is to provide shared mutable access to IO resources such as a database connection. If the value behind the mutex is just data, it’s usually appropriate to use a blocking mutex such as the one in the standard library or `parking_lot`.

Note that, although the compiler will not prevent the std `Mutex` from holding its guard across `.await` points in situations where the task is not movable between threads, this virtually never leads to correct concurrent code in practice as it can easily lead to deadlocks.

A common pattern is to wrap the `Arc<Mutex<...>>` in a struct that provides non-async methods for performing operations on the data within, and only lock the mutex inside these methods. The mini-redis example provides an illustration of this pattern.

Additionally, when you _do_ want shared access to an IO resource, it is often better to spawn a task to manage the IO resource, and to use message passing to communicate with that task.

§Examples:
----------

```
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
let data1 = Arc::new(Mutex::new(0));
let data2 = Arc::clone(&data1);

tokio::spawn(async move {
let mut lock = data2.lock().await;
*lock += 1;
});

let mut lock = data1.lock().await;
*lock += 1;
}
```

```
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
let count = Arc::new(Mutex::new(0));

for i in 0..5 {
let my_count = Arc::clone(&count);
tokio::spawn(async move {
for j in 0..10 {
let mut lock = my_count.lock().await;
*lock += 1;
println!("{} {} {}", i, j, lock);
}
});
}

loop {
if *count.lock().await >= 50 {
break;
}
}
println!("Count hit 50.");
}
```

There are a few things of note here to pay attention to in this example.

1.  The mutex is wrapped in an `Arc` to allow it to be shared across threads.
2.  Each spawned task obtains a lock and releases it on every iteration.
3.  Mutation of the data protected by the Mutex is done by de-referencing the obtained lock as seen on lines 13 and 20.

Tokio’s Mutex works in a simple FIFO (first in, first out) style where all calls to `lock` complete in the order they were performed. In that way the Mutex is “fair” and predictable in how it distributes the locks to inner data. Locks are released and reacquired after every iteration, so basically, each thread goes to the back of the line after it increments the value once. Note that there’s some unpredictability to the timing between when the threads are started, but once they are going they alternate predictably. Finally, since there is only a single valid lock at any given time, there is no possibility of a race condition when mutating the inner value.

Note that in contrast to `std::sync::Mutex`, this implementation does not poison the mutex when a thread holding the `MutexGuard` panics. In such a case, the mutex will be unlocked. If the panic is caught, this might leave the data protected by the mutex in an inconsistent state.

Implementations§
----------------

Source§

### impl<T: ?Sized\> Mutex<T>

Source

#### pub fn new(t: T) -> Self

where T: Sized,

Creates a new lock in an unlocked state ready for use.

##### §Examples

```
use tokio::sync::Mutex;

let lock = Mutex::new(5);
```

Source

#### pub const fn const\_new(t: T) -> Self

where T: Sized,

Creates a new lock in an unlocked state ready for use.

When using the `tracing` unstable feature, a `Mutex` created with `const_new` will not be instrumented. As such, it will not be visible in `tokio-console`. Instead, `Mutex::new` should be used to create an instrumented object if that is needed.

##### §Examples

```
use tokio::sync::Mutex;

static LOCK: Mutex<i32> = Mutex::const_new(5);
```

Source

#### pub async fn lock(&self) -> MutexGuard<'\_, T>

Locks this mutex, causing the current task to yield until the lock has been acquired. When the lock has been acquired, function returns a `MutexGuard`.

If the mutex is available to be acquired immediately, then this call will typically not yield to the runtime. However, this is not guaranteed under all circumstances.

##### §Cancel safety

This method uses a queue to fairly distribute locks in the order they were requested. Cancelling a call to `lock` makes you lose your place in the queue.

##### §Examples

```
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
let mutex = Mutex::new(1);

let mut n = mutex.lock().await;
*n = 2;
}
```

Source

#### pub fn blocking\_lock(&self) -> MutexGuard<'\_, T>

Blockingly locks this `Mutex`. When the lock has been acquired, function returns a `MutexGuard`.

This method is intended for use cases where you need to use this mutex in asynchronous code as well as in synchronous code.

##### §Panics

This function panics if called within an asynchronous execution context.

*   If you find yourself in an asynchronous execution context and needing to call some (synchronous) function which performs one of these `blocking_` operations, then consider wrapping that call inside `spawn_blocking()` (or `block_in_place()`).

##### §Examples

```
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
let mutex =  Arc::new(Mutex::new(1));
let lock = mutex.lock().await;

let mutex1 = Arc::clone(&mutex);
let blocking_task = tokio::task::spawn_blocking(move || {
// This shall block until the `lock` is released.
let mut n = mutex1.blocking_lock();
*n = 2;
});

assert_eq!(*lock, 1);
// Release the lock.
drop(lock);

// Await the completion of the blocking task.
blocking_task.await.unwrap();

// Assert uncontended.
let n = mutex.try_lock().unwrap();
assert_eq!(*n, 2);
}
```
Source

#### pub fn blocking\_lock\_owned(self: Arc<Self>) -> OwnedMutexGuard<T>

Blockingly locks this `Mutex`. When the lock has been acquired, function returns an `OwnedMutexGuard`.

This method is identical to `Mutex::blocking_lock`, except that the returned guard references the `Mutex` with an `Arc` rather than by borrowing it. Therefore, the `Mutex` must be wrapped in an `Arc` to call this method, and the guard will live for the `'static` lifetime, as it keeps the `Mutex` alive by holding an `Arc`.

##### §Panics

This function panics if called within an asynchronous execution context.

*   If you find yourself in an asynchronous execution context and needing to call some (synchronous) function which performs one of these `blocking_` operations, then consider wrapping that call inside `spawn_blocking()` (or `block_in_place()`).

##### §Examples

```
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
let mutex =  Arc::new(Mutex::new(1));
let lock = mutex.lock().await;

let mutex1 = Arc::clone(&mutex);
let blocking_task = tokio::task::spawn_blocking(move || {
// This shall block until the `lock` is released.
let mut n = mutex1.blocking_lock_owned();
*n = 2;
});

assert_eq!(*lock, 1);
// Release the lock.
drop(lock);

// Await the completion of the blocking task.
blocking_task.await.unwrap();

// Assert uncontended.
let n = mutex.try_lock().unwrap();
assert_eq!(*n, 2);
}
```

Source

#### pub async fn lock\_owned(self: Arc<Self>) -> OwnedMutexGuard<T>

Locks this mutex, causing the current task to yield until the lock has been acquired. When the lock has been acquired, this returns an `OwnedMutexGuard`.

If the mutex is available to be acquired immediately, then this call will typically not yield to the runtime. However, this is not guaranteed under all circumstances.

This method is identical to `Mutex::lock`, except that the returned guard references the `Mutex` with an `Arc` rather than by borrowing it. Therefore, the `Mutex` must be wrapped in an `Arc` to call this method, and the guard will live for the `'static` lifetime, as it keeps the `Mutex` alive by holding an `Arc`.

##### §Cancel safety

This method uses a queue to fairly distribute locks in the order they were requested. Cancelling a call to `lock_owned` makes you lose your place in the queue.

##### §Examples

```
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
let mutex = Arc::new(Mutex::new(1));

let mut n = mutex.clone().lock_owned().await;
*n = 2;
}
```

Source

#### pub fn try\_lock(&self) -> Result<MutexGuard<'\_, T>, TryLockError\>

Attempts to acquire the lock, and returns `TryLockError` if the lock is currently held somewhere else.

##### §Examples

```
use tokio::sync::Mutex;

let mutex = Mutex::new(1);

let n = mutex.try_lock()?;
assert_eq!(*n, 1);
```

Source

#### pub fn get\_mut(&mut self) -> &mut T

Returns a mutable reference to the underlying data.

Since this call borrows the `Mutex` mutably, no actual locking needs to take place – the mutable borrow statically guarantees no locks exist.

##### §Examples

```
use tokio::sync::Mutex;

fn main() {
let mut mutex = Mutex::new(1);

let n = mutex.get_mut();
*n = 2;
}
```

Source

#### pub fn try\_lock\_owned( self: Arc<Self>, ) -> Result<OwnedMutexGuard<T>, TryLockError\>

Attempts to acquire the lock, and returns `TryLockError` if the lock is currently held somewhere else.

This method is identical to `Mutex::try_lock`, except that the returned guard references the `Mutex` with an `Arc` rather than by borrowing it. Therefore, the `Mutex` must be wrapped in an `Arc` to call this method, and the guard will live for the `'static` lifetime, as it keeps the `Mutex` alive by holding an `Arc`.

##### §Examples

```
use tokio::sync::Mutex;
use std::sync::Arc;

let mutex = Arc::new(Mutex::new(1));

let n = mutex.clone().try_lock_owned()?;
assert_eq!(*n, 1);
```

Source

#### pub fn into\_inner(self) -> T

where T: Sized,

Consumes the mutex, returning the underlying data.

##### §Examples

```
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
let mutex = Mutex::new(1);

let n = mutex.into_inner();
assert_eq!(n, 1);
}
```

Trait Implementations§
----------------------

Source§

### impl<T> Debug for Mutex<T>

where T: Debug + ?Sized,

Source§

#### fn fmt(&self, f: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

Source§

### impl<T> Default for Mutex<T>

where T: Default,

Source§

#### fn default() -> Self

Returns the “default value” for a type. Read more

Source§

### impl<T> From<T> for Mutex<T>

Source§

#### fn from(s: T) -> Self

Converts to this type from the input type.

Source§

### impl<T> Send for Mutex<T>

where T: ?Sized + Send,

Source§

### impl<T> Sync for Mutex<T>

where T: ?Sized + Send,

Auto Trait Implementations§
---------------------------

§

### impl<T> !Freeze for Mutex<T>

§

### impl<T> !RefUnwindSafe for Mutex<T>

§

### impl<T> Unpin for Mutex<T>

where T: Unpin + ?Sized,

§

### impl<T> !UnwindSafe for Mutex<T>

Blanket Implementations§
------------------------

Source§

### impl<T> Any for T

where T: 'static + ?Sized,

Source§

#### fn type\_id(&self) -> TypeId

Gets the `TypeId` of `self`. Read more

Source§

### impl<T> Borrow<T> for T

where T: ?Sized,

Source§

#### fn borrow(&self) -> &T

Immutably borrows from an owned value. Read more

Source§

### impl<T> BorrowMut<T> for T

where T: ?Sized,

Source§

#### fn borrow\_mut(&mut self) -> &mut T

Mutably borrows from an owned value. Read more

Source§

### impl<T> From<!\> for T

Source§

#### fn from(t: !) -> T

Converts to this type from the input type.

Source§

### impl<T> From<T> for T

Source§

#### fn from(t: T) -> T

Returns the argument unchanged.

Source§

### impl<T> Instrument for T

Source§

#### fn instrument(self, span: Span) -> Instrumented<Self> ⓘ

Instruments this type with the provided `Span`, returning an `Instrumented` wrapper. Read more

Source§

#### fn in\_current\_span(self) -> Instrumented<Self> ⓘ

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

Source§

### impl<T, U> Into<U> for T

where U: From<T>,

Source§

#### fn into(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

Source§

### impl<T, U> TryFrom<U> for T

where U: Into<T>,

Source§

#### type Error = Infallible

The type returned in the event of a conversion error.

Source§

#### fn try\_from(value: U) -> Result<T, <T as TryFrom<U>>::Error\>

Performs the conversion.

Source§

### impl<T, U> TryInto<U> for T

where U: TryFrom<T>,

Source§

#### type Error = <U as TryFrom<T>>::Error

The type returned in the event of a conversion error.

Source§

#### fn try\_into(self) -> Result<U, <U as TryFrom<T>>::Error\>

Performs the conversion.

Source§

### impl<T> WithSubscriber for T

Source§

#### fn with\_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> ⓘ

where S: Into<Dispatch\>,

Attaches the provided `Subscriber` to this type, returning a `WithDispatch` wrapper. Read more

Source§

#### fn with\_current\_subscriber(self) -> WithDispatch<Self> ⓘ

Attaches the current default `Subscriber` to this type, returning a `WithDispatch` wrapper. Read more