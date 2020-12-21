use num::complex::Complex;
/// Sample code testing various Fourier transform implementations
use std::f32::consts::PI;

use rustfft::num_traits::Zero;

use vkdsp::fio::*;

/// Test plain-old-Rust implementations of the DFT, radix-2 (one level) FFT, and
/// the "recursive" arbitrary final size FFT that is used by vkdsp
fn main() {
    // Read input from file
    let complex_input = get_binfile_as_vec_c32("tmp.bin");
    let n = complex_input.len();

    // Execute full-sized FFT
    let x_spec = _dft(&complex_input);

    // Compute Radix-2 FFT
    let mut cmplx_in_e = vec![Complex::<f32> { re: 0.0, im: 0.0 }; n / 2];
    let mut cmplx_in_o = vec![Complex::<f32> { re: 0.0, im: 0.0 }; n / 2];
    for i in 0..n / 2 {
        cmplx_in_e[i] = complex_input[2 * i];
        cmplx_in_o[i] = complex_input[2 * i + 1];
    }
    let x_e_spec = _dft(&cmplx_in_e);
    let x_o_spec = _dft(&cmplx_in_o);
    let mut combined_spec = vec![Complex::<f32> { re: 0.0, im: 0.0 }; n];
    for i in 0..n / 2 {
        combined_spec[i] = x_e_spec[i]
            + Complex::i()
                .scale(-2.0 * PI * (i as f32) / (n as f32))
                .exp()
                * x_o_spec[i];
    }
    for i in 0..n / 2 {
        combined_spec[i + n / 2] = x_e_spec[i]
            - Complex::i()
                .scale(-2.0 * PI * (i as f32) / (n as f32))
                .exp()
                * x_o_spec[i];
    }

    // Compare output
    for i in 0..n {
        println!(
            "x_spec: {:}+{:}\tcombined_spec: {:}+{:}",
            x_spec[i].re, x_spec[i].im, combined_spec[i].re, combined_spec[i].im
        );
    }
}

/// Get the value of the FFT twiddle table for a given signal length and
/// frequency index.
fn get_fft_w(i: usize, k: usize, n: usize) -> Complex<f32> {
    Complex::i()
        .scale(-2.0 * PI * (i as f32) * (k as f32) / (n as f32))
        .exp()
}

/// Compute DFT of input vector
fn _dft(x: &Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let n = x.len();
    let mut x_spec: Vec<Complex<f32>> = vec![Complex::zero(); n];
    for i in 0..n {
        for j in 0..n {
            x_spec[i] += x[j] * get_fft_w(i, j, n);
        }
    }
    x_spec
}
