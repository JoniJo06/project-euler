#![allow(dead_code)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
};

#[path = "math.rs"]
mod math;

#[path = "1-10.rs"]
mod _1_10;

#[path = "11-20.rs"]
mod _11_20;

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
        println!();
    }
    fn print(&mut self, amount: i16, character: char) {
        for _ in 0..amount {
            print!("{}", character);
        }
        println!();
    }
}

fn compare(n: usize, result: i128) {
    let path = Path::new("./../results.txt");
    if !path.exists() {
        panic!("File does not exist");
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<i128> = reader
        .lines()
        .enumerate()
        .map(|(num, line)| {
            line.unwrap()
                .parse::<i128>()
                .unwrap_or_else(|_| panic!("unable to parse line {}", num))
        })
        .collect();
    lines.insert(0, 0);

    if lines.len() <= n {
        println!("Does not exist in results.txt");
        return;
    }

    assert_eq!(lines[n], result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut gap: Gap = Gap::new(None, None);
    gap.default();

    let problems: Vec<fn() -> (&'static str, i128)> = vec![
        _1_10::multiples_of_3_or_5,
        _1_10::even_fibonacci_numbers,
        _1_10::largest_prime_factor,
        _1_10::largest_palindrome_product,
        _1_10::smallest_multiple,
        _1_10::sum_square_difference,
        _1_10::_10001st_prime,
        _1_10::largest_product_in_a_series,
        _1_10::special_pythagorean_triplet,
        _1_10::summation_of_primes,
        _11_20::largest_product_in_a_grid,
        _11_20::highly_divisible_triangular_number,
        _11_20::large_sum,
        _11_20::longest_collatz_sequence,
        _11_20::lattice_paths,
        _11_20::power_digit_sum,
    ];

    let problem = args[1].parse::<usize>().expect("you must provide a number");
    if problem < 1 || problem > problems.len() {
        eprintln!("problem out of range");
        eprintln!("try to access problem {} of {}!", problem, problems.len());
        exit(1);
    }
    let (name, result) = problems[problem - 1]();
    println!("{}", name);
    println!("Result {}", result);
    compare(problem, result);

    gap.default();
}
// mod test;
