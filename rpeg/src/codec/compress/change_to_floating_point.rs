use array2::array2::Array2;
use csc411_image::Rgb;

/// Output Array2 containing component color space values (Y, Pb, Pr) as floating point values
pub fn change_to_floating_point(pixels: &mut Array2<Rgb>, denom: u16) -> Array2<(f64, f64, f64)> {
    let width = pixels.width();
    let height = pixels.height();
    let denom = denom as f64;

    let mut results_vec: Vec<(f64, f64, f64)> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let rgb = pixels.get_mut(j, i).unwrap();
            let red = rgb.red as f64 / denom;
            let blue = rgb.blue as f64 / denom;
            let green = rgb.green as f64 / denom;
            results_vec.push((red, green, blue));
        }
    }
    let floating_points = array2::array2::Array2::from_row_major(width, height, results_vec);
    return floating_points.unwrap();
}
