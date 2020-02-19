use std::fmt;
use crate::etc::{prompt_i32, prompt_usize, num_digits};
use std::cmp::Ordering;

const MAX_ROWS_AND_COLUMNS: usize = 5;

pub struct Matrix {
    pub matrix: [[Option<i32>; MAX_ROWS_AND_COLUMNS]; MAX_ROWS_AND_COLUMNS],
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Matrix {
        let new_matrix = Matrix {
//             matrix: [[None; 3]; 3],
            matrix: [[None; MAX_ROWS_AND_COLUMNS]; MAX_ROWS_AND_COLUMNS],
            rows,
            columns,
        };
        new_matrix
    }

    pub fn new_prompt_size() -> Matrix {
        println!("What size matrix would you like to manipulate?");
        println!("Number of rows:");
        let rows = prompt_usize();
        println!("Number of columns:");
        let columns = prompt_usize();
        Matrix::new(rows, columns)
    }

    //get_value works with array indices starting at 0. It is only when 
    //a value is presented to the user that values start at 1.
    pub fn get_value(&self, row: usize, column: usize) -> i32 {
        let mut row_to_return = self.rows - 1;
        if let Ordering::Greater = row_to_return.cmp(&row) {
            row_to_return = row;
        }

        let mut col_to_return = self.columns - 1;
        if let Ordering::Greater = col_to_return.cmp(&column) {
            col_to_return = column;
        }
        if let Some(value) = self.matrix[row_to_return][col_to_return] {
            return value;
        }
        0
    }

    pub fn top_bot_spaces(&self) -> String {
        let mut string = String::new();
        let extra_spaces = 3 * (self.columns - 1) + 2;
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
        for rows in 0..self.rows {
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
                self.matrix[row][column] = Some(prompt_i32());
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
