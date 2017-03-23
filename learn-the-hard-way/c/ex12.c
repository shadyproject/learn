#include <stdio.h>

int main(int argc, const char *argv[]) {
  int i = 0;

  if (1 == argc) {
    printf("You only have one argument, you suck.\n");
  } else if (argc > 1 && argc < 4) {
    printf("Here are your arguments:\n");
    for (i = 0; i < argc; i++){
      printf("%s ", argv[i]);
    }
    printf("\n");
  } else {
    printf("You have too may arguments, you suck.\n");
  }
  return 0;

}
