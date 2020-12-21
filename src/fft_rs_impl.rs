use std::f32::consts::PI;
use num::complex::Complex;


use rustfft::num_traits::Zero;


use vkdsp::fio::*;

fn main() {

    //read input from file
    let complex_input = get_binfile_as_vec_c32("tmp.bin");

    //execute radix fft
    let x_spec = radix_k_fft(&complex_input, complex_input.len());
    
    
    for i in 0..x_spec.len() {
        if x_spec[i].im >= 0.0 {
            println!("{:}+{:}j", x_spec[i].re, x_spec[i].im);
        } else {
            println!("{:}-{:}j", x_spec[i].re, -x_spec[i].im);
        }
    }
    
    // try again doing half and half
    let cmplx_in_e = complex_input.into_iter().filter(|&j| &j % 2 == 0).collect::<Vec<Complex<f32>>>();
    let cmplx_in_o = complex_input.into_iter().filter(|&j| &j % 2 == 1).collect::<Vec<Complex<f32>>>();

    let x_e_spec = radix_k_fft(cmplx_in_e, cmplx_in_e.len() / 2);
    let x_o_spec = radix_k_fft(cmplx_in_o, cmplx_in_o.len() / 2);
}


fn get_fft_w(i: usize, k: usize, n: usize) -> Complex<f32> {
    Complex::i().scale(-2.0 * PI * (i as f32)  * (k as f32) / (n as f32)).exp()
}


fn radix_k_fft(x: &Vec<Complex<f32>>, k: usize) -> Vec<Complex<f32>>{

    let mut x_spec: Vec<Complex<f32>> = vec![Complex::zero(); x.len()];

    for i in 0..x.len() {
        for j in 0..k {
            x_spec[i] += x[j] * get_fft_w(i, j, x.len());
        }
    }

    x_spec
}
