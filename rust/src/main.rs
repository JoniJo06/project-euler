#![allow(dead_code)]

use std::env;


#[path = "0001#_multiples_of_3_or_5.rs"] mod multiples_of_3_or_5;
#[path = "0002#_even_fibonacci_numbers.rs"] mod even_fibonacci_numbers;
#[path = "0003#_largest_prime_factor.rs"] mod largest_prime_factor;
#[path = "0004#_largest_palindrome_product.rs"] mod largest_palindrome_product;
#[path = "0005#_smallest_multiple.rs"] mod smallest_multiple;
#[path = "0006#_sum_square_difference.rs"] mod sum_square_difference;
#[path = "0007#_10001st_prime.rs"] mod _10001st_prime;

struct Gap {
    amount: i16,
    character: char,
}

impl Gap {

    fn new(amount: Option<i16>, character: Option<char>) -> Self {
        Gap { 
            amount: amount.unwrap_or(20),
            character: character.unwrap_or('-') 
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
        _ => ()
    }
    gap.default();
}
