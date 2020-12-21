use std::f32::consts::PI;
use num::complex::Complex;

use vkdsp::fio::*;

fn main() {
    let x = get_binfile_as_vec_c32("tmp.bin");
    let x_spec = radix_k_fft(&x, x.len() as u32);

    for i in 0..x_spec.len() {
        println!("{:}+{:}j", x_spec[i].re, x_spec[i].im);
    }
}

fn get_fft_w(i: u32, k: u32, n: u32) -> Complex<f32> {
    Complex::i().scale(-2.0 * PI * (i as f32)  * (k as f32) / (n as f32)).exp()
}


fn radix_k_fft(x: &Vec<Complex<f32>>, k: u32) -> Vec<Complex<f32>>{
    let mut x_spec: Vec<Complex<f32>> = (0..x.len()).map(|_| Complex::<f32>{ re: 0.0, im: 0.0 }).collect();

    for i in 0..x.len() {
        for j in 0..k {
            x_spec[i] += x[j as usize] * get_fft_w(i as u32, j as u32, x.len() as u32);
        }
    }

    x_spec
}
