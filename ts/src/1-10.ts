/** @format */

export const multiples_of_3_or_5 = (): number => {
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
};

export const even_fibonacci_numbers = (): number => {
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
};

export const largest_prime_factor = (): number => {
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
};