use array2::array2::Array2;

/// Performs the inverse operations of the discrete cosine transformation and returns an Array2 of tuples consisting
/// of brightnesses (Y1, Y2, Y3, Y4) by taking in an Array2 of tuples representing (a, b, c, d, pb_avg, pr_avg)
/// 
/// # Arguments
/// unpacked_array2: tuple of (a, b, c, d, pb_avg, pr_avg)
pub fn reverse_dct(
    unpacked_array2: &Array2<(f64, f64, f64, f64, f32, f32)>,
) -> Array2<(f64, f64, f64, f64)> {
    let width = unpacked_array2.width();
    let height = unpacked_array2.height();

    let mut results_vec = Vec::new();

    for i in 0..height {
        for j in 0..width {
            let tuple = unpacked_array2.get(j, i).unwrap();
            let a = tuple.0;
            let b = tuple.1;
            let c = tuple.2;
            let d = tuple.3;

            // calculate brightnesses
            let y1 = a - b - c + d;
            let y2 = a - b + c - d;
            let y3 = a + b - c - d;
            let y4 = a + b + c + d;
            results_vec.push((y1, y2, y3, y4));
        }
    }

    let brightnesses_array2 = Array2::from_row_major(width, height, results_vec).unwrap();
    return brightnesses_array2;
}
