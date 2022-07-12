#include "main.h"

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
  ret->name = "even fibonacci numbers";
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