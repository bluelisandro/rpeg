use array2::array2::Array2;

/// Outputs brightnesses of each pixel in 2x2 blocks as y1, y2, y3, y4
/// Computes brightnesses from Array2 containing componenet color space values Y, Pb, Pr using discrete cosine transformation formula
pub fn discrete_cosine_transform(pixels: &Array2<(f64, f64, f64)>) -> Array2<(f64, f64, f64, f64)> {
    let width: usize = pixels.width();
    let height: usize = pixels.height();

    let mut start_i: usize = 0;
    let mut start_j: usize = 0;
    let mut end_i: usize = 1;
    let mut end_j: usize = 1;

    let mut result_vec: Vec<(f64, f64, f64, f64)> = Vec::new();

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

        let mut brightnesses: Vec<f64> = Vec::new();
        for i in start_i..=end_i {
            for j in start_j..=end_j {
                let tuple = pixels.get(j, i).unwrap();
                brightnesses.push(tuple.0);
            }
        }

        let y1: f64 = brightnesses[0];
        let y2: f64 = brightnesses[1];
        let y3: f64 = brightnesses[2];
        let y4: f64 = brightnesses[3];

        let a: f64 = (y4 + y3 + y2 + y1) / 4.0;
        let b: f64 = (y4 + y3 - y2 - y1) / 4.0;
        let c: f64 = (y4 - y3 + y2 - y1) / 4.0;
        let d: f64 = (y4 - y3 - y2 + y1) / 4.0;
        result_vec.push((
            a,
            num::clamp(b, -0.3, 0.3),
            num::clamp(c, -0.3, 0.3),
            num::clamp(d, -0.3, 0.3),
        ));

        // increment j by 2 to move to next block
        start_j = end_j + 1;
        end_j += 2;
    }

    let adj_width: usize = width / 2;
    let adj_height: usize = height / 2;
    let cosine_coeffs = Array2::from_row_major(adj_width, adj_height, result_vec);

    return cosine_coeffs.unwrap();
}
