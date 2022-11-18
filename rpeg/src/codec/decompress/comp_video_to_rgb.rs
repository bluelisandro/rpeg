use array2::array2::Array2;
use csc411_image::Rgb;
use num;

/// Returns an Array2 of Rgbs after converting each (y, pb_avg, pr_avg) tuple in each pixel to its corresponding Rgb.
///
/// # Arguments
/// component_video_colors: An Array2 of tuples consisting of (Y, pb_avg, pr_avg)
pub fn comp_video_to_rgb(component_video_colors: &Array2<(f64, f32, f32)>) -> Array2<Rgb> {
    let width = component_video_colors.width();
    let height = component_video_colors.height();

    let mut results_vec = Vec::new();

    for i in 0..height {
        for j in 0..width {
            let tuple = component_video_colors.get(j, i).unwrap();
            let y = tuple.0;
            let pb_avg = tuple.1 as f64;
            let pr_avg = tuple.2 as f64;

            let mut r = 1.0 * y + 0.0 * pb_avg + 1.402 * pr_avg;
            let mut g = 1.0 * y - 0.344136 * pb_avg - 0.714136 * pr_avg;
            let mut b = 1.0 * y + 1.772 * pb_avg + 0.0 * pr_avg;

            r *= 255.0;
            g *= 255.0;
            b *= 255.0;

            r = r.round();
            g = g.round();
            b = b.round();

            let rgb = csc411_image::Rgb {
                red: num::clamp(r as u16, 0, 255),
                green: num::clamp(g as u16, 0, 255),
                blue: num::clamp(b as u16, 0, 255),
            };

            results_vec.push(rgb);
        }
    }

    let rgb_array2 = Array2::from_row_major(width, height, results_vec).unwrap();
    // dbg!(rgb_array2.elements_row_major());
    return rgb_array2;
}
