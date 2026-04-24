#ifndef WFC_H
#define WFC_H

#include "common.h"

#define TILE_IDX_VOID 0
#define TILE_IDX_FLOOR 1
#define TILE_IDX_WALL 2
#define TILE_IDX_ALTAR 3
#define NUM_TILES 4

typedef enum {
  TILE_VOID = 1 << 0,
  TILE_FLOOR = 1 << 1,
  TILE_WALL = 1 << 2,
  TILE_ALTAR = 1 << 3
} TileType;

#define ALL_TILES (TILE_VOID | TILE_FLOOR | TILE_WALL | TILE_ALTAR)

typedef struct {
  bool possible[NUM_TILES];
  bool collapsed;
  TileType final_type;
} Cell;

extern Cell map[MAP_HEIGHT][MAP_WIDTH];

typedef struct {
  int y, x;
} Coord;

extern Coord uncollapsed[MAP_HEIGHT * MAP_WIDTH];

extern int uncollapsed_count;

void init_map(void);
unsigned int get_allowed_neighbors(TileType center_tile);
int count_possibilities(bool possible[NUM_TILES]);
bool wfc_step(void);
void collapse_cell(int y, int x);
void propagate(int y, int x);

#endif // WFC_H
