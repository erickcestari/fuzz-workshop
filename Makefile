all: fuzz_no_coverage fuzz_coverage fuzz_coverage_log fuzz_no_coverage_log

fuzz_coverage: fuzz_target.c
	clang -fsanitize=fuzzer -g fuzz_target.c -o fuzz_coverage

fuzz_coverage_log: fuzz_target.c
	clang -DLOG_FUZZER_INPUT -fsanitize=fuzzer -g fuzz_target.c -o fuzz_coverage_log

fuzz_no_coverage: fuzz_target.o
	clang fuzz_target.o -fsanitize=fuzzer -o fuzz_no_coverage

fuzz_target.o: fuzz_target.c
	clang -c fuzz_target.c -O2

fuzz_no_coverage_log: fuzz_target_log.o
	clang fuzz_target_log.o -fsanitize=fuzzer -o fuzz_no_coverage_log

fuzz_target_log.o: fuzz_target.c
	clang -DLOG_FUZZER_INPUT -c fuzz_target.c -o fuzz_target_log.o

clean:
	rm -f *.o *_coverage fuzz-* crash-* *_log