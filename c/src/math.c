#include "math.h"

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