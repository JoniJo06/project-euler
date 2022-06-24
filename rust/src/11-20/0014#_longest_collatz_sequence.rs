pub fn start() {
  println!("0014# longest collatz sequence");
  let mut result: (u64, u64) = (0, 0);

  for i in 0..1_000_000 {
    let mut terms = 0;
    let mut n = i;
    while n > 1 {
      if n % 2 == 0 {
        n /= 2;
      } else {
        n = n * 3 + 1;
      }
      terms += 1;
    }
    if terms > result.1 {
      result = (i, terms);
    }
  }
  println!("Result: {}", result.0);
}