#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "pe_math.h"
#include "pe_string.h"

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

char *math_add_str(char *a, char *b) {

  char *buffer = malloc(MATH_STRING_LIMIT * 2);

  bool a_negative = a[0] == '-' ? true : false;
  bool b_negative = b[0] == '-' ? true : false;
  if (a_negative)
    a++;
  if (b_negative)
    b++;

  int len_a = strlen(a);
  int len_b = strlen(b);

  char a_reverse[MATH_STRING_LIMIT];
  strcpy(a_reverse, a);
  char b_reverse[MATH_STRING_LIMIT];
  strcpy(b_reverse, b);

  string_reverse(a_reverse, 0, len_a - 1);
  string_reverse(b_reverse, 0, len_b - 1);

  int num_save = 0;
  int position = 0;

  int largest_string = len_a > len_b ? len_a : len_b;

  if (a_negative || b_negative) {
    fprintf(stderr, "for %s is not for negative numbers implemented yet\n", __func__);
    exit(EXIT_FAILURE);
  }
  // only implemented for positiv numbers
  if (!a_negative && !b_negative) {
    while (true) {
      if (position < largest_string) {
        int num1 = position < len_a ? a_reverse[position] - '0' : 0;
        int num2 = position < len_b ? b_reverse[position] - '0' : 0;
        // fprintf(stdout, "---------\nnum1: %d\n", num1);
        // fprintf(stdout, "num2: %d\n", num2);
        int result = num1 + num2 + num_save;
        if (result >= 10) {
          num_save = 1;
          buffer[position] = (result % 10) + '0';
        } else {
          num_save = 0;
          buffer[position] = (result % 10) + '0';
        }
      } else if ((position >= largest_string && num_save > 0)) {
        buffer[position] = num_save + '0';
        num_save = 0;
      } else {
        break;
      }
      position++;
    }
  }

  buffer[position] = '\0';

  // fprintf(stdout, "strlen(buffer): %d\n", position);

  // fprintf(stdout, "%s\n", buffer);

  string_reverse(buffer, 0, strlen(buffer) - 1);
  // fprintf(stdout, "%s\n", buffer);

  return buffer;

  // fprintf(stdout, "str a: %s\nstrlen(a): %d\n-----\nstr b: %s\nstrlen(b): %d\n", a_reverse, len_a,
          // b_reverse, len_b);
}