#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#define TRUE 1
#define FALSE -1
#define MAX_VET 100001

int main()
{
  clock_t begin = clock();

  /* here, do your time-consuming job */

  clock_t end = clock();
  double time_spent = (double)(end - begin) / CLOCKS_PER_SEC;
  printf("time_spent: %f\n", time_spent);

  return 0;
}