use std::io; // For user input
use std::io::Write; // For stdout flush
use num_bigint::BigUint; // For arbitrarily large integers
use num_traits::One; // For BigUint::one()
use rayon::prelude::*; // For parallel iterators
use std::time::{Duration, Instant}; // For timing

// Function to compute the factorial of a number
fn factorial(n: usize) -> BigUint {
    (1..=n).into_iter().fold(BigUint::one(), |f, i| f * BigUint::from(i))
}

// Function to compute the number of digits in a number
fn num_digits(n: &BigUint) -> f64 {
    let n_str = n.to_string();
    n_str.len() as f64
}

fn main() {
    // Prompt user for maximum number of decimal places
    println!("Please enter a number between 1000 and 5000:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let max_value: usize = match input.trim().parse() {
        Ok(num) => {
            if num >= 1000 && num <= 5000 {
                num
            } else {
                println!("Number out of range. Defaulting to 1000.");
                1000
            }
        },
        Err(_) => {
            println!("Invalid input. Defaulting to 5000.");
            5000
        },
    };

    let mut work_units_completed: usize = 0;

    loop {
        // Create a parallel iterator over the range from 0 to max_value
        let input_data: Vec<usize> = (0..=max_value).collect();

        // Process each number in parallel, and reduce the computation times into a total
        let total_computation_time: Duration = input_data.into_par_iter().map(|i| {
            let start = Instant::now(); // Start timing
            let factorial = factorial(i); // Compute the factorial
            let computation_time = start.elapsed(); // Compute elapsed time

            let _digits = num_digits(&factorial); // Compute number of digits

            computation_time // Return the computation time
        }).reduce(|| Duration::new(0, 0), |a, b| a + b); // Reduce computation times into a total

        // Increment completed work units counter
        work_units_completed += 1;

        // Clear the line and print message
        print!("\x1B[2K\r{} factorials computed to {} decimal places in {:.2} seconds", work_units_completed, max_value, total_computation_time.as_secs_f64());
        std::io::stdout().flush().unwrap();
    }
}
