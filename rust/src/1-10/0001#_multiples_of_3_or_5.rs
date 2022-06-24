pub fn start() {
  println!("0001# multiples_of_3_or_5");

  let mut result = 0;
  let mut numbers: Vec<i32> = Vec::new();

  for n in 0..1000 {
    if n % 3 == 0 {
      numbers.push(n);
      continue;
    }
    if n % 5 == 0 {
      numbers.push(n);
      continue;
    }
  }

  for number in numbers.iter() {
    result += number;
  }

  println!("Result: {}", result);
}