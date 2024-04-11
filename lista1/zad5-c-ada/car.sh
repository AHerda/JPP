#!/usr/bin/env bash

cd src
gcc -c -fdump-ada-spec -C ./lib_iter.h -lm
gcc -c -fdump-ada-spec -C ./lib_recur.h -lm

cd ..
gprbuild -p -P AdaWrapper.gpr
./test_iter
./test_recur
