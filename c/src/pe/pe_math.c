#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "pe_math.h"

bool isPrime(int k) {
  if (k <= 1) {
    return false;
  }

  if (k == 2 || k == 3) {
    return true;
  }

  if (k % 2 == 0 || k % 3 == 0) {
    return false;
  }

  int i = 5;
  while (true) {
    if (i * i > k) {
      break;
    }

    if (k % i == 0 || k % (i + 2) == 0) {
      return false;
    }

    i += 6;
  }

  return true;
}

void math_add_str(char *a, char *b, char *dst) {

  bool a_negative = a[0] == '-' ? true : false;
  bool b_negative = b[0] == '-' ? true : false;
  if (a_negative)
    a++;
  if (b_negative)
    b++;

  int len_a = strlen(a);
  int len_b = strlen(b);

  int num_save = 0;

  if (!a_negative && !b_negative) {
    for (int i = 0; i < len_a > len_b ? len_a : len_b; i++) {
      
    }
  }

  fprintf(stdout, "str a: %s\nstrlen(a): %d\n-----\nstr b: %s\nstrlen(b): %d\n", a, len_a, b,
          len_b);
}