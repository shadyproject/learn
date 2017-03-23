#include <stdio.h>

int main(int argc, const char *argv[]) {
  int i = 0;

  while(i < argc) {
    printf("arg %d: %s\n", i, argv[i]);
    i++;
  }

  //our own array, again
  char *states[] = {
    "Oregon", "Minnesota", "Wisconsin", "New York"
  };

  int num_states = 4;
  i = 0; //watch for this
  while(i < num_states){
    printf("state %d: %s\n", states[i]);
  }

  return 0;
}
