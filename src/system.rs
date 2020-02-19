use std::fmt;
use crate::matrix::Matrix;

// pub enum Unknowns {
//     Two,
//     Three,
// }

pub struct System {
    pub matrix_a: Matrix,
    pub matrix_b: Matrix,
}

impl System {
    pub fn new(unknowns: usize) -> System {
        match unknowns {
            2 => {
                let new_system = System {
                    matrix_a: Matrix::new(2, 2),
                    matrix_b: Matrix::new(2, 1),
                };
                return new_system;
            }
            _ => {
                let new_system = System {
                    matrix_a: Matrix::new(3, 3),
                    matrix_b: Matrix::new(3, 1),
                };
                new_system
            }
        }
    }

    pub fn prompt_values(&mut self) {
        println!("Please fill in coefficients matrix...");
        println!("{}", self.matrix_a);
        self.matrix_a.prompt_values();
        println!("Please fill in right side matrix...");
        println!("{}", self.matrix_b);
        self.matrix_b.prompt_values();
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_to_write = "\u{250C}".to_owned();
        string_to_write.push_str(&self.matrix_a.top_bot_spaces());
        string_to_write.push_str("\u{2510}\u{250C}   \u{2510}   \u{250C}");
        string_to_write.push_str(&self.matrix_b.top_bot_spaces());

        string_to_write.push_str("\u{2510}\n\u{2502}");
//         for row in 0..self.rows {
//             for column in 0..self.columns {
//                 string_to_write.push_str(&self.print_value(row,column));
//                 if column < self.columns - 1 {
//                     string_to_write.push_str(" , ");
//                 }
//             }
//             if row < self.rows - 1 {
//             string_to_write.push_str(" \u{2502}\n\u{2502} ");
//             }
//         }
        string_to_write.push_str(" \u{2502}\n\u{2514}");
        string_to_write.push_str(&self.matrix_a.top_bot_spaces());
        string_to_write.push_str("\u{2518}");
        write!(f, "{}", string_to_write)
    }
}
