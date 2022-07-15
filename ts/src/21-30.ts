/** @format */

import * as math from './math'

export const amicable_numbers = (): [string, number] => {

  const numbers: number[] = [];

  for (let i = 0; i < 10000; i++) {
    if (math.is_amicable_number(i))
      numbers.push(i);
  }

  let result: number = numbers.reduce((a, b) => a + b);

  return ["21# amicable numbers", result];
}
