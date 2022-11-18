use array2::array2::Array2;
use csc411_image::Rgb;

/// Trims RGB image and makes width and height even numbers
// Trim last row, column, or both of image so that width and height are even numbers
pub fn trim_img(pixels: &mut Array2<Rgb>) {
    trim_col(pixels);
    trim_row(pixels);
}

// Removes row from an Array2 by reference
fn trim_row(pixels: &mut Array2<Rgb>) {
    if pixels.height() % 2 != 0 {
        // Traverse through each column in the last row and remove the element
        for col_index in 0..pixels.width() - 1 {
            pixels.remove(col_index, pixels.height() - 1);
        }
        pixels.set_height(pixels.height() - 1);
    }
}

// Removes col from an Array2 by reference
fn trim_col(pixels: &mut Array2<Rgb>) {
    if pixels.width() % 2 != 0 {
        // Traverse down each row in the last column and remove the element
        for row_index in 0..pixels.height() - 1 {
            pixels.remove(pixels.width() - 1, row_index); // Note that the col index is fixed
        }
        pixels.set_width(pixels.width() - 1);
    }
}
