use std::fmt;
use super::*;
use std::cmp::Ordering;

pub struct Matrix {
    pub matrix: [[i32; 2]; 2],
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(num1: i32, num2: i32, num3: i32, num4: i32) -> Matrix {
        let new_matrix = Matrix {
            matrix: [[num1,num2],[num3,num4]],
            rows: 2,
            columns: 2,
        };
        new_matrix
    }

    pub fn get_value(&self, row: usize, column: usize) -> i32 {
        let mut row_to_return = self.rows - 1;
        if let Ordering::Greater = row_to_return.cmp(&row) {
            row_to_return = row;
        }

        let mut col_to_return = self.columns - 1;
        if let Ordering::Greater = col_to_return.cmp(&column) {
            col_to_return = column;
        }
        self.matrix[row_to_return][col_to_return]
    }

    pub fn top_bot_spaces(&self) -> String {
        let mut string = String::new();
        let extra_spaces = 3 * (self.columns - 1) + 2;
        println!("extra_spaces is {}",extra_spaces);
        for widest in self.col_widths() {
            for _ in 0..widest {
                string.push_str(" ");
            }
        }
        for _ in 0..extra_spaces {
            string.push_str(" ");
        }
        string
}

    pub fn print_value(&self, row: usize, column: usize) -> String {
        let mut string_to_print = String::from(self.get_value(row,column).to_string());
        for rows in 0..2 {
            while num_digits(self.get_value(rows,column)) > string_to_print.len() {
                string_to_print.push_str(" ");
            }
        }
        string_to_print
    }

    pub fn prompt_values(&mut self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                println!("Please enter a value for position {},{}", row + 1, column + 1);
                self.matrix[row][column] = prompt_i32();
                println!("{}", self);
            }
        }
    }

    pub fn col_widths(&self) -> Vec<usize> {
        let mut widest: usize = 0;
        let mut col_widths = Vec::new();
        for column in 0..self.columns {
            for row in 0..self.rows {
                if num_digits(self.get_value(row,column)) > widest {
                    widest = num_digits(self.get_value(row,column)); 
                }
            }
            col_widths.push(widest);
            widest = 0;
        }
        col_widths
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_to_write = "\u{250C}".to_owned();
        string_to_write.push_str(&self.top_bot_spaces());
        string_to_write.push_str("\u{2510}\n\u{2502} ");
        for row in 0..self.rows {
            for column in 0..self.columns {
                string_to_write.push_str(&self.print_value(row,column));
                if column < self.columns - 1 {
                    string_to_write.push_str(" , ");
                }
            }
            if row < self.rows - 1 {
            string_to_write.push_str(" \u{2502}\n\u{2502} ");
            }
        }
        string_to_write.push_str(" \u{2502}\n\u{2514}");
        string_to_write.push_str(&self.top_bot_spaces());
        string_to_write.push_str("\u{2518}");
        write!(f, "{}", string_to_write)
    }
}

//Counts the number of spaces occupied by an integer
pub fn num_digits(num: i32) -> usize {
    if num == 0 { 
        return 1; 
    }
    let mut count: usize = 0;
    let mut num_copy = num;
    if num < 0 {
        count = 1;
        num_copy *= -1;
    }
    while num_copy > 0 {
        num_copy /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_looks_right() {
        let matrix = Matrix::new(123,-1,12345,1);
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
    fn negative_values_print_correctly() {
        assert_eq!(num_digits(-1),2);
        let matrix = Matrix::new(-12321, 1, 123, 32);
        assert_eq!(matrix.print_value(0,0), "-12321");
        assert_eq!(matrix.print_value(1,0), "123   ");
    }
    #[test]
    fn array_out_of_bounds_uses_min_max() {
        let matrix = Matrix::new(1,1,1,1);
        assert_eq!(matrix.get_value(5,6),matrix.get_value(1,1));
    }

    #[test]
    fn number_of_digits_is_counted_correctly() {
        assert_eq!(num_digits(134),3);
        assert_eq!(num_digits(1234),4);
    }

    #[test]
    fn spaces_4() {
        let matrix = Matrix::new(12,1,123,1);
        assert_eq!(matrix.top_bot_spaces(), "         ");
    }

    #[test]
    fn col_widths() {
        let matrix = Matrix::new(1232112,1,123,1);
        assert_eq!(matrix.col_widths(),vec!(7,1));
    }

    #[test]
    fn top_bot_match_length_of_contents() {
        let mut matrix = Matrix::new(123, 1234, 1, 1);
        assert_eq!(matrix.top_bot_spaces(), "            ");
        matrix.matrix[1][0] = 12345;
        assert_eq!(matrix.top_bot_spaces(), "              ");
    }

    #[test]
    fn print_value_shows_trailing_spaces() {
        let matrix = Matrix::new(12321, 1, 123, 32);
        assert_eq!(matrix.print_value(0,0), "12321");
        assert_eq!(matrix.print_value(0,1), "1 ");
        assert_eq!(matrix.print_value(1,0), "123  ");
        assert_eq!(matrix.print_value(1,1), "32");
    }
}
