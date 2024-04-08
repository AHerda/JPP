# Compiler options
CC = gcc
CFLAGS = -Wall -Wextra

# Directory containing Ada files
ADADIR = ../zad2-ada

# Main target
all: iter recur

iter: $(ADADIR)/iterative.adb test.c
	gnatmake -c $(ADADIR)/iterative.adb
	gcc -c test.c
	gnatbind $(ADADIR)/iterative.ali test.o
	gnatlink test.o $(ADADIR)/iterative.ali

recur: $(ADADIR)/recursive.adb test.c
	gnatmake -c $(ADADIR)/recursive.adb
	gcc -c test.c
	gnatbind $(ADADIR)/recursive.ali test.o
	gnatlink test.o $(ADADIR)/recursive.ali

# Clean rule
clean:
	rm -f test *.o $(ADADIR)/*.ali $(ADADIR)/*.o $(ADADIR)/*.s
