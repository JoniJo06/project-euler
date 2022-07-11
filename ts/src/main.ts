/** @format */

import { exit } from 'process';
import fs from 'fs-extra';
import multiples_of_3_or_5 from './1-10/1#_multiples_of_3_or_5';
import { equal } from 'assert';

const DEBUG = false;
const SOLVED = true;

const compare = (n: number, result: string | number) => {
	let rawInput: string = fs.readFileSync('./../results.txt', 'utf8').toString();
	let results: (number | string)[] = rawInput.split('\n').map(Number);
	results.unshift(0);
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

	if (DEBUG) console.log(argv);

	// @ts-ignore-next-line
	const problem: number = parseInt(argv.shift(), 10);
	if (isNaN(problem)) {
		console.error(`Usage: ${program} <problem number>`);
		exit(1);
	}

	let result: string | number = 0;

	switch (problem) {
		case 1: {
			result = multiples_of_3_or_5();
			break;
		}
	}
	console.log('Result: ' + result);
	if (SOLVED) compare(problem, result);

	if (DEBUG) console.log(problem);
};
main();
