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
