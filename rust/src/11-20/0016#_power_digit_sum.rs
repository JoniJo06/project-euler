use num::BigUint;

pub fn start() {
  println!("0016# power digit sum");
  let mut result = 0;
  let number = BigUint::parse_bytes(b"2", 10).unwrap();
  let number = number.pow(1000);
  let numbers = number.to_string().chars().map(|x| x.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
  for num in numbers {
    result += num;
  }
  println!("Result: {}", result);
}