CC = gcc
CFLAGS = -Wall -g
LIBS = -lm

all: tester_rec tester_iter

tester_rec: tester.c lib_recur.c lib.h
	$(CC) $(CFLAGS) -o $@ $^ $(LIBS)

tester_iter: tester.c lib_iter.c lib.h
	$(CC) $(CFLAGS) -o $@ $^ $(LIBS)

clean:
	rm -f tester_rec tester_iter
