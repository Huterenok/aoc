#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 101
#define LINE_COUNT 100
#define STEADY 100
#define CYCLES 1000000000

// Sorry, im retarded :(

int tiltNorth(char* input) {
  int res = 0;

  for (int i = 0; i < LINE_COUNT; i++) {
    for (int j = 0; j < LINE_LENGTH - 1; j++) {
      if (input[i * LINE_LENGTH + j] == 'O') {

        int count = 0;

        while (i - count - 1 >= 0 &&
               input[(i - count - 1) * LINE_LENGTH + j] == '.') {
          count += 1;
        }

        if (count != 0) {
          input[i * LINE_LENGTH + j] = '.';
          input[(i - count) * LINE_LENGTH + j] = 'O';
        }
        res += LINE_COUNT - i + count;
      }
    }
  }

  return res;
}

int tiltSouth(char* input) {
  int res = 0;

  for (int i = LINE_COUNT - 1; i >= 0; i--) {
    for (int j = 0; j < LINE_LENGTH - 1; j++) {
      if (input[i * LINE_LENGTH + j] == 'O') {
        int count = 0;
        while (i + count + 1 < LINE_COUNT &&
               input[(i + count + 1) * LINE_LENGTH + j] == '.') {
          count += 1;
        }

        if (count != 0) {
          input[i * LINE_LENGTH + j] = '.';
          input[(i + count) * LINE_LENGTH + j] = 'O';
        }
        res += LINE_COUNT - i + count;
      }
    }
  }

  return res;
}

int tiltWest(char* input) {
  int res = 0;

  for (int i = 0; i < LINE_LENGTH - 1; i++) {
    for (int j = 0; j < LINE_COUNT; j++) {
      if (input[j * LINE_LENGTH + i] == 'O') {

        int count = 0;

        while (i - count - 1 >= 0 &&
               input[j * LINE_LENGTH + i - count - 1] == '.') {
          count += 1;
        }

        if (count != 0) {
          input[j * LINE_LENGTH + i] = '.';
          input[j * LINE_LENGTH + i - count] = 'O';
        }
        res += LINE_LENGTH - i + count;
      }
    }
  }

  return res;
}

int tiltEast(char* input) {
  int res = 0;

  for (int i = LINE_LENGTH - 1; i >= 0; i--) {
    for (int j = 0; j < LINE_COUNT; j++) {
      if (input[j * LINE_LENGTH + i] == 'O') {

        int count = 0;

        while (i + count + 1 < LINE_LENGTH - 1 &&
               input[j * LINE_LENGTH + i + count + 1] == '.') {
          count += 1;
        }

        if (count != 0) {
          input[j * LINE_LENGTH + i] = '.';
          input[j * LINE_LENGTH + i + count] = 'O';
        }

        res += LINE_LENGTH - i + count;
      }
    }
  }

  return res;
}

void cycle(char* input) {
  tiltNorth(input);
  tiltWest(input);
  tiltSouth(input);
  tiltEast(input);
}

char* readInput(FILE* fptr) {
  rewind(fptr);
  char* res = malloc(LINE_LENGTH * LINE_COUNT);

  char buffer[LINE_LENGTH];
  while (fgets(buffer, LINE_LENGTH, fptr)) {
    strcat(res, buffer);
  }

  return res;
}

int load(char* plate) {
  int sum = 0;
  for (int i = 0; i < LINE_COUNT; ++i) {
    const int val = LINE_COUNT - i;
    for (int j = 0; j < LINE_LENGTH - 1; ++j)
      sum += (plate[i * LINE_LENGTH + j] == 'O') * val;
  }
  return sum;
}

int main() {
  FILE* fptr = fopen("input.txt", "r");
  char* input = readInput(fptr);

  int res1 = tiltNorth(input);
  printf("Result 1 - %d\n", res1);

  for (int i = 0; i < STEADY; i++) {
    cycle(input);
  }
  int ref = load(input);

  int count = 0;
  do {
    cycle(input);
    ++count;
  } while (load(input) != ref);

  int mod = (CYCLES - STEADY) % count;
  for (int i = 0; i < mod; ++i)
    cycle(input);

  printf("Result 2 - %d\n", load(input));

  free(input);
  fclose(fptr);
  return 0;
}