coverage:
  ./fuzz_coverage -print_final_stats=1 -print_pcs=1

coverage_log:
  ./fuzz_coverage_log -print_final_stats=1 -print_pcs=1

no_coverage:
  ./fuzz_no_coverage -print_final_stats=1 -print_pcs=1

no_coverage_log:
  ./fuzz_no_coverage_log -print_final_stats=1 -print_pcs=1

dictionary:
  ./fuzz_coverage -runs=3000000

arbitrary:
  cd arbitrary-fuzz-target && cargo hfuzz run arbitrary-fuzz-target