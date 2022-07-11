/** @format */

import { isPrime } from "./math";

export const multiples_of_3_or_5 = (): [string, number] => {
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

	return ['1# multiples of 3 or 5', result];
};

export const even_fibonacci_numbers = (): [string, number] => {
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

	return ['2# even fibonacci numbers', result];
};

export const largest_prime_factor = (): [string, number] => {
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

	return ['3# largest prime factor', result];
};

export const largest_palindrome_product = (): [string, number] => {
	let result: number = 0;

	for (let i = 100; i < 999; i++) {
		for (let j = 100; j < 999; j++) {
			let product: number = i * j;
			let rev_product: number = Number(product.toString().split('').reverse().join(''));
			if (product === rev_product && product > result) {
				result = product;
			}
		}
	}
	return ['4# largest palindrome product', result];
};

export const smallest_multiple = (): [string, number] => {
	let result: number = 0;
	let n: number = 20;
	while (true) {
		let dividable: boolean = true;
		for (let i = 1; i <= 20; i++) {
			if (n % i !== 0) {
				dividable = false;
			}
		}
		if (dividable) {
			result = n;
			break;
		}
		n += 20;
	}
	return ['5# smallest multiple', result];
};

export const sum_square_difference = (): [string, number] => {
	let n: number = 100;
	let sum_square: number = 0;

	for (let i = 1; i <= n; i++) {
		sum_square += Math.pow(i, 2);
	}

	let square_sum: number = 0;
	for (let i = 1; i <= n; i++) {
		square_sum += i;
	}

	square_sum = Math.pow(square_sum, 2);

	let result: number = square_sum - sum_square;

	return ['6# sum square difference', result];
}

export const _10001st_prime = (): [string, number] => {
	let n: number = 10001;
	let i: number = 2;

	while (n > 0) {
		if (isPrime(i)) {
			
			n--;
		}
		i++;
	}
	i--;

	return ['7# 10001st prime', i];
}

export const largest_product_in_a_series = (): [string, number] => {
	let input: string = `73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450`
	
	let result: number = 0;

	let numbers: number[] = input
		.split('\n')
		.join('')
		.split('')
		.map(Number)
	
	let n: number = 13;

	for (let i = 0; i < numbers.length - n; i++) {
		let product: number = 1;
		for (let j = 0; j < n; j++) {
			product *= numbers[i + j];
		}
		if (product > result) {
			result = product;
		}
	}
	
	return ['8# largest product in a series', result];
}