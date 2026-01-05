# rustfft - Rust

Expand description

RustFFT is a high-performance FFT library written in pure Rust.

On X86_64, RustFFT supports the AVX instruction set for increased performance. No special code is needed to activate AVX: Simply plan a FFT using the FftPlanner on a machine that supports the `avx` and `fma` CPU features, and RustFFT will automatically switch to faster AVX-accelerated algorithms.

For machines that do not have AVX, RustFFT also supports the SSE4.1 instruction set. As for AVX, this is enabled automatically when using the FftPlanner.

Additionally, there is automatic support for the Neon instruction set on AArch64, and support for WASM SIMD when compiling for WASM targets.

#### [§](#usage)Usage

The recommended way to use RustFFT is to create a [`FftPlanner`](struct.FftPlanner.html 'struct rustfft::FftPlanner') instance and then call its [`plan_fft`](about:blank/struct.FftPlanner.html#method.plan_fft 'method rustfft::FftPlanner::plan_fft') method. This method will automatically choose which FFT algorithms are best for a given size and initialize the required buffers and precomputed data.

```
// Perform a forward FFT of size 1234
use rustfft::{FftPlanner, num_complex::Complex};

let mut planner = FftPlanner::new();
let fft = planner.plan_fft_forward(1234);

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
fft.process(&mut buffer);
```

The planner returns trait objects of the [`Fft`](trait.Fft.html 'trait rustfft::Fft') trait, allowing for FFT sizes that aren’t known until runtime.

RustFFT also exposes individual FFT algorithms. For example, if you know beforehand that you need a power-of-two FFT, you can avoid the overhead of the planner and trait object by directly creating instances of the [`Radix4`](algorithm/struct.Radix4.html 'struct rustfft::algorithm::Radix4') algorithm:

```
// Computes a forward FFT of size 4096
use rustfft::{Fft, FftDirection, num_complex::Complex, algorithm::Radix4};

let fft = Radix4::new(4096, FftDirection::Forward);

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 4096];
fft.process(&mut buffer);
```

For the vast majority of situations, simply using the [`FftPlanner`](struct.FftPlanner.html 'struct rustfft::FftPlanner') will be enough, but advanced users may have better insight than the planner into which algorithms are best for a specific size. See the [`algorithm`](algorithm/index.html 'mod rustfft::algorithm') module for a complete list of scalar algorithms implemented by RustFFT.

Users should beware, however, that bypassing the planner will disable all AVX, SSE, Neon, and WASM SIMD optimizations.

#### [§](#feature-flags)Feature Flags

- `avx` (Enabled by default)

  On x86_64, the `avx` feature enables compilation of AVX-accelerated code. Enabling it greatly improves performance if the client CPU supports AVX and FMA, while disabling it reduces compile time and binary size.

  On every platform besides x86_64, this feature does nothing, and RustFFT will behave like it’s not set.

- `sse` (Enabled by default)

  On x86_64, the `sse` feature enables compilation of SSE4.1-accelerated code. Enabling it improves performance if the client CPU supports SSE4.1, while disabling it reduces compile time and binary size. If AVX is also supported and its feature flag is enabled, RustFFT will use AVX instead of SSE4.1.

  On every platform besides x86_64, this feature does nothing, and RustFFT will behave like it’s not set.

- `neon` (Enabled by default)

  On AArch64 (64-bit ARM) the `neon` feature enables compilation of Neon-accelerated code. Enabling it improves performance, while disabling it reduces compile time and binary size.

  On every platform besides AArch64, this feature does nothing, and RustFFT will behave like it’s not set.

- `wasm_simd` (Disabled by default)

  On the WASM platform, this feature enables compilation of WASM SIMD accelerated code.

  To execute binaries compiled with `wasm_simd`, you need a [target browser or runtime which supports `fixed-width SIMD`](https://webassembly.org/roadmap/). If you run your SIMD accelerated code on an unsupported platform, WebAssembly will specify a [trap](https://webassembly.github.io/spec/core/intro/overview.html#trap) leading to immediate execution cancelation.

  On every platform besides WASM, this feature does nothing and RustFFT will behave like it is not set.

#### [§](#normalization)Normalization

RustFFT does not normalize outputs. Callers must manually normalize the results by scaling each element by `1/len().sqrt()`. Multiple normalization steps can be merged into one via pairwise multiplication, so when doing a forward FFT followed by an inverse callers can normalize once by scaling each element by `1/len()`

#### [§](#output-order)Output Order

Elements in the output are ordered by ascending frequency, with the first element corresponding to frequency 0.

#### [§](#avx-performance-tips)AVX Performance Tips

In any FFT computation, the time required to compute a FFT of size N relies heavily on the [prime factorization](https://en.wikipedia.org/wiki/Integer_factorization) of N. If N’s prime factors are all very small, computing a FFT of size N will be fast, and it’ll be slow if N has large prime factors, or if N is a prime number.

In most FFT libraries (Including RustFFT when using non-AVX code), power-of-two FFT sizes are the fastest, and users see a steep falloff in performance when using non-power-of-two sizes. Thankfully, RustFFT using AVX acceleration is not quite as restrictive:

- Any FFT whose size is of the form `2^n * 3^m` can be considered the “fastest” in RustFFT.
- Any FFT whose prime factors are all 11 or smaller will also be very fast, but the fewer the factors of 2 and 3 the slower it will be. For example, computing a FFT of size 13552 `(2^4*7*11*11)` is takes 12% longer to compute than 13824 `(2^9 * 3^3)`, and computing a FFT of size 2541 `(3*7*11*11)` takes 65% longer to compute than 2592 `(2^5 * 3^4)`
- Any other FFT size will be noticeably slower. A considerable amount of effort has been put into making these FFT sizes as fast as they can be, but some FFT sizes just take more work than others. For example, computing a FFT of size 5183 `(71 * 73)` takes about 5x longer than computing a FFT of size 5184 `(2^6 * 3^4)`.

In most cases, even prime-sized FFTs will be fast enough for your application. In the example of 5183 above, even that “slow” FFT only takes a few tens of microseconds to compute.

Some applications of the FFT allow for choosing an arbitrary FFT size (In many applications the size is pre-determined by whatever you’re computing). If your application supports choosing your own size, our advice is still to start by trying the size that’s most convenient to your application. If that’s too slow, see if you can find a nearby size whose prime factors are all 11 or smaller, and you can expect a 2x-5x speedup. If that’s still too slow, find a nearby size whose prime factors are all 2 or 3, and you can expect a 1.1x-1.5x speedup.

`pub use [num_complex](https://docs.rs/num-complex/0.4.3/x86_64-unknown-linux-gnu/num_complex/index.html "mod num_complex");`

`pub use [num_traits](https://docs.rs/num-traits/0.2.15/x86_64-unknown-linux-gnu/num_traits/index.html "mod num_traits");`

[algorithm](algorithm/index.html 'mod rustfft::algorithm')

Individual FFT algorithms

[FftPlanner](struct.FftPlanner.html 'struct rustfft::FftPlanner')

The FFT planner creates new FFT algorithm instances.

[FftPlannerAvx](struct.FftPlannerAvx.html 'struct rustfft::FftPlannerAvx')

The AVX FFT planner creates new FFT algorithm instances which take advantage of the AVX instruction set.

[FftPlannerNeon](struct.FftPlannerNeon.html 'struct rustfft::FftPlannerNeon')

The Neon FFT planner creates new FFT algorithm instances using a mix of scalar and Neon accelerated algorithms. It is supported when using the 64-bit AArch64 instruction set.

[FftPlannerScalar](struct.FftPlannerScalar.html 'struct rustfft::FftPlannerScalar')

The Scalar FFT planner creates new FFT algorithm instances using non-SIMD algorithms.

[FftPlannerSse](struct.FftPlannerSse.html 'struct rustfft::FftPlannerSse')

The SSE FFT planner creates new FFT algorithm instances using a mix of scalar and SSE accelerated algorithms. It requires at least SSE4.1, which is available on all reasonably recent x86_64 cpus.

[FftPlannerWasmSimd](struct.FftPlannerWasmSimd.html 'struct rustfft::FftPlannerWasmSimd')

The WASM FFT planner creates new FFT algorithm instances using a mix of scalar and WASM SIMD accelerated algorithms. It is supported when using fairly recent browser versions as outlined in [the WebAssembly roadmap](https://webassembly.org/roadmap/).

[FftDirection](enum.FftDirection.html 'enum rustfft::FftDirection')

Represents a FFT direction, IE a forward FFT or an inverse FFT

[Direction](trait.Direction.html 'trait rustfft::Direction')

A trait that allows FFT algorithms to report whether they compute forward FFTs or inverse FFTs

[Fft](trait.Fft.html 'trait rustfft::Fft')

Trait for algorithms that compute FFTs.

[FftNum](trait.FftNum.html 'trait rustfft::FftNum')

Generic floating point number, implemented for f32 and f64

[Length](trait.Length.html 'trait rustfft::Length')

A trait that allows FFT algorithms to report their expected input/output size
