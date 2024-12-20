# Fibonacci Recursive Algorithm Implementation

## Table of Contents

- [Project Overview](#project-overview)
- [Languages Implemented](#languages-implemented)
  - [Rust](#rust)
  - [Python](#python)
  - [C](#c)
- [Installation](#installation)
- [Usage](#usage)
- [Performance Comparison](#performance-comparison)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Project Overview

This project aims to implement the Fibonacci recursive algorithm in multiple programming languages, specifically Rust, Python, and C. The Fibonacci sequence is a classic example in computer science and mathematics, defined as follows:

- F(0) = 0
- F(1) = 1
- F(n) = F(n-1) + F(n-2) for n > 1

The recursive implementation provides a straightforward approach to understanding recursion, though it is not the most efficient for large values of `n` due to its exponential time complexity. This project serves as an educational tool to demonstrate the differences in syntax, performance, and recursion handling across different programming languages.

## Languages Implemented

### Rust

Rust is a systems programming language that emphasizes safety and performance. The recursive Fibonacci implementation in Rust showcases its strong type system and memory safety features.

#### Example Implementation

```rust
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

### Python

Python is a high-level, interpreted language known for its readability and simplicity. The recursive Fibonacci implementation in Python demonstrates its concise syntax.

#### Example Implementation

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

### C

C is a low-level programming language that provides a good balance between performance and control over system resources. The recursive Fibonacci implementation in C highlights its procedural programming paradigm.

#### Example Implementation

```C

#include <stdio.h>

int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

## Installation

To run the implementations, ensure you have the respective programming environments set up on your machine.

### Rust

1. Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone the repository:

```bash
git clone https://github.com/awminsheikh/fibonacci.git
cd fibonacci/rust
```

3. Build and run the program:

```bash
cargo run
```

### Python

1. Install Python from [python.org](https://www.python.org/).
2. Clone the repository:

```bash
git clone https://github.com/awminsheikh/fibonacci.git
cd fibonacci/python
```

3. Run the program:

```bash
python fibonacci.py
```

### C

1. install a C compiler (e.g., GCC). Instructions can be found at [gcc.gnu.org](https://gcc.gnu.org/)
2. Clone the repository :

```bash
git clone https://github.com/awminsheikh/fibonacci.git
cd fibonacci/c
```

3. Compile amd run the program:

```bash
gcc fibonacci.c -o fibonacci
./fibonacci
```

## Usage

To compute the Fibonacci number for a given n, simply call the fibonacci function with the desired integer as an argument. For example, calling fibonacci(5) will return 5, which is the fifth number in the Fibonacci sequence.

## Performance Comparison

While the recursive approach is elegant, it is not efficient for large values of n due to repeated calculations. As part of this project, you may consider implementing memoization or iterative approaches in the future to improve performance.

### Benchmarking

You can benchmark the performance of each implementation using a simple test script. Ensure you have the necessary tools installed for each language to measure execution time.

### Contributing

Contributions are welcome! If you would like to add more programming languages, improve existing implementations, or provide additional features, please follow these steps:

1. Fork the repository.
2. Create a new branch (git checkout -b feature/YourFeature).
3. Make your changes and commit them (git commit -m 'Add some feature').
4. Push to the branch (git push origin feature/YourFeature).
5. Open a Pull Request.

## License

This project is licensed under the Amin Sheikh License
