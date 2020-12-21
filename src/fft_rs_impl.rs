/// Sample code testing various Fourier transform implementations
use num::complex::Complex;
use rustfft::num_traits::Zero;
use std::f32::consts::PI;
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

    // Compute Cooley-Turkey of given max depth
    let ct_spec = fft_ct_md(&complex_input, 64, 1).unwrap();

    // Compare output
    for i in 0..n {
        println!(
            "x_spec: {:}+{:}\t combined_spec: {:}+{:}\t cooley-turkey: {:}+{:}",
            x_spec[i].re,
            x_spec[i].im,
            combined_spec[i].re,
            combined_spec[i].im,
            ct_spec[i].re,
            ct_spec[i].im
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

/*
fn split_vec_every_k<T>(x: &Vec<T>, k: u32) -> Result<Vec<&Vec<T>>> {
    let n = x.len();
    if n % k != 0 {
        Err("input vector of length {:} incompatible with split of width {:}",
            n, k)
    }
    let l = n / k;
    let mut split = vec![Vec::new(); k];

    for i = 0 .. l {
        for j = 0 .. k {
            split[k].push(x[i * k + j]);
        }
    }

    Ok(split);
}
*/

fn ct_split_half<T: Copy>(x: &Vec<T>) -> Result<[Vec<T>; 2], &'static str> {
    let n = x.len();
    if n % 2 != 0 {
        return Err("Size of vec to split is not even");
    }
    let mut result = [Vec::new(), Vec::new()];
    /*
    let mut s0 = Vec::new();
    let mut s1 = Vec::new();
    */
    for i in 0..n / 2 {
        result[0].push(x[2 * i]);
        result[1].push(x[2 * i + 1]);
    }

    Ok(result)
}

/// FFT using Cooley-Turkey method that recurses down to FFTs of size max_d with
/// a given offset
fn fft_ct_md(
    x: &Vec<Complex<f32>>,
    max_d: u32,
    offset: u32,
) -> Result<Vec<Complex<f32>>, &'static str> {
    let n = x.len();
    if n % 2 != 0 || max_d % 2 != 0 {
        return Err("Input buffer size or max depth not a power of 2");
    }

    // When we hit max depth, do the DFT
    if n as u32 == max_d {
        Ok(_dft(x))
    } else if (n as u32) < max_d {
        Err("Size of input buffer smaller than max depth")
    } else {
        // If we're below max depth, run on evens and odds
        let samps_split = ct_split_half::<Complex<f32>>(x).unwrap();
        let spec_e = fft_ct_md(&samps_split[0], max_d, 2 * offset).unwrap();
        let spec_o = fft_ct_md(&samps_split[1], max_d, 2 * offset).unwrap();
        let mut combined: Vec<Complex<f32>> =
            spec_e.into_iter().chain(spec_o.into_iter()).collect();
        for k in 0..n / 2 {
            let rep: Complex<f32> = combined[k];
            combined[k] = rep
                + Complex::i()
                    .scale(-2.0 * PI * (k as f32) / (n as f32))
                    .exp()
                    * combined[k + n / 2];
            combined[k + n / 2] = rep
                - Complex::i()
                    .scale(-2.0 * PI * (k as f32) / (n as f32))
                    .exp()
                    * combined[k + n / 2];
        }

        Ok(combined)
    }
}
