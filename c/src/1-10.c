#include "main.h"

void multiples_of_3_or_5(retTuple *ret) {
  ret->name = "1# multiples_of_3_or_5";

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