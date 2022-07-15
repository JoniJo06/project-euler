/** @format */

import * as math from './math'
import fs from 'fs-extra'

export const amicable_numbers = (): [string, number] => {

  const numbers: number[] = [];

  for (let i = 0; i < 10000; i++) {
    if (math.is_amicable_number(i))
      numbers.push(i);
  }

  let result: number = numbers.reduce((a, b) => a + b);

  return ["21# amicable numbers", result];
}

const string_score = (str: string): number => {
  return str.toUpperCase().charCodeAt(0) - 64
}

export const names_scores = (): [string, number] => {
  let result: number = 0;

  const input: string = fs.readFileSync('./../inputs/22.txt', 'utf8').toString();
  const names: string[] = input.split(',').map(x => x.replaceAll("\"", ""));
  names.sort()

  for (let i = 0; i < names.length; i++) {
    let name_score: number = 0;
		for (let char of names[i].split('')) {
			name_score += string_score(char);
    }
    result += name_score * (i + 1);
	}

  return ['22# names scores', result];
}
