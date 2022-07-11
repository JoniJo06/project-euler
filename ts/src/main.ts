/** @format */

import { exit } from 'process';
import { equal } from 'assert';

import fs from 'fs-extra';
import {
	multiples_of_3_or_5,
	even_fibonacci_numbers,
	largest_prime_factor,
	largest_palindrome_product,
	smallest_multiple,
	sum_square_difference,
	_10001st_prime,
	largest_product_in_a_series,
	special_pythagorean_triplet,
} from './1-10';

const DEBUG = false;
const SOLVED = true;

const compare = (n: number, result: string | number) => {
	let rawInput: string = fs.readFileSync('./../results.txt', 'utf8').toString();
	let results: (number | string)[] = rawInput.split('\n').map(Number);
  results.unshift(0);
  if (results.length < n) return;
	equal(result, results[n]);
};

const main = () => {
	let argv = process.argv;
	// remove first irrelevant argument
	argv.shift();
	const program: string = argv.shift() || exit(1);

	if (argv.length < 1) {
		console.error(`Usage: ${program} <problem number>`);
		exit(1);
	}

	if (DEBUG) console.log("command line arguments" + argv);

	// @ts-ignore-next-line
	const problem: number = parseInt(argv.shift(), 10);
	if (isNaN(problem)) {
		console.error(`Usage: ${program} <problem number>`);
		exit(1);
	}

	let startTime = new Date();

  let problems: Function[] = [
    multiples_of_3_or_5,
    even_fibonacci_numbers,
		largest_prime_factor,
		largest_palindrome_product,
		smallest_multiple,
		sum_square_difference,
		_10001st_prime,
		largest_product_in_a_series,
		special_pythagorean_triplet,
  ];


  if (problem > problems.length) {
    console.error("problem out of range");
    console.error("try to access problem " + problem + " of " + problems.length)
    exit(1);
  }

	let [name, result]: [string, number] = problems[problem - 1].call(this);

	console.log("-----------------------")
	console.log(name)
	console.log('Result: ' + result);
	console.log("-----------------------")
	if (SOLVED) compare(problem, result);

	let endTime = new Date();
	let timeElapsed = endTime.getTime() - startTime.getTime();
	if (DEBUG) {
		console.log(problem);
		console.log("ms elapsed: " + timeElapsed);
	}
};
main();
