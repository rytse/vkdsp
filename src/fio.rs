use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

use num::complex::Complex;

// TODO (ryan) make this less horribly inefficient and non-general
/**
 * Read ReIm32 complex float values (32 per half, 64 total) from a
 * bin file
 */
pub fn get_binfile_as_vec_c32(filename: &str) -> Vec<Complex<f32>> {
    // Read the bin file
    let mut fi = File::open(&filename).expect("no such file");
    let meta = std::fs::metadata(&filename).expect("can't read file metadata");
    let mut buffer = vec![0; meta.len() as usize];
    fi.read(&mut buffer).expect("buffer overflow");

    // Load bytes into a ReIm32 float vec, then pack into complex
    let mut dest_re = Vec::new();
    let mut dest_cplx = Vec::new();
    for i in 0 .. meta.len() / 4 {
        dest_re.push(f32::from_ne_bytes(buffer[(4*i) as usize .. (4*i+4) as usize]
                                     .try_into().expect("wrong size")));
    }
    for i in 0 .. meta.len() / 8 {
       dest_cplx.push(Complex::<f32>{ re: dest_re[(i / 2) as usize], im: dest_re[(i / 2 + 1) as usize] });
    }

    dest_cplx
}

