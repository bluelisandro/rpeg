use array2::array2::Array2;
use num::integer::div_floor;

/// Returns an expanded 2d matrix where each pixel consists of (y, pb_avg, pr_avg). Traverses via 2x2 block and places the corresponding 
/// brightness values in a pixel.
/// 
/// # Arguments
/// brightnesses = Array2 of tuples consisting of (Y1, Y2, Y3, Y4)
/// unpack_array2 = Array2 of tuples consisting of (a, b, c, d, pb_avg, pr_avg)
pub fn component_video_color(
    brightnesses: &Array2<(f64, f64, f64, f64)>,
    unpack_array2: &Array2<(f64, f64, f64, f64, f32, f32)>,
) -> Array2<(f64, f32, f32)> {
    // GOAL: create an expanded 2d matrix where each cell in a 2x2 block has their respective brightness (Y), avg pb, and avg pr

    let new_width = brightnesses.width() * 2;
    let new_height = brightnesses.height() * 2;

    let mut start_i: usize = 0;
    let mut start_j: usize = 0;
    let mut end_i: usize = 1;
    let mut end_j: usize = 1;

    let mut component_video = Array2::new(new_width, new_height, (0.0, 0.0, 0.0));

    while end_i < new_height {
        // move onto the next row block once finished with the current one
        if end_j > new_width - 1 {
            // reset to the first two columns
            start_j = 0;
            end_j = 1;

            // move to the next two rows
            start_i = end_i + 1;
            end_i += 2;

            // end once all blocks have been visited
            if end_i > new_height {
                break;
            }
        }

        // y_index keeps track of the brightness to place in the current coordinate
        let mut y_index: i32 = 0;

        for i in start_i..=end_i {
            for j in start_j..=end_j {
                if y_index == 4 {
                    y_index = 0;
                }

                let brightness = brightnesses
                    .get(div_floor(j, 2), div_floor(i, 2))
                    .unwrap();
                let mut y = 0.0;
                if y_index == 0 {
                    y = brightness.0;
                } else if y_index == 1 {
                    y = brightness.1;
                } else if y_index == 2 {
                    y = brightness.2;
                } else if y_index == 3 {
                    y = brightness.3;
                }

                let tuple = unpack_array2
                    .get(div_floor(j, 2), div_floor(i, 2))
                    .unwrap();
                let pb_avg = tuple.4;
                let pr_avg = tuple.5;

                let tuple = component_video.get_mut(j, i).unwrap();
                tuple.0 = y;
                tuple.1 = pb_avg;
                tuple.2 = pr_avg;

                y_index += 1;
                // dbg!((y, pb_avg, pr_avg));
            }
        }

        // increment j by 2 to move to next block
        start_j = end_j + 1;
        end_j += 2;
    }

    return component_video;
}
