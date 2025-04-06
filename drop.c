#include <stdio.h>
#include <stdlib.h>
#include <string.h>
const char *skip_first_word(const char *str) {
  while (*str && *str != ' ') {
    str++;
  }

  return str;
}

int main() {
  char *sentence = strdup("Hello world!");
  const char *skipped = skip_first_word(sentence);

  free(sentence);

  printf("%s", skipped);
}