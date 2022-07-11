const multiples_of_3_or_5 = (): number => {
  let result: number = 0;

  for (let n = 0; n < 1000; n++) {
    if (n % 3 == 0) {
      result += n;
      continue;
    }
    if (n % 5 == 0) {
      result += n;
      continue;
    }
  }

  return result;
}

export default multiples_of_3_or_5;