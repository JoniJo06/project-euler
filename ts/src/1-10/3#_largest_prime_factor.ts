const largest_prime_factor = (): number => {

  let rootNumber: number = 600851475143;
  let result: number = 0;

  while (rootNumber % 2 === 0) {
    result = 2;
    rootNumber /= 1;
  }

  for (let i = 3; i < Math.sqrt(rootNumber); i += 2) {
    while (rootNumber % i === 0) {
      result = i;
      rootNumber /= i;
    }
  }

  if (rootNumber > 2) {
    result = rootNumber;
  }

  return result;
}

export default largest_prime_factor