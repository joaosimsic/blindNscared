#include "common.h"
#include "player.h"
#include "render.h"
#include "wfc.h"
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <time.h>
#include <unistd.h>

static struct termios orig_term;

static void disable_raw(void) {
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_term);
  printf("\033[?25h");
}

static void enable_raw(void) {
  struct termios raw;

  tcgetattr(STDIN_FILENO, &orig_term);
  raw = orig_term;
  raw.c_lflag &= ~(ECHO | ICANON);
  raw.c_cc[VMIN] = 1;
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
  printf("\033[?25l");
}

int main() {
  srand(time(NULL));

  init_map();
  while (wfc_step())
    ;

  init_player();
  enable_raw();
  atexit(disable_raw);

  while (1) {
    render_frame();
    char c = getchar();

    if (c == 'q')
      break;
    if (c == 'w')
      move_player(-1, 0);
    if (c == 's')
      move_player(1, 0);
    if (c == 'a')
      move_player(0, -1);
    if (c == 'd')
      move_player(0, 1);
  }

  return 0;
}
