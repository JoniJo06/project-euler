mod math;

pub fn start() {
  println!("0007# 10001st prime");

  let mut n = 10001;
  let mut i = 2;
  while n > 0 {
    if math::is_prime(i) {
      n -= 1;
    }

    i += 1;
  }
  i -= 1;

  println!("Result: {}", i);
}