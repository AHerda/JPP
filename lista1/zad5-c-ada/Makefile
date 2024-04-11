all: AdaWrapper.gpr iter recur
	gprbuild -p -P AdaWrapper.gpr

iter:
	cd src && gcc -c -fdump-ada-spec -C ./lib_iter.h -lm && cd ..

recur:
	cd src && gcc -c -fdump-ada-spec -C ./lib_recur.h -lm && cd ..

clean:
	rm -rf test_iter test_recur src/build src/*.ads
