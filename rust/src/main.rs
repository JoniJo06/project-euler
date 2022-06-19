#![allow(dead_code)]

#[path = "1#_multiples_of_3_or_5.rs"] mod multiples_of_3_or_5;
#[path = "2#_even_fibonacci_numbers.rs"] mod even_fibonacci_numbers;

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
    let mut gap: Gap = Gap::new(None, None);
    gap.default();
    multiples_of_3_or_5::start();
    gap.default();
    even_fibonacci_numbers::start();
    gap.default();
}
