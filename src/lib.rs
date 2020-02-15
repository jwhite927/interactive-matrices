pub mod matrix;
pub mod etc;

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::etc::{num_digits};

    #[test]
    fn display_looks_right_2x2() {
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(123); 
        matrix.matrix[0][1] = Some(-1); 
        matrix.matrix[1][0] = Some(12345); 
        matrix.matrix[1][1] = Some(1); 
        let mut check_string = "\u{250C}     ".to_owned();
        check_string.push_str("       ");
        check_string.push_str("\u{2510}\n\u{2502} ");
        check_string.push_str("123  ");
        check_string.push_str(" , ");
        check_string.push_str("-1");
        check_string.push_str(" \u{2502}\n\u{2502} ");
        check_string.push_str("12345");
        check_string.push_str(" , ");
        check_string.push_str("1 ");
        check_string.push_str(" \u{2502}\n\u{2514}     ");
        check_string.push_str("       ");
        check_string.push_str("\u{2518}");
        assert_eq!(&format!("{}", matrix), &check_string);
    }
    
    #[test]
    fn display_looks_right_3x3() {
        let mut matrix = Matrix::new(3,3);
        matrix.matrix[0] = [Some(1), Some(1), Some(1)];
        matrix.matrix[1] = [Some(1), Some(1), Some(1)];
        matrix.matrix[2] = [Some(1), Some(1), Some(1)];
        let mut check_string = "\u{250C}".to_owned();
        check_string.push_str("           ");
        check_string.push_str("\u{2510}\n\u{2502} ");
        check_string.push_str("1 , 1 , 1");
        check_string.push_str(" \u{2502}\n\u{2502} ");
        check_string.push_str("1 , 1 , 1");
        check_string.push_str(" \u{2502}\n\u{2502} ");
        check_string.push_str("1 , 1 , 1");
        check_string.push_str(" \u{2502}\n\u{2514}");
        check_string.push_str("           ");
        check_string.push_str("\u{2518}");
        assert_eq!(&format!("{}", matrix), &check_string);
    }

    #[test]
    fn negative_values_print_correctly() {
        assert_eq!(num_digits(-1),2);
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(-12321);
        matrix.matrix[0][1] = Some(1);
        matrix.matrix[1][0] = Some(123);
        matrix.matrix[1][1] = Some(32);
        assert_eq!(matrix.print_value(0,0), "-12321");
        assert_eq!(matrix.print_value(1,0), "123   ");
    }

    #[test]
    fn array_out_of_bounds_uses_min_max() {
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(1);
        matrix.matrix[0][1] = Some(2);
        matrix.matrix[1][0] = Some(3);
        matrix.matrix[1][1] = Some(4);
        assert_eq!(matrix.get_value(5,6),matrix.get_value(1,1));
    }

    #[test]
    fn number_of_digits_is_counted_correctly() {
        assert_eq!(num_digits(134),3);
        assert_eq!(num_digits(1234),4);
    }

    #[test]
    fn top_bot_spaces_produces_correct_number_of_spaces() {
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(12);
        matrix.matrix[0][1] = Some(1); 
        matrix.matrix[1][0] = Some(123);
        matrix.matrix[1][1] = Some(1);
        assert_eq!(matrix.top_bot_spaces(), "         ");
    }

    #[test]
    fn col_widths_are_counted_properly() {
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(1232112);
        matrix.matrix[0][1] = Some(1);
        matrix.matrix[1][0] = Some(123);
        matrix.matrix[1][1] = Some(1);
        assert_eq!(matrix.col_widths(),vec!(7,1));
    }

    #[test]
    fn top_bot_match_length_of_contents() {
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(123);
        matrix.matrix[0][1] = Some(1234);
        matrix.matrix[1][0] = Some(1);
        matrix.matrix[1][1] = Some(1);
        assert_eq!(matrix.top_bot_spaces(), "            ");
        matrix.matrix[1][0] = Some(12345);
        assert_eq!(matrix.top_bot_spaces(), "              ");
    }

    #[test]
    fn print_value_shows_trailing_spaces() {
        let mut matrix = Matrix::new(2,2);
        matrix.matrix[0][0] = Some(12321);
        matrix.matrix[0][1] = Some(1);
        matrix.matrix[1][0] = Some(123);
        matrix.matrix[1][1] = Some(32);
        assert_eq!(matrix.print_value(0,0), "12321");
        assert_eq!(matrix.print_value(0,1), "1 ");
        assert_eq!(matrix.print_value(1,0), "123  ");
        assert_eq!(matrix.print_value(1,1), "32");
    }
}
