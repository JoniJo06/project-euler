/** @format */

import * as math from './math';
import fs from 'fs-extra';

export const amicable_numbers = (): [string, number] => {
	const numbers: number[] = [];

	for (let i = 0; i < 10000; i++) {
		if (math.is_amicable_number(i)) numbers.push(i);
	}

	let result: number = numbers.reduce((a, b) => a + b);

	return ['21# amicable numbers', result];
};

const string_score = (str: string): number => {
	return str.toUpperCase().charCodeAt(0) - 64;
};

export const names_scores = (): [string, number] => {
	let result: number = 0;

	const input: string = fs.readFileSync('./../inputs/22.txt', 'utf8').toString();
	const names: string[] = input.split(',').map((x) => x.replaceAll('"', ''));
	names.sort();

	for (let i = 0; i < names.length; i++) {
		let name_score: number = 0;
		for (let char of names[i].split('')) {
			name_score += string_score(char);
		}
		result += name_score * (i + 1);
	}

	return ['22# names scores', result];
};

export const non_abundant_sums = (): [string, number] => {
	// TODO: Convert from Brute Force to dynamic
	let result: number = 0;
	const abundants: number[] = [];

	const N: number = 28123;

	for (let i = 0; i < N; i++) {
		const divisors = math.get_all_divisors(i);
		if (i < divisors.reduce((a, b) => a + b)) abundants.push(i);
	}

	for (let n = 0; n < N; n++) {
		console.log(n);
		let can: boolean = false;
		outer: for (let i = 0; i < abundants.length; i++) {
			for (let j = 0; j < abundants.length; j++) {
				if (abundants[j] > n) continue outer;
				if (abundants[i] > n) break outer;
				if (abundants[i] + abundants[j] === n) can = true;
			}
		}
		if (!can) result += n;
	}

	console.log(abundants);

	return ['23# Non-abundant sums', result];
};
