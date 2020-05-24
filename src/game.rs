extern crate piston_window;

use piston_window::*;

use crate::board;
use crate::entity;

use board::*;
use entity::*;

pub struct Game {
    pub player: Player,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        let player = Player {
            entity: Entity { 
                pos: board::Position{ x: 5, y: 5 },
                is_blocking: true,
            },
            max_hp:     100,
            current_hp: 100,
        };

        let board = Board{
            size: board::Size{ width: 20, height: 15 },
            scale: 30,
            entity_map: Vec::new(),
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
            Key::W => self.player.entity.move_dir(1, Direction::Up,    &mut self.board),
            Key::D => self.player.entity.move_dir(1, Direction::Right, &mut self.board),
            Key::S => self.player.entity.move_dir(1, Direction::Down,  &mut self.board),
            Key::A => self.player.entity.move_dir(1, Direction::Left,  &mut self.board),
            _ => {},
        }
    }

    pub fn on_render(&mut self, event: Event, window: &mut PistonWindow) {
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear screen.
            clear([1.0; 4], graphics);

            let player_sprite = [
                (self.player.entity.pos.x * self.board.scale) as f64,
                (self.player.entity.pos.y * self.board.scale) as f64,
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
