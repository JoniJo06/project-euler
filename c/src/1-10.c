#include "main.h"
#include <math.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

void multiples_of_3_or_5(retTuple *ret) {
  ret->name = "1# multiples of 3 or 5";

  int result = 0;

  for (int n = 0; n < 1000; n++) {
    if (n % 3 == 0) {
      result += n;
      continue;
    }
    if (n % 5 == 0) {
      result += n;
      continue;
    }
  }

  ret->result = (long) result;
}

void even_fibonacci_numbers(retTuple *ret) {
  ret->name = "2# even fibonacci numbers";

  int result = 0;
  int a = 1;
  int b = 2;

  while (b < 4000000) {
    if (b % 2 == 0) {
      result += b;
    }
    int temp = a + b;
    a = b;
    b = temp;
  }

  ret->result = (long) result;
}

void largest_prime_factor(retTuple *ret) {
  ret->name = "3# largest prime factor";

  long rootNumber = 600851475143;
  int result = 0;

  while (rootNumber % 2 == 0) {
    result = 2;
    rootNumber /= 1;
  }

  for (int i = 3; i < sqrt(rootNumber); i += 2) {
    while (rootNumber % i == 0) {
      result = i;
      rootNumber /= i;
    }
  }

  if (rootNumber > 2) {
    result = rootNumber;
  }

  ret->result = result;
}

void reverse(char *x, int begin, int end)
{
  char c;

  if (begin >= end)
      return;

  c          = *(x+begin);
  *(x+begin) = *(x+end);
  *(x+end)   = c;

  reverse(x, ++begin, --end);
}

void largest_palindrome_product(retTuple *ret) {
  ret->name = "4# largest palindrome product";

  int result = 0;

  for (int i = 999; i > 100; i--) {
    for (int j = 999; j > 100; j--) {
      int product = i * j;
      char productString[(int)log10(product) + 1];
      sprintf(productString, "%ld", (long)product);
      reverse(productString, 0, strlen(productString)-1);

      int rev_product = atoi(productString);
      if (product == rev_product) {
        result = product;
        goto after;
      }
    }
  }
  after:

  ret->result = result;
}
void smallest_multiple(retTuple *ret) {
  ret->name = "5# smallest multiple";

  int result = 0;
  int n = 20;

  while (true) {
		bool dividable = true;
		for (int i = 1; i <= 20; i++) {
			if (n % i != 0) {
				dividable = false;
			}
		}
		if (dividable) {
			result = n;
			break;
		}
		n += 20;
	}
  ret->result = result;
}
void sum_square_difference(retTuple *ret) {
  ret->name = "6# sum square difference";
  int n = 100;
  int sum_square = 0;

	for (int i = 1; i <= n; i++) {
		sum_square += pow(i, 2);
	}

	int square_sum = 0;
	for (int i = 1; i <= n; i++) {
		square_sum += i;
	}

	square_sum = pow(square_sum, 2);

	int result = square_sum - sum_square;

	ret->result = result;
}
void _10001st_prime(retTuple *ret) {
  ret->name = "7# 10001st prime";
  
  int n = 10001;
	int i = 2;

	while (n > 0) {
		if (isPrime(i)) {
			
			n--;
		}
		i++;
	}
	i--;

  ret->result = i;
}
void largest_product_in_a_series(retTuple *ret) {

}
void special_pythagorean_triplet(retTuple *ret) {

}
void summation_of_primes(retTuple *ret) {

}