CFLAGS = -O3 -Wall -Wextra -Wno-unused-parameter -pthread

.PHONY: clean prepare test

clean:
	sh tests/clean.sh

prepare: clean
	sh tests/setup.sh

test: prepare
	sh tests/run.sh
	make clean
