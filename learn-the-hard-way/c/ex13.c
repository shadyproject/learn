#include <stdio.h>

int main(int argc, const char *argv[]) {
  if (2 != argc) {
    printf("ERROR: You need one argument\n");
    return 1;
  }

  int i = 0;
  for (i = 0; i < argv[1][i]; i++) {
    char letter = argv[1][i];

    switch (letter) {
      case 'a':
      case 'A':
        printf("%d: 'A'\n", i);
        break;

      case 'e':
      case 'E':
        printf("%d: 'E'\n", i);
        break;

      case 'i':
      case 'I':
        printf("%d: 'I'\n", i);
        break;

      case 'o':
      case 'O':
        printf("%d: 'O'\n", i);
        break;

      case 'u':
      case 'U':
        printf("%d: 'U'\n", i);
        break;

      case 'y':
      case 'Y':
        if (i > 2) { //y is only a vowel sometimes
          printf("%d: 'Y'\n", i);
        }
        break;

      default:
        printf("%d: %c is not a vowel\n", i, letter);
    }
  }
  return 0;
}
