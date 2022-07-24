#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "main.h"
#include "pe/pe_math.h"
#include "pe/pe_string.h"

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

  ret->result = (long)result;
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

  ret->result = (long)result;
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

  ret->result = (long)result;
}

void largest_palindrome_product(retTuple *ret) {
  ret->name = "4# largest palindrome product";

  int result = 0;

  for (int i = 100; i < 999; i++) {
    for (int j = 100; j < 999; j++) {
      int product = i * j;
      char productString[(int)log10(product) + 1];
      sprintf(productString, "%d", product);
      string_reverse(productString, 0, strlen(productString) - 1);

      int rev_product = atoi(productString);
      if (product == rev_product && product > result) {
        result = product;
      }
    }
  }

  ret->result = (long)result;
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
  ret->result = (long)result;
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

  ret->result = (long)result;
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

  ret->result = (long)i;
}
void largest_product_in_a_series(retTuple *ret) {
  ret->name = "8# largest product in a series";

  char *numbers = "73167176531330624919225119674426574742355349194934"
                  "96983520312774506326239578318016984801869478851843"
                  "85861560789112949495459501737958331952853208805511"
                  "12540698747158523863050715693290963295227443043557"
                  "66896648950445244523161731856403098711121722383113"
                  "62229893423380308135336276614282806444486645238749"
                  "30358907296290491560440772390713810515859307960866"
                  "70172427121883998797908792274921901699720888093776"
                  "65727333001053367881220235421809751254540594752243"
                  "52584907711670556013604839586446706324415722155397"
                  "53697817977846174064955149290862569321978468622482"
                  "83972241375657056057490261407972968652414535100474"
                  "82166370484403199890008895243450658541227588666881"
                  "16427171479924442928230863465674813919123162824586"
                  "17866458359124566529476545682848912883142607690042"
                  "24219022671055626321111109370544217506941658960408"
                  "07198403850962455444362981230987879927244284909188"
                  "84580156166097919133875499200524063689912560717606"
                  "05886116467109405077541002256983155200055935729725"
                  "71636269561882670428252483600823257530420752963450";

  long result = 0;

  const int n = 13;

  for (int i = 0; i < 999 - n; i++) {
    long product = 1;
    for (int j = 0; j < n; j++) {
      product *= numbers[i + j] - '0';
    }
    if (product > result) {
      result = product;
    }
  }
  printf("\n");

  ret->result = result;
}
void special_pythagorean_triplet(retTuple *ret) {
  ret->name = "9# special pythagorean triplet";

  int result = 0;
  int a = 0;
  int b = 0;
  int c = 0;

cLoop:
  while (true) {
    c++;
  bLoop:
    while (true) {
      b++;
    aLoop:
      while (true) {
        a++;
        if (a >= b) {
          a = 1;
          goto bLoop;
        }
        if (b >= c) {
          b = 1;
          goto cLoop;
        }

        if (pow(a, 2) + pow(b, 2) != pow(c, 2)) {
          goto aLoop;
        }

        if (a + b + c == 1000) {
          result = a * b * c;
          goto after;
        }
      }
    }
  }
after:

  ret->result = (long)result;
}
void summation_of_primes(retTuple *ret) {
  ret->name = "10# summation of primes";

  int result = 0;

  for (int i = 0; i < 2000000; i++) {
    if (isPrime(i))
      result += i;
  }

  ret->result = (long)result;
}