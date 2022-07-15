/** @format */

import { exit } from "process";

export enum WeekdayEnum {
	Sun,
	Mon,
	Tue,
	Wed,
	Thu,
	Fri,
	Sat,
	length,
}

export enum MonthEnum {
	Jan,
	Feb,
	Mar,
	Apr,
	May,
	Jun,
	Jul,
	Aug,
	Sep,
	Oct,
	Nov,
	Dec,
	length,
}

export const isPrime = (k: number): boolean => {
    if (k <= 1) {
        return false;
    }

    if (k == 2 || k == 3) {
        return true;
    }

    if (k % 2 == 0 || k % 3 == 0) {
        return false;
    }

    let i = 5;
    while (true) {
        if (i * i > k) {
            break;
        }

        if (k % i == 0 || k % (i + 2) == 0) {
            return false;
        }

        i += 6;
    }

    return true
}

export const number_to_english = (number: number): string => {
	const ones: string[] = [
		'zero',
		'one',
		'two',
		'three',
		'four',
		'five',
		'six',
		'seven',
		'eight',
		'nine',
		'ten',
		'eleven',
		'twelve',
		'thirteen',
		'fourteen',
		'fifteen',
		'sixteen',
		'seventeen',
		'eighteen',
		'nineteen',
	];

	const tens: (string | undefined)[] = [undefined, undefined, 'twenty', 'thirty', 'forty', 'fifty', 'sixty', 'seventy', 'eighty', 'ninety'];

	if (number >= 0 && number <= 19) return ones[number];

	if (number >= 20 && number <= 90 && number % 10 == 0)
		// @ts-ignore
		return tens[Math.floor(number / 10)] === undefined ? exit(1) : tens[Math.floor(number / 10)];

	if (number >= 21 && number <= 99) return tens[Math.floor(number / 10)] + '-' + ones[number % 10];

	if (number >= 100 && number <= 900 && number % 100 == 0) return ones[number / 100] + ' hundred';

	if (number >= 101 && number <= 999) return ones[Math.floor(number / 100)] + ' hundred and ' + number_to_english(number % 100);

	if (number == 1000) return 'one thousand';

	console.assert(false, 'unexpected number');
    exit(1);
};

export const factorial = (n: bigint): bigint => {
	if (n <= 1n) return 1n;
	return n * factorial(n - 1n);
}

export const is_leap_year = (year: number): boolean => {
    let four = year % 4 == 0;
    let hundred = year % 100 == 0;
    let four_hundred = year % 400 == 0;
    if (!four) {
        return false;
    }
    if (hundred && !four_hundred) {
        return false;
    }
    return true;
}

export const month_length = (month: number, leap_year: boolean): number => {
    switch (month) {
			case MonthEnum.Jan: return 31;
			case MonthEnum.Feb: return leap_year ? 29 : 28;
      case MonthEnum.Mar: return 31;
      case MonthEnum.Apr: return 30;
      case MonthEnum.May: return 31;
      case MonthEnum.Jun: return 30;
      case MonthEnum.Jul: return 31;
      case MonthEnum.Aug: return 31;
      case MonthEnum.Sep: return 30;
      case MonthEnum.Oct: return 31;
      case MonthEnum.Nov: return 30;
			case MonthEnum.May: return 31;
			default: return -1;
    }
}