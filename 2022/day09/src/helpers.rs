fn is_diagonal(position1: &(i32, i32), position2: &(i32, i32)) -> bool {
    let x_distance: i32 = (position1.0 as i32 - position2.0 as i32).abs();
    let y_distance: i32 = (position1.1 as i32 - position2.1 as i32).abs();
    if x_distance > 1 || y_distance > 1 {
        false
    } else {
        true
    }
}

pub fn is_adjacent(position1: &(i32, i32), position2: &(i32, i32)) -> bool {
    // Same means adjacent.
    if position1 == position2 {
        true
    } else if
    // Same column, different row.
    (position1.0 == position2.0 && (
            position1.1 == position2.1 + 1 ||
            position1.1 == position2.1 - 1
        ))
        ||
        // Same row, different column.
        (position1.1 == position2.1 && (
            position1.0 == position2.0 + 1 ||
            position1.0 == position2.0 - 1
        ))
    {
        true
    // Diagonal means adjacent.
    } else if is_diagonal(position1, position2) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adjacent() {
        assert_eq!(true, is_adjacent(&(2, 3), &(3, 3)));
        assert_eq!(true, is_adjacent(&(3, 2), &(3, 3)));
        assert_eq!(false, is_adjacent(&(2, 3), &(4, 3)));
        assert_eq!(false, is_adjacent(&(3, 2), &(5, 5)));
    }

    #[test]
    fn test_is_diagonal() {
        assert_eq!(true, is_diagonal(&(1, 1), &(2, 2)));
        assert_eq!(false, is_diagonal(&(1, 1), &(3, 3)));
        assert_eq!(false, is_diagonal(&(1, 1), &(3, 2)));
    }
}
