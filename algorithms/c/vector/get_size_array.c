#include <stdio.h>
int main() {
    int faveNumbers[] = {7, 33, 13, 9, 29};

    size_t size = sizeof(faveNumbers) / sizeof(faveNumbers[0]);

    printf("The length of the array is %d \n", size);
}
