# ndrustfft - Rust

Expand description

## [§](#ndrustfft-n-dimensional-complex-to-complex-fft-real-to-complex-fft-and-real-to-real-dct)ndrustfft: _n_\-dimensional complex-to-complex FFT, real-to-complex FFT and real-to-real DCT

This library is a wrapper for `RustFFT`, `RustDCT` and `RealFft` that enables performing FFTs and DCTs of complex- and real-valued data on _n_\-dimensional arrays (ndarray).

ndrustfft provides Handler structs for FFT’s and DCTs, which must be provided alongside with the arrays to the respective functions (see below) . The Handlers implement a process function, which is a wrapper around Rustfft’s process. Transforms along the outermost axis are in general the fastest, while transforms along other axis’ will temporarily create copies of the input array.

### [§](#parallel)Parallel

The library ships all functions with a parallel version which leverages the parallel iterators of the ndarray crate.

### [§](#available-transforms)Available transforms

#### [§](#complex-to-complex)Complex-to-complex

- `fft` : [`ndfft`](fn.ndfft.html 'fn ndrustfft::ndfft'), [`ndfft_par`](fn.ndfft_par.html 'fn ndrustfft::ndfft_par')
- `ifft`: [`ndifft`](fn.ndifft.html 'fn ndrustfft::ndifft'),[`ndifft_par`](fn.ndifft_par.html 'fn ndrustfft::ndifft_par')

#### [§](#real-to-complex)Real-to-complex

- `fft_r2c` : [`ndfft_r2c`](fn.ndfft_r2c.html 'fn ndrustfft::ndfft_r2c'), [`ndfft_r2c_par`](fn.ndfft_r2c_par.html 'fn ndrustfft::ndfft_r2c_par'),

#### [§](#complex-to-real)Complex-to-real

- `ifft_r2c`: [`ndifft_r2c`](fn.ndifft_r2c.html 'fn ndrustfft::ndifft_r2c'),[`ndifft_r2c_par`](fn.ndifft_r2c_par.html 'fn ndrustfft::ndifft_r2c_par')

#### [§](#real-to-real)Real-to-real

- `dct1`: [`nddct1`](fn.nddct1.html 'fn ndrustfft::nddct1'),[`nddct1_par`](fn.nddct1_par.html 'fn ndrustfft::nddct1_par')
- `dct2`: [`nddct2`](fn.nddct2.html 'fn ndrustfft::nddct2'),[`nddct2_par`](fn.nddct2_par.html 'fn ndrustfft::nddct2_par')
- `dct3`: [`nddct3`](fn.nddct3.html 'fn ndrustfft::nddct3'),[`nddct3_par`](fn.nddct3_par.html 'fn ndrustfft::nddct3_par')
- `dct4`: [`nddct4`](fn.nddct4.html 'fn ndrustfft::nddct4'),[`nddct4_par`](fn.nddct4_par.html 'fn ndrustfft::nddct4_par')

### [§](#example)Example

2-Dimensional real-to-complex fft along first axis

```
use ndarray::{Array2, Dim, Ix};
use ndrustfft::{ndfft_r2c, Complex, R2cFftHandler};

let (nx, ny) = (6, 4);
let mut data = Array2::<f64>::zeros((nx, ny));
let mut vhat = Array2::<Complex<f64>>::zeros((nx / 2 + 1, ny));
for (i, v) in data.iter_mut().enumerate() {
    *v = i as f64;
}
let mut fft_handler = R2cFftHandler::<f64>::new(nx);
ndfft_r2c(
    &data.view(),
    &mut vhat.view_mut(),
    &mut fft_handler,
    0,
);
```

## [§](#normalization)Normalization

`RustFFT`, `RustDCT` and `RealFft` do not normalise, while this library applies normalization as scipy by default. This means, inverse ffts are divided by a factor of `data.len()`, and dcts are multiplied by two. It is possible to switch from the default normalization to no normalization, or to apply a custom normalization by using the normalization builder.

See: `examples/fft_norm`

## [§](#features)Features

