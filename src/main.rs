mod common;
mod dungeon;
mod player;
mod render;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use dungeon::World;
use player::Player;
use std::io;

use crate::common::{MAP_HEIGHT, MAP_WIDTH};

struct RawGuard;

impl RawGuard {
    fn enable() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        print!("\x1b[?25l");
        Ok(RawGuard)
    }
}

impl Drop for RawGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        print!("\x1b[?25h");
    }
}

fn main() -> io::Result<()> {
    let mut world = World::new(MAP_WIDTH, MAP_HEIGHT);
    world.generate();

    let mut player = Player::spawn(&world);

    let _guard = RawGuard::enable()?;

    loop {
        render::render_frame(&world, &player)?;

        if let Event::Key(k) = event::read()? {
            match k.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('w') => {
                    player.try_move(&world, -1, 0);
                }
                KeyCode::Char('s') => {
                    player.try_move(&world, 1, 0);
                }
                KeyCode::Char('a') => {
                    player.try_move(&world, 0, -1);
                }
                KeyCode::Char('d') => {
                    player.try_move(&world, 0, 1);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
