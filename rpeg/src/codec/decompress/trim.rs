use array2::array2::Array2;

/// Trim last row, column, or both of image so that width and height are even numbers
/// 
/// # Arguments
/// words: words of the compressed image 
pub fn trim_img(words: &mut Array2<u32>) {
    trim_col(words);
    trim_row(words);
}

// Removes row from an Array2 by reference
fn trim_row(words: &mut Array2<u32>) {
    if words.height() % 2 != 0 {
        // Traverse through each column in the last row and remove the element
        for col_index in 0..words.width() - 1 {
            words.remove(col_index, words.height() - 1);
        }
        words.set_height(words.height() - 1);
    }
}

// Removes col from an Array2 by reference
fn trim_col(words: &mut Array2<u32>) {
    if words.width() % 2 != 0 {
        // Traverse down each row in the last column and remove the element
        for row_index in 0..words.height() - 1 {
            words.remove(words.width() - 1, row_index); // Note that the col index is fixed
        }
        words.set_width(words.width() - 1);
    }
}
