use array2::array2::Array2;

use crate::codec::compress::{
    avg_of_block::avg_of_block, convert_to_32bit_word::convert_to_32bit_word,
    convert_to_4bit::convert_to_4bit, dct::discrete_cosine_transform,
};

/// Outputs an Array2 of packed u32 words with compressed image data coded inside
/// Abstraction that calls smaller functions to compute quantized integers to pack into u32 words
pub fn pack_blocks(pixels_component: Array2<(f64, f64, f64)>) -> Array2<u32> {
    // get averages for Y, pb, and pr for 2x2 blocks and convert to 4bit
    let averages = avg_of_block(&pixels_component);
    let four_bits = convert_to_4bit(&averages);

    // DCT function
    let cosine_coeffs = discrete_cosine_transform(&pixels_component);

    let mut packs: Vec<u32> = Vec::new();
    let width: usize = four_bits.width();
    let height: usize = four_bits.height();
    for i in 0..height {
        for j in 0..width {
            // Convert a, b, c, d, pb, pr to integers (multiply b
            let four_bits_tuple = four_bits.get(j, i).unwrap();
            let consine_coeffs_tuple = cosine_coeffs.get(j, i).unwrap();

            let a = consine_coeffs_tuple.0;
            let b = consine_coeffs_tuple.1;
            let c = consine_coeffs_tuple.2;
            let d = consine_coeffs_tuple.3;
            let index_pb = four_bits_tuple.0;
            let index_pr = four_bits_tuple.1;

            // Convert a, b, c, d, pb, pr to integers (multiply by 511 and round to nearest integer)
            let a_int = (a * 511 as f64).round() as u64;
            let mut b_int = (b * 50 as f64).round() as i64;
            let mut c_int = (c * 50 as f64).round() as i64;
            let mut d_int = (d * 50 as f64).round() as i64;

            // Clamp values to appropriate ranges
            // a_int = num::clamp(a_int, 0, 32);
            b_int = num::clamp(b_int, -15, 15);
            c_int = num::clamp(c_int, -15, 15);
            d_int = num::clamp(d_int, -15, 15);

            // Create 32 bit word using a, b, c, d, pb, pr
            let word =
                convert_to_32bit_word(a_int, b_int, c_int, d_int, index_pb as u64, index_pr as u64);

            // Push 32 bit word into pack
            packs.push(word);
        }
    }

    let packs_array2 = Array2::from_row_major(width, height, packs).unwrap();

    packs_array2
}
