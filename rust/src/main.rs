use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Please enter a non-negative number (or type 'exit' to quit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("exit") {
            println!("Exiting the program. Goodbye!");
            break;
        }
        match trimmed_input.parse::<i32>() {
            Ok(number) => {
                if number < 0 {
                    println!("Error: Negative numbers are not allowed. Please try again.");
                } else {
                    let result = fibonacci(number);
                    println!("The result is: {}", result);
                }
            }
            Err(_) => {
                println!("Error: Please enter a valid number.");
            }
        }
    }
}

fn fibonacci(num: i32) -> i32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
