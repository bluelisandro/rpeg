use array2::array2;
use csc411_image::{Read, RgbImage};
use rpeg::codec::{compress::compress, decompress};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let argnum = args.len();

    assert!(argnum == 2 || argnum == 3);

    let filename = args.iter().nth(2).unwrap();

    match args[1].as_str() {
        "-c" => compress_helper(filename),
        "-d" => decompress_helper(filename),
        _ => eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]"),
    }
}

fn compress_helper(filename: &String) {
    let filename_slice: &str = &&filename[..];
    let img = RgbImage::read(Some(filename_slice).as_deref()).unwrap();
    let mut pixels =
        array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels)
            .unwrap();
    let denom = img.denominator;

    compress::compress(&mut pixels, denom);
}

fn decompress_helper(filename: &String) {
    let filename_slice: &str = &&filename[..];
    let words = csc411_rpegio::read_in_rpeg_data(Some(filename_slice)).unwrap();

    // let mut words_array2_u32: array2::Array2<u32> =
    //     array2::Array2::new(words.1 as usize, words.2 as usize, 0);

    let mut temp_vec: Vec<u32> = Vec::new();
    for byte_array in words.0 {
        // words_array2_u32.push(as_u32_be(&byte_array));
        temp_vec.push(as_u32_be(&byte_array));
    }

    let mut words_array2_u32: array2::Array2<u32> =
        array2::Array2::from_row_major(words.1 as usize / 2, words.2 as usize / 2, temp_vec)
            .unwrap();

    decompress::trim::trim_img(&mut words_array2_u32);

    decompress::decompress::decompress(
        &words_array2_u32,
        words_array2_u32.width() as u32,
        words_array2_u32.height() as u32,
    );
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}
