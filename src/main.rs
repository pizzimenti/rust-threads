use std::io; // For user input
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
    // Request maximum number from user
    println!("Please enter the maximum number to compute the factorial for:");

    let mut max_value_string = String::new();
    io::stdin().read_line(&mut max_value_string).unwrap();
    let max_value: usize = max_value_string.trim().parse().unwrap();

    // Create a parallel iterator over the range from 0 to max_value
    let input_data: Vec<usize> = (0..=max_value).collect();

    // Process each number in parallel, and reduce the computation times into a total
    let total_computation_time: Duration = input_data.into_par_iter().map(|i| {
        let start = Instant::now(); // Start timing
        let factorial = factorial(i); // Compute the factorial
        let computation_time = start.elapsed(); // Compute elapsed time

        let digits = num_digits(&factorial); // Compute number of digits
        println!("The factorial of {} has approximately {:.2e} digits", i, digits);
        println!("Computation time: {:?}", computation_time);

        computation_time // Return the computation time
    }).reduce(|| Duration::new(0, 0), |a, b| a + b); // Reduce computation times into a total

    // Print total computation time
    println!("Total computation time: {:?}", total_computation_time);
}
