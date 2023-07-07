# Rust-Threads

Rust-Threads is a simple CPU load generator implemented in Rust written by GPT-4 with 17 human prompts, including the creation of this README. It leverages Rust's powerful multithreading capabilities to stress test your CPU. By calculating the factorials of numbers and their logarithms in parallel, it takes advantage of Rust's efficiency in concurrent computing.

Rust-Threads uses the logarithm base 10. The Stirling's approximation formula is used to calculate the number of digits in a factorial, which is log10(n!) ≈ n * log10(n) - n * log10(e). You will be prompted for the number of digits. Any amount over 10000 should provide a few seconds to a few minutes of CPU on low end machines.

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

## AI Disclaimer

This file was written by AI and edited by a human.

