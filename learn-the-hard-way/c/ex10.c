#include <stdio.h>

int main(int argc, const char *argv[]) {
  int i = 0;

  //loop over each string in argv
  //we skip argv 0 because its the name of the program
  for (i = 1; i < argc; i++) {
    printf("arg %d: %s\n", i, argv[i]);
  }

  //make our own array of strings
  char *states[] = {
    "Oregon", "Minnesota", "Florida", "New York"
  };

  int num_states = 4;

  for (i = 0; i < num_states; i++) {
    printf("state %d: %s\n", i, states[i]);
  }
  return 0;
}
