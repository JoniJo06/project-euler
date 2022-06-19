pub fn start() {
  println!("0006#_sum_square_difference");

  let n = 100;
  let result;

  let mut sum_square = 0;
  for i in 1..n + 1 {
    sum_square += i32::pow(i, 2);
  }

  let mut square_sum = 0;
  for i in 1..n + 1 { 
    square_sum += i;
  }
  square_sum = i32::pow(square_sum, 2);


  result = square_sum - sum_square;
  println!("Result: {}", result);
}