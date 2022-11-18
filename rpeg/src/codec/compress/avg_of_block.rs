use array2::array2::Array2;

/// Outputs Array2 of averages of componenet color space values (Y, Pb, and Pr) for each 2x2 block in pixels
/// Computes averages of componenet color space values by iterating through each 2x2 block in pixels 
pub fn avg_of_block(pixels: &Array2<(f64, f64, f64)>) -> Array2<(f64, f64, f64)> {
    let width: usize = pixels.width();
    let height: usize = pixels.height();

    let mut start_i: usize = 0;
    let mut start_j: usize = 0;
    let mut end_i: usize = 1;
    let mut end_j: usize = 1;

    let mut result_vec: Vec<(f64, f64, f64)> = Vec::new();

    while end_i < height {
        // move onto the next row block once finished with the current one
        if end_j > width - 1 {
            // reset to the first two columns
            start_j = 0;
            end_j = 1;

            // move to the next two rows
            start_i = end_i + 1;
            end_i += 2;

            // end once all blocks have been visited
            if end_i > height {
                break;
            }
        }

        // sum up all the y, pb, and pr values in the current 2x2 block
        let mut y: f64 = 0.0;
        let mut pb: f64 = 0.0;
        let mut pr: f64 = 0.0;
        for i in start_i..=end_i {
            for j in start_j..=end_j {
                let tuple = pixels.get(j, i);
                y += tuple.unwrap().0;
                pb += tuple.unwrap().1;
                pr += tuple.unwrap().2;
            }
        }

        // calculate averages
        let y_avg: f64 = y / 4.0;
        let pb_avg: f64 = pb / 4.0;
        let pr_avg: f64 = pr / 4.0;

        result_vec.push((
            y_avg,
            num::clamp(pb_avg, -0.5, 0.5),
            num::clamp(pr_avg, -0.5, 0.5),
        ));

        // increment j by 2 to move to next block
        start_j = end_j + 1;
        end_j += 2;
    }

    let adj_width: usize = width / 2;
    let adj_height: usize = height / 2;
    let avg_array2 = Array2::from_row_major(adj_width, adj_height, result_vec);
    return avg_array2.unwrap();
}
