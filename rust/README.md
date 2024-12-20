# Fibonacci Recursive Algorithm in Rust

## Overview

This project implements a recursive algorithm to compute Fibonacci numbers in Rust. The Fibonacci sequence is a series of numbers where each number (after the first two) is the sum of the two preceding ones, often used in mathematical and computer science applications.

The program prompts the user to enter a non-negative integer and computes the corresponding Fibonacci number using recursion. It continues to accept inputs until the user decides to exit the program.

## Features

- Computes Fibonacci numbers recursively.
- User-friendly prompts for input.
- Error handling for invalid inputs.
- Option to exit the program gracefully.

## Functionality

1. The program will continually prompt the user to enter a non-negative integer.
2. If the user inputs a valid integer, the program will calculate and display the Fibonacci number.
3. If the user inputs a negative integer or an invalid number, the program will alert the user and prompt for input again.
4. The user can type "exit" to terminate the program gracefully.

## Requirements

- **Rust**: Ensure that you have Rust installed. You can install it using `rustup` by following the instructions on the [Rust Programming Language website](https://www.rust-lang.org/tools/install).

## Runnig the Program

1. Build the Project: You need to compile the Rust code. Run the following command in your terminal:

```bash
cargo build
```

2. Run the Project: After building, execute the program with:

```bash
cargo run
```

3.Interact with the Program: Follow the prompts to enter a non-negative number or type 'exit' to quit.

## Example Usage

```
Please enter a non-negative number (or type 'exit' to quit):
5
The result is: 5

Please enter a non-negative number (or type 'exit' to quit):
10
The result is: 55

Please enter a non-negative number (or type 'exit' to quit):
exit
Exiting the program. Goodbye!
```

## Error Handling
 - **The program will alert you if you enter a negative number or an invalid input (e.g., letters).** 

 - **If you enter 'exit', the program will terminate gracefully.**


### goodbye❤