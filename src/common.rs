pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 40;

pub const TILE_FLOOR: char = ' ';
pub const TILE_CORRIDOR: char = ',';
pub const TILE_WALL: char = '█';
pub const TILE_DOOR: char = '/';
pub const TILE_EXIT: char = 'X';

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}
