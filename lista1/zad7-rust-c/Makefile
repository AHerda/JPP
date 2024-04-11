# Compiler
CC = gcc

# Compiler flags
CFLAGS = -Wall -Wextra -Werror -g

# Libraries
LIBRECUR = ../zad3-rust/recursive
LIBITER = ../zad3-rust/iterative


# Default target
all: recur iter

recur: main.c recursive
	$(CC) $(CFLAGS) -o $@ main.c -L$(LIBRECUR)/target/release -lrecursive -lpthread -ldl

iter: main.c iterative
	$(CC) $(CFLAGS) -o $@ main.c -L$(LIBITER)/target/release -literative -lpthread -ldl

iterative:
	cd ../zad3-rust/iterative && cargo build --release && cd ../../zad7-rust-c/

recursive:
	cd ../zad3-rust/recursive && cargo build --release && cd ../../zad7-rust-c/

clean:
	rm -rf recur iter $(LIBRECUR)/taget $(LIBITER)/target $(LIBRECUR)/Cargo.lock $(LIBITER)/Cargo.lock
