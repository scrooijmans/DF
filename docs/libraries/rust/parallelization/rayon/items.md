# rayon - Rust

Expand description

Rayon is a data-parallelism library that makes it easy to convert sequential computations into parallel.

It is lightweight and convenient for introducing parallelism into existing code. It guarantees data-race free executions and takes advantage of parallelism when sensible, based on work-load at runtime.

## [§](#how-to-use-rayon)How to use Rayon

There are two ways to use Rayon:

- **High-level parallel constructs** are the simplest way to use Rayon and also typically the most efficient.
  - [Parallel iterators](iter/index.html) make it easy to convert a sequential iterator to execute in parallel.
    - The [`ParallelIterator`](iter/trait.ParallelIterator.html) trait defines general methods for all parallel iterators.
    - The [`IndexedParallelIterator`](iter/trait.IndexedParallelIterator.html) trait adds methods for iterators that support random access.
  - The [`par_sort`](about:blank/slice/trait.ParallelSliceMut.html#method.par_sort) method sorts `&mut [T]` slices (or vectors) in parallel.
  - [`par_extend`](about:blank/iter/trait.ParallelExtend.html#tymethod.par_extend) can be used to efficiently grow collections with items produced by a parallel iterator.
- **Custom tasks** let you divide your work into parallel tasks yourself.
  - [`join`](fn.join.html) is used to subdivide a task into two pieces.
  - [`scope`](fn.scope.html) creates a scope within which you can create any number of parallel tasks.
  - [`ThreadPoolBuilder`](struct.ThreadPoolBuilder.html) can be used to create your own thread pools or customize the global one.

## [§](#basic-usage-and-the-rayon-prelude)Basic usage and the Rayon prelude

First, you will need to add `rayon` to your `Cargo.toml`.

Next, to use parallel iterators or the other high-level methods, you need to import several traits. Those traits are bundled into the module [`rayon::prelude`](prelude/index.html). It is recommended that you import all of these traits at once by adding `use rayon::prelude::*` at the top of each module that uses Rayon methods.

These traits give you access to the `par_iter` method which provides parallel implementations of many iterative functions such as [`map`](about:blank/iter/trait.ParallelIterator.html#method.map), [`for_each`](about:blank/iter/trait.ParallelIterator.html#method.for_each), [`filter`](about:blank/iter/trait.ParallelIterator.html#method.filter), [`fold`](about:blank/iter/trait.ParallelIterator.html#method.fold), and [more](about:blank/iter/trait.ParallelIterator.html#provided-methods).

## [§](#crate-layout)Crate Layout

Rayon extends many of the types found in the standard library with parallel iterator implementations. The modules in the `rayon` crate mirror [`std`](https://doc.rust-lang.org/std/) itself: so, e.g., the `option` module in Rayon contains parallel iterators for the `Option` type, which is found in [the `option` module of `std`](https://doc.rust-lang.org/std/option/index.html). Similarly, the `collections` module in Rayon offers parallel iterator types for [the `collections` from `std`](https://doc.rust-lang.org/std/collections/index.html). You will rarely need to access these submodules unless you need to name iterator types explicitly.

## [§](#targets-without-threading)Targets without threading

Rayon has limited support for targets without `std` threading implementations. See the [`rayon_core`](https://docs.rs/rayon-core/1.12.1/x86_64-unknown-linux-gnu/rayon_core/index.html 'mod rayon_core') documentation for more information about its global fallback.

## [§](#other-questions)Other questions?

See [the Rayon FAQ](https://github.com/rayon-rs/rayon/blob/main/FAQ.md).

[array](array/index.html 'mod rayon::array')

Parallel iterator types for [arrays](https://doc.rust-lang.org/std/primitive.array.html) (`[T; N]`)

[collections](collections/index.html 'mod rayon::collections')

Parallel iterator types for [standard collections](https://doc.rust-lang.org/stable/std/collections/)

[iter](iter/index.html 'mod rayon::iter')

Traits for writing parallel programs using an iterator-style interface

[option](option/index.html 'mod rayon::option')

Parallel iterator types for [options](https://doc.rust-lang.org/stable/std/option/)

[prelude](prelude/index.html 'mod rayon::prelude')

The rayon prelude imports the various `ParallelIterator` traits. The intention is that one can include `use rayon::prelude::*` and have easy access to the various traits and methods you will need.

[range](range/index.html 'mod rayon::range')

Parallel iterator types for [ranges](https://doc.rust-lang.org/core/ops/struct.Range.html), the type for values created by `a..b` expressions

[range_inclusive](range_inclusive/index.html 'mod rayon::range_inclusive')

Parallel iterator types for [inclusive ranges](https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html), the type for values created by `a..=b` expressions

[result](result/index.html 'mod rayon::result')

Parallel iterator types for [results](https://doc.rust-lang.org/stable/std/result/)

[slice](slice/index.html 'mod rayon::slice')

Parallel iterator types for [slices](https://doc.rust-lang.org/stable/std/slice/)

[str](str/index.html 'mod rayon::str')

Parallel iterator types for [strings](https://doc.rust-lang.org/stable/std/str/)

[string](string/index.html 'mod rayon::string')

This module contains the parallel iterator types for owned strings (`String`). You will rarely need to interact with it directly unless you have need to name one of the iterator types.

[vec](vec/index.html 'mod rayon::vec')

Parallel iterator types for [vectors](https://doc.rust-lang.org/stable/std/vec/) (`Vec<T>`)

[BroadcastContext](struct.BroadcastContext.html 'struct rayon::BroadcastContext')

Provides context to a closure called by `broadcast`.

[FnContext](struct.FnContext.html 'struct rayon::FnContext')

Provides the calling context to a closure called by `join_context`.

[Scope](struct.Scope.html 'struct rayon::Scope')

Represents a fork-join scope which can be used to spawn any number of tasks. See [`scope()`](fn.scope.html) for more information.

[ScopeFifo](struct.ScopeFifo.html 'struct rayon::ScopeFifo')

Represents a fork-join scope which can be used to spawn any number of tasks. Those spawned from the same thread are prioritized in relative FIFO order. See [`scope_fifo()`](fn.scope_fifo.html) for more information.

[ThreadBuilder](struct.ThreadBuilder.html 'struct rayon::ThreadBuilder')

Thread builder used for customization via [`ThreadPoolBuilder::spawn_handler`](about:blank/struct.ThreadPoolBuilder.html#method.spawn_handler).

[ThreadPool](struct.ThreadPool.html 'struct rayon::ThreadPool')

Represents a user created [thread-pool](https://en.wikipedia.org/wiki/Thread_pool).

[ThreadPoolBuildError](struct.ThreadPoolBuildError.html 'struct rayon::ThreadPoolBuildError')

Error when initializing a thread pool.

[ThreadPoolBuilder](struct.ThreadPoolBuilder.html 'struct rayon::ThreadPoolBuilder')

Used to create a new [`ThreadPool`](struct.ThreadPool.html) or to configure the global rayon thread pool.

[Yield](enum.Yield.html 'enum rayon::Yield')

Result of [`yield_now()`](fn.yield_now.html 'fn rayon::yield_now') or [`yield_local()`](fn.yield_local.html 'fn rayon::yield_local').

[broadcast](fn.broadcast.html 'fn rayon::broadcast')

Executes `op` within every thread in the current threadpool. If this is called from a non-Rayon thread, it will execute in the global threadpool. Any attempts to use `join`, `scope`, or parallel iterators will then operate within that threadpool. When the call has completed on each thread, returns a vector containing all of their return values.

[current_num_threads](fn.current_num_threads.html 'fn rayon::current_num_threads')

Returns the number of threads in the current registry. If this code is executing within a Rayon thread-pool, then this will be the number of threads for the thread-pool of the current thread. Otherwise, it will be the number of threads for the global thread-pool.

[current_thread_index](fn.current_thread_index.html 'fn rayon::current_thread_index')

If called from a Rayon worker thread, returns the index of that thread within its current pool; if not called from a Rayon thread, returns `None`.

[in_place_scope](fn.in_place_scope.html 'fn rayon::in_place_scope')

Creates a “fork-join” scope `s` and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete.

[in_place_scope_fifo](fn.in_place_scope_fifo.html 'fn rayon::in_place_scope_fifo')

Creates a “fork-join” scope `s` with FIFO order, and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete.

[join](fn.join.html 'fn rayon::join')

Takes two closures and _potentially_ runs them in parallel. It returns a pair of the results from those closures.

[join_context](fn.join_context.html 'fn rayon::join_context')

Identical to `join`, except that the closures have a parameter that provides context for the way the closure has been called, especially indicating whether they’re executing on a different thread than where `join_context` was called. This will occur if the second job is stolen by a different thread, or if `join_context` was called from outside the thread pool to begin with.

[max_num_threads](fn.max_num_threads.html 'fn rayon::max_num_threads')

Returns the maximum number of threads that Rayon supports in a single thread-pool.

[scope](fn.scope.html 'fn rayon::scope')

Creates a “fork-join” scope `s` and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete.

[scope_fifo](fn.scope_fifo.html 'fn rayon::scope_fifo')

Creates a “fork-join” scope `s` with FIFO order, and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete.

[spawn](fn.spawn.html 'fn rayon::spawn')

Puts the task into the Rayon threadpool’s job queue in the “static” or “global” scope. Just like a standard thread, this task is not tied to the current stack frame, and hence it cannot hold any references other than those with `'static` lifetime. If you want to spawn a task that references stack data, use [the `scope()` function](fn.scope.html) to create a scope.

[spawn_broadcast](fn.spawn_broadcast.html 'fn rayon::spawn_broadcast')

Spawns an asynchronous task on every thread in this thread-pool. This task will run in the implicit, global scope, which means that it may outlast the current stack frame – therefore, it cannot capture any references onto the stack (you will likely need a `move` closure).

[spawn_fifo](fn.spawn_fifo.html 'fn rayon::spawn_fifo')

Fires off a task into the Rayon threadpool in the “static” or “global” scope. Just like a standard thread, this task is not tied to the current stack frame, and hence it cannot hold any references other than those with `'static` lifetime. If you want to spawn a task that references stack data, use [the `scope_fifo()` function](fn.scope_fifo.html) to create a scope.

[yield_local](fn.yield_local.html 'fn rayon::yield_local')

Cooperatively yields execution to local Rayon work.

[yield_now](fn.yield_now.html 'fn rayon::yield_now')

Cooperatively yields execution to Rayon.
