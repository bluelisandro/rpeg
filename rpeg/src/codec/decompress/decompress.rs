use array2::array2::Array2;
use csc411_image::{Rgb, Write};

use crate::codec::decompress::{
    comp_video_to_rgb::comp_video_to_rgb, component_video_color::component_video_color,
    reverse_dct::reverse_dct, unpack_blocks::unpack_blocks,
};

#[allow(unused_must_use)]

/// Writes a decompressed image to standard output by taking in an array2 of Rgbs, unpacking it to retrieve (a, b, c, d, pb_avg, pr_avg) values which
/// helps retrieve brightnesses which in turn helps retrieve component video color pixels which is transformed into the decompressed image 
/// 
/// # Arguments
/// pixels: Array2 of words
/// width: width of original image
/// height: height of original image
pub fn decompress(pixels: &Array2<u32>, _width: u32, _height: u32) {
    //!!! UNDERSCORES
    // let temp_u32_array2: Array2<u32> = Array2::new(2, 2, 100);

    // 32bit -> Array2<(a, b, c, d, pb_avg, pr_avg)>
    let unpacked_array2: Array2<(f64, f64, f64, f64, f32, f32)> = unpack_blocks(&pixels);

    // retrieve Y1, Y2, Y3, Y4
    let brightnesses: Array2<(f64, f64, f64, f64)> = reverse_dct(&unpacked_array2);

    let component_video_colors = component_video_color(&brightnesses, &unpacked_array2);
    let rgb_array2: Array2<Rgb> = comp_video_to_rgb(&component_video_colors);

    let out_img = csc411_image::RgbImage {
        width: rgb_array2.width() as u32,
        height: rgb_array2.height() as u32,
        denominator: 255,
        pixels: rgb_array2.elements_row_major().clone(),
    };

    out_img.write(None);
}
