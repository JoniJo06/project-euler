const even_fibonacci_numbers = (): number => {
  let result: number = 0;

  let a: number = 1;
  let b: number = 2;

  while (b < 4_000_000) {
    if (b % 2 === 0) {
      result += b;
    }
    let temp = a + b;
    a = b;
    b = temp;
  }

  return result;
}

export default even_fibonacci_numbers;