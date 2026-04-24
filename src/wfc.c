#include "wfc.h"
#include "common.h"

Cell map[MAP_HEIGHT][MAP_WIDTH];

void init_map(void) {
  const Cell default_cell = {.possible = {true, true, true, true},
                             .collapsed = false,
                             .final_type = 0};

  Cell *ptr = &map[0][0];

  int total_cells = MAP_HEIGHT * MAP_WIDTH;

  for (int i = 0; i < total_cells; i++) {
    ptr[i] = default_cell;
  }

  uncollapsed_count = 0;

  for (int y = 0; y < MAP_HEIGHT; y++) {
    for (int x = 0; x < MAP_WIDTH; x++) {
      uncollapsed[uncollapsed_count].y = y;
      uncollapsed[uncollapsed_count].x = x;
      uncollapsed_count++;
    }
  }
}

unsigned int get_allowed_neighbors(TileType center_tile) {
  switch (center_tile) {
  case TILE_ALTAR:
    return TILE_FLOOR;
  case TILE_FLOOR:
    return TILE_WALL | TILE_FLOOR | TILE_ALTAR;
  case TILE_WALL:
    return TILE_VOID | TILE_FLOOR | TILE_WALL;
  case TILE_VOID:
    return TILE_VOID | TILE_WALL;
  default:
    return 0;
  }
}

int count_possibilities(bool possible[NUM_TILES]) {
  int count = 0;

  for (int i = 0; i < NUM_TILES; i++) {
    if (possible[i])
      count++;
  }

  return count;
}

static TileType pick_random_tile(bool possible[NUM_TILES]) {
  int count = count_possibilities(possible);

  if (count == 0)
    return 0;

  int choice = rand() % count;

  int current = 0;

  for (int i = 0; i < NUM_TILES; i++) {
    if (possible[i]) {
      if (current == choice)
        return (1 << i);
      current++;
    }
  }

  return 0;
}

void collapse_cell(int y, int x) {
  TileType chosen = pick_random_tile(map[y][x].possible);

  map[y][x].collapsed = true;
  map[y][x].final_type = chosen;

  for (int i = 0; i < NUM_TILES; i++) {
    map[y][x].possible[i] = (chosen & (1 << i)) != 0;
  }

  for (int i = 0; i < uncollapsed_count; i++) {
    if (uncollapsed[i].y == y && uncollapsed[i].x == x) {
      uncollapsed[i] = uncollapsed[uncollapsed_count - 1];
      uncollapsed_count--;
      break;
    }
  }
}

void propagate(int y, int x) {
  int dy[] = {-1, 1, 0, 0};
  int dx[] = {0, 0, -1, 1};

  for (int dir = 0; dir < 4; dir++) {
    int ny = y + dy[dir];
    int nx = x + dx[dir];

    if (ny < 0 || ny >= MAP_HEIGHT || nx < 0 || nx >= MAP_WIDTH)
      continue;

    if (map[ny][nx].collapsed)
      continue;

    unsigned int allowed = get_allowed_neighbors(map[y][x].final_type);

    bool changed = false;

    for (int i = 0; i < NUM_TILES; i++) {
      TileType tile_bit = (1 << i);

      if (map[ny][nx].possible[i] && !(allowed & tile_bit)) {
        map[ny][nx].possible[i] = false;
        changed = true;
      }
    }

    if (changed && count_possibilities(map[ny][nx].possible) == 1) {
      collapse_cell(ny, nx);
      propagate(ny, nx);
    }
  }
}

bool wfc_step(void) {
  if (uncollapsed_count == 0)
    return false;

  int min_entropy = NUM_TILES + 1;
  int best_idx = -1;

  for (int i = 0; i < uncollapsed_count; i++) {
    int y = uncollapsed[i].y;
    int x = uncollapsed[i].x;

    int count = count_possibilities(map[y][x].possible);

    if (count > 0 && count < min_entropy) {
      min_entropy = count;
      best_idx = i;
    }
  }

  int y = uncollapsed[best_idx].y;
  int x = uncollapsed[best_idx].x;

  collapse_cell(y, x);
  propagate(y, x);

  return true;
}
