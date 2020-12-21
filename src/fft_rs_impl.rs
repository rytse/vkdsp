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
