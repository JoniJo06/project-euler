CFLAGS=-Wall -Wextra -std=c17 -pedantic -ggdb
LIBS=-lm -lgmp

project-euler: src/main.c
	$(CC) $(CFLAGS) -o project-euler src/* $(LIBS)

clean:
	rm -f project-euler