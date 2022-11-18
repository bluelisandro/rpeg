use array2::array2::Array2;

/// Outputs Array2 containing pixels in componenet color space
/// Converts Array2 of RGB values by applying component color space mathematical formula
pub fn rgb_to_component(pixels: &Array2<(f64, f64, f64)>) -> Array2<(f64, f64, f64)> {
    // RGB to Component color space formulas
    // y = 0.299 * r + 0.587 * g + 0.114 * b
    // pb = -0.168736 * r - 0.331264 * g + 0.5 * b
    // pr = 0.5 * r - 0.418688 * g - 0.081312

    let mut temp_vec: Vec<(f64, f64, f64)> = Vec::new();

    // red = 0, green = 1, blue = 2
    // Iterate through pixels, compute component values, put into temp_vec
    for rgb in pixels.iter_row_major() {
        let y = 0.299 * rgb.2 .0 + 0.587 * rgb.2 .1 + 0.114 * rgb.2 .2;
        let pb = -0.168736 * rgb.2 .0 - 0.331264 * rgb.2 .1 + 0.5 * rgb.2 .2;
        let pr = 0.5 * rgb.2 .0 - 0.418688 * rgb.2 .1 - 0.081312 * rgb.2 .2;
        temp_vec.push((y, pb, pr));
    }

    // Create Array2 of tuples from temp_vec to store component values as follows: (y, pb, pr)
    let pixels_component = Array2::from_row_major(pixels.width(), pixels.height(), temp_vec);

    return pixels_component.unwrap();
}
