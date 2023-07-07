# Rust-Threads

Rust-Threads is a simple CPU load generator implemented in Rust. It leverages Rust's powerful multithreading capabilities to stress test your CPU. By calculating the factorials of numbers and their logarithms in parallel, this project showcases both Rust's efficiency and its prowess in concurrent computing.

The factorial of a non-negative integer n is the product of all positive integers less than or equal to n. It's denoted by n!, and it can grow very rapidly with the size of n. This makes it an excellent candidate for testing the capabilities of a CPU.

To represent these large factorials, Rust-Threads uses the logarithm base 10. The Stirling's approximation formula is used to calculate the number of digits in a factorial, which is log10(n!) ≈ n * log10(n) - n * log10(e).

<p align="center"><img src="https://render.githubusercontent.com/render/math?math=n! = n %2A (n-1) %2A ... %2A 2 %2A 1"></p>

## Installation

To install Rust-Threads, you need to have Rust installed. You can download Rust from the official website at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and build the project:

```bash
git clone https://github.com/pizzimenti/Rust-Threads.git
cd Rust-Threads
cargo build --release
```

## License

This project is licensed under the terms of the MIT license.

