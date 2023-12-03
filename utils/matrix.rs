use std::fmt::Display;

fn to_2d_vector(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn print_2d_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn check_adjacent_cells(matrix: &Vec<Vec<bool>>, row_index: usize, column_index: usize) -> bool {
    /// Checks adjacent cells of a specified cell in a boolean matrix for a `true` value.
    ///
    /// # Arguments
    /// * `matrix` - A reference to a 2D vector of `bool` representing the matrix.
    /// * `row_index` - The row index of the cell whose adjacent cells are to be checked.
    /// * `column_index` - The column index of the cell whose adjacent cells are to be checked.
    ///
    /// # Returns
    /// * `true` if any of the adjacent cells (up to 8 surrounding cells) are `true`.
    /// * `false` if none of the adjacent cells are `true`.
    let row_min = row_index.saturating_sub(1);
    let row_max = std::cmp::min(row_index + 1, matrix.len() - 1);
    let col_min = column_index.saturating_sub(1);
    let col_max = std::cmp::min(column_index + 1, matrix[0].len() - 1);

    for r in row_min..=row_max {
        for c in col_min..=col_max {
            // Avoid checking the cell itself.
            if r == row_index && c == column_index {
                continue;
            }
            if matrix[r][c] {
                return true;
            }
        }
    }

    false
}
