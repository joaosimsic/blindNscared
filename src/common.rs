pub const MAP_WIDTH: usize = 20;
pub const MAP_HEIGHT: usize = 20;

pub const TILE_FLOOR: char = ' ';
pub const TILE_WALL: char = '█';
pub const TILE_DOOR: char = '/';
pub const TILE_EXIT: char = 'X';

pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}
