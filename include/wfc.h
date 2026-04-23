#ifndef WFC_H
#define WFC_H

#include "common.h"

typedef enum {
  TILE_VOID = 1 << 0,
  TILE_FLOOR = 1 << 1,
  TILE_WALL = 1 << 2,
  TILE_ALTAR = 1 << 3
} TileType;

#define ALL_TILES (TILE_VOID | TILE_FLOOR | TILE_WALL | TILE_ALTAR)

typedef struct {
  unsigned int entropy;
  bool collapsed;
  TileType final_type;
} Cell;

extern Cell map[MAP_HEIGHT][MAP_WIDTH];

void init_map(void);
unsigned int get_allowed_neighbors(TileType conter_tile);
int count_possibilities(unsigned int entropy);

#endif // WFC_H
