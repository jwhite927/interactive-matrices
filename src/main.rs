mod matrix;
use matrix::Matrix;
use std::io;

fn main() {
    let mut my_matrix = Matrix::new(0, 0, 0, 0);
    println!("{}", my_matrix);
    my_matrix.prompt_values();

}

pub fn prompt_i32() -> i32 {
    let mut value = String::new();

    loop {
    match io::stdin().read_line(&mut value) {
        Ok(_) => {
            match value.trim().parse::<i32>() {
                Ok(num) => return num,
                Err(_) => {
                    println!("{} is invalid, please enter an integer.", value.trim());
                    value = String::new();
                    continue;
                },
            }
        },
        Err(_) => {
            println!("{} is invalid, please enter an integer.", value.trim());
            value = String::new();
            continue;
        },
    }
    }
}
