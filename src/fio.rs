use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

use num::complex::Complex;

// TODO (ryan) make this less horribly inefficient and non-general
/**
 * Read ReIm32 complex float values (32 per half, 64 total) from a
 * bin file
 */
/*
pub fn get_binfile_as_vec_c32(filename: &str) -> Vec<Complex<f32>> {
    // Read the bin file
    let mut fi = File::open(&filename).expect("no such file");
    let meta = std::fs::metadata(&filename).expect("can't read file metadata");
    let file_size = meta.len() as usize;
    let mut buffer = vec![0; file_size];
    fi.read(&mut buffer).expect("buffer overflow");

    // Load bytes into a ReIm32 float vec, then pack into complex
    let mut dest_re = Vec::new();
    let mut dest_cplx = Vec::new();
    

    //Iterate over bytes 4 at a time... read as floats
    for i in (0 ..file_size).step_by(4) {
        dest_re.push(f32::from_le_bytes(buffer[i .. i+4].try_into().expect("wrong size")));
    }


    //combine read floats in order to form complex numbers
    for i in (0 .. dest_re.len()).step_by(2) {
       dest_cplx.push(Complex::<f32>{ re: dest_re[i], im: dest_re[i + 1] });
    }
    dest_cplx
}

*/

pub fn get_binfile_as_vec_c32(filename: &str) -> Vec<Complex<f32>> {
    // Read the bin file
    let mut fi = File::open(&filename).expect("no such file");
    let meta = std::fs::metadata(&filename).expect("can't read file metadata");
    let file_size = meta.len() as usize;
    let mut buffer = vec![0; file_size];
    fi.read(&mut buffer).expect("buffer overflow");

    // Load bytes into a ReIm32 float vec, then pack into complex
    let mut dest_re = Vec::new();
    let mut dest_cplx = Vec::new();
    

    //Iterate over bytes 4 at a time... read as floats
    for float_chunk in buffer.chunks(4) {
        dest_re.push(f32::from_le_bytes(float_chunk.try_into().expect("wrong size")));
    }


    //combine read floats in order to form complex numbers
    for i in 0..dest_re.len()/2 {
       dest_cplx.push(Complex::<f32>{ re: dest_re[i], im: dest_re[i+1]});
    }
    dest_cplx
}






