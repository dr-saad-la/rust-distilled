// you need to add the rand crate to Cargo.toml
// You can do this by running: `cargo add rand`

use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// Function to generate a random array of integers
pub fn gen_rand_ints(size: usize, seed: u64) -> Vec<i32> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    return (0..size).map(|_| rng.gen_range(0..100)).collect();
}

// Function to generate a random array of floats with two decimal points
pub fn gen_rand_floats(size: usize, seed: u64) -> Vec<f64> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    (0..size)
        .map(|_| {
            let num: f64 = rng.gen_range(0.0..100.0);
            (num * 100.0).round() / 100.0
        })
        .collect()
}

// Function to generate a random array of characters
pub fn gen_rand_chars(size: usize, seed: u64) -> Vec<char> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    (0..size)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

pub fn banner(sep: &str, nchar: usize, message: &str) {
    let sep = sep.repeat(nchar);
    let message = format!("{:^width$}", message, width = nchar);
    println!("\n{}\n{}\n{}", sep, message, sep);
}
