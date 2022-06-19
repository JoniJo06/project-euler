pub fn start() {
  println!("0005#_smallest_multiple");
  let result;
  let mut n: u128 = 20;
  loop {
    let mut dividable: bool = true;
    for i in 1..20 {
      if n % i != 0 {
        dividable = false;
      }
    }
    if dividable {
      result = n;
      break;
    }

    n += 20;
  }

  println!("Result: {}", result);
}