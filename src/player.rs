use crate::common::{MAP_HEIGHT, MAP_WIDTH, TILE_EXIT, TILE_FLOOR, TILE_WALL};
use crate::dungeon::World;

pub struct Player {
    pub y: usize,
    pub x: usize,
}

impl Player {
    pub fn spawn(world: &World) -> Self {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if world.map[y][x] == TILE_FLOOR {
                    return Player { y, x };
                }
            }
        }
        Player {
            y: MAP_HEIGHT / 2,
            x: MAP_WIDTH / 2,
        }
    }

    pub fn try_move(&mut self, world: &World, dy: isize, dx: isize) -> bool {
        let ny = self.y as isize + dy;
        let nx = self.x as isize + dx;
        if ny < 0 || ny >= MAP_HEIGHT as isize || nx < 0 || nx >= MAP_WIDTH as isize {
            return false;
        }
        let ny = ny as usize;
        let nx = nx as usize;

        let tile = world.map[ny][nx];
        if tile == TILE_WALL || tile == TILE_EXIT {
            return false;
        }

        self.y = ny;
        self.x = nx;
        true
    }
}
