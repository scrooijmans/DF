Convolving in frequency space is typically much faster.

i.e. perform a fast fourier transforms on your kernel and input, multiply, then an inverse fourier transform to fetch the result.

If you are chaining multiple convolutions you can cut out all but the last inverse transform for even more gains.

multiplication becomes an addition in log space: ln(a * b) = ln(a) + ln(b). The same goes for convolution and multiplication in time vs frequency domains. So FFT(conv(a, b)) = FFT(a) .* FFT(b). (where a and b are vectors and .* is an entrywise product).

