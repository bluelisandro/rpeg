use bitpack::bitpack;

// Uses bitpack crate to pack quantized integers into a single u32 word
/// Outputs 32-bit word containing coded coefficients
/// Packs coefficients using bitpack functions that puts quantized integer values into a single u32 word
pub fn convert_to_32bit_word(a: u64, b: i64, c: i64, d: i64, pb: u64, pr: u64) -> u32 {
    let mut word: Option<u64> = Some(0);
    word = bitpack::newu(word.unwrap(), 9, 23, a);
    word = bitpack::news(word.unwrap(), 5, 18, b);
    word = bitpack::news(word.unwrap(), 5, 13, c);
    word = bitpack::news(word.unwrap(), 5, 8, d);
    word = bitpack::newu(word.unwrap(), 4, 4, pb);
    word = bitpack::newu(word.unwrap(), 4, 0, pr);
    return word.unwrap() as u32;
}
