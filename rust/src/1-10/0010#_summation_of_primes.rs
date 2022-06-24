mod math;

pub fn start() {
  println!("0010# summation of primes");
  let mut result: u64 = 0;

  for num in 0..2_000_000_u64 {
    if math::is_prime(num) {
      result += num;
    }
  }

  println!("Result: {}", result);
}