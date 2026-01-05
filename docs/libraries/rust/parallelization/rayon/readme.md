# GitHub - rayon-rs/rayon: Rayon: A data parallelism library for Rust

## Rayon

[](#rayon)

[![Rayon crate](https://camo.githubusercontent.com/ad32a4cf2d8e4a8775419b93a8360dd33437052e26fb8004f6c6209a816bce9e/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f7261796f6e2e737667)](https://crates.io/crates/rayon) [![Rayon documentation](https://camo.githubusercontent.com/407023f84609b74e7df005b9bed1f874d86528b12bc5c82b61ad37a33084a004/68747470733a2f2f646f63732e72732f7261796f6e2f62616467652e737667)](https://docs.rs/rayon) [![minimum rustc 1.63](https://camo.githubusercontent.com/53788bb70a95951367650402b70d398fb69333f4a23506f5fc2c58fbdfbf7458/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f72757374632d312e36332b2d7265642e737667)](https://camo.githubusercontent.com/53788bb70a95951367650402b70d398fb69333f4a23506f5fc2c58fbdfbf7458/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f72757374632d312e36332b2d7265642e737667) [![build status](https://github.com/rayon-rs/rayon/workflows/main/badge.svg)](https://github.com/rayon-rs/rayon/actions)

Rayon is a data-parallelism library for Rust. It is extremely lightweight and makes it easy to convert a sequential computation into a parallel one. It also guarantees data-race freedom. (You may also enjoy [this blog post](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/) about Rayon, which gives more background and details about how it works, or [this video](https://www.youtube.com/watch?v=gof_OEv71Aw), from the Rust Belt Rust conference.) Rayon is [available on crates.io](https://crates.io/crates/rayon), and [API documentation is available on docs.rs](https://docs.rs/rayon).

## Parallel iterators and more

[](#parallel-iterators-and-more)

Rayon makes it drop-dead simple to convert sequential iterators into parallel ones: usually, you just change your `foo.iter()` call into `foo.par_iter()`, and Rayon does the rest:

```
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

[Parallel iterators](https://docs.rs/rayon/*/rayon/iter/index.html) take care of deciding how to divide your data into tasks; it will dynamically adapt for maximum performance. If you need more flexibility than that, Rayon also offers the [join](https://docs.rs/rayon/*/rayon/fn.join.html) and [scope](https://docs.rs/rayon/*/rayon/fn.scope.html) functions, which let you create parallel tasks on your own. For even more control, you can create [custom threadpools](https://docs.rs/rayon/*/rayon/struct.ThreadPool.html) rather than using Rayon's default, global threadpool.

## No data races

[](#no-data-races)

You may have heard that parallel execution can produce all kinds of crazy bugs. Well, rest easy. Rayon's APIs all guarantee **data-race freedom**, which generally rules out most parallel bugs (though not all). In other words, **if your code compiles**, it typically does the same thing it did before.

For the most, parallel iterators in particular are guaranteed to produce the same results as their sequential counterparts. One caveat: If your iterator has side effects (for example, sending methods to other threads through a [Rust channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) or writing to disk), those side effects may occur in a different order. Note also that, in some cases, parallel iterators offer alternative versions of the sequential iterator methods that can have higher performance.

## Using Rayon

[](#using-rayon)

[Rayon is available on crates.io](https://crates.io/crates/rayon). The recommended way to use it is to add a line into your Cargo.toml such as:

```
[dependencies]
rayon = "1.10"
```

To use the parallel iterator APIs, a number of traits have to be in scope. The easiest way to bring those things into scope is to use the [Rayon prelude](https://docs.rs/rayon/*/rayon/prelude/index.html). In each module where you would like to use the parallel iterator APIs, just add:

Rayon currently requires `rustc 1.63.0` or greater.

### Usage with WebAssembly

[](#usage-with-webassembly)

By default, when building to WebAssembly, Rayon will treat it as any other platform without multithreading support and will fall back to sequential iteration. This allows existing code to compile and run successfully with no changes necessary, but it will run slower as it will only use a single CPU core.

You can build Rayon-based projects with proper multithreading support for the Web, but you'll need an adapter and some project configuration to account for differences between WebAssembly threads and threads on the other platforms.

Check out the [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon) docs for more details.

## Contribution

[](#contribution)

Rayon is an open source project! If you'd like to contribute to Rayon, check out [the list of "help wanted" issues](https://github.com/rayon-rs/rayon/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22). These are all (or should be) issues that are suitable for getting started, and they generally include a detailed set of instructions for what to do. Please ask questions if anything is unclear! Also, check out the [Guide to Development](https://github.com/rayon-rs/rayon/wiki/Guide-to-Development) page on the wiki. Note that all code submitted in PRs to Rayon is assumed to [be licensed under Rayon's dual MIT/Apache 2.0 licensing](https://github.com/rayon-rs/rayon/blob/main/README.md#license).

## Quick demo

[](#quick-demo)

To see Rayon in action, check out the `rayon-demo` directory, which includes a number of demos of code using Rayon. For example, run this command to get a visualization of an N-body simulation. To see the effect of using Rayon, press `s` to run sequentially and `p` to run in parallel.

```
> cd rayon-demo
> cargo run --release -- nbody visualize

```

For more information on demos, try:

```
> cd rayon-demo
> cargo run --release -- --help

```

## Other questions?

[](#other-questions)

See [the Rayon FAQ](https://github.com/rayon-rs/rayon/blob/main/FAQ.md).

## License

[](#license)

Rayon is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-APACHE](https://github.com/rayon-rs/rayon/blob/main/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/rayon-rs/rayon/blob/main/LICENSE-MIT) for details. Opening a pull request is assumed to signal agreement with these licensing terms.
