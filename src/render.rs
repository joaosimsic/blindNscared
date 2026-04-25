use crate::common::{MAP_HEIGHT, MAP_WIDTH, TILE_CORRIDOR, TILE_DOOR, TILE_EXIT, TILE_FLOOR, TILE_WALL};
use crate::dungeon::World;
use crate::player::Player;
use crossterm::terminal;
use std::io::{self, Write};

pub fn render_frame(world: &World, player: &Player) -> io::Result<()> {
    let (cols, rows) = terminal::size().unwrap_or((80, 24));
    let oy = (rows as i32 - MAP_HEIGHT as i32) / 2;
    let ox = (cols as i32 - (MAP_WIDTH * 2) as i32) / 2;

    let mut out = io::stdout().lock();
    write!(out, "\x1b[2J")?;

    for y in 0..MAP_HEIGHT {
        write!(out, "\x1b[{};{}H", oy + y as i32 + 1, ox + 1)?;
        for x in 0..MAP_WIDTH {
            if player.y == y && player.x == x {
                write!(out, "\x1b[38;5;226m@ \x1b[0m")?;
            } else {
                let tile = world.map[y][x];
                let (symbol, color) = match tile {
                    c if c == TILE_FLOOR => ("· ", 82),
                    c if c == TILE_CORRIDOR => ("· ", 214),
                    c if c == TILE_WALL => ("██", 196),
                    c if c == TILE_EXIT => ("X ", 226),
                    c if c == TILE_DOOR => ("/ ", 208),
                    _ => ("? ", 255),
                };
                write!(out, "\x1b[38;5;{}m{}\x1b[0m", color, symbol)?;
            }
        }
    }

    out.flush()
}
