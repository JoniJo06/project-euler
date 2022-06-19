use core::num;

pub fn start() {
  println!("0002# even_fibonacci_numbers");

  let mut result: i32 = 0;
  let mut numbers: Vec<i32> = Vec::new();

  let mut a = 1;
  let mut b = 2;

  while b < 4_000_000 {
    if b % 2 == 0 {
      numbers.push(b);
    }
    let temp = a + b;
    a = b;
    b = temp;
  }

  for number in numbers {
    result += number;
  }

  println!("Result: {}", result);
}