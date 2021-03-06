#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "main.h"
#include "pe/pe_math.h"

void compare(int problem, long result) {
  FILE *file;
  char line[255];
  int lines = 0;
  file = fopen("./../results.txt", "r");
  if (file == NULL) {
    exit(EXIT_FAILURE);
  }

  while (fgets(line, sizeof(line), file) != NULL) {
    lines++;
  }

  if (lines <= problem) {
    fprintf(stderr, "Does not exist in results.txt\n");
    return;
  }

  file = fopen("./../results.txt", "r");

  int i = 1;
  while (fgets(line, sizeof(line), file) != NULL) {
    if (i == problem)
      break;
    i++;
  }

  if (atol(line) != result) {
    fprintf(stderr, "This answer is incorrect!\n");
    exit(EXIT_FAILURE);
  }
  fclose(file);
}

void (*problems[])(retTuple *ret) = {
    // 1-10
    multiples_of_3_or_5,
    even_fibonacci_numbers,
    largest_prime_factor,
    largest_palindrome_product,
    smallest_multiple,
    sum_square_difference,
    _10001st_prime,
    largest_product_in_a_series,
    special_pythagorean_triplet,
    summation_of_primes,
    // 11-20
    largest_product_in_a_grid,
    highly_divisible_triangular_number,
    large_sum,
    longest_collatz_sequence,
    lattice_paths,
    power_digit_sum,
};

int main(int argc, char **argv) {
  const char *program = argv[0];
  argv++;
  argc--;

  if (argc < 1) {
    fprintf(stderr, "usage: %s <problem number>\n", program);
    exit(EXIT_FAILURE);
  }

  int problem = atoi(argv[0]);

  if (problem <= 0) {
    fprintf(stderr, "usage: %s <problem number>\n", program);
    exit(EXIT_FAILURE);
  }

  if ((unsigned long)problem > sizeof(problems) / sizeof(problems[0])) {
    fprintf(stderr, "problem out of range\n");
    fprintf(stderr, "try to access problem %d of %ld\n", problem,
            sizeof(problems) / sizeof(problems[0]));
    exit(EXIT_FAILURE);
  }

  retTuple tuple;
  tuple.name = "unknown";
  tuple.result = -1;
  problems[problem - 1](&tuple);

  printf("--------------------------------\n");
  printf("%s\n", tuple.name);
  printf("Result: %ld\n", tuple.result);
  printf("--------------------------------\n");
  compare(problem, tuple.result);
  exit(EXIT_SUCCESS);
}