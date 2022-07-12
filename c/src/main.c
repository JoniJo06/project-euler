#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "main.h"

void (*problems[1])(retTuple *ret) = {
  multiples_of_3_or_5,
  };

int main(int argc, char **argv) {
  const char *program = argv[0];
  argv++;
  argc--;

  if (argc < 1) {
    fprintf(stderr, "usage: %s <problem number>\n", program);
    // std::cerr << "Usage: " << program << " <problem number>" << std::endl;
    exit(EXIT_FAILURE);
  }

  int problem = atoi(argv[0]);

  if (problem <= 0) {
    fprintf(stderr, "usage: %s <problem number>\n", program);
    exit(EXIT_FAILURE);
  }

  if ((unsigned long) problem > sizeof(problems)/sizeof(problems[0])) {
    // fprintf(stderr, "usage: %s <problem number>\n", program);
    fprintf(stderr, "problem out of range\n");
    fprintf(stderr, "try to access problem %d of %ld\n", problem, sizeof(problems)/sizeof(problems[0]));
    exit(EXIT_FAILURE);
  }

  retTuple tuple;
  problems[problem - 1](&tuple);

  printf("--------------------------------\n");
  printf("%s\n", tuple.name);
  printf("Result: %ld\n", tuple.result);
  printf("--------------------------------\n");

}