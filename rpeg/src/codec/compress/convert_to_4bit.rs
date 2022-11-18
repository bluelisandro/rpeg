use array2::array2::Array2;

/// Outputs Array2 containing componenet color space Pb averages and Pr averages
/// Computes averages by iterating through averages Array2
/// Converts Pb and Pr averages to 4 bit by using csc411_arith::index_of_chroma function
pub fn convert_to_4bit(averages: &Array2<(f64, f64, f64)>) -> Array2<(usize, usize)> {
    let mut results_vec = Vec::new();

    let width: usize = averages.width();
    let height: usize = averages.height();

    // calculate 4bit values for each pb and pr value in a 2x2 block
    for i in 0..height {
        for j in 0..width {
            let pb_avg: f32 = averages.get(j, i).unwrap().1 as f32;
            let pr_avg: f32 = averages.get(j, i).unwrap().2 as f32;

            let pb_avg_4bit: usize = csc411_arith::index_of_chroma(pb_avg);
            let pr_avg_4bit: usize = csc411_arith::index_of_chroma(pr_avg);
            results_vec.push((pb_avg_4bit, pr_avg_4bit));
        }
    }

    let four_bits = Array2::from_row_major(width, height, results_vec);
    return four_bits.unwrap();
}
