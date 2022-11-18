use array2::array2::Array2;
use csc411_image::Rgb;

use crate::codec::compress::{
    change_to_floating_point::change_to_floating_point, output_img::output_img,
    pack_blocks::pack_blocks, rgb_to_component::rgb_to_component, trim::trim_img,
};

// Compresses img by packing into a u32 word
/// Outputs compressed image to standard output as matrix of u32 words
/// Abstracted function that calls smaller modules to compress image
pub fn compress(pixels: &mut Array2<Rgb>, denom: u16) {
    trim_img(pixels);
    let pixels_floats = change_to_floating_point(pixels, denom);
    let pixels_component = rgb_to_component(&pixels_floats);
    let words_array2 = pack_blocks(pixels_component);
    output_img(&words_array2);
}