- parallel: Enables parallel transform using `ndarrays` + `rayon` (enabled by default)
- avx: Enables `rustfft`’s avx feature (enabled by default)
- sse: Enables `rustfft`’s sse feature (enabled by default)
- neon: Enables `rustfft`’s neon feature (enabled by default)

## [§](#documentation)Documentation

[docs.rs](https://docs.rs/ndrustfft/)

## [§](#versions)Versions

[Changelog](CHANGELOG.md)

[Complex](struct.Complex.html 'struct ndrustfft::Complex')

A complex number in Cartesian form.

[DctHandler](struct.DctHandler.html 'struct ndrustfft::DctHandler')

_n_\-dimensional real-to-real Cosine Transform.

[FftHandler](struct.FftHandler.html 'struct ndrustfft::FftHandler')

_n_\-dimensional complex-to-complex Fourier Transform.

[R2cFftHandler](struct.R2cFftHandler.html 'struct ndrustfft::R2cFftHandler')

_n_\-dimensional real-to-complex Fourier Transform.

[Normalization](enum.Normalization.html 'enum ndrustfft::Normalization')

Represents different types of normalization methods.

[FftNum](trait.FftNum.html 'trait ndrustfft::FftNum')

Generic floating point number, implemented for f32 and f64

[Zero](trait.Zero.html 'trait ndrustfft::Zero')

Defines an additive identity element for `Self`.

[nddct1](fn.nddct1.html 'fn ndrustfft::nddct1')

Real-to-real Discrete Cosine Transform of type 1 DCT-I (serial).

[nddct2](fn.nddct2.html 'fn ndrustfft::nddct2')

Real-to-real Discrete Cosine Transform of type 2 DCT-2 (serial).

[nddct3](fn.nddct3.html 'fn ndrustfft::nddct3')

Real-to-real Discrete Cosine Transform of type 3 DCT-3 (serial).

[nddct4](fn.nddct4.html 'fn ndrustfft::nddct4')

Real-to-real Discrete Cosine Transform of type 4 DCT-4 (serial).

[nddct1_par](fn.nddct1_par.html 'fn ndrustfft::nddct1_par')

Real-to-real Discrete Cosine Transform of type 1 DCT-I (parallel).

[nddct2_par](fn.nddct2_par.html 'fn ndrustfft::nddct2_par')

Real-to-real Discrete Cosine Transform of type 2 DCT-2 (parallel).

[nddct3_par](fn.nddct3_par.html 'fn ndrustfft::nddct3_par')

Real-to-real Discrete Cosine Transform of type 3 DCT-3 (parallel).

[nddct4_par](fn.nddct4_par.html 'fn ndrustfft::nddct4_par')

Real-to-real Discrete Cosine Transform of type 4 DCT-4 (parallel).

[ndfft](fn.ndfft.html 'fn ndrustfft::ndfft')

Complex-to-complex Fourier Transform (serial).

[ndfft_par](fn.ndfft_par.html 'fn ndrustfft::ndfft_par')

Complex-to-complex Fourier Transform (parallel).

[ndfft_r2c](fn.ndfft_r2c.html 'fn ndrustfft::ndfft_r2c')

Real-to-complex Fourier Transform (serial).

[ndfft_r2c_par](fn.ndfft_r2c_par.html 'fn ndrustfft::ndfft_r2c_par')

Real-to-complex Fourier Transform (parallel).

[ndifft](fn.ndifft.html 'fn ndrustfft::ndifft')

Complex-to-complex Inverse Fourier Transform (serial).

[ndifft_par](fn.ndifft_par.html 'fn ndrustfft::ndifft_par')

Complex-to-complex inverse Fourier Transform (parallel).

[ndifft_r2c](fn.ndifft_r2c.html 'fn ndrustfft::ndifft_r2c')

Complex-to-real inverse Fourier Transform (serial).

[ndifft_r2c_par](fn.ndifft_r2c_par.html 'fn ndrustfft::ndifft_r2c_par')

Complex-to-real inverse Fourier Transform (parallel).
