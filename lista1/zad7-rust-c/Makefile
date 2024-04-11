# Compiler
CC = gcc

# Compiler flags
CFLAGS = -Wall -Wextra -Werror -g

# Libraries
LIBITER = ./libiter.a
LIBRECUR = ./librecur.a


# Default target
all: recur iter

recur: main.c librecursive.rlib
	$(CC) $(CFLAGS) -o $@ main.c -L../zad3-rust/recursive/target/release -llista1
	export LD_LIBRARY_PATH=../zad3-rust/recursive/target/release ./recur

iter: main.c libiterative.rlib
	$(CC) $(CFLAGS) -o $@ main.c -L../zad3-rust/iterative/target/release -llista1
	export LD_LIBRARY_PATH=../zad3-rust/iterative/target/release ./iter

libiterative.rlib:
	cd ../zad3-rust/iterative && cargo build --release && cd ../../zad7-rust-c/

librecursive.rlib:
	cd ../zad3-rust/recursive && cargo build --release && cd ../../zad7-rust-c/

clean:
	rm recur iter
