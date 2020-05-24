extern crate piston_window;

use piston_window::*;

use crate::geo;
use crate::entity;

use geo::*;
use entity::*;

pub struct Game {
    pub player: Entity,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        let player = Entity {
            pos:        geo::Position{ x: 5, y: 5 },
            size:       geo::Size{ width: 30, height: 20 },
            max_hp:     100,
            current_hp: 100,
        };

        let board = Board{
            size: geo::Size{ width: 20, height: 15 },
            scale: 30
        };

        Game {
            player: player,
            board: board,
        }
    }

    fn on_update(&mut self) {
    }
    
    pub fn on_input(&mut self, key: Key) {
        match key {
            Key::W => self.player.move_dir(1, Direction::Up,    &self.board),
            Key::D => self.player.move_dir(1, Direction::Right, &self.board),
            Key::S => self.player.move_dir(1, Direction::Down,  &self.board),
            Key::A => self.player.move_dir(1, Direction::Left,  &self.board),
            _ => {},
        }
    }

    pub fn on_render(&mut self, event: Event, window: &mut PistonWindow) {
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear screen.
            clear([1.0; 4], graphics);

            let player_sprite = [
                (self.player.pos.x * self.board.scale) as f64,
                (self.player.pos.y * self.board.scale) as f64,
                self.board.scale as f64,
                self.board.scale as f64,
            ];

            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                player_sprite,
                context.transform,
                graphics
            );
        });
    }
}
