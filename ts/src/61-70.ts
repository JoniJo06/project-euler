/** @format */

import fs from 'fs-extra'

const calcPath = (triangle: number[][]): number => {
	for (let row = triangle.length - 2; row >= 0; row--) {
		for (let col = 0; col <= row; col++) {
			triangle[row][col] += Math.max(triangle[row + 1][col], triangle[row + 1][col + 1]);
		}
	}
	return triangle[0][0];
};

export const maximum_path_sum_II = (): [string, number] => {
	let result: number = 0;

	const input: string = fs.readFileSync('./../inputs/67.txt', 'utf8');
	const triangle: number[][] = input.split('\n').map((x) => x.split(' ').map(Number));

	result = calcPath(triangle);

	return ['67# maximum path sum II', result];
};
