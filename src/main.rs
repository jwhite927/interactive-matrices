pub mod matrix;
pub mod etc;
use matrix::Matrix;

fn main() {
    let mut my_matrix = Matrix::new(3, 3);
    println!("{}", my_matrix);
    my_matrix.prompt_values();

}
