pub mod matrix;
pub mod etc;
pub mod system;
// use matrix::Matrix;
// use system::System;
use etc::prompt_usize;

fn main() {
    let mut system = system::System::new(3);
    system.prompt_values();
    println!("{}", system.matrix_a);
    println!("{}", system.matrix_b);
    println!("{}", system);

//     let mut matrices = Vec::new();
//     loop {
//         match intro_prompt() {
//             1 => {
//                 matrices.push(Matrix::new_prompt_size());
//                 println!("{}", matrices.last().unwrap());
//                 matrices.last_mut().unwrap().prompt_values();
//             },
//             2 => {
//                 println!("Quitting...");
//                 break;
//             },
//             _ => println!("Invalid, please try again."),
//         }
//         println!("Matrix(ces) in memory:");
//         for matrix in &matrices {
//             println!("{}", matrix);
//         }
//     }
}

pub fn intro_prompt() -> usize {
    println!("1. Store a matrix");
    println!("2. Quit");
    println!("Please select an option:");

    prompt_usize()
}



