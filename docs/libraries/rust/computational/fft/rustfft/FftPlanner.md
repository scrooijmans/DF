# FftPlanner in rustfft - Rust

```
pub struct FftPlanner<T: FftNum> { /* private fields */ }
```

Expand description

The FFT planner creates new FFT algorithm instances.

RustFFT has several FFT algorithms available. For a given FFT size, the `FftPlanner` decides which of the available FFT algorithms to use and then initializes them.

```
// Perform a forward Fft of size 1234
use std::sync::Arc;
use rustfft::{FftPlanner, num_complex::Complex};

let mut planner = FftPlanner::new();
let fft = planner.plan_fft_forward(1234);

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
fft.process(&mut buffer);

// The FFT instance returned by the planner has the type `Arc<dyn Fft<T>>`,
// where T is the numeric type, ie f32 or f64, so it's cheap to clone
let fft_clone = Arc::clone(&fft);
```

If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This is because the planner re-uses internal data across FFT instances wherever possible, saving memory and reducing setup time. (FFT instances created with one planner will never re-use data and buffers with FFT instances created by a different planner)

Each FFT instance owns [`Arc`s](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html 'struct alloc::sync::Arc') to its internal data, rather than borrowing it from the planner, so it’s perfectly safe to drop the planner after creating Fft instances.

In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with the fastest available instruction set. If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs.

If you’d prefer not to compute a FFT at all if a certain SIMD instruction set isn’t available, or otherwise specify your own custom fallback, RustFFT exposes dedicated planners for each instruction set:

- [`FftPlannerAvx`](struct.FftPlannerAvx.html 'struct rustfft::FftPlannerAvx')
- [`FftPlannerSse`](struct.FftPlannerSse.html 'struct rustfft::FftPlannerSse')
- [`FftPlannerNeon`](struct.FftPlannerNeon.html 'struct rustfft::FftPlannerNeon')
- [`FftPlannerWasmSimd`](struct.FftPlannerWasmSimd.html 'struct rustfft::FftPlannerWasmSimd')

If you’d prefer to opt out of SIMD algorithms, consider creating a [`FftPlannerScalar`](struct.FftPlannerScalar.html 'struct rustfft::FftPlannerScalar') instead.

[Source](about:blank/src/rustfft/plan.rs.html#69-125)
[§](#impl-FftPlanner%3CT%3E)

[Source](about:blank/src/rustfft/plan.rs.html#71-93)

Creates a new `FftPlanner` instance.

[Source](about:blank/src/rustfft/plan.rs.html#100-110)

Returns a `Fft` instance which computes FFTs of size `len`.

If the provided `direction` is `FftDirection::Forward`, the returned instance will compute forward FFTs. If it’s `FftDirection::Inverse`, it will compute inverse FFTs.

If this is called multiple times, the planner will attempt to re-use internal data between calls, reducing memory usage and FFT initialization time.

[Source](about:blank/src/rustfft/plan.rs.html#115-117)

Returns a `Fft` instance which computes forward FFTs of size `len`

If this is called multiple times, the planner will attempt to re-use internal data between calls, reducing memory usage and FFT initialization time.

[Source](about:blank/src/rustfft/plan.rs.html#122-124)

Returns a `Fft` instance which computes inverse FFTs of size `len`

If this is called multiple times, the planner will attempt to re-use internal data between calls, reducing memory usage and FFT initialization time.
