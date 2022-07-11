#![allow(dead_code)]

use std::env;

#[path = "math.rs"]
mod math;

#[path = "1-10/0007#_10001st_prime.rs"]
mod _10001st_prime;
#[path = "1-10/0002#_even_fibonacci_numbers.rs"]
mod even_fibonacci_numbers;
#[path = "11-20/0012#_highly_divisible_triangular_number.rs"]
mod highly_divisible_triangular_number;
#[path = "11-20/0013#_large_sum.rs"]
mod large_sum;
#[path = "1-10/0004#_largest_palindrome_product.rs"]
mod largest_palindrome_product;
#[path = "1-10/0003#_largest_prime_factor.rs"]
mod largest_prime_factor;
#[path = "11-20/0011#_largest_product_in_a_grid.rs"]
mod largest_product_in_a_grid;
#[path = "1-10/0008#_largest_product_in_a_series.rs"]
mod largest_product_in_a_series;
#[path = "11-20/0015#_lattice_paths.rs"]
mod lattice_paths;
#[path = "11-20/0014#_longest_collatz_sequence.rs"]
mod longest_collatz_sequence;
#[path = "1-10/0001#_multiples_of_3_or_5.rs"]
mod multiples_of_3_or_5;
#[path = "11-20/0016#_power_digit_sum.rs"]
mod power_digit_sum;
#[path = "1-10/0005#_smallest_multiple.rs"]
mod smallest_multiple;
#[path = "1-10/0009#_special_pythagorean_triplet.rs"]
mod special_pythagorean_triplet;
#[path = "1-10/0006#_sum_square_difference.rs"]
mod sum_square_difference;
#[path = "1-10/0010#_summation_of_primes.rs"]
mod summation_of_primes;

struct Gap {
    amount: i16,
    character: char,
}

impl Gap {
    fn new(amount: Option<i16>, character: Option<char>) -> Self {
        Gap {
            amount: amount.unwrap_or(20),
            character: character.unwrap_or('-'),
        }
    }

    fn default(&mut self) {
        for _ in 0..self.amount {
            print!("{}", self.character);
        }
        print!("\n");
    }
    fn print(&mut self, amount: i16, character: char) {
        for _ in 0..amount {
            print!("{}", character);
        }
        print!("\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut gap: Gap = Gap::new(None, None);
    gap.default();
    match args[1].parse::<i32>().expect("you must provide a number") {
        1 => multiples_of_3_or_5::start(),
        2 => even_fibonacci_numbers::start(),
        3 => largest_prime_factor::start(),
        4 => largest_palindrome_product::start(),
        5 => smallest_multiple::start(),
        6 => sum_square_difference::start(),
        7 => _10001st_prime::start(),
        8 => largest_product_in_a_series::start(),
        9 => special_pythagorean_triplet::start(),
        10 => summation_of_primes::start(),
        11 => largest_product_in_a_grid::start(),
        12 => highly_divisible_triangular_number::start(),
        13 => large_sum::start(),
        14 => longest_collatz_sequence::start(),
        15 => lattice_paths::start(),
        16 => power_digit_sum::start(),
        _ => (),
    }
    gap.default();
}
// mod test;
