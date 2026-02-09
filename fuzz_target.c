#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int LLVMFuzzerTestOneInput(const char *data, size_t size) {
    if (size != 12) {
        return 0;
    }

    #ifdef LOG_FUZZER_INPUT
    printf("%s\n", data);
    #endif
    
    if (memcmp(data, "FUZZ", 4) != 0) return 0;

    if (data[4] != 'F' || data[5] != 'I' || 
        data[6] != 'N' || data[7] != 'D') {
        return 0;
    }

    if (data[8] != 'B' || data[9] != 'U' || 
        data[10] != 'G' || data[11] != 'S') {
        return 0;
    }
    
    abort();
}