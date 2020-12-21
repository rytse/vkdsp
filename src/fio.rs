use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

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
    let file_size = meta.len() as usize;
    let mut buffer = vec![0; file_size];
    fi.read(&mut buffer).expect("buffer overflow");

    // Load bytes into a ReIm32 float vec, then pack into complex
    let mut dest_re = Vec::new();
    let mut dest_cplx = Vec::new();

    //Iterate over bytes 4 at a time... read as floats
    for float_chunk in buffer.chunks_exact(4) {
        dest_re.push(f32::from_le_bytes(
            float_chunk.try_into().expect("wrong size"),
        ));
    }

    //combine read floats in order to form complex numbers
    //should be a cleaner way to do this
    for complex_chunk in dest_re.chunks_exact(2) {
        if let &[real, imaginary] = complex_chunk {
            dest_cplx.push(Complex::<f32> {
                re: real,
                im: imaginary,
            });
        }
    }
    dest_cplx
}
