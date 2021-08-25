#include <stdio.h>

int main(void) {
    unsigned long y = 0x123457 & 0xfffffffffffffffe;
    printf("y: %lx\n", y);
    return 0;
}