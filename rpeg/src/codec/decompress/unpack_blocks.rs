use array2::array2::Array2;
use bitpack::bitpack;

/// Returns an Array2 of tuples consisting of (a, b, c, d, pb_avg, pr_avg) after converting each word in input array2 of words using
/// bit manipulation. These values will be used to retrive brightnesses and component video colors.
/// 
/// # Arguments
/// packs_array: Array2 of words from a compressed image
pub fn unpack_blocks(packs_array2: &Array2<u32>) -> Array2<(f64, f64, f64, f64, f32, f32)> {
    let width = packs_array2.width();
    let height = packs_array2.height();

    let mut decoded_words: Vec<(f64, f64, f64, f64, f32, f32)> = Vec::new();

    // For each word in packs_array2, unpack the word
    for tuple in packs_array2.iter_row_major() {
        let word = *tuple.2 as u64;

        // Unpack a, b, c, d, index_pb, index_pr, and convert back to floats
        let a = bitpack::getu(word, 9, 23) / 511;
        let b = bitpack::gets(word, 5, 18) / 50;
        let c = bitpack::gets(word, 5, 13) / 50;
        let d = bitpack::gets(word, 5, 8) / 50;
        let index_pb = bitpack::getu(word, 4, 4);
        let index_pr = bitpack::getu(word, 4, 0);

        let pb = csc411_arith::chroma_of_index(index_pb as usize) as f32;
        let pr = csc411_arith::chroma_of_index(index_pr as usize) as f32;

        // Append a, b, c, d, pb, and pr to temp_vec
        let temp_tuple = (a as f64, b as f64, c as f64, d as f64, pb, pr);
        decoded_words.push(temp_tuple);
    }

    let output_array2: Array2<(f64, f64, f64, f64, f32, f32)> =
        Array2::from_row_major(width, height, decoded_words).unwrap();

    return output_array2;
}
