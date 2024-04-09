# Compiler options
CC = gcc
CFLAGS = -Wall -Wextra

# Directory containing Ada files
ADADIR = ../zad2-ada

# Main target
all: test

test: test.o iterative.o recursive.o
	$(CC) $(CFLAGS) -o $@ $^ -lgnat

test.o: test.c
	$(CC) $(CFLAGS) -c $^

iterative.o: $(ADADIR)/iterative.adb
	gnatmake -c $^

recursive.o: $(ADADIR)/recursive.adb
	gnatmake -c $^

# Clean rule
clean:
	rm -f test *.o *.ali
