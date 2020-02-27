use std::io;

pub fn prompt_i32() -> i32 {
    let mut value = String::new();

    loop {
        match io::stdin().read_line(&mut value) {
            Ok(_) => match value.trim().parse::<i32>() {
                Ok(num) => return num,
                Err(_) => {
                    println!("{} is invalid, please enter an integer.", value.trim());
                    value = String::new();
                    continue;
                }
            },
            Err(_) => {
                println!("{} is invalid, please enter an integer.", value.trim());
                value = String::new();
                continue;
            }
        }
    }
}

pub fn prompt_usize() -> usize {
    let mut value = String::new();

    loop {
        match io::stdin().read_line(&mut value) {
            Ok(_) => match value.trim().parse::<usize>() {
                Ok(num) => return num,
                Err(_) => {
                    println!(
                        "{} is invalid, please enter a positive integer.",
                        value.trim()
                    );
                    value = String::new();
                    continue;
                }
            },
            Err(_) => {
                println!(
                    "{} is invalid, please enter a positive integer.",
                    value.trim()
                );
                value = String::new();
                continue;
            }
        }
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
