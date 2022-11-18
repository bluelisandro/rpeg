use array2::array2::Array2;
use csc411_rpegio::{self, output_rpeg_data};

/// Outputs compressed image as matrix of u32 words to standard output
/// Uses csc411_rpegio crate to output array of byte array of u32 words
/// Uses to_be_bytes to convert u32 words to byte arrays
pub fn output_img(words_array2: &Array2<u32>) {
    let mut words_bytes = Vec::new();

    for tuple in words_array2.iter_row_major() {
        let bytes = tuple.2.to_be_bytes();
        words_bytes.push(bytes);
        // words_bytes.write(&bytes);
    }

    output_rpeg_data(
        &words_bytes[..],
        words_array2.width() as u32 * 2,
        words_array2.height() as u32 * 2,
    );
}
