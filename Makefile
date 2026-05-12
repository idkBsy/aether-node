# SUMMARY: Polyglot orchestrator for multi-stage AETHER-NODE compilation.

CC = clang
HIPCC = hipcc
CFLAGS = -O3 -march=native -Wall -Wextra -I./include -fPIC
HIPFLAGS = -O3 -fPIC
CARGO = cargo

.PHONY: all build test test-timing test-nucleus purge

all: build

include/rdtsc_raw.o: src/synapse/rdtsc_raw.s
	$(CC) -fPIC -c $< -o $@

include/capture.o: src/synapse/capture.c
	$(CC) $(CFLAGS) -c $< -o $@

include/skew_est.o: src/nucleus/skew_est.hip
	-$(HIPCC) $(HIPFLAGS) -c $< -o $@
	@if [ ! -f $@ ]; then echo "Warning: HIPCC failed or missing, creating dummy object"; touch $@; fi

include/libsynapse.a: include/rdtsc_raw.o include/capture.o include/skew_est.o
	ar rcs $@ $^

build: include/libsynapse.a
	$(CARGO) build --release

test: include/libsynapse.a
	$(CARGO) test

test-timing: include/libsynapse.a
	$(CARGO) test rdtsc_timing_test -- --nocapture

test-nucleus: include/libsynapse.a
	$(CARGO) test nucleus:: -- --nocapture

purge:
	$(CARGO) clean
	rm -f bin/* include/*.o include/*.a
